//! Government (FERRUM-INDUSTRY-010): permits applied for, reviewed in stages, paid through ERP,
//! issued on conditions with their expiry and renewed, a public register, cases with deadlines, and
//! a citizens' portal.
//!
//! * `permits`: an application goes from draft, with every document its type needs and its fee
//!   invoiced, through its type's review stages (each passed by its role, never by the same person
//!   twice running, a site inspection saying what it found) to a permit on conditions, or is
//!   rejected saying why; permits expire, are revoked, or are renewed; applicants are told as
//!   their application moves, and holders when a permit may be renewed.
//! * `cases`: due within their category's days, resolved saying how, overdue when late.
//! * `site`: the portal's "Your services", where citizens apply, upload documents, renew permits and
//!   raise cases; and the public register of the types published on it.
//!
//! This app's own design; no code was copied (docs/PROVENANCE.md). Hooks are registered per entity,
//! so a workspace without the `government` package never runs them. No public-sector rule (records
//! retention, accessibility law, a country's e-government standards) is claimed to be met
//! (BLOCK-LEGAL-REVIEW).

mod cases;
mod permits;
mod site;

use chrono::NaiveDate;
use ferrum_domain::reports::ReportSource;
use ferrum_kernel::{Ctx, Doc, Error, HookRegistry, Kernel, Result};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

include!(concat!(env!("OUT_DIR"), "/packages.rs"));
include!(concat!(env!("OUT_DIR"), "/ui.rs"));

/// This app's manifest (`app.yaml`).
pub const MANIFEST: &str = include_str!("../app.yaml");

/// This app's metadata packages, embedded at build time.
pub fn packages() -> std::result::Result<Vec<ferrum_meta::Package>, ferrum_meta::MetaError> {
    ferrum_meta::catalog::load_embedded(EMBEDDED)
}

/// Registers this app's document hooks, and the portal's "Your services".
pub fn register(hooks: &mut HookRegistry) {
    hooks
        .register("PermitApplication", Arc::new(permits::ApplicationHook))
        .register("Permit", Arc::new(permits::PermitHook))
        .register("PermitType", Arc::new(permits::TypeHook))
        .register("PublicCase", Arc::new(cases::CaseHook));
    hooks.add_site("government", Arc::new(site::GovernmentSite));
}

/// This app's reports: none of its own yet.
pub fn reports() -> Vec<Arc<dyn ReportSource>> {
    vec![]
}

/// Upkeep, by each workspace's own day: permits past their expiry expired, holders told when their
/// renewal opens, and late cases marked overdue.
pub async fn maintenance(k: &Kernel) -> Result<()> {
    run(k, None).await
}

/// [`maintenance`] as of `today`, in every workspace.
pub async fn maintenance_on(k: &Kernel, today: NaiveDate) -> Result<()> {
    run(k, Some(today)).await
}

async fn run(k: &Kernel, on: Option<NaiveDate>) -> Result<()> {
    let mut tx = k.db.system_tx().await?;
    let tenants: Vec<Uuid> = sqlx::query_scalar("select id from tenant where status = 'active' order by id").fetch_all(&mut *tx).await?;
    tx.commit().await?;
    let mut first_error: Option<Error> = None;
    for tenant in tenants {
        let run = async {
            let schema = k.registry.schema(tenant).await?;
            if !schema.packages.iter().any(|p| p == "government") {
                return Ok(());
            }
            let ctx = Ctx::system(tenant, schema);
            let today = on.unwrap_or_else(|| ctx.today());
            let mut tx = k.db.tenant_tx(tenant).await?;
            permits::expire(k, &ctx, &mut tx, today).await?;
            permits::remind(k, &ctx, &mut tx, today).await?;
            cases::mark_overdue(k, &ctx, &mut tx, today).await?;
            tx.commit().await?;
            Ok::<(), Error>(())
        };
        if let Err(e) = run.await {
            first_error.get_or_insert(e);
        }
    }
    first_error.map_or(Ok(()), Err)
}

/// A document from JSON.
pub(crate) fn obj(v: Value) -> Doc {
    v.as_object().cloned().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_interface_is_a_release_the_platform_takes() {
        let m = ferrum_apps::AppManifest::parse(MANIFEST).expect("app.yaml");
        assert_eq!(m.routes(), vec![("government".to_string(), vec![])]);
        let files = UI.iter().map(|(p, b)| (p.to_string(), b.to_vec())).collect();
        let ui = ferrum_apps::ui::prepare("government", files).expect("ui/dist is an interface release");
        assert!(UI.iter().any(|(p, _)| *p == "index.html") && !UI.iter().any(|(p, _)| p.starts_with('.')), "{}", ui.digest);
    }
}
