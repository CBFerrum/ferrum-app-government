//! Government's page of the workspace's portal (Ferrum docs/plan/apps-ui.md §9.1), "Your services"
//! at `/portal/services/`, from `/api/app/government/portal/…`: the permits one may apply for, a
//! citizen's own applications, permits and cases, an application drafted, its documents uploaded
//! and submitted, and a case raised. Portal people hold no roles: what is theirs is what carries
//! the address they signed in with, and what they do runs as the system after these checks.

use base64::Engine;
use ferrum_domain::util::{elevated, id, rows, s, sys};
use ferrum_kernel::{Ctx, Doc, Error, Filter, Kernel, Result, Site, SiteRequest, SiteResponse, with_retry};
use serde_json::{Map, Value, json};
use uuid::Uuid;

/// The largest request: a document of up to [`MAX_FILE`] bytes, as base64.
const MAX_BODY: usize = 8 << 20;
const MAX_FILE: usize = 5 << 20;

pub struct GovernmentSite;

#[async_trait::async_trait]
impl Site for GovernmentSite {
    async fn handle(&self, k: &Kernel, req: SiteRequest) -> Result<SiteResponse> {
        let user = req.user.as_ref().ok_or_else(|| Error::Unauthorized("sign in first".into()))?;
        let email = user.email.to_lowercase();
        if email.is_empty() {
            return Err(Error::Forbidden("this account has no email address to answer".into()));
        }
        let segs: Vec<&str> = req.path.split('/').filter(|p| !p.is_empty()).collect();
        let (status, out) = match (req.method.as_str(), segs.as_slice()) {
            ("GET", ["portal", "home"]) => (200, home(k, user, &email).await?),
            ("POST", ["portal", "applications"]) => (201, draft(k, user, &email, &body(&req)?).await?),
            ("POST", ["portal", "applications", a, "documents"]) => (200, attach(k, user, &email, application(a)?, &body(&req)?).await?),
            ("POST", ["portal", "applications", a, "submit"]) => (200, act(k, user, &email, application(a)?, "submit_application").await?),
            ("POST", ["portal", "applications", a, "withdraw"]) => (200, act(k, user, &email, application(a)?, "withdraw").await?),
            ("POST", ["portal", "cases"]) => (201, raise(k, user, &email, &body(&req)?).await?),
            _ => return Err(Error::NotFound(format!("government endpoint {} {}", req.method, req.path))),
        };
        let mut r = SiteResponse::json(status, &out);
        r.headers.push(("cache-control".into(), "no-store".into()));
        Ok(r)
    }

    fn max_body(&self) -> usize {
        MAX_BODY
    }
}

fn body(req: &SiteRequest) -> Result<Map<String, Value>> {
    match serde_json::from_slice::<Value>(&req.body) {
        Ok(Value::Object(m)) => Ok(m),
        _ => Err(Error::invalid("the request body must be a JSON object")),
    }
}

fn application(raw: &str) -> Result<Uuid> {
    raw.parse().map_err(|_| Error::NotFound("application".into()))
}

fn text(body: &Map<String, Value>, key: &str, max: usize) -> Result<Option<String>> {
    let v = body.get(key).and_then(Value::as_str).map(str::trim).unwrap_or_default();
    if v.chars().count() > max {
        return Err(Error::field(key, format!("at most {max} characters")));
    }
    Ok((!v.is_empty()).then(|| v.to_string()))
}

/// The caller's application, as the system sees it; another person's reads as missing.
async fn own(k: &Kernel, user: &Ctx, email: &str, app: Uuid) -> Result<Doc> {
    let doc = match k.get(&sys(user), "PermitApplication", app, false).await {
        Ok(d) => d,
        Err(Error::NotFound(_)) => return Err(Error::NotFound("application".into())),
        Err(e) => return Err(e),
    };
    if s(&doc, "applicant_email").map(str::to_lowercase).as_deref() != Some(email) {
        return Err(Error::NotFound("application".into()));
    }
    Ok(doc)
}

fn summary(a: &Doc) -> Value {
    json!({"id": s(a, "id"), "number": s(a, "number"), "permit_type": s(a, "permit_type"), "status": s(a, "status"), "details": s(a, "details"),
           "documents": rows(a, "documents").iter().map(|d| json!({"document": s(d, "document"), "file": s(d, "file")})).collect::<Vec<_>>(),
           "fee_invoice": s(a, "fee_invoice"), "decision_note": s(a, "decision_note"), "permit": s(a, "permit")})
}

/// `GET portal/home`: what may be applied for, and the caller's applications, permits and cases.
async fn home(k: &Kernel, user: &Ctx, email: &str) -> Result<Value> {
    let sctx = sys(user);
    let mut tx = k.db.tenant_tx(user.tenant).await?;
    let types = k.find_tx(&sctx, &mut tx, "PermitType", &[Filter::eq("active", true)], Some("type_name asc"), 100).await?;
    let mut open = vec![];
    for t in types.iter().filter_map(|t| id(t, "id")) {
        let t = k.get_tx(&sctx, &mut tx, "PermitType", t, false).await?;
        open.push(json!({"id": s(&t, "id"), "type_name": s(&t, "type_name"), "fee": s(&t, "fee"), "valid_months": t.get("valid_months"), "description": s(&t, "description"),
                         "documents": rows(&t, "documents").iter().filter_map(|d| s(d, "document")).collect::<Vec<_>>()}));
    }
    let mut applications = vec![];
    for a in k
        .find_tx(&sctx, &mut tx, "PermitApplication", &[Filter::eq("applicant_email", email)], Some("created_at desc"), 50)
        .await?
        .iter()
        .filter_map(|a| id(a, "id"))
    {
        applications.push(summary(&k.get_tx(&sctx, &mut tx, "PermitApplication", a, false).await?));
    }
    let permits: Vec<Value> = k
        .find_tx(&sctx, &mut tx, "Permit", &[Filter::eq("holder_email", email)], Some("issued_on desc"), 50)
        .await?
        .iter()
        .map(|p| json!({"id": s(p, "id"), "number": s(p, "number"), "permit_type": s(p, "permit_type"), "issued_on": s(p, "issued_on"), "expires_on": s(p, "expires_on"), "status": s(p, "status")}))
        .collect();
    let cases: Vec<Value> = k
        .find_tx(&sctx, &mut tx, "PublicCase", &[Filter::eq("citizen_email", email)], Some("created_at desc"), 50)
        .await?
        .iter()
        .map(|c| json!({"id": s(c, "id"), "number": s(c, "number"), "subject": s(c, "subject"), "status": s(c, "status"), "due_on": s(c, "due_on"), "resolution": s(c, "resolution")}))
        .collect();
    let categories: Vec<Value> = k
        .find_tx(&sctx, &mut tx, "CaseCategory", &[], Some("category_name asc"), 100)
        .await?
        .iter()
        .map(|c| json!({"id": s(c, "id"), "category_name": s(c, "category_name")}))
        .collect();
    tx.commit().await?;
    Ok(json!({"types": open, "applications": applications, "permits": permits, "cases": cases, "categories": categories}))
}

/// `POST portal/applications` `{permit_type, details?}`: a draft application of the caller's.
async fn draft(k: &Kernel, user: &Ctx, email: &str, body: &Map<String, Value>) -> Result<Value> {
    let kind: Uuid = body.get("permit_type").and_then(Value::as_str).and_then(|v| v.parse().ok()).ok_or_else(|| Error::field("permit_type", "is required"))?;
    let details = text(body, "details", 10_000)?;
    k.rate_limit(
        &format!("government|draft|{}|{}", user.tenant, user.user),
        std::time::Duration::from_secs(3600),
        20,
        "too many applications in an hour; try again later",
    )
    .await?;
    let name = if user.full_name.is_empty() { email.to_string() } else { user.full_name.clone() };
    let saved = k
        .insert(
            &elevated(user),
            "PermitApplication",
            crate::obj(json!({"permit_type": kind.to_string(), "applicant_name": name, "applicant_email": email, "details": details})),
        )
        .await?;
    Ok(summary(&saved))
}

/// `POST portal/applications/{id}/documents` `{document, file_name, content_type, data}`: a document
/// for a draft of the caller's, its file sent as base64; it takes the place of one of the same name.
async fn attach(k: &Kernel, user: &Ctx, email: &str, app: Uuid, body: &Map<String, Value>) -> Result<Value> {
    let doc = own(k, user, email, app).await?;
    if s(&doc, "status") != Some("Draft") {
        return Err(Error::invalid("documents go with an application before it is submitted"));
    }
    let name = text(body, "document", 200)?.ok_or_else(|| Error::field("document", "is required"))?;
    let file_name = text(body, "file_name", 200)?.unwrap_or_else(|| "document".into());
    let ctype = text(body, "content_type", 100)?.unwrap_or_else(|| "application/octet-stream".into());
    let data = base64::engine::general_purpose::STANDARD
        .decode(body.get("data").and_then(Value::as_str).unwrap_or_default())
        .map_err(|_| Error::field("data", "is the file in base64"))?;
    if data.is_empty() || data.len() > MAX_FILE {
        return Err(Error::field("data", format!("a file of 1 byte to {} MB", MAX_FILE >> 20)));
    }
    let stored = k.store_file(&sys(user), Some(("PermitApplication", app)), &file_name, &ctype, &data).await?;
    let url = stored.get("url").and_then(Value::as_str).ok_or_else(|| Error::internal("a stored file without its address"))?.to_string();
    let mut documents: Vec<Value> =
        rows(&doc, "documents").into_iter().filter(|r| s(r, "document").map(str::to_lowercase) != Some(name.to_lowercase())).map(Value::Object).collect();
    documents.push(json!({"document": name, "file": url}));
    let saved = k.update(&elevated(user), "PermitApplication", app, crate::obj(json!({"documents": documents})), None).await?;
    Ok(summary(&saved))
}

/// `POST portal/applications/{id}/submit|withdraw`: the caller's application submitted or withdrawn.
async fn act(k: &Kernel, user: &Ctx, email: &str, app: Uuid, action: &str) -> Result<Value> {
    own(k, user, email, app).await?;
    let result = k.run_action(&elevated(user), "PermitApplication", app, action, Value::Null).await?;
    let mut out = summary(&own(k, user, email, app).await?);
    out["result"] = result;
    Ok(out)
}

/// `POST portal/cases` `{category, subject, description?}`: a case raised by the caller.
async fn raise(k: &Kernel, user: &Ctx, email: &str, body: &Map<String, Value>) -> Result<Value> {
    let category: Uuid = body.get("category").and_then(Value::as_str).and_then(|v| v.parse().ok()).ok_or_else(|| Error::field("category", "is required"))?;
    let subject = text(body, "subject", 300)?.ok_or_else(|| Error::field("subject", "is required"))?;
    let description = text(body, "description", 10_000)?;
    k.rate_limit(
        &format!("government|case|{}|{}", user.tenant, user.user),
        std::time::Duration::from_secs(3600),
        20,
        "too many cases in an hour; try again later",
    )
    .await?;
    let name = if user.full_name.is_empty() { email.to_string() } else { user.full_name.clone() };
    let saved = with_retry(|| async {
        k.insert(
            &elevated(user),
            "PublicCase",
            crate::obj(json!({"category": category.to_string(), "subject": subject, "description": description, "citizen_name": name, "citizen_email": email})),
        )
        .await
    })
    .await?;
    Ok(json!({"id": s(&saved, "id"), "number": s(&saved, "number"), "status": s(&saved, "status"), "due_on": s(&saved, "due_on")}))
}
