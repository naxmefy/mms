# 0005 — Commands and queries as the core API

- **Status:** Accepted
- **Date:** 2026-11-08

## Context

The core is used by several frontends, in-process now and over the network later. Validation must happen in one
place, changes should be traceable, and the same operations must work locally and remotely.

## Decision

- Every change is a command object (e.g. `ReceiveGoods`), every read a query object.
- Commands are validated in the core before anything is persisted (e.g. positive quantity,
  article exists, sufficient stock for a goods issue).
- Commands and queries are plain, `serde`-serialisable data structures, so the same command can run
  in-process or be sent to a server ([ADR 0008](0008-access-modes.md)).
- This is command/query separation at the API level, **not** full CQRS: there are no separate read/write
  stores and no eventual consistency. Full CQRS or event sourcing for everything is on the roadmap under
  *Later & Ideas*.
- The core returns typed errors and results, never preformatted user-facing text. Frontends render the text,
  which keeps localisation possible and lets a server return structured errors ([ADR 0009](0009-error-model.md)).

## Consequences

- One validation path for every frontend.
- Remote access later needs no new API design, only a transport.
- Every operation needs its own type, which is more code than calling functions with loose parameters.
