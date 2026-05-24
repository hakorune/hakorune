---
Status: Landed
Date: 2026-05-24
Scope: select the next huge-ish comparison workload seam.
Blocker: MIMALLOC-COMPARISON-HUGE-ISH-WORKLOAD-SEAM-SELECTION-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-23-MIMALLOC-COMPARISON-MIXED-SIZE-CLOSEOUT.md
  - docs/development/current/main/phases/phase-294x/294x-58-MIMALLOC-COMPARISON-HUGE-OSVM-SLICE-PILOT.md
  - apps/hako-alloc-mimalloc-comparison-huge-osvm-slice-proof/main.hako
  - tools/checks/k2_wide_phase295x_huge_ish_workload_seam_selection_guard.sh
---

# 295x-24 Mimalloc Comparison Huge-Ish Workload Seam Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-HUGE-ISH-WORKLOAD-SEAM-SELECTION-295X-001
```

Select the next comparison workload family:

```text
workload=representative-huge-ish-v0
operation_family=huge-ish
operation_sequence_id=representative-huge-ish-v0-seq
free_order_id=ascending-release-v0
```

The selected V0 sequence is:

```text
alloc sizes:
  4194305, 16

free order:
  release in allocation order
```

Expected structural fields for the contract refresh are:

```text
allocation_count=2
free_count=2
requested_bytes=4194321
large_request_count=1
```

## Boundary

This is a huge-ish comparison workload, not an OSVM/page-source equivalence
claim. C mimalloc will observe a large request through its normal allocator API.
The `.hako` side may reuse the existing huge/OSVM slice vocabulary, but
OSVM/page-source counts remain model-side evidence unless a later row explicitly
opens an equivalence contract.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-HUGE-ISH-CONTRACT-295X-REFRESH-001
```

The next row should add C runner dispatch and `.hako` evidence output for the
selected workload while keeping the base output contracts unchanged.

## Stop Line

This row does not:

- implement the huge-ish runner path;
- run huge-ish C or `.hako` evidence;
- claim OSVM/page-source parity;
- add benchmark warmup or final summary statistics;
- make performance or memory winner claims;
- replace the process allocator or install hooks;
- enable provider packages, DLL generation, provider activation, provider API
  execution, backend matchers, or `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_huge_ish_workload_seam_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
