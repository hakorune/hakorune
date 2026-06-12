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

The active lane is the MIM-PORT-FMEM-002 PageMeta scalar verified evidence row.
Use the FastMemory handoff and proof rows when pointer hunting gets noisy, use
the layout/table producer SSOT before changing FastMemory lowering, and use the
MIM-PORT-FMEM rows when you want the next narrow hako_alloc body migration. The
next implementation row is MIM-PORT-FMEM-003 PageMeta TableIndex proof surface.
