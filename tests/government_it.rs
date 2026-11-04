//! A town hall (FERRUM-INDUSTRY-010) against a real PostgreSQL
//! (`FERRUM_GOVERNMENT_TEST_DATABASE_URL`, default `postgres://ferrum:ferrum@localhost:5432/ferrum_government_test`;
//! reset first): a citizen on the portal applies for a permit, uploads the documents it needs and
//! submits it, its fee invoiced through ERP; an officer takes it up, approves it once the fee is
//! paid and issues a permit valid for its months; a rejection says why; a permit is revoked saying
//! why, and expires by itself; cases are due within their category's days, resolved saying how and
//! marked overdue when late; a citizen sees only what is theirs; and who may do what.

use ferrum_kernel::secret::SecretKey;
use ferrum_kernel::tenant::NewTenant;
use ferrum_kernel::{Ctx, Db, DbConfig, Doc, Error, HookRegistry, Kernel, KernelSettings, ListQuery};
use ferrum_meta::Catalog;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

fn obj(v: Value) -> Doc {
    v.as_object().cloned().unwrap()
}

fn uid(v: &Value) -> Uuid {
    v.as_str().unwrap_or_else(|| panic!("not an id: {v}")).parse().unwrap()
}

fn dec(v: &Value) -> f64 {
    v.as_str().map(|s| s.parse().unwrap()).or_else(|| v.as_f64()).unwrap_or_else(|| panic!("not a number: {v}"))
}

/// An in-memory object store standing in for S3.
#[derive(Default)]
struct MemFiles(std::sync::Mutex<HashMap<String, Vec<u8>>>);

#[async_trait::async_trait]
impl ferrum_kernel::files::FileStore for MemFiles {
    async fn put(&self, key: &str, data: &[u8]) -> ferrum_kernel::Result<()> {
        self.0.lock().unwrap().insert(key.to_string(), data.to_vec());
        Ok(())
    }
    async fn get(&self, key: &str) -> ferrum_kernel::Result<Vec<u8>> {
        self.0.lock().unwrap().get(key).cloned().ok_or_else(|| Error::NotFound("object".into()))
    }
    async fn delete(&self, key: &str) -> ferrum_kernel::Result<()> {
        self.0.lock().unwrap().remove(key);
        Ok(())
    }
}

struct T {
    k: Arc<Kernel>,
    tenant: Uuid,
    admin: Ctx,
}

impl T {
    async fn insert(&self, ctx: &Ctx, entity: &str, v: Value) -> Doc {
        self.k.insert(ctx, entity, obj(v)).await.unwrap_or_else(|e| panic!("insert {entity}: {e:?}"))
    }
    async fn get(&self, entity: &str, id: Uuid) -> Doc {
        self.k.get(&self.admin, entity, id, false).await.unwrap_or_else(|e| panic!("get {entity}: {e:?}"))
    }
    async fn find_one(&self, entity: &str, filters: Value) -> Doc {
        let q = ListQuery { filters: ferrum_kernel::query::parse_filters(&filters).unwrap(), limit: Some(50), ..Default::default() };
        let rows = self.k.list(&self.admin, entity, &q).await.unwrap().rows;
        assert_eq!(rows.len(), 1, "{entity} {filters}");
        rows[0].clone()
    }
    async fn user(&self, email: &str, roles: &[&str]) -> Ctx {
        let roles: Vec<String> = roles.iter().map(|r| r.to_string()).collect();
        let id = self.k.create_user(&self.admin, email, email, Some("Correct-Horse-9"), &roles, false).await.expect("user");
        let s = self.k.login("town", email, "Correct-Horse-9").await.expect("login");
        self.k.load_ctx(self.tenant, id, Some(s.token_version), "it".into()).await.expect("ctx")
    }
    async fn citizen(&self, email: &str, name: &str) -> Ctx {
        let id = self.k.create_user(&self.admin, email, name, None, &[], false).await.unwrap();
        self.k.set_user_kind(&self.admin, id, "portal").await.unwrap();
        self.k.load_ctx(self.tenant, id, None, "it".into()).await.unwrap()
    }
    async fn act(&self, ctx: &Ctx, entity: &str, id: Uuid, action: &str, params: Value) -> Result<Value, Error> {
        self.k.run_action(ctx, entity, id, action, params).await
    }
}

/// A call to Government's endpoints as `who` (`/api/app/government/{path}`).
async fn call(k: &Kernel, who: Option<&Ctx>, method: &str, path: &str, body: Value) -> Result<Value, Error> {
    let site = k.hooks.site("government").expect("government's endpoints").clone();
    let req = ferrum_kernel::SiteRequest {
        tenant: who.map(|c| c.tenant).unwrap_or_default(),
        user: who.cloned(),
        method: method.into(),
        path: format!("/{path}"),
        query: Default::default(),
        headers: Default::default(),
        body: if body.is_null() { vec![] } else { body.to_string().into_bytes() },
        client_ip: "203.0.113.9".into(),
    };
    let r = site.handle(k, req).await?;
    Ok(serde_json::from_slice(&r.body).unwrap())
}

fn refused<V: std::fmt::Debug>(r: Result<V, Error>, kind: &str, says: &str) {
    let msg = format!("{r:?}");
    let ok = matches!(
        (&r, kind),
        (Err(Error::Invalid(_) | Error::Validation(_)), "invalid") | (Err(Error::Forbidden(_)), "forbidden") | (Err(Error::NotFound(_)), "not found")
    );
    assert!(ok && msg.contains(says), "expected {kind} saying {says:?}, got {msg}");
}

#[test]
fn a_town_hall() {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().thread_stack_size(16 << 20).build().unwrap();
    rt.block_on(async { tokio::spawn(scenario()).await.unwrap() });
}

async fn scenario() {
    let url = std::env::var("FERRUM_GOVERNMENT_TEST_DATABASE_URL").unwrap_or_else(|_| "postgres://ferrum:ferrum@localhost:5432/ferrum_government_test".into());
    let db = Db::connect(DbConfig { url, max_connections: 16, ..Default::default() }).await.expect("connect");
    sqlx::raw_sql("drop schema if exists public cascade; create schema public;").execute(&db.pool).await.expect("reset");
    let mut packages = ferrum_domain::packages().expect("platform packages");
    packages.extend(ferrum_app_erp::packages().expect("ERP's packages"));
    packages.extend(ferrum_app_government::packages().expect("app packages"));
    let catalog = Catalog::from_packages(packages).expect("catalog");
    let mut hooks = HookRegistry::default();
    ferrum_domain::register(&mut hooks);
    ferrum_app_erp::register(&mut hooks);
    ferrum_app_government::register(&mut hooks);
    let settings = KernelSettings { secret_key: Some(SecretKey::new([7u8; 32])), ..Default::default() };
    let k = Kernel::boot(db, catalog, hooks, settings, None).await.expect("boot");
    k.set_file_store(Arc::new(MemFiles::default()));
    ferrum_domain::maintenance(&k).await.expect("partitions");
    ferrum_app_erp::maintenance(&k).await.expect("partitions");
    let tenant = k
        .create_tenant(NewTenant {
            slug: "town".into(),
            name: "Riverton Town Hall".into(),
            admin_email: "admin@town.test".into(),
            admin_password: "Correct-Horse-9".into(),
            admin_name: None,
            packages: vec!["government".into()],
            settings: Some(json!({"fiscal_year_start_month": 4, "timezone": "Asia/Kolkata"})),
        })
        .await
        .expect("tenant");
    let session = k.login("town", "admin@town.test", "Correct-Horse-9").await.expect("login");
    let admin = k.load_ctx(tenant, session.user, Some(session.token_version), "it".into()).await.expect("ctx");
    let t = T { k: k.clone(), tenant, admin };
    let a = t.admin.clone();
    let today = chrono::Utc::now().date_naive();

    // ───────────── the books, the permits and the categories ─────────────
    let inr = t.find_one("Currency", json!({"code": "INR"})).await;
    let company = uid(&t.insert(&a, "Company", json!({"company_name": "Riverton Town Hall", "abbr": "RT", "default_currency": inr["id"]})).await["id"]);
    let services = t.find_one("ItemGroup", json!({"group_name": "Services"})).await;
    let nos = t.find_one("Uom", json!({"uom_name": "Nos"})).await;
    let fee_item = uid(&t
        .insert(
            &a,
            "Item",
            json!({"item_code": "PERMIT-FEE", "item_name": "Permit fee", "item_group": services["id"], "stock_uom": nos["id"], "is_stock_item": false}),
        )
        .await["id"]);
    let manager = t.user("manager@town.test", &["Services Manager"]).await;
    let officer = t.user("officer@town.test", &["Permit Officer"]).await;
    let caseworker = t.user("cases@town.test", &["Case Officer"]).await;
    t.insert(&manager, "GovernmentSettings", json!({"company": company.to_string(), "fee_item": fee_item.to_string()})).await;
    let building = uid(&t
        .insert(
            &manager,
            "PermitType",
            json!({"type_name": "Building permit", "fee": "2500", "valid_months": 24, "documents": [{"document": "Site plan"}, {"document": "Proof of ownership"}]}),
        )
        .await["id"]);
    let party = uid(&t.insert(&manager, "PermitType", json!({"type_name": "Street party", "fee": "0", "valid_months": 1})).await["id"]);
    let stall = uid(&t.insert(&manager, "PermitType", json!({"type_name": "Market stall", "fee": "50", "valid_months": 3, "active": false})).await["id"]);
    let signage = uid(&t.insert(&manager, "PermitType", json!({"type_name": "Signage", "fee": "100", "valid_months": 12})).await["id"]);
    let potholes = uid(&t.insert(&manager, "CaseCategory", json!({"category_name": "Potholes", "days_to_resolve": 5})).await["id"]);
    let noise = uid(&t.insert(&manager, "CaseCategory", json!({"category_name": "Noise", "days_to_resolve": 3})).await["id"]);

    // ───────────── a citizen applies on the portal ─────────────
    let ria = t.citizen("Ria@Home.test", "Ria Sen").await;
    let sam = t.citizen("sam@home.test", "Sam Lee").await;
    let home = call(&k, Some(&ria), "GET", "portal/home", Value::Null).await.unwrap();
    let open: Vec<&str> = home["types"].as_array().unwrap().iter().map(|x| x["type_name"].as_str().unwrap()).collect();
    assert_eq!(open, ["Building permit", "Signage", "Street party"], "the closed market stall is not offered");
    let draft = call(&k, Some(&ria), "POST", "portal/applications", json!({"permit_type": building.to_string(), "details": "A rear extension"})).await.unwrap();
    let app = uid(&draft["id"]);
    assert_eq!(draft["status"], "Draft");
    let stored = t.get("PermitApplication", app).await;
    assert_eq!((stored["applicant_email"].as_str(), stored["applicant_name"].as_str()), (Some("ria@home.test"), Some("Ria Sen")));
    refused(
        call(&k, Some(&ria), "POST", &format!("portal/applications/{app}/submit"), json!({})).await,
        "invalid",
        "still needed: Site plan, Proof of ownership",
    );
    let pdf = |s: &str| {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(format!("%PDF-1.7 {s}"))
    };
    let upload =
        |name: &'static str, data: String| json!({"document": name, "file_name": format!("{name}.pdf"), "content_type": "application/pdf", "data": data});
    refused(
        call(&k, Some(&ria), "POST", &format!("portal/applications/{app}/documents"), upload("Site plan", "not base64!".into())).await,
        "invalid",
        "base64",
    );
    refused(
        call(&k, Some(&ria), "POST", &format!("portal/applications/{app}/documents"), upload("Site plan", String::new())).await,
        "invalid",
        "a file of 1 byte",
    );
    let with_plan = call(&k, Some(&ria), "POST", &format!("portal/applications/{app}/documents"), upload("Site plan", pdf("plan"))).await.unwrap();
    assert!(with_plan["documents"][0]["file"].as_str().unwrap().starts_with("/files/"));
    // Sam sees nothing of Ria's, and changes nothing.
    refused(
        call(&k, Some(&sam), "POST", &format!("portal/applications/{app}/documents"), upload("Proof of ownership", pdf("not mine"))).await,
        "not found",
        "",
    );
    refused(call(&k, Some(&sam), "POST", &format!("portal/applications/{app}/submit"), json!({})).await, "not found", "");
    refused(call(&k, Some(&ria), "POST", &format!("portal/applications/{app}/submit"), json!({})).await, "invalid", "still needed: Proof of ownership");
    // Sent again, a document takes the place of the first.
    call(&k, Some(&ria), "POST", &format!("portal/applications/{app}/documents"), upload("Site plan", pdf("plan, revised"))).await.unwrap();
    call(&k, Some(&ria), "POST", &format!("portal/applications/{app}/documents"), upload("Proof of ownership", pdf("deed"))).await.unwrap();
    assert_eq!(t.get("PermitApplication", app).await["documents"].as_array().unwrap().len(), 2);
    let submitted = call(&k, Some(&ria), "POST", &format!("portal/applications/{app}/submit"), json!({})).await.unwrap();
    assert_eq!(submitted["status"], "Submitted");
    let fee = t.get("SalesInvoice", uid(&submitted["fee_invoice"])).await;
    assert_eq!(dec(&fee["grand_total"]), 2500.0);
    let customer = t.get("Customer", uid(&fee["customer"])).await;
    assert_eq!((customer["customer_name"].as_str(), customer["email"].as_str()), (Some("Ria Sen"), Some("ria@home.test")), "made from the applicant");
    refused(
        call(&k, Some(&ria), "POST", &format!("portal/applications/{app}/documents"), upload("Site plan", pdf("late"))).await,
        "invalid",
        "before it is submitted",
    );
    // Her next fee is invoiced to the same customer; a closed type takes no applications.
    let sign = uid(&call(&k, Some(&ria), "POST", "portal/applications", json!({"permit_type": signage.to_string()})).await.unwrap()["id"]);
    let sign_fee = call(&k, Some(&ria), "POST", &format!("portal/applications/{sign}/submit"), json!({})).await.unwrap();
    assert_eq!(t.get("SalesInvoice", uid(&sign_fee["fee_invoice"])).await["customer"], fee["customer"], "one customer a citizen");
    let closed = uid(&t.insert(&officer, "PermitApplication", json!({"permit_type": stall.to_string(), "applicant_name": "Walk-in"})).await["id"]);
    refused(t.act(&officer, "PermitApplication", closed, "submit_application", Value::Null).await, "invalid", "not open for applications");
    // A document named without its file is still needed.
    let counter = uid(&t
        .insert(
            &officer,
            "PermitApplication",
            json!({"permit_type": building.to_string(), "applicant_name": "Counter Visitor",
                   "documents": [{"document": "Site plan"}, {"document": "Proof of ownership", "file": "/files/deed-on-paper"}]}),
        )
        .await["id"]);
    refused(t.act(&officer, "PermitApplication", counter, "submit_application", Value::Null).await, "invalid", "still needed: Site plan");

    // ───────────── review, the fee, and the permit ─────────────
    refused(t.k.update(&officer, "PermitApplication", app, obj(json!({"details": "Two storeys"})), None).await, "invalid", "keeps what was applied for");
    t.act(&officer, "PermitApplication", app, "take_up", Value::Null).await.unwrap();
    assert_eq!(t.get("PermitApplication", app).await["officer"], "officer@town.test");
    refused(t.act(&officer, "PermitApplication", app, "approve", Value::Null).await, "invalid", "the fee is not paid yet");
    let fee_id = uid(&fee["id"]);
    t.k.submit(&a, "SalesInvoice", fee_id, None).await.expect("the fee invoice submitted");
    refused(t.act(&officer, "PermitApplication", app, "approve", Value::Null).await, "invalid", "the fee is not paid yet");
    let payment = t.k.run_action(&a, "SalesInvoice", fee_id, "make_payment", Value::Null).await.expect("a payment");
    t.k.submit(&a, "PaymentEntry", uid(&payment["id"]), None).await.expect("paid");
    t.act(&officer, "PermitApplication", app, "approve", json!({"note": "Meets the plan"})).await.expect("approved once paid");
    let issued = t.act(&officer, "PermitApplication", app, "issue", Value::Null).await.expect("issued");
    let permit = t.get("Permit", uid(&issued["permit"]["id"])).await;
    let expires = today.checked_add_months(chrono::Months::new(24)).unwrap().to_string();
    assert_eq!(
        (permit["status"].as_str(), permit["expires_on"].as_str(), permit["holder_email"].as_str()),
        (Some("Valid"), Some(expires.as_str()), Some("ria@home.test"))
    );
    assert_eq!(t.get("PermitApplication", app).await["status"], "Issued");
    refused(t.act(&officer, "Permit", uid(&permit["id"]), "revoke", Value::Null).await, "invalid", "says why");

    // ───────────── a rejection, a permit that expires, a withdrawal ─────────────
    let p1 = uid(&call(&k, Some(&ria), "POST", "portal/applications", json!({"permit_type": party.to_string()})).await.unwrap()["id"]);
    let free = call(&k, Some(&ria), "POST", &format!("portal/applications/{p1}/submit"), json!({})).await.unwrap();
    assert!(free["fee_invoice"].is_null(), "no fee, no invoice");
    t.act(&officer, "PermitApplication", p1, "take_up", Value::Null).await.unwrap();
    refused(t.act(&officer, "PermitApplication", p1, "reject", Value::Null).await, "invalid", "says why");
    t.act(&officer, "PermitApplication", p1, "reject", json!({"note": "The road is a bus route"})).await.unwrap();
    let p2 = uid(&call(&k, Some(&ria), "POST", "portal/applications", json!({"permit_type": party.to_string()})).await.unwrap()["id"]);
    call(&k, Some(&ria), "POST", &format!("portal/applications/{p2}/submit"), json!({})).await.unwrap();
    t.act(&officer, "PermitApplication", p2, "take_up", Value::Null).await.unwrap();
    t.act(&officer, "PermitApplication", p2, "approve", Value::Null).await.unwrap();
    let short = uid(&t.act(&officer, "PermitApplication", p2, "issue", Value::Null).await.unwrap()["permit"]["id"]);
    let p3 = uid(&call(&k, Some(&ria), "POST", "portal/applications", json!({"permit_type": party.to_string()})).await.unwrap()["id"]);
    call(&k, Some(&ria), "POST", &format!("portal/applications/{p3}/withdraw"), json!({})).await.unwrap();
    assert_eq!(t.get("PermitApplication", p3).await["status"], "Withdrawn");
    ferrum_app_government::maintenance_on(&k, today + chrono::Duration::days(40)).await.unwrap();
    assert_eq!(t.get("Permit", short).await["status"], "Expired", "a month's permit, forty days on");
    assert_eq!(t.get("Permit", uid(&permit["id"])).await["status"], "Valid", "the building permit runs two years");
    t.act(&officer, "Permit", uid(&permit["id"]), "revoke", json!({"reason": "Built beyond the plan"})).await.unwrap();
    assert_eq!(t.get("Permit", uid(&permit["id"])).await["status"], "Revoked");

    // ───────────── cases ─────────────
    let raised = call(&k, Some(&ria), "POST", "portal/cases", json!({"category": potholes.to_string(), "subject": "Pothole on Mill Lane"})).await.unwrap();
    assert_eq!(raised["due_on"], (today + chrono::Duration::days(5)).to_string(), "due in the category's five days");
    let pothole = uid(&raised["id"]);
    refused(t.k.update(&caseworker, "PublicCase", pothole, obj(json!({"status": "Resolved"})), None).await, "invalid", "says how");
    t.k.update(&caseworker, "PublicCase", pothole, obj(json!({"status": "Resolved", "resolution": "Filled on Tuesday"})), None).await.unwrap();
    let done = t.get("PublicCase", pothole).await;
    assert_eq!(done["resolved_on"], today.to_string());
    let loud = uid(&t
        .insert(
            &caseworker,
            "PublicCase",
            json!({"category": noise.to_string(), "subject": "Late music", "citizen_name": "Sam Lee", "citizen_email": "sam@home.test"}),
        )
        .await["id"]);
    assert_eq!(t.get("PublicCase", loud).await["due_on"], (today + chrono::Duration::days(3)).to_string());
    ferrum_app_government::maintenance_on(&k, today + chrono::Duration::days(10)).await.unwrap();
    assert_eq!(t.get("PublicCase", loud).await["overdue"], true, "open past its day");
    assert_eq!(t.get("PublicCase", pothole).await["overdue"], false, "resolved in time");
    // A case resolved late is late no longer once resolved.
    let old_case =
        uid(&t.insert(&caseworker, "PublicCase", json!({"category": noise.to_string(), "subject": "Old complaint", "citizen_name": "Sam Lee"})).await["id"]);
    let mut tx = t.k.db.tenant_tx(t.tenant).await.unwrap();
    sqlx::query("update e_public_case set opened_on = $2 where tenant_id = $1 and id = $3")
        .bind(t.tenant)
        .bind(today - chrono::Duration::days(20))
        .bind(old_case)
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    t.k.update(&caseworker, "PublicCase", old_case, obj(json!({"status": "In Progress"})), None).await.unwrap();
    assert_eq!(t.get("PublicCase", old_case).await["overdue"], true, "open, twenty days on");
    t.k.update(&caseworker, "PublicCase", old_case, obj(json!({"status": "Resolved", "resolution": "Spoke to the neighbours"})), None).await.unwrap();
    assert_eq!(t.get("PublicCase", old_case).await["overdue"], false, "resolved, it is not overdue");

    // ───────────── what a citizen sees ─────────────
    let mine = call(&k, Some(&ria), "GET", "portal/home", Value::Null).await.unwrap();
    assert_eq!(mine["applications"].as_array().unwrap().len(), 5);
    assert_eq!(mine["permits"].as_array().unwrap().len(), 2);
    assert_eq!(mine["cases"].as_array().unwrap().iter().map(|c| c["subject"].as_str().unwrap()).collect::<Vec<_>>(), ["Pothole on Mill Lane"]);
    let rejected = mine["applications"].as_array().unwrap().iter().find(|x| x["status"] == "Rejected").unwrap();
    assert_eq!(rejected["decision_note"], "The road is a bus route", "the reason reaches the citizen");
    let sams = call(&k, Some(&sam), "GET", "portal/home", Value::Null).await.unwrap();
    assert!(sams["applications"].as_array().unwrap().is_empty() && sams["permits"].as_array().unwrap().is_empty());
    assert_eq!(sams["cases"].as_array().unwrap().len(), 1, "the case raised for him at the counter");
    assert!(matches!(call(&k, None, "GET", "portal/home", Value::Null).await, Err(Error::Unauthorized(_))));

    // ───────────── who may do what ─────────────
    refused(t.act(&caseworker, "PermitApplication", p2, "issue", Value::Null).await, "forbidden", "");
    refused(t.k.insert(&officer, "PermitType", obj(json!({"type_name": "Mine", "valid_months": 1}))).await, "forbidden", "");
    let p4 = uid(&call(&k, Some(&ria), "POST", "portal/applications", json!({"permit_type": party.to_string()})).await.unwrap()["id"]);
    call(&k, Some(&ria), "POST", &format!("portal/applications/{p4}/submit"), json!({})).await.unwrap();
    refused(t.act(&manager, "PermitApplication", p4, "take_up", Value::Null).await, "forbidden", "");
}
