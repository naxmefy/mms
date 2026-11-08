# 0006 — Identifiers, quantities and money

- **Status:** Accepted
- **Date:** 2026-11-08

## Context

Identifiers, quantities and money are used everywhere and are expensive to change later. Floating-point numbers
cause rounding errors, and auto-increment IDs get in the way of merging data or distributing the system.

## Decision

- IDs are UUIDv7 (time-sortable, no auto-increment, safe for distribution later).
- Quantities and money use a decimal type (`rust_decimal`), never `f64`. Money always carries a currency.
- Quantities always carry a unit of measure.

## Consequences

- No rounding errors in stock or amounts.
- Records can be created anywhere without coordinating IDs, and IDs still sort by creation time.
- UUIDs are longer than integers in URLs and on the command line; the CLI may need short forms or lookups by SKU.
