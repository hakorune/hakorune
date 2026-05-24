---
Status: Landed
Date: 2026-05-25
Scope: implement C/.hako evidence for representative-reuse-cycle-small-v0.
Related:
  - docs/development/current/main/phases/phase-295x/295x-79-MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-CONTRACT.md
  - tools/allocator/c_mimalloc_explicit_runner.c
  - apps/hako-alloc-mimalloc-comparison-reuse-cycle-small-exe-proof/main.hako
---

# 295x-80 Reuse-Cycle Small Workload Implementation

## Blocker

```text
MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-IMPLEMENTATION-295X-001
```

## Implementation

The comparison tooling now supports:

```text
workload=representative-reuse-cycle-small-v0
operation_family=reuse-cycle-small
operation_sequence_id=representative-reuse-cycle-small-v0-seq
free_order_id=even-odd-release-then-reacquire-v0
```

The C runner and `.hako` exact-EXE app both expose:

```text
allocation_count=128
free_count=128
requested_bytes=66508
reuse_cycle_count=1
winner_claim=0
```

The `.hako` app uses the existing `HakoAllocPageModel` page-local `reuse()`
seam; it does not open remote-free, TLS, atomics, provider activation, or host
allocator replacement.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-CLOSEOUT-295X-001
```

The closeout should rerun this implementation guard and then decide whether to
add the reuse-cycle workload to the repeated measurement pack or move to the
next allocator behavior seam.

## Stop Line

This row does not compute speed winners, compute RSS winners, require pointer
identity parity, reopen `.hako` body timing, change runtime behavior, broaden
`usize` migration, or open provider/DLL/replacement/hook/global allocator
seams, worker/TLS, atomics, remote-free stress, or abandoned heap stress.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_reuse_cycle_small_workload_implementation_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
