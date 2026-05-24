---
Status: Landed
Date: 2026-05-24
Scope: close the same-workload `.hako` vs C mimalloc memory report refresh.
Blocker: MIMALLOC-COMPARISON-SAME-WORKLOAD-CLOSEOUT-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-07-MIMALLOC-COMPARISON-SAME-WORKLOAD-REFRESH.md
  - tools/checks/k2_wide_phase295x_same_workload_closeout_guard.sh
  - tools/checks/k2_wide_phase295x_same_workload_refresh_guard.sh
---

# 295x-08 Mimalloc Comparison Same-Workload Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-SAME-WORKLOAD-CLOSEOUT-295X-001
```

The same-workload memory report path is green for
`representative-small-block-v0`:

- both sides use the same workload id;
- both sides report `requested_bytes=33254`;
- both sides publish memory-use evidence;
- the shared report keeps `winner_claim=0`.

Select:

```text
MIMALLOC-COMPARISON-NEXT-PORT-SEAM-SELECTION-295X-001
```

The next row should choose one `.hako` port seam that improves comparison
quality. It should not expand into repeated benchmark statistics, provider
activation, or allocator replacement.

## Stop Line

This row does not:

- add benchmark repetition, warmup, or summary statistics;
- make performance or memory winner claims;
- change either runner output schema;
- enable provider package / DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_same_workload_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
