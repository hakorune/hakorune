---
Status: Current
Date: 2026-05-26
Scope: run the selected remote-free minimum benchmark pack on the exact-EXE-first path.
Blocker: MIMALLOC-COMPARISON-REMOTE-FREE-MINIMUM-BENCHMARK-RUN-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-244-MIMALLOC-COMPARISON-REMOTE-FREE-MINIMUM-BENCHMARK-SELECTION.md
  - apps/mimalloc-remote-free-minimum-benchmark-run-proof/main.hako
  - apps/mimalloc-remote-free-minimum-benchmark-run-proof/remote_free_publish_only.hako
  - apps/mimalloc-remote-free-minimum-benchmark-run-proof/remote_free_collect_only.hako
  - apps/mimalloc-remote-free-minimum-benchmark-run-proof/remote_free_publish_collect_cycle.hako
  - apps/mimalloc-remote-free-minimum-benchmark-run-proof/test.sh
---

# 295x-245 Remote-Free Minimum Benchmark Run

## Decision

Close:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-MINIMUM-BENCHMARK-RUN-295X-002
```

Run the selected remote-free minimum benchmark pack on the exact-EXE-first path
without widening backend split or native C comparison seams.

The fixed run contract is:

```text
output_contract=mimalloc-comparison-remote-free-minimum-benchmark-run-v0
benchmark_pack=remote-free-minimum-v0
backend_scope=exact-exe-first
timing_repeat_kind=process-invocation-v0
operation_repeat=128
warmup_count=1
sample_count=5
stop_line:
  provider_active=0
  replacement_active=0
  winner_claim=0
```

## Implementation Contract

The `.hako` benchmark-run proof set must execute one fixed workload per process
invocation via dedicated no-arg entrypoints and publish:

```text
workload_id=<selected workload>
completed_ops=128
summary=ok
```

The primary guard measures process elapsed time externally over:

```text
local-alloc-free-cycle-v0
remote-free-publish-only-v0
remote-free-collect-only-v0
remote-free-publish-collect-cycle-v0
```

and reports:

```text
<workload>_ms=min,median,max
winner_claim=0
```

## Guard Reason

Primary proof guard reason:

```text
This catches a changed .hako contract that the benchmark selection proof cannot observe.
```

## Stop Line

This row does not open backend split, VM/LLVM comparison, native C/mimalloc
comparison, provider/DLL packaging, replacement, hooks, or global allocator
seams.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-BACKEND-SPLIT-SELECTION-295X-002
```

The next row should select the first backend-split comparison seam after the
exact-EXE-first minimum run lands.
