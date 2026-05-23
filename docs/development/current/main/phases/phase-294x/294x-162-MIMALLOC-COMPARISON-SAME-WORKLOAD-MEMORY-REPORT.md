---
Status: Landed
Date: 2026-05-24
Scope: add same-workload hako memory evidence for the explicit C mimalloc small-block runner shape.
Blocker: MIMALLOC-COMPARISON-SAME-WORKLOAD-PACK-001
Related:
  - apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/main.hako
  - tools/allocator/mimalloc_comparison_memory_report.py
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_same_workload_memory_report_guard.sh
---

# 294x-162 Mimalloc Comparison Same-Workload Memory Report

## Decision

Add a hako-side proof app for the explicit C mimalloc runner workload:

```text
representative-small-block-v0
```

The app uses `HakoAllocPageModel` to execute 64 small-block requests with the
same requested-byte sequence as the C runner (`512 + i % 17`) and reports the
same total requested bytes:

```text
requested_bytes=33254
```

The existing hako EXE memory runner and C mimalloc explicit runner can now feed
the normalizer with matching workload ids, producing
`workload_match=1` and `requested_bytes_delta=0`.

## Stop Line

This row does not open:

- provider activation;
- host allocator replacement;
- hook installation;
- `#[global_allocator]`;
- worker/TLS behavior;
- remote-free stress;
- atomic bitmap execution;
- winner claims.

## Follow-On

```text
MIMALLOC-COMPARISON-SAME-WORKLOAD-PACK-CLOSEOUT-001:
  close out the same-workload memory report and choose whether to continue with
  repeated runs, RSS presentation rows, or return to explicit `usize`
  field-group migration.
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_same_workload_memory_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
