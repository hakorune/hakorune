# 293x-661 HAKO-ALLOC-REPORT-RECORD-002 Local-Free Integration Report Record Boundary Cleanup

Status: landed
Date: 2026-05-18

## Decision

Replace the local-free integration `report(...)` scalar helper boundary with a
builder-local record payload, preserving the existing report box and proof
output.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_integration_box.hako
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh
apps/hako-alloc-segment-allocation-modeled-local-free-integration-proof/main.hako
```

## Scope

- Add an owner-local record declaration for the current integration report
  scalar group.
- Collapse the `report(...)` helper call boundary into record literal
  construction/read inside the same owner.
- Keep the returned
  `HakoAllocSegmentAllocationModeledLocalFreeIntegrationReport` box unchanged.
- Preserve existing VM / pure-first EXE proof output.

## Stop Lines

- No allocator behavior change.
- No broad report cleanup sweep.
- No record pass/return/store escape.
- No packed/backend record lowering.
- No backend `.inc` matcher.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No silent fallback.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Closeout

Landed by adding
`HakoAllocSegmentAllocationModeledLocalFreeIntegrationReportFields`, replacing
the legacy 22-argument `report(...)` helper boundary with local record payloads
at each report construction site, and extending the MIMAP-119A guard to reject
the old helper shape.

Evidence:

```text
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 target/debug/hakorune --backend vm apps/hako-alloc-segment-allocation-modeled-local-free-integration-proof/main.hako
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh
```

Next row:

```text
MIMAP-146A post-report-record-cleanup row selection
```
