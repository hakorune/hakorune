---
Status: Landed
Date: 2026-05-24
Scope: refresh the explicit C mimalloc runner evidence contract for phase-295x.
Blocker: MIMALLOC-COMPARISON-C-RUNNER-EVIDENCE-CONTRACT-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-01-MIMALLOC-COMPARISON-EXECUTION-ROW-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-268-MIMALLOC-COMPARISON-VSLICE-PAGE-HEAP-USIZE-REFRESH.md
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_page_heap_usize_refresh_guard.sh
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh
---

# 295x-02 Mimalloc Comparison C Runner Evidence Contract Refresh

## Decision

Close:

```text
MIMALLOC-COMPARISON-C-RUNNER-EVIDENCE-CONTRACT-REFRESH-001
```

The explicit C mimalloc runner evidence contract remains usable as the
phase-295x comparison execution baseline.

This row composes:

- the current `.hako` / `hako_alloc` vertical-slice refresh after page-heap
  exact `usize` closeout;
- the existing MIMAP-451A explicit C mimalloc runner execution pilot guard at
  L2.

## Stable Evidence Contract

The C runner remains explicit and bounded:

- stable output contract:
  `allocator-comparison-c-mimalloc-explicit-runner-v0`;
- representative small-block workload;
- memory-use evidence present;
- allocation/free counts match;
- process allocator replacement, hooks, backend matchers, global allocator
  install, hidden discovery, and provider package generation remain inactive.

## Stop Line

This row does not:

- add benchmark repetition or a new workload family;
- change the C runner output schema;
- enable provider package / DLL generation;
- enable provider activation or provider API execution;
- replace the process allocator, install hooks, add backend matchers, or use
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Next Row

Return to row selection from:

```text
MIMALLOC-COMPARISON-POST-C-RUNNER-EVIDENCE-ROW-SELECTION-001
```

The expected next direction is a `.hako` vs C comparison ledger refresh that
consumes both existing evidence surfaces without rerunning or widening the
benchmark matrix.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_c_mimalloc_runner_evidence_refresh_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
