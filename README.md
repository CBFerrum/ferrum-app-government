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

- Several reviewers or stages, site inspections, and conditions on a permit.
- Renewing a permit, and paying the fee on the portal (ERP's invoices page shows what is due).
- Notifying citizens by mail or SMS as their application moves.
- A public register of permits, and open data.
- Any claim to meet public-sector rules (BLOCK-LEGAL-REVIEW).

## Changelog

| Version | Highlights |
|---|---|
| **0.1.2** | Rebuilt with Ferrum-Frontend 80ff179: labels name their fields, and linked titles show their titles. |
| **0.1.1** | One month says "1 month", and the citizens' forms are named. |
| **0.1.0** | Permits applied for, reviewed, paid through ERP and issued with their expiry, cases with deadlines, and citizens' "Your services" on the portal (FERRUM-INDUSTRY-010). |

## License

AGPL-3.0-only. See [LICENSE](LICENSE).
