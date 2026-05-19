# 293x-922 HAKO-ALLOC-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-HELPER-SPLIT Local Free Reuse Ledger Release Apply Helper Split

Status: landed
Date: 2026-05-20

## Decision

Split the local-free reuse ledger release-apply responsibilities into a thin
wrapper and a dedicated helper box while preserving the same scalar release
apply behavior.

## Context

`segment_allocation_modeled_local_free_reuse_ledger_box.hako` was carrying both
the reuse-ledger storage and the release-apply report construction / reject
aggregation. That mixed the ledger state with the scalar report construction
helper and made the box harder to keep narrow.

This row keeps the same release-apply contract while moving the report fields,
report construction, and apply/reject helper logic into a dedicated helper
module.

## Scope

- Keep `segment_allocation_modeled_local_free_reuse_ledger_box.hako` as the
  stable reuse-ledger owner.
- Move release-apply report construction into
  `segment_allocation_modeled_local_free_reuse_ledger_release_apply_box.hako`.
- Keep the same `applyReuseLedgerRelease` and
  `applyReuseLedgerLifecycleKeyedRelease` entrypoints by delegating through the
  helper surface.
- Update the hako module export and memory README to mention the new helper.

## Non-Goals

- Do not change the reuse-ledger behavior.
- Do not change the release-apply acceptance contract.
- Do not widen the allocator seams.
- Do not open raw pointer, segment-map, arena, atomic, OSVM, worker, provider,
  or backend matcher behavior.
- Do not change the current blocker token.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
