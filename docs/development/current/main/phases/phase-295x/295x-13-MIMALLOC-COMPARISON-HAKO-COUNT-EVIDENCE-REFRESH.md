---
Status: Landed
Date: 2026-05-24
Scope: surface `.hako` allocation/free counts in same-workload memory evidence.
Blocker: MIMALLOC-COMPARISON-HAKO-COUNT-EVIDENCE-295X-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-12-MIMALLOC-COMPARISON-COUNT-EVIDENCE-SELECTION.md
  - tools/allocator/hako_exe_memory_runner.sh
  - tools/allocator/mimalloc_comparison_memory_report.py
  - tools/checks/k2_wide_phase295x_hako_count_evidence_refresh_guard.sh
---

# 295x-13 Mimalloc Comparison Hako Count Evidence Refresh

## Decision

Close:

```text
MIMALLOC-COMPARISON-HAKO-COUNT-EVIDENCE-295X-REFRESH-001
```

The hako memory evidence runner now reads the selected comparison app's
`page=` evidence and publishes:

- `allocation_count`;
- `free_count`.

The shared memory report also publishes both hako and C counts plus:

- `allocation_count_delta`;
- `free_count_delta`.

For `representative-small-block-v0`, the deltas are zero.

Select:

```text
MIMALLOC-COMPARISON-COUNT-EVIDENCE-CLOSEOUT-295X-001
```

## Stop Line

This row does not:

- change allocation behavior or workload shape;
- change the C runner output schema;
- add benchmark repetition, warmup, or summary statistics;
- make performance or memory winner claims;
- enable provider package / DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_hako_count_evidence_refresh_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
