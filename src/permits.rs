//! Permits. An application is submitted with every document its type needs, and its fee, if any,
//! is invoiced through ERP to a customer made from the applicant. An officer takes it up and
//! approves it once the fee is paid, or rejects it saying why. Approved, it is issued as a permit
//! valid for its type's months; a permit may be revoked, saying why, and expires by itself.
//! Once submitted, an application keeps what was applied for.

use crate::obj;
use async_trait::async_trait;
use chrono::{Months, NaiveDate, Utc};
use ferrum_domain::flow::created;
use ferrum_domain::util::{d, elevated, id, money, rows, s, sys};
use ferrum_kernel::{Ctx, Doc, DocHook, Error, Filter, HookCtx, Kernel, Result, Tx};
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

fn today() -> NaiveDate {
    Utc::now().date_naive()
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
        let mut out = json!({});
        let values = match name {
            "submit_application" => {
                let (fee_invoice, customer) = submit(hc.kernel, &system, hc.tx, doc).await?;
                if let Some(inv) = &fee_invoice {
                    out["fee_invoice"] = created("SalesInvoice", inv);
                }
                json!({"status": "Submitted", "submitted_on": today().to_string(), "fee_invoice": fee_invoice.as_ref().and_then(|i| i.get("id").cloned()),
                       "customer": customer.map(|c| c.to_string())})
            }
            "take_up" => json!({"status": "Under Review", "officer": hc.ctx.email}),
            "approve" => {
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
                json!({"status": "Approved", "decision_note": note(params), "decided_on": today().to_string()})
            }
            "reject" => {
                let why = note(params).ok_or_else(|| Error::field("note", "a rejection says why"))?;
                json!({"status": "Rejected", "decision_note": why, "decided_on": today().to_string()})
            }
            "issue" => {
                let kind = hc
                    .kernel
                    .get_tx(&system, hc.tx, "PermitType", id(doc, "permit_type").ok_or_else(|| Error::field("permit_type", "is required"))?, false)
                    .await?;
                let months = kind.get("valid_months").and_then(Value::as_i64).unwrap_or(12).max(1) as u32;
                let on = today();
                let expires = on.checked_add_months(Months::new(months)).ok_or_else(|| Error::internal("a date past the calendar"))?;
                let permit = hc
                    .kernel
                    .insert_tx(
                        &system,
                        hc.tx,
                        "Permit",
                        obj(json!({"permit_type": s(doc, "permit_type"), "application": application.to_string(), "holder_name": s(doc, "applicant_name"),
                                   "holder_email": s(doc, "applicant_email"), "issued_on": on.to_string(), "expires_on": expires.to_string()})),
                    )
                    .await?;
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
