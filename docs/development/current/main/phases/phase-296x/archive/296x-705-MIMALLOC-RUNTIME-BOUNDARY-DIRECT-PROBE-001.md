---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-RUNTIME-BOUNDARY-DIRECT-PROBE-001
Scope: Design the next direct runtime boundary probe after body timer scaling.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-704-MIMALLOC-BODY-TIMER-ALIGNMENT-PROBE-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-RUNTIME-BOUNDARY-DIRECT-PROBE-001

## Purpose

296x-704 should only select this row if scaled body timing shows Hako timer
resolution is no longer the dominant uncertainty but the body gap remains. This
row is a probe-design row, not an implementation row.

## Candidate Probe Families

```text
generated runtime helper boundary:
  measure env.now / body timer and exact-AOT helper entry overhead separately

host handle / object boundary:
  measure HostHandle lookup / Arc carrier / ObjectHandle seam without changing
  object representation

BoxCallable / RoutePlan boundary:
  measure whether the active front hits dynamic route lookup or already has a
  closed-world route shape
```

## Stop Line

```text
do not implement closed-world direct lowering
do not change runtime object representation
do not change product NyRT default
do not patch tracked source .hako
do not add benchmark/helper-name special cases
```

## Acceptance

```text
source_evidence=296x-704
implementation_started=0
output_contract=hako-mimalloc-runtime-boundary-direct-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
perf_runs=20
in_process_operation_repeat=65536
body_elapsed_ns=58000000
top_symbol=nyash_array_length_h
top_symbol_percent=70.61
selected_owner=generated_runtime_array_length_boundary
selected_owner_confidence=medium
compiler_lowering_implementation_started=0
runtime_object_changed=0
product_default_changed=0
source_hako_changed=0
winner_claim=0
next_task=direct_array_length_boundary_design
summary=ok
```

## Result

The direct runtime boundary probe found a concrete owner candidate:
`nyash_array_length_h` dominates the scaled direct-exact run. This is not a
Box/method dispatch owner and not an Arc-retirement owner. It is a generated
runtime helper boundary around Array length access.

```text
output_contract=hako-mimalloc-runtime-boundary-direct-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-704
perf_runs=20
in_process_operation_repeat=65536
body_elapsed_ns=58000000
top_symbol=nyash_array_length_h
top_symbol_percent=70.61
selected_owner=generated_runtime_array_length_boundary
selected_owner_confidence=medium
compiler_lowering_implementation_started=0
runtime_object_changed=0
product_default_changed=0
source_hako_changed=0
winner_claim=0
next_task=direct_array_length_boundary_design
summary=ok
```

## Artifact

```text
artifact_dir=/tmp/hakorune_runtime_boundary_asm.madThG/asm.out.artifacts.d
perf_report=/tmp/hakorune_runtime_boundary_asm.madThG/asm.out.artifacts.d/perf-report.txt
perf_annotate=/tmp/hakorune_runtime_boundary_asm.madThG/asm.out.artifacts.d/perf-annotate.txt
```
