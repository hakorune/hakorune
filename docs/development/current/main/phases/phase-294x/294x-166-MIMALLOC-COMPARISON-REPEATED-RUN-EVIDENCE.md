---
Status: Landed
Date: 2026-05-24
Scope: add narrow repeated-run RSS evidence over the same-workload pair.
Blocker: MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-165-MIMALLOC-COMPARISON-RSS-PRESENTATION-CLOSEOUT.md
  - tools/allocator/mimalloc_comparison_repeated_run_evidence.py
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_repeated_run_evidence_guard.sh
---

# 294x-166 Mimalloc Comparison Repeated-Run Evidence

## Decision

Add a narrow repeated-run evidence contract over the same-workload hako-vs-C
mimalloc RSS presentation samples.

The row aggregates multiple `mimalloc-comparison-rss-presentation-v0` samples
into:

```text
mimalloc-comparison-repeated-run-evidence-v0
measurement_scope=repeated-rss-samples
sample_count=N
```

The aggregate reports only sample count and min/max RSS byte ranges for hako,
C mimalloc, and their deltas. It does not decide a winner.

## Stop Line

This row does not open:

- performance or memory-use winner claims;
- mean/median/statistical significance claims;
- provider activation;
- host allocator replacement;
- hook installation;
- `#[global_allocator]`;
- worker/TLS behavior;
- remote-free stress;
- atomic bitmap execution;
- provider package / DLL generation.

## Follow-On

```text
MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-CLOSEOUT-001:
  close out repeated-run evidence, then choose either a small comparison
  summary row or return to the next explicit `usize` field-group row.
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_repeated_run_evidence_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
