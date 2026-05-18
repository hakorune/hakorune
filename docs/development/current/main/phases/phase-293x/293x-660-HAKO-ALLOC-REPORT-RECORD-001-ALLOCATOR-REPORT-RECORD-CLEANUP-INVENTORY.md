# 293x-660 HAKO-ALLOC-REPORT-RECORD-001 Allocator Report Record Cleanup Inventory

Status: landed
Date: 2026-05-18

## Decision

Inventory allocator proof report shapes before introducing record-based report
cleanup.

This row should find one small report/record pilot candidate, or select a
focused compiler row if the current record support is not enough.

## Scope

- Inventory wide allocator proof reports and helper signatures in
  `lang/src/hako_alloc/memory/`.
- Classify each candidate as:
  - safe for current record semantics,
  - blocked by record literal / construction / read support,
  - not worth changing because the scalar fields are clearer.
- Pick at most one source pilot if current compiler support can preserve the
  existing proof output.
- If a compiler gap is found, select one focused compiler acceptance row instead
  of working around it in `.hako`.

## Stop Lines

- No allocator behavior implementation.
- No broad report rewrite.
- No backend lowering or `.inc` matcher.
- No packed/backend record lowering.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No silent fallback.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Candidate Inventory

| Candidate | Shape | Current state | Decision |
| --- | --- | --- | --- |
| `purge_bounded_scheduler_box.hako` | former report/16 | already record-shaped by MIMAP-041A | no new work |
| `segment_allocation_modeled_local_free_integration_box.hako` | `report(...)` with 22 scalar arguments | widest live helper boundary found in allocator proof reports | select first cleanup pilot |
| `segment_allocation_modeled_local_free_page_apply_box.hako` | `localFreePageApplyReport(...)` with 19 scalar arguments | good later candidate, but downstream of apply-plan proof | park |
| `segment_allocation_modeled_local_free_reuse_box.hako` | `report(...)` with 18 scalar arguments | good later candidate, but depends on integration report shape | park |
| `page_lifecycle_invariant_box.hako` | `report(...)` with 17 scalar arguments | broad lifecycle observer used by many later rows | park until narrower pilots hold |
| short reclaim / OSVM reports | 5-12 scalar arguments | readable enough or tied to route-specific proof | no pilot now |

## Landed Result

`HAKO-ALLOC-REPORT-RECORD-001` selected the next narrow row:

```text
HAKO-ALLOC-REPORT-RECORD-002
  local-free integration report record boundary cleanup
```

Rationale:

```text
MIMAP-041A already proved the local record payload pattern for report cleanup.
The next largest allocator report helper is the local-free integration
`report(...)` boundary with 22 scalar arguments. It has one owner and a focused
proof/guard, so it is the smallest high-impact record cleanup pilot.
```
