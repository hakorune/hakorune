---
Status: Landed
Date: 2026-05-23
Scope: production page-model size/byte exact `usize` migration.
Related:
  - lang/src/hako_alloc/memory/page_box.hako
  - tools/checks/k2_wide_mimalloc_page_model_guard.sh
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-46-HAKO-ALLOC-USIZE-REQUEST-BYTE-SUM-PROBE.md
---

# 294x-47 Hako Alloc Usize Page Model Size Bytes

## Decision

Migrate only the `HakoAllocPageModel` owner-local size/byte field group to exact
`usize` storage:

- `block_size`;
- `requested_bytes`.

This follows the proof-only `294x-46` request byte-sum probe, which fixed the
`requested_size <= block_size` comparison and accepted-request byte-sum
accumulation before touching production page-model storage.

`HakoAllocPageModel.birth` now declares `block_size`, `capacity`, and `reserved`
as exact `usize` parameters. `HakoAllocPageModel.acquire` declares
`requested_size` as exact `usize`.

Proof apps that report signed observer deltas must keep those deltas on the
signed lane. This row updates the M175 alloc-copy-release proof to avoid
computing a negative delta directly from exact `usize` fields.

## Stop Line

This row does not migrate page identity, lifecycle state flags, queue indexes,
remote-free mailbox fields, provider activation, host allocator replacement,
hooks, or global allocator integration.

`HakoAllocPageModel.page_id`, `retired`, and `decommitted` remain `i64`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_page_model_guard.sh
bash tools/checks/k2_wide_mimalloc_local_free_retire_guard.sh
bash tools/checks/k2_wide_mimalloc_request_path_usize_guard.sh
bash tools/checks/k2_wide_mimalloc_realloc_alloc_copy_release_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
