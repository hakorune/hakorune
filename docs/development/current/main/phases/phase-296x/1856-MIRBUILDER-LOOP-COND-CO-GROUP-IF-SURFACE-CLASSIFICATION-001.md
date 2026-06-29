# 1856 - MIRBUILDER-LOOP-COND-CO-GROUP-IF-SURFACE-CLASSIFICATION-001

## Token

```text
MIRBUILDER-LOOP-COND-CO-GROUP-IF-SURFACE-CLASSIFICATION-001
```

## Purpose

Classify the `LoopCondCoGroupIfCluster` into smaller group-if projection policy
buckets.

This remains diagnostic-only. It does not choose a family, emit Hako, create a
projection policy, or claim Source Selfhost.

## Output

The unconverted surface report fixture now includes:

```text
loop_cond_co_group_if_subcluster_rules
loop_cond_co_group_if_subcluster_summary
items[].loop_cond_co_group_if_subcluster
```

## Result

```text
LoopCondCoGroupIfCluster = 2

subclusters:
  LoopCondCoGroupIfBranchCluster = 1
  LoopCondCoGroupIfNestedLoopCluster = 1

decision = KeepStopped
reason_token = AmbiguousUnconvertedSurfaceCandidates
```

## Acceptance

```text
loop_cond_co_group_if_items_subclustered = 1
loop_cond_co_group_if_summary_count_matches_LoopCondCoGroupIfCluster = 1
manual_family_selection = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Recommended Next Tasks

```text
1. MIRBUILDER-LOOP-COND-CO-GROUP-IF-BRANCH-PROJECTION-POLICY-001
   Define whether lower_group_if is a projection surface or private branch
   lowering helper.

2. MIRBUILDER-LOOP-COND-CO-GROUP-IF-NESTED-LOOP-PROJECTION-POLICY-001
   Define whether lower_continue_if_nested_loop is a projection surface or
   private nested-loop lowering helper.
```

## Non-Claims

```text
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
no new projection policy
no route repair
```
