# 293x-899 MIMAP-296A Segment Arena Backing Modeled Allocation-Ledger Release-Applied Recycle Second-Release Diagnostic

Status: selected current
Date: 2026-05-20

## Decision

Add a diagnostic proof for the current one-release-applied-recycle boundary
after the segment arena backing modeled allocation-ledger release-applied
recycle closeout.

## Context

MIMAP-292A proved:

```text
accepted release-apply report
  -> modeled release-applied recycle row
```

MIMAP-296A proves the next boundary without opening generation/lifecycle token
semantics:

```text
recycled model row
  -> second release diagnostic attempt
  -> duplicate / stale reject
```

## Scope

- Add one diagnostics owner, proof app, and L2 guard.
- Observe a modeled release-applied recycle report and inventory.
- Surface duplicate / stale second-release rejection as scalar facts.
- Do not create a new release-applied recycle row from the diagnostics owner.

## Stop Lines

- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation, release, or recycle.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No generation/lifecycle token introduction in this row.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_second_release_diagnostic_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-296A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
