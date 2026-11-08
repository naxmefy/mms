# 0009 — Error model

- **Status:** Accepted
- **Date:** 2026-11-08

## Context

Errors from the core reach users through the CLI today and through a server API, translations and scripts later.
They need a stable identity that does not depend on wording.

## Decision

- Every error the core can return has a stable, machine-readable code in the form `area.reason`,
  e.g. `stock.insufficient`, `article.not_found`, `workspace.migration_required`.
  Text codes are preferred over numeric ones (`E1042`): they are self-explanatory and need no number registry.
- Errors are `thiserror` enums in the core with a `code() -> &'static str` method. Details (article, quantity, …)
  are structured fields, never embedded in text.
- The code is the single key for remote API responses (`{"code": "...", "details": {...}}`),
  localisation (it doubles as the message ID) and the error reference in the user documentation.
- A released code never changes its meaning. Codes are removed only after deprecation,
  following the same rule as schema changes ([ADR 0007](0007-schema-migrations.md)).
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

## Consequences

- Clients, scripts and translations rely on codes instead of parsing messages.
- Each new error needs a deliberately chosen code.
- Codes become part of the public interface and are subject to deprecation rules.
