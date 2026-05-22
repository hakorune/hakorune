---
Status: Complete
Date: 2026-05-22
Scope: migrate one huge-threshold router counter field group from `i64` to
  exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-26-HAKO-ALLOC-USIZE-ALIGNED-SMALL-PATH-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/huge_threshold_router_box.hako
  - tools/checks/k2_wide_mimalloc_huge_threshold_routing_guard.sh
---

# 294x-27 Hako Alloc Usize Huge-Threshold Router Counters

## Decision

Migrate only the `HakoAllocHugeThresholdRouter` route/event/reject counter
group:

- `small_route_count`
- `small_success_count`
- `small_reject_count`
- `huge_route_count`
- `huge_reject_count`
- `invalid_alignment_count`
- `invalid_size_count`
- `reject_count`

These fields are non-negative counters owned by the threshold router. They
classify route outcomes and fail-fast rejection totals, without carrying
pointer, size, threshold, or route-status meaning.

## Stop Line

`last_route_kind`, `last_result_ptr`, `last_padded_size`, `last_good_size`, and
`last_huge_threshold` remain `i64` in this row. They are status, pointer, and
size/threshold observers and need separate exactness contracts.

No huge-page model fields, aligned metadata-store count fields, page-map entry
pointer/id fields, provider activation, host allocator replacement, hooks, or
`#[global_allocator]` are opened by this row.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_huge_threshold_routing_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
