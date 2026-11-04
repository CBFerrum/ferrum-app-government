//! Cases. A case opens on the day it is raised and is due its category's days later. Resolved or
//! closed, it says how, and keeps the day; upkeep marks the open ones past their day overdue.

use crate::obj;
use async_trait::async_trait;
use chrono::{Duration, NaiveDate, Utc};
use ferrum_domain::util::{id, s};
use ferrum_kernel::{Ctx, Doc, DocHook, Error, HookCtx, Kernel, Result, Tx};
use serde_json::{Value, json};
use uuid::Uuid;

pub struct CaseHook;

fn day(doc: &Doc, k: &str) -> Option<NaiveDate> {
    s(doc, k).and_then(|v| NaiveDate::parse_from_str(v.get(..10).unwrap_or(v), "%Y-%m-%d").ok())
}

#[async_trait]
impl DocHook for CaseHook {
    fn name(&self) -> &'static str {
        "government.case"
    }

    fn fills(&self) -> &'static [&'static str] {
        &["opened_on", "due_on", "resolved_on", "overdue"]
    }

    async fn before_validate(&self, hc: &mut HookCtx<'_>, doc: &mut Doc) -> Result<()> {
        let today = Utc::now().date_naive();
        let opened = day(doc, "opened_on").unwrap_or(today);
        let days: Option<i64> = sqlx::query_scalar("select days_to_resolve from e_case_category where tenant_id = $1 and id = $2")
            .bind(hc.ctx.tenant)
            .bind(id(doc, "category"))
            .fetch_optional(&mut **hc.tx)
            .await?
            .flatten();
        let due = opened + Duration::days(days.unwrap_or(10).max(0));
        doc.insert("opened_on".into(), json!(opened.to_string()));
        doc.insert("due_on".into(), json!(due.to_string()));
        let done = matches!(s(doc, "status"), Some("Resolved" | "Closed"));
        if done && doc.get("resolved_on").is_none_or(Value::is_null) {
            doc.insert("resolved_on".into(), json!(today.to_string()));
        }
        if !done {
            doc.insert("resolved_on".into(), Value::Null);
        }
        doc.insert("overdue".into(), json!(!done && due < today));
        Ok(())
    }

    async fn validate(&self, _hc: &mut HookCtx<'_>, doc: &Doc) -> Result<()> {
        if matches!(s(doc, "status"), Some("Resolved" | "Closed")) && s(doc, "resolution").is_none() {
            return Err(Error::field("resolution", "a resolved case says how"));
        }
        Ok(())
    }
}

/// Open cases past their day, marked overdue.
pub(crate) async fn mark_overdue(k: &Kernel, ctx: &Ctx, tx: &mut Tx, today: NaiveDate) -> Result<()> {
    let late: Vec<Uuid> = sqlx::query_scalar(
        "select id from e_public_case where tenant_id = $1 and status in ('Open', 'In Progress') and due_on < $2 and not coalesce(overdue, false)",
    )
    .bind(ctx.tenant)
    .bind(today)
    .fetch_all(&mut **tx)
    .await?;
    for case in late {
        k.set_values_tx(ctx, tx, "PublicCase", case, obj(json!({"overdue": true}))).await?;
    }
    Ok(())
}
