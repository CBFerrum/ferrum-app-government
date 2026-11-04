# Changelog

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
