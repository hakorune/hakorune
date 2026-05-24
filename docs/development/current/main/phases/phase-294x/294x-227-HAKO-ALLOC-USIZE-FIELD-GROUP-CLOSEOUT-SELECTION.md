---
Status: Landed
Date: 2026-05-24
Scope: close the current exact `usize` field-group drain and return to the
  mimalloc comparison vertical-slice closeout path.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-223
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/development/current/main/phases/phase-294x/294x-59-MIMALLOC-COMPARISON-VERTICAL-SLICE-CLOSEOUT.md
  - docs/development/current/main/phases/phase-294x/294x-usize-field-group-ledger.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_vertical_slice_closeout_guard.sh
---

# 294x-227 Hako Alloc Usize Field-Group Closeout Selection

## Decision

Do not select another exact `usize` production field group under
`HAKO-ALLOC-USIZE-FIELD-GROUP-223`.

The remaining visible candidates are either already migrated by earlier rows or
belong to explicit carryover categories:

- report mirrors / `ReportFields` payload mirrors;
- status/reason vocabularies and bool-like flags;
- signed sentinel-bearing ids, indexes, and deltas;
- pointer-like payloads and byte-count payloads outside the current
  comparison-quality vertical slice;
- broad provider, worker/TLS, atomic, remote-free, hook, host replacement, or
  `#[global_allocator]` seams.

Select `MIMALLOC-COMPARISON-VSLICE-009` as the next blocker. Its job is to
refresh the comparison vertical-slice closeout after the landed exact `usize`
field-group rows and keep the V5 closeout guard independent from stale current
taskboard follow-on pointers.

## Stop Line

This row does not migrate any `.hako` field and does not open:

- C mimalloc execution beyond the existing explicit runner planning surface;
- provider activation or provider package / DLL generation;
- host allocator replacement, hooks, or `#[global_allocator]`;
- worker/TLS, true threads, atomics, remote-free stress, or abandoned-heap
  stress;
- native allocator replacement claims.

## Next Row

`MIMALLOC-COMPARISON-VSLICE-009` should verify the current comparison
vertical-slice closeout entry after the field-group migration series. If the
existing closeout proof remains stable, return to the C mimalloc comparison
execution/evidence lane instead of draining unrelated numeric fields.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_vertical_slice_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
