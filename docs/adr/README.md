# Architecture decision records

Architecture decision records (ADRs) document why mms is built the way it is. Accepted ADRs are not rewritten;
a changed decision gets a new ADR that supersedes the old one ([ADR 0001](0001-record-architecture-decisions.md)).
New ADRs start from [`template.md`](template.md).

| Number | Title | Status |
|---|---|---|
| [0001](0001-record-architecture-decisions.md) | Record architecture decisions | Accepted |
| [0002](0002-workspace-structure.md) | Workspace structure: core library plus frontend crates | Accepted |
| [0003](0003-storage-behind-ports.md) | Storage behind ports, SQLite as first adapter | Accepted |
| [0004](0004-stock-as-movement-ledger.md) | Stock as a ledger of stock movements | Accepted |
| [0005](0005-commands-and-queries.md) | Commands and queries as the core API | Accepted |
| [0006](0006-identifiers-quantities-money.md) | Identifiers, quantities and money | Accepted |
| [0007](0007-schema-migrations.md) | Schema migrations | Accepted |
| [0008](0008-access-modes.md) | Direct and remote access modes | Accepted |
| [0009](0009-error-model.md) | Error model | Accepted |
