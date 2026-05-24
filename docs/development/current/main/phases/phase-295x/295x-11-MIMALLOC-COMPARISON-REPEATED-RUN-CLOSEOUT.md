---
Status: Landed
Date: 2026-05-24
Scope: close repeated same-workload RSS evidence refresh for phase-295x.
Blocker: MIMALLOC-COMPARISON-REPEATED-RUN-CLOSEOUT-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-10-MIMALLOC-COMPARISON-REPEATED-RUN-REFRESH.md
  - tools/checks/k2_wide_phase295x_repeated_run_closeout_guard.sh
  - tools/checks/k2_wide_phase295x_repeated_run_refresh_guard.sh
---

# 295x-11 Mimalloc Comparison Repeated-Run Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-REPEATED-RUN-CLOSEOUT-295X-001
```

The same-workload repeated-run evidence path is green for three RSS samples.
The row closes only the evidence range contract; it does not convert the range
into a winner claim.

Select:

```text
MIMALLOC-COMPARISON-COUNT-EVIDENCE-SELECTION-295X-001
```

The next useful comparison-quality decision is whether to expose `.hako`
allocation/free counts through the hako memory evidence runner, because the C
mimalloc runner already publishes `allocation_count` and `free_count`.

## Stop Line

This row does not:

- add warmup or final benchmark statistics;
- make performance or memory winner claims;
- change either runner output schema;
- enable provider package / DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_repeated_run_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
