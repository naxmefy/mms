# Sub-project A: Planning foundation — design

- **Status:** Accepted
- **Date:** 2026-11-08
- **Scope:** Documentation, planning and project organisation only. No product code changes.

## 1. Context

`mms` is meant to become an extensible, modular merchandise management system (MMS) written in Rust.
The long-term vision is a fully automated warehouse: trucks dock, robots unload and load, and the system
tracks exactly what is where and how much — with monitoring and, later, AI support such as bottleneck prevention.

Current state (as of 2026-11-08):

- The repository has solid tooling (Cargo workspace, CI for check/fmt/test, mdBook on mms.nax.me)
  but almost no domain functionality.
- The latest code (Dec 2025) drifted towards an axum + Postgres server, while the issues describe
  a local, CLI-first tool on SQLite.
- There is no written roadmap, scope or record of architecture decisions.
- 15 open issues, several obsolete or duplicated; one discussion (#14).

## 2. Overall decomposition

The work is split into sub-projects, each with its own spec → plan → implementation cycle:

| | Sub-project | Outcome |
|---|---|---|
| **A** | Planning foundation (this spec) | Documentation map, ADRs, glossary, roadmap, cleaned-up GitHub |
| **B** | v0.1 Inventory core | `mms_core`, SQLite adapter, CLI in direct mode |
| **C** | Sample data & import | Own import format, sample data sets, first foreign format |
| **D** | User documentation | mdBook, grows alongside B and C |

TUI, server, web, background process and plugins go to the roadmap under *Later & Ideas*.

## 3. Goals and non-goals of A

**Goals**

- Every architectural decision made so far is written down as an ADR.
- A roadmap exists, and everything that is not built now is recorded there.
- A glossary gives the domain a shared vocabulary; its terms are the names used in code.
- GitHub (labels, milestones, issues, discussions) reflects the plan.

**Non-goals**

- No changes to Rust code, `Cargo.toml`, toolchain or CI (belongs to B).
- No restructuring of the mdBook beyond including roadmap and glossary (belongs to D).
- No detailed design of v0.1 commands, tables or CLI syntax (belongs to B's spec).

## 4. Documentation map

All project documentation is in English.

| Place | Purpose | Content |
|---|---|---|
| Repository | Decisions and technical material | `ROADMAP.md`, `docs/adr/`, `docs/specs/`, `docs/glossary.md` |
| mdBook | User documentation | Includes `ROADMAP.md` and `docs/glossary.md` via `{{#include}}` |
| GitHub | Work | Milestones, issues, labels |

**One home per kind of information:** decisions are ADRs, planned work is issues, deferred work is the roadmap.
Nothing is maintained in two places.

## 5. Architecture decisions (ADR content)

ADRs live in `docs/adr/NNNN-kebab-title.md` using a lean MADR format
(Status, Context, Decision, Consequences). A writes them with status *Accepted*; B implements them.

### ADR 0001 — Record architecture decisions

Architecture decisions are recorded as ADRs in `docs/adr/`. Accepted ADRs are not edited in substance;
a changed decision gets a new ADR that supersedes the old one.

### ADR 0002 — Workspace structure: core library plus frontend crates

- `mms_core`: domain types, commands, queries, events, ports (traits). No database code, no HTTP.
- `mms_store_sqlite`: first storage adapter.
- `mms_cli`: the `mms` binary.
- Later: `mms_tui`, `mms_server`, `mms_web`, … — whether these end up as one or several executables stays open;
  the core-as-library structure keeps both options available.
- Everything is Rust, including future TUI, server and web frontends (web candidates: Leptos, Yew).
- Internal extensibility is achieved through additional crates. A plugin layer for third-party developers
  is *Later & Ideas*.
- The existing code (axum endpoint, Postgres via sqlx, `Article` service with `f64` price, empty `mms_tui`)
  is replaced in B.

### ADR 0003 — Storage behind ports, SQLite as first adapter

- The core defines storage ports (async traits); adapters implement them. SQLite is the first adapter;
  the core must not depend on SQLite-specific behaviour, so that other backends (e.g. Postgres) and
  very large warehouses remain possible.
- Each adapter owns its SQL and its migrations.
- SQL lives in `.sql` files inside the adapter crate (`queries/`) and is used via
  `sqlx::query_file!` / `query_file_as!`, checked against the schema at compile time.
  Short one-liners may use inline `query!`. Dynamic queries (filters) use `sqlx::QueryBuilder`.
- The sqlx offline data (`.sqlx/`) is committed so that CI and contributors can build without a database.
- No ORM or query DSL: the ports already decouple the core from the database; a second abstraction adds
  learning cost without benefit. Consequence: a future Postgres adapter writes its own SQL.
- Nothing is loaded fully on start-up. Data, indexes (e.g. a future search index) and caches are read on demand
  and persisted on disk, keeping cold start cheap even for large workspaces.

### ADR 0004 — Stock as a ledger of stock movements

- **Stock movements** (goods receipt, goods issue, stock transfer, stock adjustment) form an append-only journal.
  Movements are never updated or deleted; mistakes are corrected by a reversal (counter-movement).
- **Stock** (per article and location) is derived from movements and updated in the same transaction
  as the movement — immediately consistent, no event bus.
- **Master data** (articles, locations) is plain create/update/deactivate.
- **Locations** form a tree: each location has an optional parent, with no fixed levels.
  This covers a single shelf as well as warehouse → hall → zone → rack → bin.
- Each movement is a domain event; later consumers (server workers, webhooks, plugins) attach here
  without changes to the core.

### ADR 0005 — Commands and queries as the core API

- Every change is a command object (e.g. `ReceiveGoods`), every read a query object.
- Commands are validated in the core before anything is persisted (e.g. positive quantity,
  article exists, sufficient stock for a goods issue).
- Commands and queries are plain, `serde`-serialisable data structures, so the same command can run
  in-process or be sent to a server (see ADR 0008).
- This is command/query separation at the API level, **not** full CQRS: there are no separate read/write
  stores and no eventual consistency. Full CQRS or event sourcing for everything is *Later & Ideas*.
- The core returns typed errors and results, never preformatted user-facing text. Frontends render the text,
  which keeps localisation possible and lets a server return structured errors.

### ADR 0006 — Identifiers, quantities and money

- IDs are UUIDv7 (time-sortable, no auto-increment, safe for distribution later).
- Quantities and money use a decimal type (`rust_decimal`), never `f64`. Money always carries a currency.
- Quantities always carry a unit of measure.

### ADR 0007 — Schema migrations

- sqlx embedded migrations (`sqlx::migrate!`), one set per adapter, compiled into the binary.
- Forward-only; no down migrations. A released migration is never changed (sqlx verifies checksums).
- On opening a workspace the CLI compares schema version and binary:
  - database older than binary → abort with a hint to run `mms migrate` (no silent auto-migration);
  - database newer than binary → abort (protects newer data from older binaries).
- `mms migrate` creates a backup first (SQLite: `VACUUM INTO`), then applies pending migrations.
  `mms migrate --status` shows pending migrations without applying them.
- Removing columns or tables follows expand/contract:
  1. add the new structure, switch the code to it, mark the old one as deprecated in the changelog;
  2. remove the old structure in a later release — before 1.0 at the earliest in the next minor release,
     from 1.0 on only after at least one full minor release carrying the deprecation.
- The stock movement journal only ever changes additively; historic movements are never lost.
- Export/import (sub-project C) is the escape hatch across larger breaks.

### ADR 0008 — Direct and remote access modes

- **Direct:** a frontend uses core and adapters in-process and works on a workspace folder.
  A future server is itself a direct-mode user of its own folder.
- **Remote:** a frontend sends the same commands and queries to a server, local or elsewhere.
- Frontends only talk to the core API — never to SQL or adapters directly.
- v0.1 implements direct mode only. A switching abstraction is introduced when the second implementation
  (remote) arrives.
- **Workspace = folder:** `mms init` creates `.mms/` containing `config.toml` and `mms.db`. The CLI finds
  the workspace by walking up the directory tree, like git (cf. issue #5). The layout is documented so that
  further areas (`objects/`, `index/`, …) can be added without changing existing ones.
- A separate daemon is not needed: if a warm process (caches, workers) is ever required, it is the local
  server running in the background (`mms serve`), used by CLI/TUI in remote mode against `localhost`.

### ADR 0009 — Error model

- Every error the core can return has a stable, machine-readable code in the form `area.reason`,
  e.g. `stock.insufficient`, `article.not_found`, `workspace.migration_required`.
  Text codes are preferred over numeric ones (`E1042`): they are self-explanatory and need no number registry.
- Errors are `thiserror` enums in the core with a `code() -> &'static str` method. Details (article, quantity, …)
  are structured fields, never embedded in text.
- The code is the single key for remote API responses (`{"code": "...", "details": {...}}`),
  localisation (it doubles as the message ID) and the error reference in the user documentation.
- A released code never changes its meaning. Codes are removed only after deprecation,
  following the same rule as schema changes (ADR 0007).
- CLI exit codes are coarse categories, not one per error:

  | Exit code | Meaning |
  |---|---|
  | 0 | Success |
  | 1 | Unexpected error |
  | 2 | Usage error (clap default) |
  | 3 | Validation or business rule violated |
  | 4 | Not found |
  | 5 | Conflict (e.g. SKU already exists) |
  | 6 | Workspace problem (not initialised, migration required) |

## 6. Roadmap

`ROADMAP.md` in the repository root, included in the mdBook. Structure:

- **Now:** A – Planning foundation
- **Next:** v0.1 – Inventory core; Sample data & import
- **Later & Ideas**, grouped, one sentence per item, no dates:
  - *Frontends:* TUI, server/API and remote mode, web (Leptos/Yew), desktop, mobile,
    localisation of CLI/TUI/web (e.g. Fluent) with locale-aware formatting, machine-readable CLI output
    (`--output json`), `mms explain <code>` for detailed error explanations
  - *Domain:* batches, serial numbers, best-before dates, physical inventory, purchase and sales orders,
    suppliers, customers, pricing and inventory valuation, returns, reports, demand forecasting,
    multilingual product data
  - *Platform:* Postgres adapter, multi-tenancy, users/roles/authentication, local server as background process
    (warm caches, workers), job queue, webhooks/event stream, full CQRS/event sourcing, search, redb for locks
    (issue #26), multiple workspaces/organisations (issue #6), plugin layer for third-party extensions,
    mapping of error codes to HTTP status codes
  - *Integrations:* further foreign import formats, EDI/GS1 (EANCOM), shop connectors, barcode/RFID
  - *Vision:* automated warehouse with robots, AI assistance, bottleneck prevention

**Rule:** an item moves to *Next* only once it has a milestone and issues.

## 7. Glossary

`docs/glossary.md`, included in the mdBook. Columns: term (as used in code), definition,
status (`v0.1` / `later`). Initial entries:

- `v0.1`: Article, SKU, Unit of measure, Warehouse, Location, Stock, Stock movement, Goods receipt,
  Goods issue, Stock transfer, Stock adjustment, Reversal, Master data, Transaction data, Workspace
- `later`: Physical inventory, Batch, Serial number, Best-before date

The glossary grows with every sub-project.

## 8. GitHub

### Labels

- *type:* `feature`, `bug`, `docs`, `chore`, `adr`
- *area:* `core`, `storage`, `cli`, `import`, `book`, `project`
- keep `good first issue`
- remove `evaluation`, `community` (this also removes them from issues carrying them)

### Milestones

- `A – Planning foundation`
- `v0.1 – Inventory core`
- `Sample data & import`

Each gets coarse issues now; fine-grained issues come from the plans of B and C.
User documentation (D) uses the `book` label instead of a milestone.

### Existing issues and discussion

| # | Title | Action |
|---|---|---|
| 1 | create readme | close — README exists |
| 5 | add cli configuration support | rewrite for workspace/config (ADR 0008), milestone v0.1 |
| 6 | workspace/org support | close — moved to roadmap |
| 7 | manifest for cli command structure | rewrite as v0.1 CLI command design, milestone v0.1 |
| 13 | cargo-make instead of Makefile? | close — obsolete |
| 15 | Primary idea | close — superseded by roadmap and milestones |
| 16 | data structure for items database | close — absorbed into B |
| 18 | Sub-Book for rfc? | close — answered by ADRs in `docs/adr/` |
| 19–22 | security policy, code of conduct, contributing, support | keep, relabel `docs`/`project` |
| 26 | redb evaluation | close — moved to roadmap |
| 27 | Changelog generation | keep, relabel `chore`/`project` |
| Discussion 14 | Primary idea | close with link to roadmap |

Every closed issue gets a short comment stating the reason and linking to where the topic now lives.

## 9. Execution order

1. **Repository** (branch `docs/planning-foundation`): `ROADMAP.md`, `docs/adr/0001`–`0009`,
   `docs/glossary.md`, this spec; mdBook includes roadmap and glossary; README replaces the outdated
   design doc link with links to roadmap, ADRs and book.
2. **Verification:** `mdbook build` succeeds, links resolve, every item deferred during planning appears
   in the roadmap. Then open a PR; the maintainer reviews and merges.
3. **GitHub**, in batches — labels, milestones, existing issues, discussion. Before each batch the full list
   with exact texts is shown, and the batch runs only after explicit approval.

## 10. Done criteria

- The PR from step 1 is merged.
- Every existing issue is either closed with a comment or has labels and a milestone.
- The roadmap contains every item deferred during planning.
