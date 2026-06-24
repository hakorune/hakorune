Status: Done
Date: 2026-06-18
Scope: build-time crate split planning
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-CRATE-SPLIT-PLAN-001

## Purpose

Add build-time reduction to the compiler-construction plan without mixing it
with language, VM, or optimization behavior changes.

## Decision

Use the staged crate split from `build-crate-split-plan-ssot.md`.

```text
selected_first_real_split=hakorune-mir-plans
required_preface=mir_core_growth
deep_lowering_split_deferred=1
runtime_boxes_split_deferred=1
```

## Audit Input

```text
main_crate_lines=469k
main_crate_files=2370
total_build_time_sec=41.6
main_crate_compile_time_sec=33.8
main_crate_compile_time_percent=81
src_mir_lines=278k
src_mir_percent_of_main_crate=59
```

## Plan

```text
rank_1=hakorune-mir-plans
rank_2=mir_core_growth
rank_3=hakorune-backend
rank_4=hakorune-frontend
rank_5=box-core-config
rank_6=hakorune-lowering
rank_7=runtime-boxes
```

Implementation order:

```text
next_task=BUILD-MIR-CORE-GROWTH-PREFLIGHT-001
then=BUILD-MIR-PLANS-CRATE-PREFLIGHT-001
```

## Contract

```text
output_contract=build-crate-split-plan-v0

behavior_changed=0
crate_split_policy_defined=1
first_split_requires_dependency_audit=1
boxshape_only=1
boxcount_allowed=0

summary=ok
```

## Stop Lines

```text
do not move control_flow lowering in the first split
do not move runtime boxes in the first split
do not mix compiler acceptance work with crate split work
do not change behavior while moving crate boundaries
```

