# Changelog

## 0.2.10

Rebuilt with Ferrum-Frontend 5b22e7b: its own words carry built-in Arabic and Hindi (38 words each, ui/src/locales, translated by the model and to be reviewed by native speakers), fetched for the page's language only. The labels it keeps in objects, its dialogs, its page titles and the words it keeps in variables go through t() as well, and the names and sizes it shows as they are keep their order right to left. Its pages are drawn once their words are in, and its entry carries only the parts of the kit it uses.

## 0.2.9

Rebuilt with Ferrum-Frontend 6537d65: an action's link, code or token shown once is kept in a dialog to copy, and on a page that reads right to left, English left untranslated keeps its own order. Its own words go through the kit's t() (31 places), so a workspace's own words for its language reach them, and Admin, Interface words lists them under it (dist/words.json).

## 0.2.8

Rebuilt with Ferrum-Frontend d5d98f2: a workspace's own words for its interface (Admin, Interface words) reach its pages as they load; an attachment field that holds a picture shows it; and a value another record already has is said so, where the form said the record had been changed by someone else.

## 0.2.7

Pass This Stage's dialog no longer comes filled with the whole Waiting For list. Its Stage was
filled from the application's field of the same name, so with two stages waiting side by side it
said "Fire Safety, Health Check" and was refused unless cleared. The param is now `which`: empty,
the caller passes the first stage they may; named, that stage.

## 0.2.6

Rebuilt with Ferrum-Frontend 63280d3: right to left, its pages mirror as they read (icons that point along the line, margins and alignment by the line's start and end, arrow keys along it), and the records pages' words are in Arabic and Hindi.

## 0.2.5

Review stages side by side. A stage marked Alongside The One Before is passed side by side with it,
in any order, each by someone holding its own role, and no one passes two stages of one group; the
application's Waiting For names them all, and it moves on once every one is passed. Pass This
Stage takes the stage the caller may pass, or the one named when they may pass more than one. A
rejection records each stage it still waited for as failed. Two stages of a type may no longer
share a name.

## 0.2.4

Rebuilt with Ferrum-Frontend a64416a: a new record's box whose default is 1 shows ticked, as it is saved.

## 0.2.3

Found walking the citizens' portal on 7 November: a permit type's fee shows as money, and a type
with none says "no fee" (it read "fee 0").

## 0.2.2

Rebuilt with Ferrum-Frontend 922fcfb: an action's dialog says every refusal, the errors on what the action made (another record's rows) among them, and an action with parameters and a confirmation asks once.

## 0.2.1

Rebuilt with Ferrum-Frontend a76295e: its pages show times on a person's own clock when they chose one on their Account page, while today and due dates stay on the workspace's day.

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

Rebuilt with Ferrum-Frontend 24c3951: its pages show times on the workspace's clock, naming the zone when it is not the browser's, a draft numbered on submit is headed as a draft, and a record of an app that is not installed says so.

## 0.1.7

Rebuilt with Ferrum-Frontend bb60115: a record page reads an empty number or check as the server does, so an action whose condition needs one shows, and a workspace's settings open as their one record.

## 0.1.6

Rebuilt with Ferrum-Frontend 00dea07: in a workflow state that names what may change, the other fields show read-only, and the fields the state needs show required.

## 0.1.5

Personal-data requests find applicants, permit holders and citizens by email or name. An application still in draft, or withdrawn, is erased; one put to the office, a permit and a case are kept as public records, and the answer says why (Ferrum's FERRUM-PRIVACY-002).

## 0.1.4

Rebuilt with Ferrum-Frontend daeadda: after an action runs, a record page shows what the action said it did rather than only that it is done.

## 0.1.3

From the browser walk of a permit's fee.

- An application links its fee's invoice once the invoice is issued. A draft fee invoice was linked
  as soon as the application was submitted, and Your invoices answered "invoice not found", as it
  opens only issued ones; until then the application says the invoice is not issued yet.
- Rebuilt with Ferrum-Frontend 16b5c57.

## 0.1.2

Rebuilt with Ferrum-Frontend 80ff179: a field's label names its control for assistive technology, and a record whose title is a link shows that record's title, not its id.

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
