# Phase-293x Mimalloc Blueprint Lane Close Criteria (SSOT)

Status: Active
Decision: accepted
Updated: 2026-05-22
Owner: phase-293x mimalloc blueprint lane

## Purpose

Define the terminal boundary for `phase-293x` so row expansion does not continue
without an explicit close condition.

This SSOT defines completion for the **blueprint lane**, not full mimalloc port
completion.

## Scope Split

`phase-293x` closes on A/B only:

```text
A. explicit C mimalloc external evidence contract
B. hako_alloc vs C mimalloc comparison-ready schema contract
```

Out of scope for `phase-293x` close:

```text
C. mimalloc-inspired allocator design import decisions
D. provider/DLL/process allocator replacement/hook/global allocator activation
```

## Lane Completion Criteria

`phase-293x` may close when all conditions are true:

1. Explicit C mimalloc runner contract is fixed as external evidence source.
2. hako_alloc representative report contract is fixed.
3. Shared `workload_id` and shared metric schema are fixed.
4. Memory-use field semantics are fixed (`peak_rss_bytes`, `steady_rss_bytes`,
   `requested_bytes`, `allocation_count`, `free_count`).
5. Reason vocabulary includes accepted/blocked/missing/stop-line cases.
6. Execution seams remain closed in this phase (or are explicitly moved to the
   next phase).
7. Provider/DLL/hook/`#[global_allocator]` stay future-track only.
8. Taskboard carryover boundary and next-phase handoff are fixed.

## Stop-Line Contract (still closed in this lane)

- no repeated/heavy benchmark rerun
- no process allocator replacement
- no hook installation
- no backend matcher additions
- no `#[global_allocator]`
- no provider package / DLL generation
- no explicit C runner execution
- no worker/thread execution

## Terminal Row Sequence

Canonical closure sequence:

| Row | Role | Output |
| --- | --- | --- |
| `MIMAP-566A` | terminal planning pilot | explicit-runner planning pilot contract (execution closed) |
| `MIMAP-567A` | close criteria row | phase-293x close criteria card synced with this SSOT |
| `MIMAP-568A` | inventory/carryover row | keep/archive/carryover boundary for rows/proof apps/.hako fixtures |
| `MIMAP-569A` | phase closeout row | phase-293x closeout and next-phase selection |

## Next-Phase Boundary

Execution belongs to the next phase (example naming):

```text
phase-294x explicit C mimalloc evidence execution lane
```

`phase-293x` does not open execution seams; it hands off a stable contract pack.
