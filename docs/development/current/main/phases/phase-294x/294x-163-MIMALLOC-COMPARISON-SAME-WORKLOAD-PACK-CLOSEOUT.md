---
Status: Landed
Date: 2026-05-24
Scope: close out the same-workload hako-vs-C mimalloc memory report pack.
Blocker: MIMALLOC-COMPARISON-SAME-WORKLOAD-PACK-CLOSEOUT-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-162-MIMALLOC-COMPARISON-SAME-WORKLOAD-MEMORY-REPORT.md
  - apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_same_workload_memory_report_guard.sh
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_same_workload_closeout_guard.sh
---

# 294x-163 Mimalloc Comparison Same-Workload Pack Closeout

## Decision

Close out the same-workload hako-vs-C mimalloc memory report pack.

The landed pack now has:

- a hako-side representative small-block proof app;
- explicit C mimalloc runner evidence for the same workload id;
- normalized `mimalloc-comparison-memory-report-v0` output;
- `workload_match=1`;
- `requested_bytes_delta=0`;
- positive single-run RSS evidence on both sides;
- closed provider / host replacement / hook / global allocator / winner seams.

This closeout does not make a performance or memory-use winner claim. The RSS
values are still single-run process evidence and need presentation wording
before they are user-facing comparison evidence.

## Follow-On Selection

Select:

```text
MIMALLOC-COMPARISON-RSS-PRESENTATION-001
```

The next row should format the existing single-run RSS evidence into a clearer
presentation contract. It may name the measurement as single-run, expose the
raw byte delta, and keep workload matching explicit. It must not add repeated
run aggregation or claim a winner.

## Stop Line

This closeout does not open:

- provider activation;
- host allocator replacement;
- hook installation;
- `#[global_allocator]`;
- worker/TLS behavior;
- remote-free stress;
- atomic bitmap execution;
- repeated-run statistics;
- performance or memory-use winner claims.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_same_workload_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
