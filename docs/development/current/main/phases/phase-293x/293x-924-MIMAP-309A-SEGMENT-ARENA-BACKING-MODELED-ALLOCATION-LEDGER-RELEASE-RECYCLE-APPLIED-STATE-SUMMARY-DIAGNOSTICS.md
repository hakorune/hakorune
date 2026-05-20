# 293x-924 MIMAP-309A Segment Arena Backing Modeled Allocation-Ledger Release/Recycle Applied-State Summary Diagnostics

Status: selected current
Date: 2026-05-20

## Decision

Select a scalar/model applied-state summary diagnostics row after MIMAP-308A.

## Context

MIMAP-308A publishes compact release/recycle applied-state summary facts from
an accepted MIMAP-304A continuation application report. MIMAP-309A should
observe those summary facts and publish diagnostic counters before any closeout
pack or real arena backing release/recycle execution opens.

## Scope

- Add one observer-only diagnostics owner, proof app, and L2 guard.
- Consume MIMAP-308A summary report facts.
- Publish accepted / rejected / missing summary diagnostic facts.
- Keep all real execution and provider routes closed.

## Stop Lines

- No new continuation application row.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-309A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
