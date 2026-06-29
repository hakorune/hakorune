# 1841 - MIRBUILDER-LOOP-COND-BC-ITEM-LOWERING-SURFACE-CLASSIFICATION-001

## Token

```text
MIRBUILDER-LOOP-COND-BC-ITEM-LOWERING-SURFACE-CLASSIFICATION-001
```

## Purpose

Classify the `LoopCondBcItemLoweringCluster` into item dispatcher and statement
lowering buckets.

This remains diagnostic-only. It does not choose a family, emit Hako, create a
projection policy, or claim Source Selfhost.

## Output

The unconverted surface report fixture now includes:

```text
loop_cond_bc_item_lowering_subcluster_rules
loop_cond_bc_item_lowering_subcluster_summary
items[].loop_cond_bc_item_lowering_subcluster
```

## Result

```text
LoopCondBcItemLoweringCluster = 2

subclusters:
  LoopCondBcItemDispatcherCluster = 1
  LoopCondBcStatementLoweringCluster = 1

decision = KeepStopped
reason_token = AmbiguousUnconvertedSurfaceCandidates
```

## Acceptance

```text
loop_cond_bc_item_lowering_items_subclustered = 1
loop_cond_bc_item_lowering_summary_count_matches_LoopCondBcItemLoweringCluster = 1
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
1. MIRBUILDER-LOOP-COND-BC-ITEM-DISPATCHER-PROJECTION-POLICY-001
   Define whether lower_loop_cond_item is a projection surface or parent
   item-dispatch helper.

2. MIRBUILDER-LOOP-COND-BC-STATEMENT-LOWERING-PROJECTION-POLICY-001
   Define whether lower_loop_cond_stmt is a projection surface or parent
   statement-lowering helper.
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
