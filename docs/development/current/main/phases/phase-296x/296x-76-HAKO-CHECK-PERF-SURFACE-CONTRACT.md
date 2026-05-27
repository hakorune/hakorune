---
Status: Current
Date: 2026-05-27
Scope: define the observation-only hako_check perf-surface report contract.
Blocker: HAKO-CHECK-PERF-SURFACE-CONTRACT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-75-HAKO-MIMALLOC-PERF-PARITY-SELFHOST-HANDOFF-GATE.md
  - tools/hako_check/README.md
---

# 296x-76 hako_check Perf Surface Contract

## Purpose

Define the first `hako_check perf-surface` report contract before using it to
select another `.hako` mimalloc keeper optimization.

This row is observation-only. It must not rewrite `.hako` source, optimize MIR,
activate providers, replace the process allocator, install hooks, or claim a
benchmark winner.

## Required Output

```text
output_contract=hako-check-perf-surface-contract-v0
tool_surface=hako_check_perf_surface
observation_only=1
rewrite_executed=0
target_file
target_box
target_method
method_call_count
loop_method_call_count
array_access_count
linear_search_candidate=0|1
result_capsule_churn=0|1
observer_call_count
hot_path_risk=low|medium|high
suggested_next
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

The row only defines the report shape and the `hako_check` entry point contract.
The first inventory over `object_lifecycle_facade_box.hako` is row 77.
