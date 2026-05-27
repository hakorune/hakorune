---
Status: Current
Date: 2026-05-27
Scope: define an in-process operation-repeat measurement contract for hako/C mimalloc parity.
Blocker: HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-CONTRACT-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-53-HAKO-MIMALLOC-PERF-RUNTIME-VS-WORKLOAD-REPEAT-SPLIT-DIAGNOSTIC.md
---

# 296x-54 Hako Mimalloc In-Process Operation Repeat Contract

## Purpose

The current repeated measurement runner uses `operation_repeat` as process
invocation repeat. Row 53 showed that this makes empty workload growth explain
the small-block growth, so allocator hot-path comparison needs a separate
contract that repeats operations inside one process.

## Required Contract

Define a new measurement profile without replacing the existing process-repeat
profile:

```text
measurement_profile=hako-mimalloc-in-process-operation-repeat-v0
timing_repeat_kind=in-process-operation-loop-v0
workload_id=representative-small-block-v0
operation_repeat=<inner workload repeat>
process_repeat=<sample process count>
sample_count=3
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
```

This row is contract-first. It may add a fixture runner or adapter only if the
contract and fail-fast fields are documented in the same row.

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.
