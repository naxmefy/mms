# 0004 — Stock as a ledger of stock movements

- **Status:** Accepted
- **Date:** 2026-11-08

## Context

In merchandise management, stock is not a number that gets overwritten: it is the result of goods coming in,
going out and moving around. Tracing exactly what happened, when and why is a core requirement — from a single
shelf up to a fully automated warehouse.

## Decision

- **Stock movements** (goods receipt, goods issue, stock transfer, stock adjustment) form an append-only journal.
  Movements are never updated or deleted; mistakes are corrected by a reversal (counter-movement).
- **Stock** (per article and location) is derived from movements and updated in the same transaction
  as the movement — immediately consistent, no event bus.
- **Master data** (articles, locations) is plain create/update/deactivate.
- **Locations** form a tree: each location has an optional parent, with no fixed levels.
  This covers a single shelf as well as warehouse → hall → zone → rack → bin.
- Each movement is a domain event; later consumers (server workers, webhooks, plugins) attach here
  without changes to the core.

## Consequences

- A complete audit trail exists from the first version.
- Stock can always be recomputed from the journal to verify it.
- Corrections are visible as reversals instead of silent edits.
- The journal grows forever; archiving or snapshots may be needed for very large data sets later.
