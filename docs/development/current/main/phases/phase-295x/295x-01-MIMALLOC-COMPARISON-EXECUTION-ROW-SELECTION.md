---
Status: Landed
Date: 2026-05-24
Scope: select the first phase-295x mimalloc comparison execution row.
Blocker: MIMALLOC-COMPARISON-EXECUTION-ROW-SELECTION-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-00-MIMALLOC-COMPARISON-EXECUTION-LANE-LOCK.md
  - docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md
  - docs/development/current/main/phases/phase-294x/294x-268-MIMALLOC-COMPARISON-VSLICE-PAGE-HEAP-USIZE-REFRESH.md
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh
---

# 295x-01 Mimalloc Comparison Execution Row Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-EXECUTION-ROW-SELECTION-001
```

Select:

```text
MIMALLOC-COMPARISON-C-RUNNER-EVIDENCE-CONTRACT-REFRESH-001
```

## Why

The first phase-295x row should not add a new allocator model. It should
revalidate the existing explicit C mimalloc runner evidence contract against
the refreshed `.hako` / `hako_alloc` vertical slice.

This keeps phase-295x anchored on comparable evidence before selecting the
next `.hako` port seam.

## Stop Line

The refresh row must not:

- add benchmark repetition or a new workload family;
- enable provider package / DLL generation;
- enable provider activation or provider API execution;
- replace the process allocator, install hooks, add backend matchers, or use
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Next Row

Implement:

```text
MIMALLOC-COMPARISON-C-RUNNER-EVIDENCE-CONTRACT-REFRESH-001
```

Expected validation should compose:

- the phase-294x page-heap usize refreshed vertical-slice guard;
- the existing explicit C mimalloc runner execution pilot guard at L2.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
