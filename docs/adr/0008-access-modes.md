# 0008 — Direct and remote access modes

- **Status:** Accepted
- **Date:** 2026-11-08

## Context

The CLI should work directly on a local folder, and later also against a server that may run locally or elsewhere.
Large workspaces with search indexes or object storage may also raise the question of a long-running process.

## Decision

- **Direct:** a frontend uses core and adapters in-process and works on a workspace folder.
  A future server is itself a direct-mode user of its own folder.
- **Remote:** a frontend sends the same commands and queries to a server, local or elsewhere.
- Frontends only talk to the core API ([ADR 0005](0005-commands-and-queries.md)) — never to SQL or adapters directly.
- v0.1 implements direct mode only. A switching abstraction is introduced when the second implementation
  (remote) arrives.
- **Workspace = folder:** `mms init` creates `.mms/` containing `config.toml` and `mms.db`. The CLI finds
  the workspace by walking up the directory tree, like git. The layout is documented so that
  further areas (`objects/`, `index/`, …) can be added without changing existing ones.
- A separate daemon is not needed: if a warm process (caches, workers) is ever required, it is the local
  server running in the background (`mms serve`), used by CLI/TUI in remote mode against `localhost`.

## Consequences

- The same commands work locally and remotely; the server adds a transport, not new logic.
- A workspace is a plain folder that can be copied, backed up or versioned.
- Until remote mode exists, CLI code must keep to the core API so that switching stays cheap.
