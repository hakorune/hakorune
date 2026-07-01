# 1989 - MIRBUILDER-FORBIDDEN-NONCLAIM-BOUNDARY-SCOPE-RESOLUTION-001

## Token

```text
MIRBUILDER-FORBIDDEN-NONCLAIM-BOUNDARY-SCOPE-RESOLUTION-001
```

## Purpose

Classify the `ForbiddenNonClaimBoundary` occurrences that still block strict
native-seed candidate selection after denied-boundary normalization.

This resolver does not weaken forbidden non-claims. It only determines whether
each occurrence is required by the selected narrow seed surface, is a wider
denied-boundary mention, belongs to a scoped diagnostic/permanent-derived lane,
or remains unclassified.

## Owner Kind

```text
ForbiddenNonClaimBoundaryScopeResolution
```

## Input

```text
normalized rerun:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-strict-converter-emission-native-seed-candidate-selection-normalized-rerun-v0.json

normalization fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-strict-denied-boundary-vocabulary-normalization-v0.json

input decision:
  KeepStopped

input reason:
  NoBridgeEligibleCandidateAfterDeniedBoundaryNormalization
```

## Scope Classes

```text
RequiredBySelectedNarrowSeedSurface
  absolute seed blocker

WiderDeniedBoundaryMentionOnly
  not seed evidence
  may be excluded from selected narrow seed blockers by BridgePolicyV2

ScopedForbiddenNonClaimExclusion
  not seed eligible
  may route to diagnostic or permanent-derived lane

PermanentForbiddenNonClaim
  absolute native-seed blocker
  route to permanent-derived classification

UnclassifiedForbiddenNonClaim
  design stop
```

## Task Shape

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_forbidden_nonclaim_boundary_scope_resolution.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-forbidden-nonclaim-boundary-scope-resolution-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_forbidden_nonclaim_boundary_scope_resolution_guard.sh
```

## Acceptance

```text
normalized_rerun_consumed = 1
input_reason_token = NoBridgeEligibleCandidateAfterDeniedBoundaryNormalization
input_normalized_row_count = 3
input_forbidden_nonclaim_blocked_count = 3
input_unclassified_denied_boundary_count = 0

all_forbidden_nonclaim_occurrences_classified = 1
manual_boundary_reclassification = 0

allowed scope classes:
  RequiredBySelectedNarrowSeedSurface
  WiderDeniedBoundaryMentionOnly
  ScopedForbiddenNonClaimExclusion
  PermanentForbiddenNonClaim
  UnclassifiedForbiddenNonClaim

for each occurrence:
  seed_eligibility_evidence = 0

if RequiredBySelectedNarrowSeedSurface > 0:
  seed_eligibility_blocker = 1

if UnclassifiedForbiddenNonClaim > 0:
  decision = KeepStopped

if WiderDeniedBoundaryMentionOnly only:
  may_select_bridge_policy_v2 = 1
  seed_eligibility_from_forbidden_nonclaim = 0

if PermanentForbiddenNonClaim > 0:
  may_select_permanent_derived_classification = 1
  native_seed_candidate = 0
```

## Decision Rule

```text
1. If any UnclassifiedForbiddenNonClaim:
     KeepStopped
     reason = UnclassifiedForbiddenNonclaimRequiresDesignStop

2. Else if any RequiredBySelectedNarrowSeedSurface:
     SelectPermanentDerivedClassification
     next = MIRBUILDER-RESULT-CARRIER-REFRESH-OWNERS-PERMANENT-DERIVED-CLASSIFICATION-001

3. Else if all forbidden non-claim occurrences are WiderDeniedBoundaryMentionOnly:
     SelectBridgePolicyV2
     next = MIRBUILDER-STRICT-EMISSION-TO-NATIVE-SEED-BRIDGE-POLICY-V2-001

4. Else if occurrences are ScopedForbiddenNonClaimExclusion:
     SelectDiagnosticLane or SelectPermanentDerivedClassification
     depending on whether the owner edge still has seed-relevant bounded surface evidence

5. Else:
     KeepStopped
```

## Non-Claims

```text
manual_boundary_reclassification = 0
seed_eligibility_from_forbidden_nonclaim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_canonical_mir_instruction = 0
new_python_semantic_projector = 0
generated_artifact_as_native_edit_authority = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runner_semantic_owner = 0
```

## Recovery

```text
reason_token:
  ForbiddenNonclaimBoundaryScopeUnresolved

recovery:
  Classify each runtime_fallback / new_backend_route / new_abi /
  new_canonical_mir_instruction occurrence by scope before any bridge policy
  update or permanent-derived classification. Do not turn forbidden non-claims
  into seed evidence.
```
