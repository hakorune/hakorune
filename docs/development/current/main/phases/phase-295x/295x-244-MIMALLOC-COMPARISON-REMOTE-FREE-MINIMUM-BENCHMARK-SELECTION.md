---
Status: Current
Date: 2026-05-26
Scope: select the smallest remote-free benchmark pack through an implementation-first `.hako` proof surface.
Blocker: MIMALLOC-COMPARISON-REMOTE-FREE-MINIMUM-BENCHMARK-SELECTION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-243-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-EVIDENCE.md
  - apps/mimalloc-remote-free-minimum-benchmark-selection-proof/main.hako
  - apps/mimalloc-remote-free-minimum-benchmark-selection-proof/test.sh
---

# 295x-244 Remote-Free Minimum Benchmark Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-MINIMUM-BENCHMARK-SELECTION-295X-002
```

Fold the remote-free semantic closeout into an implementation-first selection
row. Do not open a presentation-only mimalloc row that touches no `.hako`
implementation.

The selected minimum pack is:

```text
benchmark_pack=remote-free-minimum-v0
backend_scope=exact-exe-first
workloads:
  local-alloc-free-cycle-v0
  remote-free-publish-only-v0
  remote-free-collect-only-v0
  remote-free-publish-collect-cycle-v0
policy:
  warmup_count=1
  sample_count=5
  summary=min,median,max
stop_line:
  provider_active=0
  replacement_active=0
  winner_claim=0
```

## Implementation Contract

The `.hako` proof app must prove that each selected workload shape is executable
without turning the row into a timing run:

```text
local_alloc_free=1
publish_only=1
collect_only=1
publish_collect_cycle=1
summary=ok
```

The publish-collect cycle remains anchored to the existing
`HakoAllocRemoteFreePageExerciseReport.captureFacadeExercise(...)` semantic
contract instead of widening provider/replacement seams.

## Guard Reason

Primary proof guard reason:

```text
This catches a changed .hako contract that the existing remote-free evidence guard cannot observe.
```

## Stop Line

This row does not add elapsed-time fields, repeated medians, VM/LLVM/AOT winner
claims, DLL/shared-library packaging, provider activation, replacement, hooks,
or global allocator seams.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-MINIMUM-BENCHMARK-RUN-295X-002
```

The next row should run the selected minimum pack on the exact-EXE/AOT-first
path without widening backend split or native C comparison yet.
