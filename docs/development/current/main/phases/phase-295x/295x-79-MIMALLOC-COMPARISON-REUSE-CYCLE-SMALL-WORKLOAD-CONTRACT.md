---
Status: Landed
Date: 2026-05-25
Scope: define the reuse-cycle small-block same-workload contract before implementation.
Related:
  - docs/development/current/main/phases/phase-295x/295x-78-MIMALLOC-COMPARISON-PORT-RESUME-SEAM-SELECTION.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
---

# 295x-79 Reuse-Cycle Small Workload Contract

## Blocker

```text
MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-CONTRACT-295X-001
```

## Workload Identity

```text
workload=representative-reuse-cycle-small-v0
operation_family=reuse-cycle-small
operation_sequence_id=representative-reuse-cycle-small-v0-seq
free_order_id=even-odd-release-then-reacquire-v0
```

## Operation Contract

The workload is a page-local reuse extension of `representative-small-block-v0`:

```text
1. allocate 64 small blocks with the existing 512+(i%17) request pattern
2. release all first-wave blocks in even-index then odd-index order
3. reacquire 64 small blocks with the same request pattern
4. release all second-wave blocks in even-index then odd-index order
```

Expected stable count evidence:

```text
allocation_count=128
free_count=128
requested_bytes=66508
realloc_count=0
aligned_alloc_count=0
large_request_count=0
reuse_cycle_count=1
winner_claim=0
```

`reuse_cycle_count` is evidence-only. It means the workload contains a second
allocation wave after a full first-wave release. It is not a performance claim
and does not require pointer identity parity.

## Implementation Order

Select:

```text
MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-IMPLEMENTATION-295X-001
```

The implementation row should update:

```text
tools/allocator/c_mimalloc_explicit_runner.c
tools/allocator/c_mimalloc_explicit_runner.sh
tools/allocator/hako_exe_memory_runner.sh
tools/allocator/mimalloc_repeated_measurement_runner.py
apps/hako-alloc-mimalloc-comparison-reuse-cycle-small-exe-proof/main.hako
```

## Stop Line

This row does not implement the workload, compute speed winners, compute RSS
winners, require pointer identity parity, reopen `.hako` body timing, change
runtime behavior, broaden `usize` migration, or open provider/DLL/replacement/
hook/global allocator seams, worker/TLS, atomics, remote-free stress, or
abandoned heap stress.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_reuse_cycle_small_workload_contract_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
