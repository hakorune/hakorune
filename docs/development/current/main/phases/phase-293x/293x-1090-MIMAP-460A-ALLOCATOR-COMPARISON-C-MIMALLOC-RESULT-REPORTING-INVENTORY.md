# 293x-1090 MIMAP-460A Allocator Comparison C Mimalloc Result Reporting Inventory

Status: selected current
Date: 2026-05-21

## Purpose

Open a narrow reporting inventory over the MIMAP-458A C-vs-Hako result summary
diagnostics.

This row should prepare stable scalar fields for a later presentation /
decision row. It must not rerun benchmarks and must not make a performance or
memory-use conclusion.

## Scope

- Consume the MIMAP-458A result summary diagnostic report.
- Publish scalar reporting inventory fields for:
  - summary diagnostic readiness
  - comparison availability
  - Hako/C mimalloc allocation and request-byte evidence
  - already-recorded deltas
  - closed stop-line evidence
- Keep this as inventory, not a final report or decision row.

## Stop Lines

- No repeated or heavy benchmark pack.
- No performance conclusion.
- No memory-use conclusion.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No worker/thread execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Planned validation profile: `scalar-mir`.

Expected guard:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_reporting_inventory_guard.sh --level L2
```

## Task Order

1. Add the reporting inventory owner and proof app.
2. Add a focused L2 guard consuming the MIMAP-458A summary diagnostic report.
3. Keep benchmark execution and final reporting conclusions closed.
4. Select a reporting diagnostics / presentation-preflight row only after the
   inventory guard is green.

## Forward Pack

Use this pack unless a guard exposes a real compiler/backend blocker:

```text
MIMAP-460A:
  reporting inventory
  validation: L2 scalar-mir
  opens: stable scalar reporting fields only

MIMAP-461A:
  reporting diagnostics
  validation: L2 scalar-mir
  opens: observer-only ready/blocked classification

MIMAP-462A:
  reporting closeout
  validation: L2 closeout pack
  opens: nothing; reruns 460A/461A guards

MIMAP-463A:
  presentation / decision row selection
  validation: L0 planning
  decides whether the next row is presentation-only or a guarded first
  performance/memory-use conclusion preflight
```

Do not skip directly from MIMAP-460A to a performance/memory-use conclusion.
The reporting diagnostics and closeout pack are the required boundary first.
