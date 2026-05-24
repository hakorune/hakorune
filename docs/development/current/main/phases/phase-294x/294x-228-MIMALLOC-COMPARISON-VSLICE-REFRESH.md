---
Status: Landed
Date: 2026-05-24
Scope: refresh the mimalloc comparison vertical-slice closeout after exact
  `usize` field-group migration.
Blocker: MIMALLOC-COMPARISON-VSLICE-009
Related:
  - docs/development/current/main/phases/phase-294x/294x-227-HAKO-ALLOC-USIZE-FIELD-GROUP-CLOSEOUT-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-59-MIMALLOC-COMPARISON-VERTICAL-SLICE-CLOSEOUT.md
  - docs/development/current/main/phases/phase-293x/293x-1075-MIMAP-453A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXPLICIT-RUNNER-CLOSEOUT.md
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_vertical_slice_closeout_guard.sh
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_closeout_guard.sh
---

# 294x-228 Mimalloc Comparison Vertical-Slice Refresh

## Decision

Close `MIMALLOC-COMPARISON-VSLICE-009`.

The comparison vertical-slice closeout remains stable after the exact `usize`
field-group migration series. The refreshed evidence confirms:

- V2 small-path schema remains stable;
- V3 realloc/aligned schema remains stable;
- V4 huge/OSVM schema remains stable;
- V5 comparison closeout still emits `vertical-slice-v1`;
- MIMAP-451A / MIMAP-452A explicit C mimalloc runner evidence and diagnostics
  still close through MIMAP-453A.

The next useful row is `MIMALLOC-COMPARISON-RESULT-LEDGER-REFRESH-001`: refresh
the existing MIMAP-454A C-vs-Hako result ledger path against the current
explicit C mimalloc runner evidence, without opening new benchmark repetition,
provider activation, host replacement, hooks, or global allocator behavior.

## Stop Line

This row does not:

- migrate additional exact `usize` fields;
- add a new `.hako` owner;
- run repeated or heavy benchmark packs;
- enable provider activation, host allocator replacement, hooks, backend
  matchers, provider package / DLL generation, or `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned-heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_vertical_slice_closeout_guard.sh
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
