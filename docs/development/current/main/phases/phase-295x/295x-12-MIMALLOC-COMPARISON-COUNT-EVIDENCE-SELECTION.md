---
Status: Landed
Date: 2026-05-24
Scope: select the next comparison seam after repeated same-workload RSS evidence.
Blocker: MIMALLOC-COMPARISON-COUNT-EVIDENCE-SELECTION-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-11-MIMALLOC-COMPARISON-REPEATED-RUN-CLOSEOUT.md
  - tools/allocator/hako_exe_memory_runner.sh
  - tools/allocator/mimalloc_comparison_memory_report.py
  - tools/checks/k2_wide_phase295x_count_evidence_selection_guard.sh
---

# 295x-12 Mimalloc Comparison Count Evidence Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-COUNT-EVIDENCE-SELECTION-295X-001
```

Select:

```text
MIMALLOC-COMPARISON-HAKO-COUNT-EVIDENCE-295X-REFRESH-001
```

Why: the C mimalloc runner already publishes `allocation_count` and
`free_count`, while the `.hako` memory runner currently publishes requested
bytes, committed bytes, RSS, and closed replacement/provider fields. The
representative `.hako` proof app already prints `page=alloc_count,free_count,...`
for the same workload, so the next narrow seam is to surface those counts in the
hako memory evidence runner and shared comparison report.

This improves semantic comparison without changing workload shape.

## Stop Line

This row does not:

- change allocation behavior or workload shape;
- add benchmark repetition, warmup, or summary statistics;
- make performance or memory winner claims;
- enable provider package / DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_count_evidence_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
