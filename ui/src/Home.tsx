// The officers' queues: applications waiting to be taken up, those under review, approved ones to
// issue, and cases open, the overdue first. Each opens its record.

import { useQuery } from "@tanstack/react-query";
import { Link } from "react-router";
import { api, docPath } from "@ferrum/client";
import { Badge, ErrorState, PageLoading, useShellTitle } from "@ferrum/kit";

export interface Row {
  id: string;
  [k: string]: unknown;
}

const rows = (entity: string, filters: unknown[], extra: Record<string, string | number> = {}) =>
  api.get<{ rows: Row[] }>(docPath(entity), { filters: JSON.stringify(filters), limit: 500, ...extra }).then((r) => r.rows);

/** Open cases, the overdue first, then by their day. */
export function byUrgency(cases: Row[]): Row[] {
  return [...cases].sort((a, b) => Number(Boolean(b.overdue)) - Number(Boolean(a.overdue)) || String(a.due_on ?? "").localeCompare(String(b.due_on ?? "")));
}

function Queue({ title, list, entity, label }: { title: string; list: Row[]; entity: string; label: (r: Row) => string }) {
  return (
    <section className="card card-pad stack" aria-label={title}>
      <h3>
        {title} ({list.length})
      </h3>
      <ul className="stack">
        {list.map((r) => (
          <li key={r.id}>
            <Link to={`/e/${entity}/${r.id}`}>{label(r)}</Link> {r.overdue ? <Badge tone="red">Overdue</Badge> : null}
          </li>
        ))}
      </ul>
    </section>
  );
}

export function Home() {
  useShellTitle("Government");
  const q = useQuery({
    queryKey: ["government", "queues"],
    queryFn: async () => {
      const [submitted, review, approved, cases] = await Promise.all([
        rows("PermitApplication", [["status", "eq", "Submitted"]]),
        rows("PermitApplication", [["status", "eq", "Under Review"]]),
        rows("PermitApplication", [["status", "eq", "Approved"]]),
        rows("PublicCase", [["status", "in", ["Open", "In Progress"]]]),
      ]);
      return { submitted, review, approved, cases: byUrgency(cases) };
    },
    refetchInterval: 30_000,
  });
  if (q.isPending) return <PageLoading />;
  if (q.isError) return <ErrorState error={q.error} onRetry={() => void q.refetch()} />;
  const app = (r: Row) => `${String(r.number ?? "")} · ${String(r.applicant_name ?? "")}`;
  return (
    <div className="np-page">
      <h2>Queues</h2>
      <div className="np-columns">
        <Queue title="To take up" list={q.data.submitted} entity="PermitApplication" label={app} />
        <Queue title="Under review" list={q.data.review} entity="PermitApplication" label={app} />
        <Queue title="To issue" list={q.data.approved} entity="PermitApplication" label={app} />
        <Queue title="Open cases" list={q.data.cases} entity="PublicCase" label={(r) => `${String(r.number ?? "")} · ${String(r.subject ?? "")} · due ${String(r.due_on ?? "")}`} />
      </div>
    </div>
  );
}
