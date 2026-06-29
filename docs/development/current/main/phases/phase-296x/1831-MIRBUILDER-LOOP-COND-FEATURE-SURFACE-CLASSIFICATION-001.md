# 1831 - MIRBUILDER-LOOP-COND-FEATURE-SURFACE-CLASSIFICATION-001

## Token

```text
MIRBUILDER-LOOP-COND-FEATURE-SURFACE-CLASSIFICATION-001
```

## Purpose

Classify `LoopCondFeatureCluster` into smaller loop-condition feature buckets.

This is a diagnostic classification card. It does not choose a family, emit
Hako, create a projection policy, or claim Source Selfhost.

## Output

The unconverted surface report fixture now includes:

```text
loop_cond_feature_subcluster_rules
loop_cond_feature_subcluster_summary
items[].loop_cond_feature_subcluster
```

## Result

```text
LoopCondFeatureCluster = 48

subclusters:
  LoopCondBreakContinueCluster = 14
  LoopCondContinueOnlyCluster = 12
  LoopCondReturnInBodyCluster = 5
  LoopCondUtilityCluster = 5
  LoopCondVerifierCluster = 5
  LoopTrueBreakContinueCluster = 4
  LoopCondContinueWithReturnCluster = 3

decision = KeepStopped
reason_token = AmbiguousUnconvertedSurfaceCandidates
```

## Acceptance

```text
loop_cond_feature_items_subclustered = 1
loop_cond_feature_subcluster_summary_count_matches_LoopCondFeatureCluster = 1
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
1. MIRBUILDER-LOOP-COND-BREAK-CONTINUE-SURFACE-CLASSIFICATION-001
   Split LoopCondBreakContinueCluster into pipeline, else-pattern, item,
   nested-carrier, and cleanup/verifier buckets.

2. MIRBUILDER-LOOP-COND-CONTINUE-ONLY-SURFACE-CLASSIFICATION-001
   Split LoopCondContinueOnlyCluster into pipeline, statement/block lowering,
   continuation-if handling, and verifier buckets.

3. MIRBUILDER-PHI-MATERIALIZER-FEATURE-SURFACE-CLASSIFICATION-001
   Parallel follow-up for PhiMaterializerFeatureCluster, which remains a
   same-sized next bucket.
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
