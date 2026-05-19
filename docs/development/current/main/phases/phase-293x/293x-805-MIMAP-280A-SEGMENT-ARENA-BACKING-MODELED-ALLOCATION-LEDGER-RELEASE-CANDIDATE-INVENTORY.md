# 293x-805 MIMAP-280A Segment Arena Backing Modeled Allocation Ledger Release Candidate Inventory

Status: selected current
Date: 2026-05-19

## Decision

Consume an accepted MIMAP-276A allocation-ledger report and record a model-only
release candidate row before any real arena backing release opens.

## Context

MIMAP-278A closed the modeled allocation-ledger family. The next durable bridge
should keep release/recycle preparation in scalar/model space by creating a
release candidate from accepted allocation-ledger facts.

## Scope

- Add a scalar/model release-candidate inventory owner.
- Consume accepted allocation-ledger reports only.
- Publish release candidate token, ledger token, apply token, segment/arena
  identity, applied backing bytes, applied committed bytes, and remaining
  source bytes.
- Reject missing/rejected ledger reports, invalid release-candidate token,
  duplicate release-candidate token, and closed substrate requirements.
- Keep this row L2 daily unless it introduces a new backend route shape.

## Stop Lines

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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
