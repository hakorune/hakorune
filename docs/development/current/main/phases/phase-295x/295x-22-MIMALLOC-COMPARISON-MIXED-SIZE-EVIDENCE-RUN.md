---
Status: Landed
Date: 2026-05-24
Scope: run C mimalloc and `.hako` mixed-size same-workload evidence through the normalizer.
Blocker: MIMALLOC-COMPARISON-MIXED-SIZE-EVIDENCE-295X-RUN-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-21-MIMALLOC-COMPARISON-MIXED-SIZE-CONTRACT-REFRESH.md
  - apps/hako-alloc-mimalloc-comparison-mixed-small-exe-proof/main.hako
  - tools/allocator/c_mimalloc_explicit_runner.sh
  - tools/allocator/hako_exe_memory_runner.sh
  - tools/allocator/mimalloc_comparison_memory_report.py
  - tools/checks/k2_wide_phase295x_mixed_size_evidence_run_guard.sh
---

# 295x-22 Mimalloc Comparison Mixed-Size Evidence Run

## Decision

Close:

```text
MIMALLOC-COMPARISON-MIXED-SIZE-EVIDENCE-295X-RUN-001
```

The C mimalloc explicit runner and `.hako` EXE memory runner now normalize a
same-workload report for:

```text
workload=representative-mixed-small-v0
operation_family=mixed-small
operation_sequence_id=representative-mixed-small-v0-seq
free_order_id=ascending-release-v0
```

The row requires parity for workload identity, operation identity,
allocation/free counts, and requested bytes. RSS remains side-by-side evidence
only.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-MIXED-SIZE-CLOSEOUT-295X-001
```

Reason: the mixed-size evidence path is executable on both sides and
normalized. The next row should close the mixed-size workload family and select
whether to proceed to huge/OSVM comparison seam selection or repeated
measurement policy.

## Stop Line

This row does not:

- require RSS parity;
- add benchmark warmup or final summary statistics;
- make performance or memory winner claims;
- replace the process allocator or install hooks;
- enable provider packages, DLL generation, provider activation, provider API
  execution, backend matchers, or `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, huge/OSVM execution, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mixed_size_evidence_run_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
