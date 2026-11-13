<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/assets/banner-dark.svg">
  <img alt="Ferrum Government: permits, reviews and citizens' cases for a public office" src="docs/assets/banner-light.svg" width="100%">
</picture>

<br>

[![Version](https://img.shields.io/badge/version-0.3.2-4338ca?style=flat-square)](CHANGELOG.md)
[![Licence: AGPL-3.0](https://img.shields.io/badge/licence-AGPL--3.0-blue?style=flat-square)](LICENSE)
[![Ferrum platform](https://img.shields.io/badge/runs%20on-Ferrum-2f5bea?style=flat-square)](https://github.com/CBFerrum/Ferrum)
![Rust](https://img.shields.io/badge/Rust-server-b7410e?style=flat-square&logo=rust)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-data-336791?style=flat-square&logo=postgresql&logoColor=white)
![React](https://img.shields.io/badge/React%20%2B%20TypeScript-interface-087ea4?style=flat-square&logo=react&logoColor=white)
![Tests](https://img.shields.io/badge/tests-4%20Rust%20%2B%20UI%20suite-16a34a?style=flat-square)

### Permits applied for, reviewed in stages, paid through the books, issued and renewed, with a public register.

Ferrum Government is for a municipality, a licensing board or any public office that takes applications and
citizens' requests. Citizens apply and follow their applications on the portal. Officers review in stages, and fees are
invoiced through Ferrum ERP. Cases are due within their category's days.

[💡 Why](#-why-government) · [📸 Screenshots](#-screenshots) · [✨ Features](#-features) · [📊 Compare](#-how-it-compares) · [📍 Roadmap](#-roadmap) · [🚀 Install](#-install) · [🔧 Develop](#-develop)

</div>

> [!IMPORTANT]
> No public-sector rule is claimed to be met: not records retention, accessibility law, or any country's
> e-government standard. That needs a legal review of how an office runs the app.

## 💡 Why Government

- 🧑‍⚖️ **Reviews that cannot be rubber-stamped.** Review stages are passed in order, each by someone holding the
  stage's role and never by whoever passed the stage before. Stages marked *alongside* are passed side by side, each by
  different people.
- 💳 **The fee is part of the process.** The fee is invoiced through ERP when an application is submitted, and an
  application is approved only once that invoice is paid.
- 🔁 **From application to renewal.** Approval, conditions, issue, expiry, revocation and renewal are all in one record
  trail, and applicants and holders are told as things move.
- ⚖️ **A way back.** A rejection can be appealed within its type's days and is decided by someone who took no part in
  the decision.
- 🌐 **A public register.** Permits of the types published on it, as JSON or CSV, with only what each type allows
  and for as long as it allows.
- 🔓 **Open source and self-hosted.** AGPL-3.0, on infrastructure the office controls.
- 🌍 **Arabic and Hindi** words and names, with right-to-left layout.

## 🎯 At a glance

| | |
|---|---|
| **Version** | 0.3.2 |
| **Kind** | Its own interface at `/government/` (the officers' queues); the portal's "Your services" at `/portal/services/`; every record in Ferrum's lists and forms |
| **What it adds** | 10 record types (5 documents, 4 child tables, 1 settings record), 3 roles, 1 menu |
| **Depends on** | Ferrum platform `>=0.3.2, <0.4`; ERP `>=0.1` (installing Government installs ERP) |
| **Works with** | ERP (customers, items, companies, sales invoices), the platform's notifications and portal |
| **Languages** | English, Arabic, Hindi (words and record names); right-to-left layout |
| **Tests** | 4 Rust tests (one walks a town hall on PostgreSQL) and a UI suite |
| **Licence** | AGPL-3.0-only |

## 📸 Screenshots

<div align="center">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/government-home-dark.webp"><img alt="The officers&#x27; queues" src="docs/assets/screens/government-home-light.webp" width="100%"></picture>
<p><b>🏛️ The officers&#x27; queues</b><br><sub>Open cases, applications to take up, permits to issue and applications under review, each with a count.</sub></p>
</div>

<table>
<tr>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/government-public-case-list-dark.webp"><img alt="Cases" src="docs/assets/screens/government-public-case-list-light.webp"></picture>
<p><b>📨 Cases</b><br><sub>Citizens&#x27; cases with their category, status, due day and whether they are overdue.</sub></p>
</td>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/government-permit-type-list-dark.webp"><img alt="Permit types" src="docs/assets/screens/government-permit-type-list-light.webp"></picture>
<p><b>🗂️ Permit types</b><br><sub>Each permit&#x27;s fee, how long it lasts and whether people can apply for it now.</sub></p>
</td>
</tr>
<tr>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/government-permit-application-list-dark.webp"><img alt="Permit applications" src="docs/assets/screens/government-permit-application-list-light.webp"></picture>
<p><b>📥 Permit applications</b><br><sub>Every application with its type, applicant, status and officer.</sub></p>
</td>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/government-public-case-detail-dark.webp"><img alt="One case" src="docs/assets/screens/government-public-case-detail-light.webp"></picture>
<p><b>🔎 One case</b><br><sub>Read a case, its due day and its resolution.</sub></p>
</td>
</tr>
<tr>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/government-permit-application-detail-dark.webp"><img alt="One application" src="docs/assets/screens/government-permit-application-detail-light.webp"></picture>
<p><b>🧾 One application</b><br><sub>Follow an application through its review stages to a decision.</sub></p>
</td>
<td width="50%" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/government-permits-dark.webp"><img alt="Permits" src="docs/assets/screens/government-permits-light.webp"></picture>
<p><b>🪪 Permits</b><br><sub>The permits issued, when they expire and whether they are still valid.</sub></p>
</td>
</tr>
</table>

### 📱 On a phone

<table>
<tr>
<td align="center" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/government-home-phone-dark.webp"><img alt="Queues on a phone" src="docs/assets/screens/government-home-phone-light.webp" width="260"></picture>
<p><b>🏛️ Queues on a phone</b><br><sub>The officers&#x27; queues on a narrow screen.</sub></p>
</td>
<td align="center" valign="top">
<picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/screens/government-public-case-list-phone-dark.webp"><img alt="Cases on a phone" src="docs/assets/screens/government-public-case-list-phone-light.webp" width="260"></picture>
<p><b>📨 Cases on a phone</b><br><sub>Check a case from the counter or the street.</sub></p>
</td>
</tr>
</table>


## ✨ Features

### 🗂️ Permit types

- **A permit people apply for.** A type has a name, a fee, the item the fee is invoiced as (the settings' when empty), the
  months a permit lasts, and a description. A type that is not active takes no applications.
- **Documents needed.** The type lists each document an applicant must provide.
- **Review stages.** The type lists stages in order. Each stage has a name, the role whose people may pass it (any permit
  officer when empty), a *Site Inspection* flag, and an *Alongside The One Before* flag. Stage names are unique.
- **Renewal.** A type may be renewable, from a chosen number of days before a permit ends (60 by default).
- **Appeals.** A type says how many days after a rejection an appeal may be made (30 by default; 0 allows none).
- **The public register.** A type says whether its permits are on the register, whether holders' names and conditions
  are published (both left out by default), and for how many months an expired or revoked permit stays listed (12 by
  default; empty keeps it for good).

### 📥 Permit applications

- **Applied for on the portal or at the counter.** An application has an applicant's name and email, details, and
  documents with a file each.
- **Eight states.** *Draft, Submitted, Under Review, Approved, Rejected, Appealed, Issued, Withdrawn.* Submit It, Take It
  Up, Pass This Stage, Approve It, Reject It, Appeal the Rejection, Decide the Appeal, Issue the Permit and Withdraw It
  show only when they apply.
- **Submitted complete.** An application is submitted only with every document the type needs, each with its file, and
  only for a type that is open. Once submitted, it keeps what was applied for.
- **The fee.** Submitting invoices the fee through ERP to the customer the applicant's address already names, or one made
  from them. Without a fee, no invoice is made. Without a company and a fee item in Government Settings, submitting is
  refused with a message that says where to set them.
- **Taken up.** An officer takes a submitted application up and their email is kept on it. The "Waiting For" field says
  which stage it is at.
- **Passing stages.** Each stage is passed by someone holding its role. A person cannot pass the stage right after one
  they passed, nor two stages that are side by side. Each pass records the stage, outcome, reviewer, day and a note.
- **Site inspections** must say what they found.
- **Approval.** An application is approved only when every stage is passed and its fee is paid. Approval can set
  conditions the permit keeps. The decision, its day and who made it are recorded.
- **Rejection** says why, and the citizen sees it.
- **Appeal.** A rejected application can be appealed once, within its type's days, with grounds, on the portal or at the
  counter. It is decided by someone who is not its officer, not who decided it, and not who reviewed it. *Upheld*: the
  rejection stands. *Overturned*: it is approved as an approval would be, then issued. The applicant is told either way.
- **Issue.** An approved application is issued as a permit.
- **Withdraw.** A draft or submitted application can be withdrawn.
- **Numbered** `APP-2026-00001`.

### 🪪 Permits

- **A permit** holds its type, application, holder name and email, issue and expiry dates, and any conditions.
- **Valid, Expired, Revoked.** A permit is valid for the type's months from issue. Upkeep expires permits past their end.
  An officer revokes a valid permit, saying why, and the day is kept.
- **Renewal.** A renewable permit is renewed from its type's days before it ends, by an officer or the holder on the
  portal. The renewal is an application of its own for the holder to complete and submit; the permit issued for it
  starts the day after the old one ends. It is refused for a type that does not renew, before its window, or while
  another renewal is live.
- **Renewal notices.** Upkeep tells a holder once when their renewal opens.
- **Numbered** `PRM-2026-00001`.

### 🌐 The public register

- **No sign-in.** The permits of types published on the register, at `/api/site/{workspace}/government/register`,
  as JSON or CSV (`format=csv`), filtered by type, newest first, up to 1,000 at once.
- **Only what each type allows.** Holder names and conditions appear only where the type publishes them. Expired and
  revoked permits stay for the months the type says.
- **Spreadsheet-safe CSV.** Cells that a spreadsheet would run as formulas are made plain.
- **Types from before** a type could say what it publishes keep publishing as they did, until an administrator changes it.

### 📨 Citizens' cases

- **A case** has a subject, a category, the citizen's name and email, a description, a status (*Open, In Progress,
  Resolved, Closed*), and an officer.
- **Due by category.** A case opens on the day it is raised and is due its category's number of days later (10 by
  default).
- **Resolved says how.** Resolving or closing a case needs a resolution, and its day is kept. Reopening clears it.
- **Overdue.** Upkeep marks open and in-progress cases past their day as overdue, by the workspace's own day.
- **Raised on the portal.** A citizen raises a case with a category, a subject and a description.
- **Numbered** `CASE-2026-00001`.

### 🧑‍💻 Your services, the citizens' portal

- **At `/portal/services/`.** A citizen sees the permits they may apply for, their own applications with their review
  stages and conditions, their permits and their cases. Nothing of anyone else's.
- **Apply, upload, submit.** They draft an application, upload each document (up to 5 MB each; sending one again replaces
  the first), submit or withdraw it, and appeal a rejection.
- **Renew.** A holder renews a permit once its renewal opens.
- **Raise a case.**
- **The fee.** The fee's invoice is linked once it is issued, and opens on ERP's "Your invoices" page.
- **Told as it moves.** Applicants and holders are notified by email or in the portal, by their own choice.
- **Roles are not needed.** What belongs to a citizen is what carries the address they signed in with.

### 🔔 Notifications

- Applicants are told as their application moves; holders once when a renewal may begin; officers when an application
  is appealed. A person chooses email, the portal bar, or both.

### 📊 The officers' home

- **Queues.** Open cases (flagged when any is late), applications to take up, applications to issue, and applications
  under review with the stage each waits for. Each item opens its record, and each queue links to the full list.
- **Summary.** A line says how many applications are waiting to be taken up and how many cases are open.

### 🔐 Roles and permissions

| Who | Can |
|---|---|
| 📋 Permit Officer | Reads permit types; creates, edits and exports applications; edits and exports permits; reads sales invoices |
| 🗃️ Case Officer | Reads case categories; creates, edits and exports cases |
| 🧭 Services Manager | Sets up permit types, case categories and Government Settings; reads applications, permits and cases; reads items and companies |
| 🙋 Citizens on the portal | Their own applications, permits and cases, found by the address they sign in with |

### 🔏 Personal data

- **Erased or kept.** The platform's personal-data requests find applicants, permit holders and citizens. Draft and
  withdrawn applications are erased (name, email, details, documents). Submitted and decided applications, permits and
  cases are kept, with a stated reason: they are public records.

### 🌍 Interface

- **Its own pages.** The officers' queues, and the portal's Your services; every record uses the shared list and form pages.
- **Languages.** English, Arabic and Hindi words, and Arabic and Hindi names for every record and field. A workspace can
  add its own words in Admin.
- **Right to left.** Pages mirror as they read.
- **Assistant.** It knows the states and where figures come from, and suggests "Which applications wait for review, oldest
  first?", "Which permits expire next month?" and "Which cases are past their deadline?".

## 🧭 How it works

```mermaid
flowchart LR
  D["Draft"] --> S["Submitted: documents complete, fee invoiced"]
  S --> R["Under review: stages in order"]
  R --> A["Approved: fee paid, conditions set"]
  R --> X["Rejected, with why"]
  X --> AP["Appealed"]
  AP -->|overturned| A
  AP -->|upheld| X
  A --> I["Issued as a permit"]
  I --> E["Expires or is revoked"]
  I --> RN["Renewed from the type's window"]
```

An application is complete when submitted. Each stage is passed by a person with the stage's role who did not pass
the one before. The fee must be paid before approval. An issued permit expires by itself, may be revoked, and may be
renewed.

```mermaid
flowchart TD
  C["Citizen on the portal"] --> APP["Application and documents"]
  C --> CASE["Case raised"]
  APP --> ERP["Fee invoice in ERP"]
  ERP --> PAY["Paid in ERP"]
  PAY --> APP
  APP --> PERMIT["Permit"]
  PERMIT --> REG["Public register (JSON or CSV)"]
  CASE --> CAT["Category: due in n days"]
```

Fees live in ERP and are paid there. The register publishes only what each permit type allows.

## 🧱 Data model

| Entity | What it is | Key fields or states |
|---|---|---|
| Permit Type | A permit people apply for | Fee, fee item, valid months, documents needed, review stages, renewable, renew within days, appeal within days, register options; active |
| Document Needed, Review Stage | Rows of a permit type | Document; stage, role, site inspection, alongside |
| Permit Application | A request for a permit | Type, applicant, details, documents, officer, customer, fee invoice, waiting for, reviews, conditions, decision and appeal fields, permit; *Draft, Submitted, Under Review, Approved, Rejected, Appealed, Issued, Withdrawn* |
| Document, Review | Rows of an application | File; stage, outcome, reviewer, day, note, findings |
| Permit | A permit issued | Type, application, holder, issued and expiry dates, conditions, renewal links; *Valid, Expired, Revoked* |
| Case Category | A kind of case | Name, days to resolve |
| Case (`PublicCase`) | Something a citizen raised | Subject, category, citizen, officer, opened, due, resolution, overdue; *Open, In Progress, Resolved, Closed* |
| Government Settings | Where fees are invoiced from | Company, fee item |

**Roles:** Permit Officer, Case Officer, Services Manager; citizens on the portal.

**Reports and print formats:** none yet.

**Scheduled upkeep (by the workspace's day):** expire permits, tell holders when a renewal opens, mark late cases overdue.

## 🔌 Integrations

| With | How | Needs |
|---|---|---|
| **Ferrum ERP** | Customers, items and companies; fees become Sales Invoices; approval waits for payment | ERP installed with the app; a company and a fee item in Government Settings |
| **Ferrum portal** | "Your services" for citizens | Portal enabled for the workspace |
| **Notifications** | Email and in-app notices to applicants and holders | Outgoing mail configured for email |
| **Public register** | JSON and CSV over HTTP, for websites and spreadsheets | Nothing |
| **Personal-data requests** | Declared per record, with what is erased and what is kept | Nothing |

Not built in: online fee payment on the portal, SMS, GIS or maps, e-signature, hearings or agendas.

## 📊 How it compares

Government is compared with **Tyler Technologies** (EnerGov and its permitting and licensing products), **Accela**,
**OpenGov**, **Salesforce Public Sector Solutions** and **CivicPlus**. These are large suites, mostly sold to local and state
governments in the United States. There is no widely used open-source permitting suite to put beside them.

| Capability | Ferrum Government | Tyler | Accela | OpenGov | Salesforce Public Sector | CivicPlus |
|---|:-:|:-:|:-:|:-:|:-:|:-:|
| Permit types with documents, fee and validity | ✅ | ✅ | ✅ | ✅ | ✅ | 🟡 |
| Online application with document upload on a citizen portal | ✅ | ✅ | ✅ | ✅ | ✅ | 🟡 |
| Application forms specific to each permit type | ❌ | ✅ | ✅ | ✅ | ✅ | 🟡 |
| Staged review in order and side by side, by role | ✅ | ✅ | ✅ | ✅ | 🟡 | 🟡 |
| No one passes two stages in a row | ✅ | 🟡 | 🟡 | 🟡 | 🟡 | ❌ |
| Fee invoiced on submission and paid before approval | ✅ | ✅ | ✅ | ✅ | 🟡 | 🟡 |
| Fee schedules by valuation, area or units | ❌ | ✅ | ✅ | ✅ | 🟡 | 🟡 |
| Online payment of fees on the portal | ❌ | ✅ | ✅ | ✅ | 🟡 | ✅ |
| Fees and payments in the same ledger as accounting | ✅ | 🟡 | 🟡 | 🟡 | ❌ | ❌ |
| Conditions set at approval and kept on the permit | ✅ | ✅ | ✅ | ✅ | 🟡 | 🟡 |
| Permit expiry, revocation and renewal | ✅ | ✅ | ✅ | ✅ | ✅ | 🟡 |
| Appeal of a rejection decided by someone outside the decision | ✅ | 🟡 | 🟡 | 🟡 | 🟡 | ❌ |
| Public register of permits, JSON and CSV, no sign-in | ✅ | ✅ | ✅ | ✅ | 🟡 | 🟡 |
| Notices to applicants as the application moves | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| SMS and notice templates in the agency's words | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Plan review cycles: corrections, resubmittal, comments | ❌ | ✅ | ✅ | ✅ | 🟡 | 🟡 |
| Inspections after issue: request, schedule, result, re-inspect | ❌ | ✅ | ✅ | ✅ | 🟡 | 🟡 |
| Statutory clocks, deemed approval and breach alerts | ❌ | ✅ | ✅ | 🟡 | 🟡 | 🟡 |
| Citizen requests with categories and days to resolve | ✅ | ✅ | ✅ | 🟡 | ✅ | ✅ |
| Requests with a map pin, photo and routing by department | ❌ | ✅ | ✅ | 🟡 | ✅ | ✅ |
| Parcel, address and GIS layers, zoning | ❌ | ✅ | ✅ | ✅ | 🟡 | 🟡 |
| Business and professional licensing registry, annual cycles | 🟡 | ✅ | ✅ | ✅ | ✅ | 🟡 |
| Code enforcement: violations, notices, fines, hearings | ❌ | ✅ | ✅ | 🟡 | 🟡 | 🟡 |
| Permit certificate and decision letters with QR check | ❌ | ✅ | ✅ | ✅ | 🟡 | 🟡 |
| Plan and drawing markup in review | ❌ | ✅ | ✅ | ✅ | 🟡 | ❌ |
| Hearings, agendas and minutes | ❌ | ✅ | ❌ | ❌ | ❌ | ✅ |
| Reports: turnaround, backlog, revenue by fee type | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Personal-data erasure and retention declared per record | ✅ | 🟡 | 🟡 | 🟡 | 🟡 | 🟡 |
| Self-hosted on the agency's own servers | ✅ | 🟡 | ❌ | ❌ | ❌ | ❌ |
| Open source | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| No licence fee per user or per resident | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |

Notes. **Ferrum 🟡/❌:** an application has one free-text details field and its documents, not questions specific to
the permit type; a type has one flat fee; inspection is a review stage before approval, not a visit after issue; there
is no licensee register; the register shows the latest 1,000 permits. **Leaders:** 🟡 means a lighter feature, a
configuration the office must build, or an add-on. Whether a leader's product meets a given rule depends on the
version bought and how it is run.

### 🏆 Where Government stands out

- 💳 **The fee is in the books.** Fees are ERP invoices and approval waits for payment, with no sync between a permit system
  and a finance system.
- 🧑‍⚖️ **Review without shortcuts.** Role-based stages, side-by-side stages and a rule that no one passes two in a row.
- ⚖️ **Appeals by someone outside the decision.**
- 🌐 **A register that says only what each type allows,** for as long as it allows.
- 🔏 **Personal data handled by rule.** Drafts are erased on request and decided records kept with a stated reason.
- 🔓 **Open and owned.** AGPL, no per-user or per-resident pricing.

### 🚧 Where the leaders are still ahead

- 📝 **Forms and fees per permit type,** with conditional questions and fee schedules by valuation or area.
- 🔍 **Inspections** after issue, plan review cycles and statutory clocks.
- 🗺️ **Maps and parcels,** and location on citizen requests.
- 💬 **Reach.** Online payment, SMS and templated notices.
- ⚖️ **Enforcement and hearings,** and a licensee registry.
- 📄 **Documents.** Certificates with a QR check and plan markup.

> [!NOTE]
> Leader columns are from public product information as of November 2026 and may change. Product names belong to
> their owners.

## 📍 Roadmap

### ✅ Recently shipped

- [x] **0.3.2** Tuned design system: better menu icons, lighter first page, cleaner print.
- [x] **0.3.1** The redesigned interface with shaped loading and told-apart states.
- [x] **0.3.0** The register publishes what each type says, for as long as it says; appeals decided by someone who took no part.
- [x] **0.2.13** Names of records and fields in Arabic and Hindi; the last English sentences translated.
- [x] **0.2.11** The assistant knows the records.
- [x] **0.2.5** Review stages side by side, passed in any order by their own people.
- [x] **0.2.0** Review stages with site inspections, conditions, renewals, notices, a public register.

### 🔜 Next up

**Applying**
- [ ] Application questions specific to each permit type, with conditional questions.
- [ ] Fee schedules by valuation, area or units, with a fee estimate for the applicant.
- [ ] Pay fees online on the portal.

**Reviewing**
- [ ] Plan review cycles: corrections required, resubmittal and comments to the applicant.
- [ ] Inspections after issue: request on the portal, schedule, result, re-inspection and final sign-off.
- [ ] Statutory clocks: days to decide per type, pause on corrections and breach alerts.
- [ ] Conditions checked after issue.

**Documents and messages**
- [ ] Permit and licence certificates and decision letters, with a QR code to verify them.
- [ ] SMS and notice templates in the agency's own words.

**Reports**
- [ ] Turnaround, backlog, revenue by fee type and inspector workload.

### 🔭 Later

**Places**
- [ ] Parcel, address and map fields, with zoning on permits and cases.
- [ ] Citizen requests with a location, photo and routing by department.

**Licences and enforcement**
- [ ] A licensee registry with annual cycles and proration.
- [ ] Code enforcement: complaints, violations, notices, fines and hearings.

**Public access**
- [ ] A register of more than the latest 1,000 permits at once, searchable by address or number.
- [ ] Open data beyond JSON and CSV.
- [ ] Plan and drawing markup in review.
- [ ] Hearings, board agendas and public notices.

## 🚀 Install

Government is a native Ferrum app: the Ferrum server includes it (`runtime: native`) and a workspace turns it on from
**App Manager**. It needs platform `>=0.3.2, <0.4`. Installing it installs **ERP**, which invoices fees.

After installing:

1. Give people the roles **Permit Officer**, **Case Officer** and **Services Manager**.
2. Open **Government Settings** and set the **company** and the **fee item** that fees are invoiced as.
3. As a Services Manager, create **permit types** (fee, months valid, documents needed, review stages, renewal, appeal days,
   register options) and **case categories** (days to resolve).
4. Point citizens to the workspace's portal, where **Your services** appears.
5. To publish a register, tick *On The Public Register* on a permit type and choose what it shows.

## 🔧 Develop

```bash
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
FERRUM_GOVERNMENT_TEST_DATABASE_URL=postgres://ferrum:ferrum@localhost:5432/ferrum_government_test cargo test
```

The database must exist, be owned by a role that is not a superuser (superusers bypass row-level security) and be reset
before the run. `tests/government_it.rs` walks a town hall: a building permit applied for on the portal with its
documents uploaded, its fee paid, and issued; a rejection, a permit that expires, and a withdrawal; cases resolved and
overdue; what each citizen sees; and who may do what. Rust dependencies come from the Ferrum platform and Ferrum ERP
repositories on `main`.

The interface lives in `ui/` and is built with Ferrum-Frontend. Check the Frontend out beside the repository as
`../ferrum/web`, install its packages, then:

```bash
cd ui
npm run build       # builds into ui/dist, which the server embeds
npm run check       # interface lint
npm run typecheck
npm test
```

CI (`.github/workflows/ci.yml`) runs Rust on PostgreSQL 17 and the interface checks.

## 📂 Project layout

```text
app.yaml                    manifest: id, roles, shell entries, portal, notifications, assistant notes
packages/government/        entities, roles, personal-data rules and backfills as YAML
src/permits.rs              applications, stages, approval, appeals, permits, renewals, upkeep
src/cases.rs                cases, due days, overdue
src/site.rs                 the portal's Your services and the public register
ui/                         the officers' queues and the portal page; words in Arabic and Hindi
tests/government_it.rs      a town hall on PostgreSQL
docs/PROVENANCE.md          where the design comes from
```

## 📜 Changelog

Every release is described in [CHANGELOG.md](CHANGELOG.md).

## 🤝 Contributing

Issues and pull requests are welcome at [github.com/CBFerrum/ferrum-app-government](https://github.com/CBFerrum/ferrum-app-government/issues).
Keep the checks above green. The design and code of this app are its own; see [docs/PROVENANCE.md](docs/PROVENANCE.md).

## 📄 License

AGPL-3.0-only. See [LICENSE](LICENSE).

<div align="center">

<sub>Part of <a href="https://github.com/CBFerrum/Ferrum">Ferrum</a>, the open business platform · Made with 🦀 Rust and 🐘 PostgreSQL</sub>

</div>
