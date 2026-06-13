---
Status: Active
Date: 2026-06-09
Scope: phase-296x current docs pointers and mimalloc history index.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - docs/development/current/main/investigations/phase-296x-90-taskboard-history-2026-06-08.md
---

# Phase 296x - Mimalloc Benchmark Contract

Phase-296x started after the first `.hako` mimalloc port pass and the phase-295x
comparison rows landed.

## Goal

Keep the current pointers thin enough that the active lane is easy to find.

## Stop Line

Keep closed unless a later row explicitly opens them:

- provider package / DLL generation;
- provider activation and provider API execution;
- process allocator replacement, hooks, backend matchers, and
  `#[global_allocator]`;
- performance or memory winner claims.

## Current Work

Read these first:

```text
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md
docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md
docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
```

The active lane is named by `CURRENT_STATE.toml`. As of the current pointer,
the phase is back in the userbox/counter-heavy exact-front optimization lane
selected by `296x-649`.

Historical note: `MIM-PORT-FMEM-005` and `MIM-PORT-FMEM-006` are already Done.
They are not the next active implementation row. They remain useful as narrow
FastMemory proof examples for PageMeta `owner_worker_id` scalar read and
`free_head` read-only pointer observation.

Use the FastMemory handoff and proof rows when pointer hunting gets noisy, use
the layout/table producer SSOT before changing FastMemory lowering, and use the
MIM-PORT-FMEM rows as historical slice evidence rather than a live queue unless
`CURRENT_STATE.toml` points at one explicitly.
