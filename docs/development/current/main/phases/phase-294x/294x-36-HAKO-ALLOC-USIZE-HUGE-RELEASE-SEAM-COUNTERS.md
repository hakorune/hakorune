---
Status: Complete
Date: 2026-05-23
Scope: migrate huge release seam event/reject counters from `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/huge_release_seam_box.hako
  - tools/checks/k2_wide_mimalloc_huge_release_seam_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_huge_unregister_exe_guard.sh
---

# 294x-36 Hako Alloc Usize Huge Release Seam Counters

## Decision

Migrate the huge release seam event/reject counters:

- `release_count`
- `unregister_count`
- `lookup_miss_count`
- `not_huge_count`
- `model_reject_count`
- `reject_count`

These are monotonic seam-local counters and do not carry negative sentinels.

## Stop Line

`last_page_id` remains `i64` because `-1` is the stored missing-page sentinel.
`last_requested_size`, `last_committed_size`, and `last_failure_kind` remain
signed observer/status fields. This row does not migrate facade report fields,
OSVM, provider activation, host allocator replacement, hooks, or
`#[global_allocator]`.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_huge_release_seam_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_huge_unregister_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
