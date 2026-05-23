---
Status: Landed
Date: 2026-05-24
Scope: huge page model size-observer exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-184
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-185-HAKO-ALLOC-USIZE-HUGE-MODEL-SIZE-OBSERVER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/huge_page_model_box.hako
  - tools/checks/k2_wide_mimalloc_huge_page_model_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard.sh
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
---

# 294x-186 Hako Alloc Usize Huge Model Size Observers

## Decision

Migrate only these `HakoAllocHugePageModel` success-size observers to exact
`usize` storage:

- `last_requested_size`
- `last_committed_size`

The model resets both fields to `0`, leaves them at `0` on reject paths, and
stores accepted huge allocation request/commit sizes only after the positive
request and committed-size checks pass.

## Stop Line

This row does not migrate:

- `HakoAllocHugePageModel.next_ptr`;
- `HakoAllocHugePageModel.last_result_ptr`;
- `HakoAllocHugePageModel.last_page_id`;
- `HakoAllocHugePageModel.last_failure_kind`;
- `HakoAllocHugeReleaseSeam.last_requested_size`;
- `HakoAllocHugeReleaseSeam.last_committed_size`;
- facade route/report mirrors;
- provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_huge_page_model_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard.sh
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_huge_unregister_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
