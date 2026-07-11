---
Status: Landed
Date: 2026-05-12
Scope: isolated hako_alloc usize stored field migration probe.
Related:
  - lang/src/hako_alloc/memory/usize_field_probe_box.hako
  - apps/hako-alloc-usize-field-probe/
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
---

# 294x-18 Hako Alloc Usize Field Probe

## Decision

The first hako_alloc `usize` field migration is probe-only.

`HakoAllocUsizeFieldProbe` stores non-negative allocator-like fields as
`usize`:

- `capacity`;
- `used`;
- `alloc_count`;
- `requested_bytes`.

The proof app exercises successful records, capacity rejection, field reads,
and field writes through VM reference execution.

## Stop Line

This row does not migrate production `HakoAllocPageModel`,
`HakoAllocPageQueue`, `HakoAllocPage`, `HakoAllocHeap`, or
`HakoAllocProductionFacade` fields.

It also does not add native backend exact `usize` field lowering, OSVM, TLS,
atomics, remote-free, page-map, allocator replacement, or hook behavior.

## Verification

```bash
bash apps/hako-alloc-usize-field-probe/test.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
