---
Status: Landed
Date: 2026-05-24
Scope: run C mimalloc and `.hako` huge-ish same-workload evidence through the normalizer.
Blocker: MIMALLOC-COMPARISON-HUGE-ISH-EVIDENCE-295X-RUN-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-25-MIMALLOC-COMPARISON-HUGE-ISH-CONTRACT-REFRESH.md
  - apps/hako-alloc-mimalloc-comparison-huge-ish-exe-proof/main.hako
  - tools/allocator/c_mimalloc_explicit_runner.sh
  - tools/allocator/hako_exe_memory_runner.sh
  - tools/allocator/mimalloc_comparison_memory_report.py
  - tools/checks/k2_wide_phase295x_huge_ish_evidence_run_guard.sh
---

# 295x-26 Mimalloc Comparison Huge-Ish Evidence Run

## Decision

Close:

```text
MIMALLOC-COMPARISON-HUGE-ISH-EVIDENCE-295X-RUN-001
```

The C mimalloc explicit runner and `.hako` EXE memory runner now normalize a
same-workload report for:

```text
workload=representative-huge-ish-v0
operation_family=huge-ish
operation_sequence_id=representative-huge-ish-v0-seq
free_order_id=ascending-release-v0
```

The row requires parity for workload identity, operation identity,
allocation/free counts, requested bytes, and `large_request_count`.

RSS remains side-by-side evidence only. `large_request_count` is a workload
classification field, not a claim that `.hako` and C mimalloc use the same
OSVM, page-source, decommit, or huge allocation substrate.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-HUGE-ISH-CLOSEOUT-295X-001
```

Reason: the huge-ish evidence path is executable on both sides and normalized.
The next row should close the huge-ish workload family and select whether to
proceed to repeated measurement policy, another workload family, or a parked
carryover lane.

## Stop Line

This row does not:

- require RSS parity;
- claim OSVM/page-source/decommit/unreserve parity;
- add benchmark warmup or final summary statistics;
- make performance or memory winner claims;
- replace the process allocator or install hooks;
- enable provider packages, DLL generation, provider activation, provider API
  execution, backend matchers, or `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_huge_ish_evidence_run_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
