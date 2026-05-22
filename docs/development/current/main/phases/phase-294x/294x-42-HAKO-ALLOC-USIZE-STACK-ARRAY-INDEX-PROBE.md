---
Status: Landed
Date: 2026-05-23
Scope: proof-only exact `usize` stack-top ArrayBox index probe.
Related:
  - lang/src/hako_alloc/memory/usize_field_probe_box.hako
  - apps/hako-alloc-usize-field-probe/
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
---

# 294x-42 Hako Alloc Usize Stack Array Index Probe

## Decision

Extend the isolated `HakoAllocUsizeFieldProbe` so exact `usize` stack-top values
are used as `ArrayBox.get/set` indexes in a page-stack-like shape.

The probe covers:

- `free_top - 1` followed by `ArrayBox.get(index)`;
- `local_free_top` followed by `ArrayBox.set(index, value)`;
- `local_free_top - 1` followed by `ArrayBox.get(index)`;
- `array_index_probe_count` as an exact `usize` observation counter.

This row proves the array-index seam needed before production page stack fields
can migrate to exact `usize`.

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
