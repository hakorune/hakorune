---
Status: Current
Date: 2026-05-27
Scope: run the first hako/C in-process operation-repeat pilot for small-block parity.
Blocker: HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-PILOT-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-54-HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-CONTRACT.md
---

# 296x-55 Hako Mimalloc In-Process Operation Repeat Pilot

## Purpose

Use the row 54 contract to run the first measurement where the allocator
workload repeats inside one process instead of repeating EXE startup.

## Required Input

```text
output_contract=hako-mimalloc-in-process-operation-repeat-contract-v0
measurement_profile=hako-mimalloc-in-process-operation-repeat-v0
timing_repeat_kind=in-process-operation-loop-v0
process_invocation_repeat=0
winner_claim=0
```

## Required Evidence

```text
output_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0
workload_id=representative-small-block-v0
operation_repeat=<inner workload repeat>
process_repeat=3
same_workload=1
same_operation_count=1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.
