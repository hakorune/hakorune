# 1828 - MIRBUILDER-UNCONVERTED-SURFACE-OWNER-CLUSTERING-001

## Token

```text
MIRBUILDER-UNCONVERTED-SURFACE-OWNER-CLUSTERING-001
```

## Purpose

Reduce the crate-wide unconverted Rust source surface report from raw
function/method count to owner-cluster buckets.

This card does not convert any surface. It makes the next policy /
decomposition / verifier repair task selectable without treating 1396 raw
methods as 1396 implementation tasks.

## Output

The existing unconverted surface report fixture now includes:

```text
owner_cluster_rules
missing_projection_cluster_summary
items[].likely_owner_cluster
```

## Result

```text
missing_projection_policy_count = 1396

cluster summary:
  JoinIRPlanCluster = 628
  JoinIRRouteVerifyCluster = 213
  OtherMissingProjectionPolicyCluster = 185
  ContextRegistryCluster = 114
  CallLoweringCluster = 88
  StatementValueConstructionCluster = 59
  EmissionSsaPhiCluster = 53
  JoinIRRouteRegistryCluster = 37
  FastMemCluster = 19

decision = KeepStopped
reason_token = AmbiguousUnconvertedSurfaceCandidates
```

The large raw count is now a bounded set of owner clusters. The first likely
implementation follow-up is not "convert 1396 methods"; it is to split the
largest cluster into policy / helper / verifier / route buckets.

## Acceptance

```text
likely_owner_cluster_recorded = 1
missing_projection_items_clustered = 1
cluster summary count equals MissingProjectionPolicy item count
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
1. MIRBUILDER-MISSING-PROJECTION-POLICY-JOINIR-PLAN-CLUSTER-001
   Split JoinIRPlanCluster into route policy, recipe matcher, facts helper,
   materializer, and private helper buckets.

2. MIRBUILDER-MISSING-PROJECTION-POLICY-JOINIR-ROUTE-VERIFY-CLUSTER-001
   Split JoinIR route / verify / facts / edgecfg surfaces into policy,
   observation, and nonsemantic helper buckets.

3. MIRBUILDER-MISSING-PROJECTION-POLICY-CONTEXT-SURFACE-JOIN-001
   Reconcile builder context and native crate context rows with existing
   route/adoption evidence.

4. MIRBUILDER-MISSING-PROJECTION-POLICY-CALL-EMIT-SSA-CLUSTER-001
   Split calls, emission, SSA, PHI, and statement/value helpers into
   projection-policy and verifier/oracle repair buckets.
```

## Non-Claims

```text
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
no new projection policy
```
