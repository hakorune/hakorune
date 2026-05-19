# 293x-892 MIMAP-289A Segment Arena Backing Modeled Allocation-Ledger Release Apply Diagnostics

Status: selected current
Date: 2026-05-20

## Decision

Observe MIMAP-288A release-apply inventory counters and last-apply facts
without recording new release-apply rows or opening real allocator execution.

## Context

MIMAP-288A records a model-only release apply from accepted modeled
allocation-ledger release-intent facts. The next row should expose scalar
diagnostic summary facts so the release-apply family can later be closed out
before any real arena backing release, segment-map mutation, OSVM, atomics, or
raw pointer residence opens.

## Scope

- Add a scalar diagnostic owner for MIMAP-288A release-apply facts.
- Publish inventory / accepted / reject counters.
- Publish missing/rejected intent, invalid release-apply token, duplicate
  release-apply token, and closed-substrate reject category facts.
- Publish last reason, last segment, last arena, and last release-apply token.
- Keep this row L2 daily unless it introduces a new backend route shape.

## Stop Lines

- No new release-apply rows.
- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation or release.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_apply_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
