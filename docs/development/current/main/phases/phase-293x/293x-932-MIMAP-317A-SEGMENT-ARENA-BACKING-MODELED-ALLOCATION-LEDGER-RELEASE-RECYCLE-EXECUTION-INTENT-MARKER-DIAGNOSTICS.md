# 293x-932 MIMAP-317A Segment Arena Backing Modeled Allocation-Ledger Release/Recycle Execution Intent Marker Diagnostics

Status: landed
Date: 2026-05-20

## Decision

Add observer-only diagnostics for the MIMAP-316A release/recycle execution
intent marker.

## Context

MIMAP-316A records explicit model-only release/recycle execution intent while
keeping execution unsupported. MIMAP-317A should observe those marker facts and
publish scalar diagnostic counters before any closeout pack or real execution
row opens.

## Scope

- Add one observer-only diagnostic owner, proof app, and L2 guard.
- Observe accepted/rejected intent marker reports.
- Publish scalar diagnostics for unsupported execution and missing/rejected
  marker evidence.
- Keep real release/recycle execution closed.

## Stop Lines

- No new execution intent marker row recording from the diagnostic owner.
- No real release/recycle execution.
- No real lifecycle generation token.
- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation, release, or recycle.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_diagnostics_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

MIMAP-317A landed observer-only diagnostics for the model-only release/recycle
execution intent marker.

Selected next:

```text
MIMAP-318A Segment Arena Backing Modeled Allocation-Ledger Release/Recycle Execution Intent Marker Closeout
```
