---
Status: Landed
Date: 2026-05-24
Scope: refresh repeated same-workload RSS evidence for phase-295x.
Blocker: MIMALLOC-COMPARISON-REPEATED-RUN-295X-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-09-MIMALLOC-COMPARISON-NEXT-SEAM-SELECTION.md
  - tools/allocator/mimalloc_comparison_repeated_run_evidence.py
  - tools/checks/k2_wide_phase295x_repeated_run_refresh_guard.sh
---

# 295x-10 Mimalloc Comparison Repeated-Run Refresh

## Decision

Close:

```text
MIMALLOC-COMPARISON-REPEATED-RUN-295X-REFRESH-001
```

Refresh repeated same-workload RSS evidence for
`representative-small-block-v0`. This row runs three same-workload samples,
formats each as a single-run RSS presentation, and aggregates the range into
`mimalloc-comparison-repeated-run-evidence-v0`.

Expected refreshed facts:

- `sample_count=3`;
- `workload_match=1`;
- `requested_bytes_delta=0`;
- `.hako` and C mimalloc RSS ranges are positive;
- `winner_claim=0`.

Select:

```text
MIMALLOC-COMPARISON-REPEATED-RUN-CLOSEOUT-295X-001
```

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
bash tools/checks/k2_wide_phase295x_repeated_run_refresh_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
