---
Status: Landed
Date: 2026-05-23
Scope: proof-only exact `usize` stack-top decrement/increment probe.
Related:
  - lang/src/hako_alloc/memory/usize_field_probe_box.hako
  - apps/hako-alloc-usize-field-probe/
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
---

# 294x-41 Hako Alloc Usize Stack-Top Probe

## Decision

Extend the isolated `HakoAllocUsizeFieldProbe` with stack-top shaped exact
`usize` stored fields:

- `free_top`;
- `local_free_top`;
- `free_top_underflow_reject_count`;
- `local_free_overflow_reject_count`;
- `local_free_underflow_reject_count`.

The probe exercises guarded decrement and increment paths before any production
`HakoAllocPageModel` stack-top field migrates to exact `usize`.

## Stop Line

This row does not migrate production allocator state.

Production `HakoAllocPageModel.used`, `free_top`, `local_free_top`,
`peak_used`, capacity fields, byte-length fields, page ids, queue indexes,
remote-free mailbox fields, provider activation, host allocator replacement,
hooks, and global allocator integration remain closed.

## Verification

```bash
bash apps/hako-alloc-usize-field-probe/test.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
