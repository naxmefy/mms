# Roadmap

What `mms` works on now, what comes next, and everything that is deliberately postponed.

## Now

- **A – Planning foundation:** roadmap, architecture decision records, glossary and an organised GitHub project.

## Next

- **v0.1 – Inventory core:** `mms_core`, SQLite adapter and a CLI working directly on a workspace folder — articles, locations, stock movements and stock.
- **Sample data & import:** an own import format, sample data sets and a first foreign format from another merchandise management system.

## Later & Ideas

### Frontends

- Terminal UI (TUI) on top of the core API.
- Server with an HTTP API, and a remote mode for CLI and TUI that talks to it.
- Web application written in Rust (candidates: Leptos, Yew).
- Desktop application.
- Mobile application.
- Localisation of CLI, TUI and web (e.g. with Fluent), including locale-aware formatting of numbers, dates and currencies.
- Machine-readable CLI output (`--output json`).
- `mms explain <code>` for detailed explanations of error codes.

### Domain

- Batch tracking.
- Serial number tracking.
- Best-before date tracking.
- Physical inventory (stock counts).
- Purchase orders.
- Sales orders.
- Supplier management.
- Customer management.
- Pricing and inventory valuation.
- Returns management.
- Reports (inventory, turnover, ABC analysis).
- Demand forecasting.
- Multilingual product data.

### Platform

- Postgres storage adapter.
- Multi-tenancy.
- Users, roles and authentication.
- Local server as a background process with warm caches and workers, replacing a separate daemon.
- Job queue for background processing.
- Webhooks and an event stream for external consumers.
- Full CQRS or event sourcing, should large deployments need it.
- Search engine with an on-disk index.
- Embedded key-value store (e.g. redb) for process locks and fast state ([#26](https://github.com/naxmefy/mms/issues/26)).
- Multiple workspaces and organisations ([#6](https://github.com/naxmefy/mms/issues/6)).
- Plugin layer for third-party extensions.
- Mapping of error codes to HTTP status codes.

### Integrations

- Further foreign import formats.
- EDI and GS1 standards (e.g. EANCOM).
- Connectors to online shops.
- Barcode and RFID support.

### Vision

- Automated warehouse: robots unload and load trucks while the system tracks exactly what is where and how much.
- Monitoring of warehouse operations.
- AI assistance on top of warehouse data.
- Bottleneck prevention.

---

An item moves from *Later & Ideas* to *Next* only once it has a milestone and issues.
