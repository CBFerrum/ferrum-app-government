// The officers' queues: applications waiting to be taken up, those under review, approved ones to
// issue, and cases open, the overdue first. Each opens its record.

import { useQuery } from "@tanstack/react-query";
import { api, docPath, formatValue, t, isolate } from "@ferrum/client";
import { Badge, HomeSkeleton, LoadFailed, PageHead, Queue, QueueGrid, QueueItem, useShellTitle } from "@ferrum/kit";

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

/** A day as the person reads it ("20 Nov 2026"); empty when there is none. */
const day = (v: unknown) => formatValue({ type: "date" }, v);

/** The queues show this many rows each; the rest is "and N more", linking to the list. */
const LIMIT = 5;

export function Home() {
  useShellTitle(t("Government"));
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
  if (q.isPending) return (
    <>
      <PageHead title={t("Queues")} />
      <HomeSkeleton queues={4} />
    </>
  );
  if (q.isError) return (
    <>
      <PageHead title={t("Queues")} />
      <LoadFailed error={q.error} onRetry={() => void q.refetch()} />
    </>
  );
  const { submitted, review, approved, cases } = q.data;
  // An application is its applicant's name with its number beneath; one under review also says who it waits for.
  const application = (r: Row, meta: string) => <QueueItem key={r.id} to={`/e/PermitApplication/${r.id}`} title={isolate(String(r.applicant_name || r.number || ""))} meta={isolate(meta)} />;
  const number = (r: Row) => String(r.number ?? "");
  const late = cases.filter((c) => c.overdue).length;
  return (
    <>
      <PageHead title={t("Queues")} description={submitted.length + cases.length > 0 ? t("{waiting} to take up · {cases} open cases", { waiting: submitted.length, cases: cases.length }) : undefined} />
      <QueueGrid>
        <Queue title={t("Open cases")} count={cases.length} tone={late ? "attention" : "default"} more={{ to: "/e/PublicCase", count: cases.length - LIMIT }}>
          {cases.slice(0, LIMIT).map((c) => (
            <QueueItem key={c.id} to={`/e/PublicCase/${c.id}`} title={isolate(String(c.subject ?? c.number ?? ""))} meta={isolate([number(c), c.due_on ? t("due {day}", { day: day(c.due_on) }) : ""].filter(Boolean).join(" · "))} trailing={c.overdue ? <Badge tone="red">{t("Overdue")}</Badge> : null} />
          ))}
        </Queue>
        <Queue title={t("To take up")} count={submitted.length} more={{ to: "/e/PermitApplication", count: submitted.length - LIMIT }}>
          {submitted.slice(0, LIMIT).map((r) => application(r, number(r)))}
        </Queue>
        <Queue title={t("To issue")} count={approved.length} more={{ to: "/e/PermitApplication", count: approved.length - LIMIT }}>
          {approved.slice(0, LIMIT).map((r) => application(r, number(r)))}
        </Queue>
        <Queue title={t("Under review")} count={review.length} more={{ to: "/e/PermitApplication", count: review.length - LIMIT }}>
          {review.slice(0, LIMIT).map((r) => application(r, r.stage ? `${number(r)} · ${t("waiting for {stage}", { stage: String(r.stage) })}` : number(r)))}
        </Queue>
      </QueueGrid>
    </>
  );
}
