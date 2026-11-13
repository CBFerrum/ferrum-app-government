# Changelog

## 0.3.2

Rebuilt with Ferrum-Frontend 9e7190c, the design system as tuned after the landed build was reviewed. The left menu's icons are chosen so that none repeats on more than three rows and each fits its row; their drawings load beside the first page instead of inside it, so the page starts lighter; a row that scrolls sideways fades at the edge where there is more, in every browser and with reduced motion; a printed record leaves out empty fields and keeps its totals with the table; menus and popovers sit inside the page's landmarks for assistive technology; a selection bar folds what does not fit into More on a narrow screen. Nothing else changes.

## 0.3.1

The interface follows the upgraded design system, rebuilt with Ferrum-Frontend 76c4fad. Text uses the kit's type roles (nothing under 12px) and controls its height tokens; loading shows skeletons shaped like what is coming; empty, no matches, denied, offline and failed are told apart; status colours come from stored state ids. The interface check now runs the UI lint against an empty baseline. The home is rebuilt on the kit's work-home pattern: one page title, the main action as a button, queues with their counts, status at its natural width; its other own pages follow the same anatomy.

## 0.3.0

**The public register** publishes only what each permit type says (FEO-052):
- Holder on the Register and Conditions on the Register: Published or Left Out, Left Out for a new type.
- On the Register After It Ends: the months an expired or revoked permit stays listed (12 for a new type; empty keeps it for good). A revoked permit keeps the day it was revoked.
- A type made before this release keeps publishing its holders and conditions for good, as it did: upkeep marks it so (a backfill).

**Appeals.** A rejection may be appealed within its type's days (Appeal Within, 30 for a new type, 0 to take none), by the applicant on the portal or for them at the counter, saying why.
- The appeal is decided by someone who took no part in the decision: not its officer, not who decided it, not who reviewed it.
- Upheld, the rejection stands, and it is not appealed again.
- Overturned, it is approved as an approval would be (its fee paid, conditions set) and then issued.
- The applicant is told either way, the officer that it was appealed, and an application keeps who decided it.

Rebuilt with Ferrum-Frontend 69de8aa, with the new words in Arabic and Hindi.

## 0.2.13

Rebuilt with Ferrum-Frontend 43d7779: the names of its models, fields and manifest in Arabic and Hindi (140 each in ui/src/locales/names, which the platform serves to every page of the workspace, so a record's labels, the launcher and the bar read in the person's language), and the sentences its pages still showed in English (split around a link or a figure, built in code from a template or word by word) whole and translated (41 words each in ui/src/locales). Translated by the model, to be reviewed by native speakers.

## 0.2.12

Rebuilt with Ferrum-Frontend 046a3a2: the sentences its pages built in code (counts, dates, what a toast or a screen reader says) go through t() too, as whole sentences where code chose a word, and carry built-in Arabic and Hindi (40 words each in ui/src/locales, translated by the model and to be reviewed by native speakers).

## 0.2.11

The assistant knows what its records mean in its pages (contributes.shell.assistant): their states, what counts and where figures come from, with questions it suggests asking there.

## 0.2.10

Rebuilt with Ferrum-Frontend 389b2af: its own words carry built-in Arabic and Hindi (38 words each, ui/src/locales, translated by the model and to be reviewed by native speakers), fetched for the page's language only. The labels it keeps in objects, its dialogs, its page titles and the words it keeps in variables go through t() as well, and the names and sizes it shows as they are keep their order right to left. Its pages are drawn once their words are in, and its entry carries only the parts of the kit it uses.

## 0.2.9

Rebuilt with Ferrum-Frontend 1f5981d: an action's link, code or token shown once is kept in a dialog to copy, and on a page that reads right to left, English left untranslated keeps its own order. Its own words go through the kit's t() (31 places), so a workspace's own words for its language reach them, and Admin, Interface words lists them under it (dist/words.json).

## 0.2.8

Rebuilt with Ferrum-Frontend 8a1c2e7: a workspace's own words for its interface (Admin, Interface words) reach its pages as they load; an attachment field that holds a picture shows it; and a value another record already has is said so, where the form said the record had been changed by someone else.

## 0.2.7

Pass This Stage's dialog no longer comes filled with the whole Waiting For list. Its Stage was
filled from the application's field of the same name, so with two stages waiting side by side it
said "Fire Safety, Health Check" and was refused unless cleared. The param is now `which`: empty,
the caller passes the first stage they may; named, that stage.

## 0.2.6

Rebuilt with Ferrum-Frontend 9d80f4f: right to left, its pages mirror as they read (icons that point along the line, margins and alignment by the line's start and end, arrow keys along it), and the records pages' words are in Arabic and Hindi.

## 0.2.5

Review stages side by side. A stage marked Alongside The One Before is passed side by side with it,
in any order, each by someone holding its own role, and no one passes two stages of one group; the
application's Waiting For names them all, and it moves on once every one is passed. Pass This
Stage takes the stage the caller may pass, or the one named when they may pass more than one. A
rejection records each stage it still waited for as failed. Two stages of a type may no longer
share a name.

## 0.2.4

Rebuilt with Ferrum-Frontend 204b713: a new record's box whose default is 1 shows ticked, as it is saved.

## 0.2.3

Found walking the citizens' portal on 7 November: a permit type's fee shows as money, and a type
with none says "no fee" (it read "fee 0").

## 0.2.2

Rebuilt with Ferrum-Frontend 6cfda39: an action's dialog says every refusal, the errors on what the action made (another record's rows) among them, and an action with parameters and a confirmation asks once.

## 0.2.1

Rebuilt with Ferrum-Frontend b39fffb: its pages show times on a person's own clock when they chose one on their Account page, while today and due dates stay on the workspace's day.

## 0.2.0

Stages, conditions, renewals, notices and the public register (FERRUM-INDUSTRY-010).

- **Review stages** on a permit type, each with the role whose people pass it and whether it is a
  site inspection. Taken up, an application waits for its first stage. Each stage is passed in order
  by someone holding its role (any permit officer when it names none), never by whoever passed the
  stage before. An inspection says what it found. It is approved once every stage is passed (and
  its fee paid), and a rejection while it waits records the stage as failed. Its reviews are kept,
  and the citizen sees them.
- **Conditions:** an approval may set what the permit is issued on condition of. The permit keeps
  them, and the citizen and the register show them.
- **Renewals:** a renewable type's permits are renewed from its days before they end, by the
  officer or by the holder on the portal, as an application of its own. The permit issued for it
  runs on from the day after the old one ends. Upkeep tells the holder once when their renewal
  opens.
- **Notices:** the applicant is told when their application is submitted, approved, rejected or
  issued, by e-mail unless they choose otherwise ("Your applications and permits").
- **The public register:** `/api/site/{workspace}/government/register`, without sign-in, lists the
  permits of the types marked On The Public Register, with holder, days, status and conditions, as
  JSON or CSV (`format=csv`), filtered by `type`.
- **The workspace's day:** submissions, decisions, permits, cases and upkeep go by each workspace's
  own day, not UTC's.

## 0.1.8

Rebuilt with Ferrum-Frontend d350579: its pages show times on the workspace's clock, naming the zone when it is not the browser's, a draft numbered on submit is headed as a draft, and a record of an app that is not installed says so.

## 0.1.7

Rebuilt with Ferrum-Frontend 20d08e5: a record page reads an empty number or check as the server does, so an action whose condition needs one shows, and a workspace's settings open as their one record.

## 0.1.6

Rebuilt with Ferrum-Frontend ae13c69: in a workflow state that names what may change, the other fields show read-only, and the fields the state needs show required.

## 0.1.5

Personal-data requests find applicants, permit holders and citizens by email or name. An application still in draft, or withdrawn, is erased; one put to the office, a permit and a case are kept as public records, and the answer says why (Ferrum's FERRUM-PRIVACY-002).

## 0.1.4

Rebuilt with Ferrum-Frontend daeadda: after an action runs, a record page shows what the action said it did rather than only that it is done.

## 0.1.3

From the browser walk of a permit's fee.

- An application links its fee's invoice once the invoice is issued. A draft fee invoice was linked
  as soon as the application was submitted, and Your invoices answered "invoice not found", as it
  opens only issued ones; until then the application says the invoice is not issued yet.
- Rebuilt with Ferrum-Frontend 355dfca.

## 0.1.2

Rebuilt with Ferrum-Frontend 89b7cc7: a field's label names its control for assistive technology, and a record whose title is a link shows that record's title, not its id.

## 0.1.1

From a browser walk of the citizens' portal.

- A permit valid for one month says so ("1 month", not "1 months").
- Rebuilt with Ferrum-Frontend's fields that name their controls, so the portal's forms are named for assistive technology.

## 0.1.0

Public services (FERRUM-INDUSTRY-010).

- Permit types with their fee, the documents they need and how long they last.
- Applications from the portal or the counter, submitted with their documents, their fee invoiced
  through ERP; taken up, approved once paid or rejected saying why, and issued as permits.
- Permits that expire by themselves or are revoked saying why.
- Cases due within their category's days, resolved saying how, and marked overdue when late.
- The portal's "Your services" at `/portal/services/`: apply, upload documents, follow
  applications and permits, and raise cases.
- Its own interface at `/government/`: the officers' queues.
