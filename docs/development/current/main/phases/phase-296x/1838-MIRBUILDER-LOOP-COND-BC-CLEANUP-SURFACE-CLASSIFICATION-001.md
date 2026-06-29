# 1838 - MIRBUILDER-LOOP-COND-BC-CLEANUP-SURFACE-CLASSIFICATION-001

## Token

```text
MIRBUILDER-LOOP-COND-BC-CLEANUP-SURFACE-CLASSIFICATION-001
```

## Purpose

Classify the `LoopCondBcCleanupCluster` into cleanup application and exit
predicate buckets.

This remains diagnostic-only. It does not choose a family, emit Hako, create a
projection policy, or claim Source Selfhost.

## Output

The unconverted surface report fixture now includes:

```text
loop_cond_bc_cleanup_subcluster_rules
loop_cond_bc_cleanup_subcluster_summary
items[].loop_cond_bc_cleanup_subcluster
```

## Result

```text
LoopCondBcCleanupCluster = 2

subclusters:
  LoopCondBcCleanupApplicationCluster = 1
  LoopCondBcCleanupExitPredicateCluster = 1

decision = KeepStopped
reason_token = AmbiguousUnconvertedSurfaceCandidates
```

## Acceptance

```text
loop_cond_bc_cleanup_items_subclustered = 1
loop_cond_bc_cleanup_summary_count_matches_LoopCondBcCleanupCluster = 1
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
1. MIRBUILDER-LOOP-COND-BC-CLEANUP-APPLICATION-PROJECTION-POLICY-001
   Define whether apply_loop_cond_break_continue_cleanup is a projection
   surface or route-local cleanup helper.

2. MIRBUILDER-LOOP-COND-BC-CLEANUP-EXIT-PREDICATE-PROJECTION-POLICY-001
   Define whether body_exits_all_paths is a projection surface or private
   analysis predicate.
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
