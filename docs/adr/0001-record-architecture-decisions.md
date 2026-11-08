# 0001 — Record architecture decisions

- **Status:** Accepted
- **Date:** 2026-11-08

## Context

mms is planned as a long-lived, modular system. Decisions about its structure are easy to forget or to
contradict later, and contributors need to understand why the code looks the way it does.

## Decision

- Architecture decisions are recorded as architecture decision records (ADRs) in `docs/adr/`,
  named `NNNN-kebab-title.md` and based on [`template.md`](template.md).
- Each ADR has a status: *Proposed*, *Accepted* or *Superseded*.
- Accepted ADRs are not edited in substance. A changed decision gets a new ADR that supersedes the old one,
  and the old one links to its successor.

## Consequences

- The reasoning behind the architecture stays available to everyone.
- Changing a decision takes a little ceremony: a new ADR instead of a quiet edit.
