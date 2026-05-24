---
Status: Landed
Date: 2026-05-24
Scope: add runner/evidence contract support for representative-huge-ish-v0.
Blocker: MIMALLOC-COMPARISON-HUGE-ISH-CONTRACT-295X-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-24-MIMALLOC-COMPARISON-HUGE-ISH-WORKLOAD-SEAM-SELECTION.md
  - tools/allocator/c_mimalloc_explicit_runner.c
  - tools/allocator/hako_exe_memory_runner.sh
  - tools/allocator/mimalloc_comparison_memory_report.py
  - apps/hako-alloc-mimalloc-comparison-huge-ish-exe-proof/main.hako
  - tools/checks/k2_wide_phase295x_huge_ish_contract_refresh_guard.sh
---

# 295x-25 Mimalloc Comparison Huge-Ish Contract Refresh

## Decision

Close:

```text
MIMALLOC-COMPARISON-HUGE-ISH-CONTRACT-295X-REFRESH-001
```

The explicit C mimalloc runner and a narrow `.hako` exact-EXE evidence app now
publish the selected huge-ish workload contract:

```text
workload=representative-huge-ish-v0
operation_family=huge-ish
operation_sequence_id=representative-huge-ish-v0-seq
free_order_id=ascending-release-v0
allocation_count=2
free_count=2
requested_bytes=4194321
large_request_count=1
```

The base output contracts remain unchanged. `large_request_count` is a
comparison evidence field, not an OSVM/page-source parity claim.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-HUGE-ISH-EVIDENCE-295X-RUN-001
```

The next row should run the C and `.hako` huge-ish evidence through the
normalizer and require structural count/requested/large-request parity while
keeping RSS and OSVM/page-source details as evidence-only.

## Stop Line

This row does not run the normalized huge-ish comparison report, claim
OSVM/page-source parity, add benchmark summary policy, make winner claims,
enable provider/DLL/replacement seams, or open worker/TLS, atomics,
remote-free, abandoned heap, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_huge_ish_contract_refresh_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
