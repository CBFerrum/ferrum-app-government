//! Permits. An application is submitted with every document its type needs, and its fee, if any,
//! is invoiced through ERP to a customer made from the applicant. An officer takes it up; when its
//! type has review stages, each is passed in order by someone holding the stage's role, never by
//! whoever passed the stage before, and a site inspection says what it found. It is approved once
//! every stage is passed and the fee is paid, on conditions if any, or rejected saying why.
//! Approved, it is issued as a permit valid for its type's months; a permit may be revoked, saying
//! why, and expires by itself. A renewable permit is renewed, from its type's days before it
//! expires, by an application of its own; the permit issued for it starts the day after the old one
//! ends. The applicant is told as their application moves, and the holder when a permit may be
//! renewed. Once submitted, an application keeps what was applied for. Days are the workspace's.

use crate::obj;
use async_trait::async_trait;
use chrono::{Duration, Months, NaiveDate};
use ferrum_domain::flow::created;
use ferrum_domain::util::{d, elevated, id, money, rows, s, sys};
use ferrum_kernel::{Ctx, Doc, DocHook, Error, Filter, HookCtx, Kernel, Result, Tx, notify};
use rust_decimal::Decimal;
use serde_json::{Value, json};
use std::collections::HashSet;
use uuid::Uuid;

pub struct ApplicationHook;

/// What was applied for: the type, the applicant, the details and the documents.
fn applied(doc: &Doc) -> (Option<String>, Option<String>, Option<String>, Option<String>, Vec<(Option<String>, Option<String>)>) {
    let docs = rows(doc, "documents").iter().map(|r| (s(r, "document").map(str::to_string), s(r, "file").map(str::to_string))).collect();
    (
        s(doc, "permit_type").map(str::to_string),
        s(doc, "applicant_name").map(str::to_string),
        s(doc, "applicant_email").map(str::to_string),
        s(doc, "details").map(str::to_string),
        docs,
    )
}

fn note(params: &Value) -> Option<String> {
    params.get("note").and_then(Value::as_str).map(str::trim).filter(|n| !n.is_empty()).map(str::to_string)
}

/// The notification kind applicants and holders get (`shell.notification` `application`).
const TOLD: &str = "government.application";

/// Tells the person with `email`, when they have an account here, what became of their application
/// or permit, the way they chose (by e-mail unless they chose otherwise).
pub(crate) async fn tell(tx: &mut Tx, tenant: Uuid, email: Option<&str>, title: &str, body: Option<&str>, (entity, record): (&str, Uuid)) -> Result<()> {
    let Some(email) = email.map(str::to_lowercase).filter(|e| !e.is_empty()) else { return Ok(()) };
    let person: Option<Uuid> = sqlx::query_scalar("select id from app_user where tenant_id = $1 and lower(email) = $2 and is_active limit 1")
        .bind(tenant)
        .bind(email)
        .fetch_optional(&mut **tx)
        .await?;
    if let Some(person) = person {
        notify::send_kind(tx, tenant, person, TOLD, "email", title, body, Some(entity), Some(record)).await?;
    }
    Ok(())
}

/// A type's review stages, in order: (stage, role, inspection).
fn stages(kind: &Doc) -> Vec<(String, Option<String>, bool)> {
    rows(kind, "stages")
        .iter()
        .filter_map(|r| {
            Some((
                s(r, "stage")?.to_string(),
                s(r, "role").map(str::trim).filter(|x| !x.is_empty()).map(str::to_string),
                r.get("inspection").and_then(Value::as_bool).unwrap_or(false),
            ))
        })
        .collect()
}

/// A review row as its table keeps it: its fields and id only.
fn review_row(r: &Doc) -> Doc {
    ["id", "stage", "outcome", "reviewer", "reviewed_on", "note", "findings"].iter().filter_map(|k| r.get(*k).map(|v| (k.to_string(), v.clone()))).collect()
}

/// When a permit expiring on `expires` may be renewed, given its type's `within` days.
pub(crate) fn renewable_from(expires: NaiveDate, within: i64) -> NaiveDate {
    expires - Duration::days(within.max(0))
}

/// A renewal of `permit`, as of `today`: an application of its own for its holder, a draft to
/// complete and submit, refused for a type that does not renew, before its window, or while
/// another renewal of it is live.
pub(crate) async fn renew(k: &Kernel, ctx: &Ctx, tx: &mut Tx, permit: &Doc, today: NaiveDate) -> Result<Doc> {
    let me = id(permit, "id").ok_or_else(|| Error::internal("a permit without an id"))?;
    if !matches!(s(permit, "status"), Some("Valid" | "Expired")) {
        return Err(Error::invalid("only a valid or expired permit is renewed"));
    }
    let kind = k.get_tx(ctx, tx, "PermitType", id(permit, "permit_type").ok_or_else(|| Error::internal("a permit without its type"))?, false).await?;
    if kind.get("renewable").and_then(Value::as_bool) != Some(true) {
        return Err(Error::invalid("permits of this type are not renewed"));
    }
    let expires =
        s(permit, "expires_on").and_then(|v| NaiveDate::parse_from_str(v, "%Y-%m-%d").ok()).ok_or_else(|| Error::internal("a permit without its end"))?;
    let from = renewable_from(expires, kind.get("renew_within_days").and_then(Value::as_i64).unwrap_or(60));
    if today < from {
        return Err(Error::invalid(format!("it may be renewed from {from}")));
    }
    if let Some(app) = id(permit, "renewal") {
        let live: Option<String> =
            sqlx::query_scalar("select number from e_permit_application where tenant_id = $1 and id = $2 and status not in ('Rejected', 'Withdrawn')")
                .bind(ctx.tenant)
                .bind(app)
                .fetch_optional(&mut **tx)
                .await?
                .flatten();
        if let Some(n) = live {
            return Err(Error::invalid(format!("its renewal is {n}")));
        }
    }
    let made = k
        .insert_tx(
            ctx,
            tx,
            "PermitApplication",
            obj(json!({"permit_type": s(permit, "permit_type"), "applicant_name": s(permit, "holder_name"), "applicant_email": s(permit, "holder_email"),
                       "details": format!("Renewal of {}", s(permit, "number").unwrap_or_default()), "renewal_of": me.to_string()})),
        )
        .await?;
    let app = id(&made, "id").ok_or_else(|| Error::internal("an application without an id"))?;
    k.set_values_tx(ctx, tx, "Permit", me, obj(json!({"renewal": app.to_string()}))).await?;
    Ok(made)
}

#[async_trait]
impl DocHook for ApplicationHook {
    fn name(&self) -> &'static str {
        "government.application"
    }

    async fn validate_change(&self, hc: &mut HookCtx<'_>, old: Option<&Doc>, doc: &Doc) -> Result<()> {
        if hc.ctx.is_system {
            return Ok(());
        }
        if let Some(o) = old
            && s(o, "status") != Some("Draft")
            && applied(o) != applied(doc)
        {
            return Err(Error::invalid("a submitted application keeps what was applied for"));
        }
        Ok(())
    }

    async fn action(&self, hc: &mut HookCtx<'_>, name: &str, doc: &Doc, params: &Value) -> Result<Option<Value>> {
        let application = id(doc, "id").ok_or_else(|| Error::internal("an application without an id"))?;
        let system = elevated(hc.ctx);
        let today = hc.ctx.today();
        let number = s(doc, "number").unwrap_or_default().to_string();
        let mut out = json!({});
        let values = match name {
            "submit_application" => {
                let (fee_invoice, customer) = submit(hc.kernel, &system, hc.tx, doc).await?;
                if let Some(inv) = &fee_invoice {
                    out["fee_invoice"] = created("SalesInvoice", inv);
                }
                let fee = if fee_invoice.is_some() { Some("Its fee is invoiced: it is reviewed once paid.") } else { None };
                tell(
                    hc.tx,
                    hc.ctx.tenant,
                    s(doc, "applicant_email"),
                    &format!("Your application {number} is submitted"),
                    fee,
                    ("PermitApplication", application),
                )
                .await?;
                json!({"status": "Submitted", "submitted_on": today.to_string(), "fee_invoice": fee_invoice.as_ref().and_then(|i| i.get("id").cloned()),
                       "customer": customer.map(|c| c.to_string())})
            }
            "take_up" => {
                let kind = type_of(hc, doc).await?;
                json!({"status": "Under Review", "officer": hc.ctx.email, "stage": stages(&kind).first().map(|st| st.0.clone())})
            }
            "pass_stage" => {
                let kind = type_of(hc, doc).await?;
                let all = stages(&kind);
                let mut reviews: Vec<Doc> = rows(doc, "reviews").iter().map(review_row).collect();
                let passed = reviews.iter().filter(|r| s(r, "outcome") == Some("Passed")).count();
                let (stage, role, inspection) = all.get(passed).cloned().ok_or_else(|| Error::invalid("every stage is passed: approve it"))?;
                if let Some(role) = &role
                    && !hc.ctx.is_admin
                    && !hc.ctx.roles.iter().any(|r| r == role)
                {
                    return Err(Error::Forbidden(format!("{stage}: a {role} passes it")));
                }
                if reviews.last().and_then(|r| s(r, "reviewer")).is_some_and(|by| by.eq_ignore_ascii_case(&hc.ctx.email)) {
                    return Err(Error::Forbidden(format!("{stage}: someone other than whoever passed the stage before passes it")));
                }
                let findings = params.get("findings").and_then(Value::as_str).map(str::trim).filter(|f| !f.is_empty()).map(str::to_string);
                if inspection && findings.is_none() {
                    return Err(Error::field("findings", "a site inspection says what it found"));
                }
                reviews.push(obj(json!({"id": Uuid::now_v7().to_string(), "stage": stage, "outcome": "Passed", "reviewer": hc.ctx.email.to_lowercase(),
                                        "reviewed_on": today.to_string(), "note": note(params), "findings": findings})));
                hc.kernel.set_child_rows_tx(&system, hc.tx, "PermitApplication", "reviews", application, reviews).await?;
                json!({"status": "Under Review", "stage": all.get(passed + 1).map(|st| st.0.clone())})
            }
            "approve" => {
                if let Some(stage) = s(doc, "stage").filter(|st| !st.is_empty()) {
                    return Err(Error::invalid(format!("{stage} is not passed yet")));
                }
                if let Some(inv) = id(doc, "fee_invoice") {
                    let paid: Option<bool> = sqlx::query_scalar(
                        "select docstatus = 1 and coalesce(outstanding_amount, 0) <= 0 from e_sales_invoice where tenant_id = $1 and id = $2",
                    )
                    .bind(hc.ctx.tenant)
                    .bind(inv)
                    .fetch_optional(&mut **hc.tx)
                    .await?;
                    if paid != Some(true) {
                        return Err(Error::invalid("the fee is not paid yet"));
                    }
                }
                let conditions = params.get("conditions").and_then(Value::as_str).map(str::trim).filter(|c| !c.is_empty()).map(str::to_string);
                tell(
                    hc.tx,
                    hc.ctx.tenant,
                    s(doc, "applicant_email"),
                    &format!("Your application {number} is approved"),
                    conditions.as_deref(),
                    ("PermitApplication", application),
                )
                .await?;
                json!({"status": "Approved", "decision_note": note(params), "decided_on": today.to_string(), "conditions": conditions})
            }
            "reject" => {
                let why = note(params).ok_or_else(|| Error::field("note", "a rejection says why"))?;
                // A stage it waited for failed with it.
                if let Some(stage) = s(doc, "stage").filter(|st| !st.is_empty()) {
                    let mut reviews: Vec<Doc> = rows(doc, "reviews").iter().map(review_row).collect();
                    reviews.push(obj(json!({"id": Uuid::now_v7().to_string(), "stage": stage, "outcome": "Failed", "reviewer": hc.ctx.email.to_lowercase(),
                                            "reviewed_on": today.to_string(), "note": why})));
                    hc.kernel.set_child_rows_tx(&system, hc.tx, "PermitApplication", "reviews", application, reviews).await?;
                }
                tell(
                    hc.tx,
                    hc.ctx.tenant,
                    s(doc, "applicant_email"),
                    &format!("Your application {number} is rejected"),
                    Some(&why),
                    ("PermitApplication", application),
                )
                .await?;
                json!({"status": "Rejected", "decision_note": why, "decided_on": today.to_string(), "stage": null})
            }
            "issue" => {
                let kind = hc
                    .kernel
                    .get_tx(&system, hc.tx, "PermitType", id(doc, "permit_type").ok_or_else(|| Error::field("permit_type", "is required"))?, false)
                    .await?;
                let months = kind.get("valid_months").and_then(Value::as_i64).unwrap_or(12).max(1) as u32;
                // A renewal runs on from the day after the permit it renews ends.
                let renews = id(doc, "renewal_of");
                let after: Option<NaiveDate> = match renews {
                    Some(old) => sqlx::query_scalar("select expires_on from e_permit where tenant_id = $1 and id = $2")
                        .bind(hc.ctx.tenant)
                        .bind(old)
                        .fetch_optional(&mut **hc.tx)
                        .await?
                        .flatten(),
                    None => None,
                };
                let on = after.map_or(today, |end| (end + Duration::days(1)).max(today));
                let expires = on.checked_add_months(Months::new(months)).ok_or_else(|| Error::internal("a date past the calendar"))?;
                let permit = hc
                    .kernel
                    .insert_tx(
                        &system,
                        hc.tx,
                        "Permit",
                        obj(json!({"permit_type": s(doc, "permit_type"), "application": application.to_string(), "holder_name": s(doc, "applicant_name"),
                                   "holder_email": s(doc, "applicant_email"), "issued_on": on.to_string(), "expires_on": expires.to_string(),
                                   "conditions": s(doc, "conditions"), "renewal_of": renews.map(|r| r.to_string())})),
                    )
                    .await?;
                let title = format!("Your permit {} is issued, valid {on} to {expires}", s(&permit, "number").unwrap_or_default());
                tell(hc.tx, hc.ctx.tenant, s(doc, "applicant_email"), &title, s(doc, "conditions"), ("PermitApplication", application)).await?;
                out["permit"] = created("Permit", &permit);
                json!({"status": "Issued", "permit": permit.get("id")})
            }
            "withdraw" => json!({"status": "Withdrawn"}),
            _ => return Ok(None),
        };
        let status = values["status"].clone();
        hc.kernel.set_values_tx(&system, hc.tx, "PermitApplication", application, obj(values)).await?;
        out["status"] = status;
        Ok(Some(out))
    }
}

/// The application's permit type.
async fn type_of(hc: &mut HookCtx<'_>, doc: &Doc) -> Result<Doc> {
    let kind = id(doc, "permit_type").ok_or_else(|| Error::field("permit_type", "is required"))?;
    hc.kernel.get_tx(&elevated(hc.ctx), hc.tx, "PermitType", kind, false).await
}

/// Checks a submission and invoices its fee: every document the type needs, with its file, and the
/// type still open for applications. Returns the fee's invoice, if there is a fee, and its customer.
async fn submit(k: &Kernel, ctx: &Ctx, tx: &mut Tx, doc: &Doc) -> Result<(Option<Doc>, Option<Uuid>)> {
    let kind = k.get_tx(ctx, tx, "PermitType", id(doc, "permit_type").ok_or_else(|| Error::field("permit_type", "is required"))?, false).await?;
    if kind.get("active").and_then(Value::as_bool) == Some(false) {
        return Err(Error::field("permit_type", "is not open for applications"));
    }
    let given: HashSet<String> =
        rows(doc, "documents").iter().filter(|r| s(r, "file").is_some()).filter_map(|r| s(r, "document").map(str::to_lowercase)).collect();
    let missing: Vec<String> =
        rows(&kind, "documents").iter().filter_map(|r| s(r, "document")).filter(|n| !given.contains(&n.to_lowercase())).map(str::to_string).collect();
    if !missing.is_empty() {
        return Err(Error::field("documents", format!("still needed: {}", missing.join(", "))));
    }
    let fee = d(&kind, "fee");
    if fee <= Decimal::ZERO {
        return Ok((None, None));
    }
    let settings: Option<(Option<Uuid>, Option<Uuid>)> =
        sqlx::query_as("select company, fee_item from e_government_settings where tenant_id = $1 limit 1").bind(ctx.tenant).fetch_optional(&mut **tx).await?;
    let Some((Some(company), Some(default_item))) = settings else {
        return Err(Error::invalid("set the company and the fee item in Government Settings first"));
    };
    let customer = customer_of(k, ctx, tx, doc).await?;
    let currency: Option<Uuid> = sqlx::query_scalar("select default_currency from e_company where tenant_id = $1 and id = $2")
        .bind(ctx.tenant)
        .bind(company)
        .fetch_optional(&mut **tx)
        .await?
        .flatten();
    let what = format!("{} application {}", s(&kind, "type_name").unwrap_or("Permit"), s(doc, "number").unwrap_or_default());
    let item = id(&kind, "fee_item").unwrap_or(default_item);
    let inv = k
        .insert_tx(
            ctx,
            tx,
            "SalesInvoice",
            obj(json!({"company": company.to_string(), "customer": customer.to_string(), "currency": currency.map(|c| c.to_string()), "remarks": what,
                       "items": [{"item": item.to_string(), "qty": "1", "price_list_rate": money(fee), "discount_percent": "0", "description": what}]})),
        )
        .await?;
    Ok((Some(inv), Some(customer)))
}

/// The applicant's customer: the one their address names, or one made from them now.
async fn customer_of(k: &Kernel, ctx: &Ctx, tx: &mut Tx, doc: &Doc) -> Result<Uuid> {
    if let Some(email) = s(doc, "applicant_email").map(str::to_lowercase) {
        let found = k.find_tx(&sys(ctx), tx, "Customer", &[Filter::eq("email", email)], Some("created_at asc"), 1).await?;
        if let Some(c) = found.first().and_then(|c| id(c, "id")) {
            return Ok(c);
        }
    }
    let made = k
        .insert_tx(
            ctx,
            tx,
            "Customer",
            obj(json!({"customer_name": s(doc, "applicant_name"), "customer_type": "Individual", "email": s(doc, "applicant_email")})),
        )
        .await?;
    id(&made, "id").ok_or_else(|| Error::internal("a customer without an id"))
}

pub struct PermitHook;

#[async_trait]
impl DocHook for PermitHook {
    fn name(&self) -> &'static str {
        "government.permit"
    }

    async fn action(&self, hc: &mut HookCtx<'_>, name: &str, doc: &Doc, params: &Value) -> Result<Option<Value>> {
        if name == "renew" {
            let today = hc.ctx.today();
            let made = renew(hc.kernel, &elevated(hc.ctx), hc.tx, doc, today).await?;
            return Ok(Some(json!({"application": created("PermitApplication", &made)})));
        }
        if name != "revoke" {
            return Ok(None);
        }
        let permit = id(doc, "id").ok_or_else(|| Error::internal("a permit without an id"))?;
        let why = params
            .get("reason")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|r| !r.is_empty())
            .ok_or_else(|| Error::field("reason", "a revocation says why"))?;
        hc.kernel.set_values_tx(&elevated(hc.ctx), hc.tx, "Permit", permit, obj(json!({"status": "Revoked", "revoked_reason": why}))).await?;
        Ok(Some(json!({"status": "Revoked"})))
    }
}

/// Holders of renewable permits whose renewal has opened, told once.
pub(crate) async fn remind(k: &Kernel, ctx: &Ctx, tx: &mut Tx, today: NaiveDate) -> Result<()> {
    let due: Vec<(Uuid, Option<String>, Option<String>, NaiveDate)> = sqlx::query_as(
        "select p.id, p.number, p.holder_email, p.expires_on from e_permit p join e_permit_type t on t.tenant_id = p.tenant_id and t.id = p.permit_type \
         where p.tenant_id = $1 and p.status = 'Valid' and coalesce(t.renewable, false) and p.reminded_on is null and p.renewal is null \
         and p.expires_on - coalesce(t.renew_within_days, 60) <= $2",
    )
    .bind(ctx.tenant)
    .bind(today)
    .fetch_all(&mut **tx)
    .await?;
    for (permit, number, email, expires) in due {
        let title = format!("Your permit {} may be renewed: it ends on {expires}", number.unwrap_or_default());
        tell(tx, ctx.tenant, email.as_deref(), &title, None, ("Permit", permit)).await?;
        k.set_values_tx(ctx, tx, "Permit", permit, obj(json!({"reminded_on": today.to_string()}))).await?;
    }
    Ok(())
}

/// Permits past their expiry, expired.
pub(crate) async fn expire(k: &Kernel, ctx: &Ctx, tx: &mut Tx, today: NaiveDate) -> Result<()> {
    let due: Vec<Uuid> = sqlx::query_scalar("select id from e_permit where tenant_id = $1 and status = 'Valid' and expires_on < $2")
        .bind(ctx.tenant)
        .bind(today)
        .fetch_all(&mut **tx)
        .await?;
    for permit in due {
        k.set_values_tx(ctx, tx, "Permit", permit, obj(json!({"status": "Expired"}))).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::renewable_from;
    use chrono::NaiveDate;

    #[test]
    fn a_renewal_opens_before_the_end() {
        let d = |s: &str| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap();
        assert_eq!(renewable_from(d("2027-03-31"), 60), d("2027-01-30"));
        assert_eq!(renewable_from(d("2027-03-31"), -5), d("2027-03-31"), "never after its end");
    }
}
