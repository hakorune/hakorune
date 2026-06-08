---
Status: Active
Date: 2026-06-08
Scope: compact restart surface for the phase-296x mimalloc benchmark lane.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/phase-296x-90-taskboard-history-2026-06-08.md
  - docs/development/current/main/phases/phase-296x/296x-480-DIRECTARRAY-FMEM-COMMONALITY-AND-DOC-LENGTH-CLEANUP.md
  - docs/development/current/main/workstreams/mimalloc-current.md
---

# 296x-90 Mimalloc Benchmark Taskboard

This card is now the compact restart surface for the phase-296x mimalloc lane.
Keep the historical benchmark queue in the archive note and keep this card
short.

## Rule

- benchmark contract work stays before provider/DLL activation
- no product allocator replacement claim
- no hook installation claim
- no winner claim
- no new smoke scripts for report-only rows unless a new execution boundary opens

## Current State

Current lane and blocker pointers live in `CURRENT_STATE.toml`.

The long historical queue is archived here:

```text
docs/development/current/main/investigations/phase-296x-90-taskboard-history-2026-06-08.md
```

## Active Queue

```text
DOCS-SLIM-296X-001:
  slim docs/development/current/main/workstreams/mimalloc-current.md

DOCS-SLIM-296X-002:
  slim docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md

DOCS-SLIM-FMEM-SSOT-001:
  split the capability-gap SSOT into compact decision and investigation docs

DIRECTARRAY-FMEM-COMMON-001:
  proof/report adapter sidecar only; no auto-fastmem region
```

## Restart Notes

- read `CURRENT_STATE.toml` first for the current blocker token
- use the archive note above for historical queue detail
- keep DirectArray/FastMemory proof commonality separate from lowering payloads

## Stop Line

- do not re-expand archived benchmark rows into this active card
- do not mix docs-slim work with MIR lowering rows
- do not reopen provider activation or product replacement here
