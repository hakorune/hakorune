---
Status: Landed
Date: 2026-05-24
Scope: refresh the same-workload `.hako` vs C mimalloc memory report execution path.
Blocker: MIMALLOC-COMPARISON-SAME-WORKLOAD-295X-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-06-MIMALLOC-COMPARISON-RESULT-LEDGER-CLOSEOUT.md
  - docs/development/current/main/phases/phase-294x/294x-162-MIMALLOC-COMPARISON-SAME-WORKLOAD-MEMORY-REPORT.md
  - tools/checks/k2_wide_phase295x_same_workload_refresh_guard.sh
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_same_workload_memory_report_guard.sh
---

# 295x-07 Mimalloc Comparison Same-Workload Refresh

## Decision

Close:

```text
MIMALLOC-COMPARISON-SAME-WORKLOAD-295X-REFRESH-001
```

Refresh the same-workload memory report path by executing the existing
`representative-small-block-v0` `.hako` proof app through the hako EXE memory
runner and the explicit C mimalloc runner through the shared normalizer.

This is the first phase-295x row that directly reuses the same-workload
execution path:

```text
.hako representative-small-block-v0
  -> hako_exe_memory_runner.sh

C mimalloc representative-small-block-v0
  -> c_mimalloc_explicit_runner.sh

both
  -> mimalloc_comparison_memory_report.py
```

Expected refreshed facts:

- `workload_match=1`;
- `requested_bytes_delta=0`;
- `.hako` memory-use evidence is present;
- C mimalloc memory-use evidence is present;
- `winner_claim=0`.

Select:

```text
MIMALLOC-COMPARISON-SAME-WORKLOAD-CLOSEOUT-295X-001
```

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
bash tools/checks/k2_wide_phase295x_same_workload_refresh_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
