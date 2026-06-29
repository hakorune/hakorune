# 1832 - MIRBUILDER-LOOP-COND-BREAK-CONTINUE-SURFACE-CLASSIFICATION-001

## Token

```text
MIRBUILDER-LOOP-COND-BREAK-CONTINUE-SURFACE-CLASSIFICATION-001
```

## Purpose

Classify `LoopCondBreakContinueCluster` into smaller break/continue buckets.

This remains diagnostic-only. It does not choose a family, emit Hako, create a
projection policy, or claim Source Selfhost.

## Output

The unconverted surface report fixture now includes:

```text
loop_cond_bc_subcluster_rules
loop_cond_bc_subcluster_summary
items[].loop_cond_bc_subcluster
```

## Result

```text
LoopCondBreakContinueCluster = 14

subclusters:
  LoopCondBcElsePatternCluster = 7
  LoopCondBcCleanupCluster = 2
  LoopCondBcItemLoweringCluster = 2
  LoopCondBcPipelineCluster = 2
  LoopCondBcNestedCarrierCluster = 1

decision = KeepStopped
reason_token = AmbiguousUnconvertedSurfaceCandidates
```

## Acceptance

```text
loop_cond_break_continue_items_subclustered = 1
loop_cond_bc_subcluster_summary_count_matches_LoopCondBreakContinueCluster = 1
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
1. MIRBUILDER-LOOP-COND-BC-ELSE-PATTERN-SURFACE-CLASSIFICATION-001
   Split else-pattern lowering into break-only, guard-break, and return-only
   buckets.

2. MIRBUILDER-LOOP-COND-BC-PIPELINE-SURFACE-CLASSIFICATION-001
   Classify the pipeline/root lowering surfaces and carrier sync helper.

3. MIRBUILDER-LOOP-COND-BC-CLEANUP-SURFACE-CLASSIFICATION-001
   Classify cleanup and exit-path predicate surfaces.
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
