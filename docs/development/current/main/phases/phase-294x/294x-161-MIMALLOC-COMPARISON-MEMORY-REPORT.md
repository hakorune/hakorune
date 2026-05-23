---
Status: Landed
Date: 2026-05-24
Scope: normalize hako EXE and C mimalloc memory evidence into one comparison report.
Blocker: MIMALLOC-COMPARISON-MEMORY-REPORT-001
Related:
  - tools/allocator/hako_exe_memory_runner.sh
  - tools/allocator/c_mimalloc_explicit_runner.sh
  - tools/allocator/mimalloc_comparison_memory_report.py
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_memory_report_guard.sh
---

# 294x-161 Mimalloc Comparison Memory Report

## Decision

Add a thin normalization step that consumes the existing hako EXE memory
evidence and the existing explicit C mimalloc runner evidence, then emits one
stable comparison report:

```text
mimalloc-comparison-memory-report-v0
```

This row connects evidence contracts only. It does not introduce a new
allocator behavior, rerun benchmark packs, or claim a winner. The hako and C
workloads are still reported separately and `workload_match=0` is explicit
until a later row selects a same-workload comparison pack.

## Stable Output

The report includes:

```text
hako_workload
c_workload
workload_match
hako_requested_bytes
c_requested_bytes
hako_peak_rss_bytes
c_peak_rss_bytes
requested_bytes_delta
peak_rss_bytes_delta
memory evidence flags
closed stop-line fields
winner_claim=0
summary
```

## Stop Line

This row does not open:

- provider activation;
- host allocator replacement;
- hook installation;
- `#[global_allocator]`;
- worker/TLS behavior;
- remote-free stress;
- atomic bitmap execution;
- same-workload winner claims.

## Follow-On

```text
MIMALLOC-COMPARISON-MEMORY-REPORT-CLOSEOUT-001:
  close out the normalized memory report, then choose either a same-workload
  comparison pack or a return to the next explicit `usize` field-group row.
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_memory_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
