# 0002 — Workspace structure: core library plus frontend crates

- **Status:** Accepted
- **Date:** 2026-11-08

## Context

mms will be used through several frontends — CLI first, later a TUI, a server and a web application.
All of them must share the same domain logic, and the system must stay extensible without rewriting that logic.

## Decision

- `mms_core`: domain types, commands, queries, events and ports (traits). No database code, no HTTP.
- `mms_store_sqlite`: the first storage adapter.
- `mms_cli`: the `mms` binary.
- Later: `mms_tui`, `mms_server`, `mms_web`, … Whether these end up as one or several executables stays open;
  the core-as-library structure keeps both options available.
- Everything is Rust, including future TUI, server and web frontends (web candidates: Leptos, Yew).
- Internal extensibility is achieved through additional crates. A plugin layer for third-party developers
  is on the roadmap under *Later & Ideas*.
- The existing code (axum endpoint, Postgres via sqlx, `Article` service with `f64` price, empty `mms_tui`)
  is replaced in v0.1.

## Consequences

- Every frontend reuses the same domain logic; business rules live in exactly one place.
- The core can be tested without a database or network.
- More crates mean a little more workspace bookkeeping.
- The existing prototype code is discarded rather than evolved.
