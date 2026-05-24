---
Status: Landed
Date: 2026-05-24
Scope: select the next mixed-size comparison workload seam.
Blocker: MIMALLOC-COMPARISON-MIXED-SIZE-WORKLOAD-SEAM-SELECTION-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-19-MIMALLOC-COMPARISON-REALLOC-ALIGNED-CLOSEOUT.md
  - docs/development/current/main/phases/phase-294x/294x-53-MIMALLOC-COMPARISON-VERTICAL-SLICE-WORKLOAD-PACK.md
  - tools/checks/k2_wide_phase295x_mixed_size_workload_seam_selection_guard.sh
---

# 295x-20 Mimalloc Comparison Mixed-Size Workload Seam Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-MIXED-SIZE-WORKLOAD-SEAM-SELECTION-295X-001
```

Select the next same-workload family:

```text
workload=representative-mixed-small-v0
operation_family=mixed-small
operation_sequence_id=representative-mixed-small-v0-seq
free_order_id=ascending-release-v0
```

The selected V0 sequence is:

```text
alloc sizes:
  16, 24, 32, 48, 64, 80, 96, 112,
  128, 160, 192, 224, 256, 384, 512, 768

free order:
  release in allocation order
```

Expected structural fields for the contract refresh are:

```text
allocation_count=16
free_count=16
requested_bytes=3096
```

## Why This Workload

Small-block and realloc/aligned families are closed. The next useful step is a
mixed small-size workload that exercises size distribution without opening huge
allocation, OSVM/page-source behavior, provider activation, process allocator
replacement, or benchmark winner claims.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-MIXED-SIZE-CONTRACT-295X-REFRESH-001
```

The next row should add C runner dispatch and `.hako` evidence output for the
selected workload, keeping the base output contracts unchanged.

## Stop Line

This row does not:

- implement the mixed-size runner path;
- run mixed-size C or `.hako` evidence;
- add benchmark warmup or final summary statistics;
- make performance or memory winner claims;
- replace the process allocator or install hooks;
- enable provider packages, DLL generation, provider activation, provider API
  execution, backend matchers, or `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, huge/OSVM execution, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mixed_size_workload_seam_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
