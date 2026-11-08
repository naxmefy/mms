# 0007 — Schema migrations

- **Status:** Accepted
- **Date:** 2026-11-08

## Context

The database lives in each user's workspace folder, and new versions will change its schema. Users must be able
to upgrade safely, and an older binary must never damage data written by a newer one.

## Decision

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
- Export/import is the escape hatch across larger breaks.

## Consequences

- Upgrades are explicit and always preceded by a backup.
- Mistakes in a released migration are fixed by a new migration, never by editing the old one.
- Removing structures takes at least two releases.
