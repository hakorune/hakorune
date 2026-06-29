# 1830 - MIRBUILDER-PLAN-FEATURE-MATERIALIZER-SURFACE-CLASSIFICATION-001

## Token

```text
MIRBUILDER-PLAN-FEATURE-MATERIALIZER-SURFACE-CLASSIFICATION-001
```

## Purpose

Classify the largest JoinIR plan subcluster,
`PlanFeatureMaterializerCluster`, into smaller bounded buckets.

This is still a diagnostic card. It does not choose a family, emit Hako,
create a projection policy, or claim Source Selfhost.

## Output

The unconverted surface report fixture now includes:

```text
plan_feature_subcluster_rules
plan_feature_subcluster_summary
items[].plan_feature_subcluster
```

## Result

```text
PlanFeatureMaterializerCluster = 135

subclusters:
  LoopCondFeatureCluster = 48
  GenericLoopBodyFeatureCluster = 30
  PhiMaterializerFeatureCluster = 30
  CarrierFeatureCluster = 13
  EdgeCfgStubFeatureCluster = 7
  OtherPlanFeatureCluster = 4
  ExitIfFeatureCluster = 2
  BodyViewFeatureCluster = 1

decision = KeepStopped
reason_token = AmbiguousUnconvertedSurfaceCandidates
```

## Acceptance

```text
plan_feature_items_subclustered = 1
plan_feature_subcluster_summary_count_matches_PlanFeatureMaterializerCluster = 1
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
1. MIRBUILDER-LOOP-COND-FEATURE-SURFACE-CLASSIFICATION-001
   Split LoopCondFeatureCluster into break/continue, continue-only,
   return-in-body, and utility buckets.

2. MIRBUILDER-GENERIC-LOOP-BODY-FEATURE-SURFACE-CLASSIFICATION-001
   Split GenericLoopBodyFeatureCluster into lowering, branch, carrier, and
   terminality buckets.

3. MIRBUILDER-PHI-MATERIALIZER-FEATURE-SURFACE-CLASSIFICATION-001
   Split PhiMaterializerFeatureCluster into actual PHI lifecycle owners and
   nonsemantic accessors/helpers.
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
