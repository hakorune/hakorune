---
Status: Landed
Date: 2026-05-24
Scope: run C mimalloc and `.hako` realloc/aligned same-workload evidence through the normalizer.
Blocker: MIMALLOC-COMPARISON-REALLOC-ALIGNED-EVIDENCE-295X-RUN-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-17-MIMALLOC-COMPARISON-REALLOC-ALIGNED-HAKO-EXE-ACCEPTANCE.md
  - apps/hako-alloc-mimalloc-comparison-realloc-aligned-exe-proof/main.hako
  - tools/allocator/c_mimalloc_explicit_runner.sh
  - tools/allocator/hako_exe_memory_runner.sh
  - tools/allocator/mimalloc_comparison_memory_report.py
  - tools/checks/k2_wide_phase295x_realloc_aligned_evidence_run_guard.sh
---

# 295x-18 Mimalloc Comparison Realloc/Aligned Evidence Run

## Decision

Close:

```text
MIMALLOC-COMPARISON-REALLOC-ALIGNED-EVIDENCE-295X-RUN-001
```

The C mimalloc explicit runner and the `.hako` EXE memory runner now publish a
normalized same-workload report for:

```text
workload=representative-realloc-aligned-v0
operation_family=realloc-aligned
operation_sequence_id=representative-realloc-aligned-v0-seq
free_order_id=ascending-release-v0
```

The row requires structural parity for:

- workload / operation family / operation sequence / free order;
- allocation count and free count;
- requested bytes;
- realloc count;
- aligned allocation count;
- alignment request / ok / reject counts.

The row intentionally keeps moved/copy/RSS as side-by-side evidence only:

- realloc same-pointer count;
- realloc moved count;
- copied bytes;
- peak RSS bytes.

Those fields are observable differences between the model-side `.hako`
allocator slice and the C mimalloc implementation. They are not winner or
behavior-parity claims.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-REALLOC-ALIGNED-CLOSEOUT-295X-001
```

Reason: the realloc/aligned evidence path is now executable on both sides and
normalized. The next row should close the workload family and select whether to
move to mixed-size workload evidence or to define repeated measurement policy
before any winner claim.

## Stop Line

This row does not:

- require moved/copy/RSS parity;
- add benchmark warmup or final summary statistics;
- make performance or memory winner claims;
- replace the process allocator or install hooks;
- enable provider packages, DLL generation, provider activation, provider API
  execution, backend matchers, or `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_realloc_aligned_evidence_run_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
