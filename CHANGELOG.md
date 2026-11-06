# Changelog

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
