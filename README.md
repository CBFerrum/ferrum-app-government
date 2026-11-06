# Government

Public services for [Ferrum](https://github.com/CBFerrum/Ferrum):
- permits applied for, reviewed, paid for through ERP and issued with their expiry;
- citizens' cases, each due within its category's days;
- the portal's "Your services", where citizens apply, upload their documents and raise cases.

| | |
|---|---|
| 🏛️ App | `government`, native, in the server |
| 🧩 Package | `government` (needs ERP's selling and accounting) |
| 🖥️ Interface | its own, at `/government/`: the officers' queues, and the records; the portal's "Your services" at `/portal/services/` |
| 🔐 Roles | Permit Officer, Case Officer, Services Manager; citizens on the portal |
| 📜 Licence | AGPL-3.0-only |

No public-sector rule is claimed to be met until a legal review says so: records retention,
accessibility law, or a country's e-government standards.

## Highlights

- **Applying.** A citizen applies on the portal, or at the counter. They upload each document the
  permit's type needs; sending one again replaces the first. A closed type takes no applications.
- **Submitting.** An application is submitted only with every document the type needs. Its fee,
  if any, is then invoiced through ERP to a customer made from the applicant, or the one their
  address already names. Once submitted, it keeps what was applied for.
- **Review.** An officer takes it up. It is approved only once the fee's invoice is submitted and
  paid. A rejection says why, and the citizen sees it.
- **Permits.** An approved application is issued as a permit, valid for the type's months. A
  permit expires by itself, or is revoked saying why.
- **Cases.** A case opens the day it is raised and is due its category's days later. Resolved or
  closed, it says how, and keeps the day. Upkeep marks the open ones past their day overdue.
- **Stages.** A type's review stages are passed in order, each by someone holding its role and never
  by whoever passed the one before; a site inspection says what it found. Stages marked Alongside
  The One Before are passed side by side, in any order, each by its own people and no one passing
  two of them; the application waits for all of them before the next stage. Approval waits for
  every stage, and may set conditions the permit keeps.
- **Renewals and notices.** A renewable permit is renewed from its type's days before it ends, by the
  officer or on the portal; the new permit runs on from the day after. Applicants are told as their
  application moves, and holders once when their renewal opens.
- **The public register.** The permits of the types published on it, with holder, days, status and
  conditions, at `/api/site/{workspace}/government/register` (JSON, or CSV with `format=csv`).
- **Your services.** A citizen sees only their own applications, permits and cases, with the fee's
  invoice opening on ERP's "Your invoices".

## Develop

```bash
cargo test                       # needs FERRUM_GOVERNMENT_TEST_DATABASE_URL's database
cd ui && npm run build && npm run check
```

`tests/government_it.rs` walks a town hall:
- a building permit applied for on the portal, its documents uploaded, its fee paid, and issued;
- a rejection, a permit that expires, and a withdrawal;
- cases resolved and overdue;
- what each citizen sees;
- who may do what.

## Not yet

- Conditions checked after issue.
- Paying the fee on the portal (ERP's invoices page shows what is due), and SMS (notices go by the
  person's own choice of e-mail or the bar).
- A register of more than its latest 1,000 permits at once, and open data beyond JSON and CSV.
- Any claim to meet public-sector rules (BLOCK-LEGAL-REVIEW).

## Changelog

| Version | Highlights |
|---|---|
| **0.2.5** | Review stages side by side: passed in any order, each by its own people. |
| **0.2.4** | Rebuilt with Ferrum-Frontend a64416a: a box whose default is 1 shows ticked. |
| **0.2.3** | The portal shows a permit type's fee as money, or "no fee". |
| **0.2.2** | Rebuilt with Ferrum-Frontend 922fcfb: action dialogs say every refusal and ask once. |
| **0.2.1** | Rebuilt with Ferrum-Frontend a76295e: times on a person's own clock when they chose one. |
| **0.2.0** | Review stages with site inspections, conditions, renewals, notices to applicants and holders, a public register, and the workspace's day. |
| **0.1.8** | Rebuilt with Ferrum-Frontend 24c3951: times on the workspace's clock; drafts headed as drafts. |
| **0.1.7** | Rebuilt with Ferrum-Frontend bb60115: empty numbers read as the server reads them; settings open as their record. |
| **0.1.6** | Rebuilt with Ferrum-Frontend 00dea07: a workflow state's kept fields are read-only, its needed ones required. |
| **0.1.5** | Personal-data requests find applicants, permit holders and citizens; drafts erased, decided records kept as public records. |
| **0.1.4** | Rebuilt with Ferrum-Frontend daeadda: actions say what they did. |
| **0.1.3** | The fee's invoice is linked once issued; until then the application says it is not issued yet. |
| **0.1.2** | Rebuilt with Ferrum-Frontend 80ff179: labels name their fields, and linked titles show their titles. |
| **0.1.1** | One month says "1 month", and the citizens' forms are named. |
| **0.1.0** | Permits applied for, reviewed, paid through ERP and issued with their expiry, cases with deadlines, and citizens' "Your services" on the portal (FERRUM-INDUSTRY-010). |

## License

AGPL-3.0-only. See [LICENSE](LICENSE).
