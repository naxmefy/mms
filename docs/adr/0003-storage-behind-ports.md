# 0003 — Storage behind ports, SQLite as first adapter

- **Status:** Accepted
- **Date:** 2026-11-08

## Context

mms starts as a local tool where SQLite is the simplest fit, but it should be able to manage very large
warehouses later. The core must not be tied to one database, and SQL should stay readable and checked.

## Decision

- The core defines storage ports (async traits); adapters implement them. SQLite is the first adapter.
  The core must not depend on SQLite-specific behaviour, so that other backends (e.g. Postgres) and
  very large warehouses remain possible.
- Each adapter owns its SQL and its migrations ([ADR 0007](0007-schema-migrations.md)).
- SQL lives in `.sql` files inside the adapter crate (`queries/`) and is used via
  `sqlx::query_file!` / `query_file_as!`, checked against the schema at compile time.
  Short one-liners may use inline `query!`. Dynamic queries (filters) use `sqlx::QueryBuilder`.
- The sqlx offline data (`.sqlx/`) is committed so that CI and contributors can build without a database.
- No ORM or query DSL: the ports already decouple the core from the database; a second abstraction adds
  learning cost without benefit.
- Nothing is loaded fully on start-up. Data, indexes (e.g. a future search index) and caches are read on demand
  and persisted on disk, keeping cold start cheap even for large workspaces.

## Consequences

- Typos and type mismatches in SQL become compile errors.
- SQL can be read and tried out in any database tool.
- Contributors who change queries need `sqlx-cli` to refresh `.sqlx/`.
- A future Postgres adapter writes its own SQL; some queries will exist once per backend.
