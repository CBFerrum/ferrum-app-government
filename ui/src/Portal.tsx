// Government's page of the workspace's portal (Ferrum docs/plan/apps-ui.md §9.1) at
// /portal/services/: the permits one may apply for, a citizen's own applications with their
// documents (uploaded here) and fees (opening on ERP's "Your invoices"), their permits, and their
// cases, with a case raised. Everything comes from /api/app/government/portal/…, which answers by
// the address the person signed in with.

import { useQuery, useQueryClient } from "@tanstack/react-query";
import { type FormEvent, useState } from "react";
import { api, errorMessage, todayIso } from "@ferrum/client";
import { Badge, Button, ErrorState, Field, PageLoading, statusTone, toast, useShellTitle } from "@ferrum/kit";

const BASE = "/api/app/government/portal";

interface PermitType {
  id: string;
  type_name: string;
  fee?: string | null;
  valid_months?: number | null;
  description?: string | null;
  documents: string[];
}

export interface Application {
  id: string;
  number?: string | null;
  permit_type?: string | null;
  status?: string | null;
  documents: { document?: string | null; file?: string | null }[];
  fee_invoice?: string | null;
  fee_drafted?: boolean;
  decision_note?: string | null;
  stage?: string | null;
  conditions?: string | null;
  renewal_of?: string | null;
  reviews?: { stage?: string | null; outcome?: string | null; reviewed_on?: string | null; findings?: string | null }[];
}

export interface PortalPermit {
  id: string;
  number?: string;
  permit_type?: string;
  expires_on?: string;
  status?: string;
  conditions?: string | null;
  /** From when it may be renewed, for a type that renews; none otherwise. */
  renew_from?: string | null;
  renewal?: string | null;
}

/** Whether a permit may be renewed from the portal on `today`: its type renews, its window is open, and no renewal is under way. */
export function canRenew(p: PortalPermit, today: string): boolean {
  return !!p.renew_from && p.renew_from <= today && !p.renewal;
}

interface Services {
  types: PermitType[];
  applications: Application[];
  permits: PortalPermit[];
  cases: { id: string; number?: string; subject?: string; status?: string; due_on?: string; resolution?: string | null }[];
  categories: { id: string; category_name: string }[];
}

/** The documents a draft still needs before it can be submitted. */
export function stillNeeded(needed: string[], app: Application): string[] {
  const have = new Set(app.documents.filter((d) => d.file).map((d) => String(d.document ?? "").toLowerCase()));
  return needed.filter((n) => !have.has(n.toLowerCase()));
}

/** A file's content as base64, for the upload. */
async function base64(file: File): Promise<string> {
  const bytes = new Uint8Array(await file.arrayBuffer());
  let s = "";
  for (let i = 0; i < bytes.length; i += 0x8000) s += String.fromCharCode(...bytes.subarray(i, i + 0x8000));
  return btoa(s);
}

export function PortalServices() {
  useShellTitle("Your services");
  const qc = useQueryClient();
  const q = useQuery({ queryKey: ["government", "portal"], queryFn: ({ signal }) => api.get<Services>(`${BASE}/home`, {}, signal) });
  const [busy, setBusy] = useState<string | null>(null);
  const refresh = () => qc.invalidateQueries({ queryKey: ["government", "portal"] });
  const run = async (key: string, f: () => Promise<unknown>, done: string) => {
    setBusy(key);
    try {
      await f();
      await refresh();
      toast(done, "success");
    } catch (err) {
      toast(errorMessage(err), "error");
    } finally {
      setBusy(null);
    }
  };
  const [subject, setSubject] = useState("");
  const [category, setCategory] = useState("");
  const [description, setDescription] = useState("");
  if (q.error) return <ErrorState error={q.error} onRetry={() => void q.refetch()} />;
  if (!q.data) return <PageLoading />;
  const { types, applications, permits, cases, categories } = q.data;
  const typeOf = (id?: string | null) => types.find((t) => t.id === id);
  const raise = (e: FormEvent) => {
    e.preventDefault();
    void run("case", () => api.post(`${BASE}/cases`, { category: category || categories[0]?.id, subject: subject.trim(), description: description.trim() }), "Your case is with the team").then(() => {
      setSubject("");
      setDescription("");
    });
  };
  return (
    <div className="stack np-page">
      <div className="page-head">
        <div>
          <h1 className="page-title">Your services</h1>
          <p className="page-sub">Apply for permits, follow your applications, and raise cases.</p>
        </div>
      </div>
      <section className="card card-pad stack" aria-label="Apply for a permit">
        <h3>Apply for a permit</h3>
        <ul className="stack">
          {types.map((t) => (
            <li key={t.id} className="row" style={{ justifyContent: "space-between" }}>
              <span>
                <strong>{t.type_name}</strong> · fee {t.fee ?? "0"} · {t.valid_months === 1 ? "1 month" : `${t.valid_months} months`}{t.documents.length ? ` · needs ${t.documents.join(", ")}` : ""}
              </span>
              <Button loading={busy === `apply-${t.id}`} onClick={() => void run(`apply-${t.id}`, () => api.post(`${BASE}/applications`, { permit_type: t.id }), "Draft started")}>
                Apply
              </Button>
            </li>
          ))}
        </ul>
      </section>
      {applications.map((a) => {
        const t = typeOf(a.permit_type);
        const needed = stillNeeded(t?.documents ?? [], a);
        return (
          <section key={a.id} className="card card-pad stack" aria-label={`Application ${a.number ?? ""}`}>
            <header className="row" style={{ justifyContent: "space-between" }}>
              <h3>
                {t?.type_name ?? "Permit"} · {a.number}
              </h3>
              <Badge tone={statusTone(a.status)}>{a.status}</Badge>
            </header>
            {a.decision_note && <p>{a.decision_note}</p>}
            {a.status === "Under Review" && a.stage && <p className="muted">Waiting for: {a.stage}</p>}
            {(a.reviews ?? []).length > 0 && (
              <ul className="stack">
                {(a.reviews ?? []).map((r, i) => (
                  <li key={i}>
                    {r.stage} · {r.outcome} · {r.reviewed_on}
                    {r.findings ? ` · found: ${r.findings}` : ""}
                  </li>
                ))}
              </ul>
            )}
            {a.conditions && <p>On condition: {a.conditions}</p>}
            {a.fee_invoice && <a href={`/portal/invoices/${a.fee_invoice}`}>The fee's invoice</a>}
            {a.fee_drafted && <p className="muted">The fee's invoice is not issued yet.</p>}
            {a.status === "Draft" && (
              <>
                {(t?.documents ?? []).map((d) => (
                  <Field key={d} label={`${d}${needed.includes(d) ? " (still needed)" : " ✓"}`}>
                    <input
                      type="file"
                      className="input"
                      onChange={(e) => {
                        const file = e.target.files?.[0];
                        if (file)
                          void run(
                            `doc-${a.id}-${d}`,
                            async () => api.post(`${BASE}/applications/${a.id}/documents`, { document: d, file_name: file.name, content_type: file.type, data: await base64(file) }),
                            `${d} added`,
                          );
                      }}
                    />
                  </Field>
                ))}
                <div className="row">
                  <Button variant="primary" disabled={needed.length > 0} loading={busy === `submit-${a.id}`} onClick={() => void run(`submit-${a.id}`, () => api.post(`${BASE}/applications/${a.id}/submit`, {}), "Submitted")}>
                    Submit
                  </Button>
                  <Button loading={busy === `withdraw-${a.id}`} onClick={() => void run(`withdraw-${a.id}`, () => api.post(`${BASE}/applications/${a.id}/withdraw`, {}), "Withdrawn")}>
                    Withdraw
                  </Button>
                </div>
              </>
            )}
          </section>
        );
      })}
      {permits.length > 0 && (
        <section className="card card-pad stack" aria-label="Your permits">
          <h3>Your permits</h3>
          <ul className="stack">
            {permits.map((p) => (
              <li key={p.id} className="stack" style={{ gap: 4 }}>
                <span>
                  {p.number} · {typeOf(p.permit_type)?.type_name ?? ""} · until {p.expires_on} <Badge tone={statusTone(p.status)}>{p.status}</Badge>
                </span>
                {p.conditions && <span className="muted">On condition: {p.conditions}</span>}
                {canRenew(p, todayIso()) && (
                  <span>
                    <Button loading={busy === `renew-${p.id}`} onClick={() => void run(`renew-${p.id}`, () => api.post(`${BASE}/permits/${p.id}/renew`, {}), "Renewal started")}>
                      Renew it
                    </Button>
                  </span>
                )}
                {!canRenew(p, todayIso()) && p.renew_from && !p.renewal && <span className="muted">It may be renewed from {p.renew_from}.</span>}
              </li>
            ))}
          </ul>
        </section>
      )}
      <section className="card card-pad stack" aria-label="Your cases">
        <h3>Your cases</h3>
        <ul className="stack">
          {cases.map((c) => (
            <li key={c.id}>
              {c.number} · {c.subject} · due {c.due_on} <Badge tone={statusTone(c.status)}>{c.status}</Badge>
              {c.resolution ? ` · ${c.resolution}` : ""}
            </li>
          ))}
        </ul>
        <form className="stack" onSubmit={raise}>
          <Field label="About">
            <select className="input" value={category || categories[0]?.id || ""} onChange={(e) => setCategory(e.target.value)}>
              {categories.map((c) => (
                <option key={c.id} value={c.id}>
                  {c.category_name}
                </option>
              ))}
            </select>
          </Field>
          <Field label="Subject">
            <input className="input" value={subject} maxLength={300} required onChange={(e) => setSubject(e.target.value)} />
          </Field>
          <Field label="What happened">
            <textarea className="input" rows={3} value={description} onChange={(e) => setDescription(e.target.value)} />
          </Field>
          <Button type="submit" variant="primary" loading={busy === "case"} disabled={!subject.trim() || !categories.length}>
            Raise a case
          </Button>
        </form>
      </section>
    </div>
  );
}
