---
Status: Landed
Date: 2026-05-23
Scope: proof-only exact `usize` capacity-bound and signed-index guard probe.
Related:
  - lang/src/hako_alloc/memory/usize_field_probe_box.hako
  - apps/hako-alloc-usize-field-probe/
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
---

# 294x-44 Hako Alloc Usize Capacity Bound Probe

## Decision

Extend the isolated `HakoAllocUsizeFieldProbe` with capacity-bound shapes needed
before production page capacity fields migrate:

- `loop(i < me.capacity)` where `i` is the current-lane signed integer loop
  cursor and `capacity` is exact `usize`;
- `index < 0` guard followed by `index >= me.capacity` rejection;
- exact `usize` counters for accepted/rejected index observations.

This row proves the signed-index guard shape used by page-local APIs before
`HakoAllocPageModel.capacity` or `reserved` can migrate.

## Stop Line

This row does not migrate production allocator state.

Production page capacity/reserved fields, page ids, block ids, byte accounting,
queue indexes, remote-free mailbox fields, provider activation, host allocator
replacement, hooks, and global allocator integration remain closed.

## Verification

```bash
bash apps/hako-alloc-usize-field-probe/test.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
