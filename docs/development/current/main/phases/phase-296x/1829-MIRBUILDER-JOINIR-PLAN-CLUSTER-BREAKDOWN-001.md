# 1829 - MIRBUILDER-JOINIR-PLAN-CLUSTER-BREAKDOWN-001

## Token

```text
MIRBUILDER-JOINIR-PLAN-CLUSTER-BREAKDOWN-001
```

## Purpose

Break down the largest unconverted surface owner cluster,
`JoinIRPlanCluster`, without selecting a family or generating Hako.

This turns the largest raw blocker bucket from 628 source surfaces into
subclusters that can be handled by future policy / helper / verifier /
decomposition cards.

## Output

The unconverted surface report fixture now includes:

```text
joinir_plan_subcluster_rules
joinir_plan_subcluster_summary
items[].joinir_plan_subcluster
```

## Result

```text
JoinIRPlanCluster = 628

subclusters:
  PlanFeatureMaterializerCluster = 135
  GenericLoopPlanCluster = 89
  RecipeTreeMatcherCluster = 74
  PlanPartsAssemblyCluster = 72
  LoopBreakPlanCluster = 57
  LoopCondPlanCluster = 42
  PlanFactsCluster = 42
  PlanNormalizerCluster = 39
  PlanLowererCluster = 30
  OtherJoinIRPlanCluster = 19
  NestedLoopPlanCluster = 14
  PlannerPolicyCluster = 9
  PlanComposerCluster = 6

decision = KeepStopped
reason_token = AmbiguousUnconvertedSurfaceCandidates
```

## Acceptance

```text
joinir_plan_items_subclustered = 1
joinir_plan_subcluster_summary_count_matches_JoinIRPlanCluster = 1
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
1. MIRBUILDER-JOINIR-PLAN-FEATURE-MATERIALIZER-SURFACE-CLASSIFICATION-001
   Split PlanFeatureMaterializerCluster into PHI materializers, carrier
   helpers, EdgeCFG stubs, and nonsemantic utilities.

2. MIRBUILDER-JOINIR-GENERIC-LOOP-SURFACE-CLASSIFICATION-001
   Split GenericLoopPlanCluster into detector, classifier, body-check,
   and policy/helper buckets.

3. MIRBUILDER-JOINIR-RECIPE-TREE-MATCHER-SURFACE-CLASSIFICATION-001
   Split RecipeTreeMatcherCluster into verifier, matcher helper, and
   policy surfaces.

4. MIRBUILDER-JOINIR-PLAN-PARTS-ASSEMBLY-SURFACE-CLASSIFICATION-001
   Split PlanPartsAssemblyCluster into entry/exit/dispatch/conditional update
   surfaces.
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
