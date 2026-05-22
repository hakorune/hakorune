---
Status: Complete
Date: 2026-05-23
Scope: migrate page-map release observer counters from `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_map_release_invariant_box.hako
  - tools/checks/k2_wide_mimalloc_pre_realloc_release_invariant_guard.sh
---

# 294x-40 Hako Alloc Usize Page-Map Release Observer Counters

## Decision

Migrate the `HakoAllocPageMapReleaseObserver` monotonic observer counters:

- `observe_count`
- `success_count`
- `reject_count`

These fields only count begin/finish observer outcomes and do not carry
negative sentinels.

## Stop Line

The `*_before` snapshot fields, `last_*` status fields, stored `-1` sentinel
fields, and signed delta fields remain `i64`. This row does not migrate
page-map release seam behavior, page-map entry pointer/id fields, object-return
API, realloc execution, byte copy, provider activation, host allocator
replacement, hooks, or `#[global_allocator]`.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_pre_realloc_release_invariant_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
