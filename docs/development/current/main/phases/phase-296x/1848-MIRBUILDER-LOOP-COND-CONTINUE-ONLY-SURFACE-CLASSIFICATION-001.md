# 1848 - MIRBUILDER-LOOP-COND-CONTINUE-ONLY-SURFACE-CLASSIFICATION-001

## Token

```text
MIRBUILDER-LOOP-COND-CONTINUE-ONLY-SURFACE-CLASSIFICATION-001
```

## Purpose

Classify the `LoopCondContinueOnlyCluster` into smaller continue-only
projection policy buckets.

This remains diagnostic-only. It does not choose a family, emit Hako, create a
projection policy, or claim Source Selfhost.

## Output

The unconverted surface report fixture now includes:

```text
loop_cond_co_subcluster_rules
loop_cond_co_subcluster_summary
items[].loop_cond_co_subcluster
```

## Result

```text
LoopCondContinueOnlyCluster = 12

subclusters:
  LoopCondCoContinueIfCluster = 3
  LoopCondCoGroupIfCluster = 2
  LoopCondCoHelperCluster = 2
  LoopCondCoStatementLoweringCluster = 2
  LoopCondCoBlockLoweringCluster = 1
  LoopCondCoCleanupCluster = 1
  LoopCondCoRootPipelineCluster = 1

decision = KeepStopped
reason_token = AmbiguousUnconvertedSurfaceCandidates
```

## Acceptance

```text
loop_cond_co_items_subclustered = 1
loop_cond_co_subcluster_summary_count_matches_LoopCondContinueOnlyCluster = 1
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
1. MIRBUILDER-LOOP-COND-CO-ROOT-PIPELINE-PROJECTION-POLICY-001
   Define whether lower_loop_cond_continue_only is a native seed candidate or
   remains an integration/root pipeline owner.

2. MIRBUILDER-LOOP-COND-CO-BLOCK-LOWERING-PROJECTION-POLICY-001
   Define whether lower_continue_only_block is a projection surface or private
   block lowering helper.
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
