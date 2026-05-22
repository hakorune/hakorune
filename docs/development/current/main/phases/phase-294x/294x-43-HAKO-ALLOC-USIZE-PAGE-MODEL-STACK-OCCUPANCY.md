---
Status: Landed
Date: 2026-05-23
Scope: production page-model stack-top and occupancy exact `usize` migration.
Related:
  - lang/src/hako_alloc/memory/page_box.hako
  - tools/checks/k2_wide_mimalloc_page_model_guard.sh
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
---

# 294x-43 Hako Alloc Usize Page Model Stack Occupancy

## Decision

Migrate only the `HakoAllocPageModel` stack-top and occupancy field group to
exact `usize` storage:

- `used`;
- `free_top`;
- `local_free_top`;
- `peak_used`.

This follows the proof-only `294x-41` and `294x-42` rows that exercised guarded
stack-top decrement/increment and exact `usize` `ArrayBox.get/set` index use.

## Stop Line

This row does not migrate page identity, block size, capacity, reserved count,
byte accounting, lifecycle state flags, queue indexes, page-map entry payloads,
remote-free mailbox fields, provider activation, host allocator replacement,
hooks, or global allocator integration.

`HakoAllocPageModel.requested_bytes` stays `i64` until allocator byte-sum
checked arithmetic diagnostics are selected by their own row.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_page_model_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
