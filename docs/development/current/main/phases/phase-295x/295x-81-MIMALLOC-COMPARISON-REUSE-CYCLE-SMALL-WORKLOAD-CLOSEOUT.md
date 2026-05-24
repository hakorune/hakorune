---
Status: Landed
Date: 2026-05-25
Scope: close reuse-cycle small-block workload evidence and select external benchmark corpus bridge.
Related:
  - docs/development/current/main/phases/phase-295x/295x-80-MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-IMPLEMENTATION.md
  - docs/development/current/main/phases/phase-295x/295x-82-MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-BENCH-BRIDGE.md
---

# 295x-81 Reuse-Cycle Small Workload Closeout

## Blocker

```text
MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-CLOSEOUT-295X-001
```

## Closeout

The reuse-cycle small-block workload is closed as a same-workload evidence
slice:

```text
workload=representative-reuse-cycle-small-v0
operation_family=reuse-cycle-small
operation_sequence_id=representative-reuse-cycle-small-v0-seq
free_order_id=even-odd-release-then-reacquire-v0
reuse_cycle_count_delta=0
winner_claim=0
```

The row remains a comparison contract/evidence closeout. It does not claim
speed or RSS winners.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-BENCH-BRIDGE-295X-001
```

The next row adds a local bridge for the extracted `hakmem_20260525` benchmark
corpus. The bridge is for external benchmark alignment and workload discovery;
it is not imported as phase-295x repeated measurement evidence yet.

## Stop Line

This row does not compute speed winners, compute RSS winners, require pointer
identity parity, reopen `.hako` body timing, change runtime behavior, broaden
`usize` migration, or open provider/DLL/replacement/hook/global allocator
seams, worker/TLS, atomics, remote-free stress, or abandoned heap stress.

