# 1873 - MIRBUILDER-UNCONVERTED-SURFACE-NEXT-OWNER-RESOLVER-TASK-CONTRACT-001

## Token

```text
MIRBUILDER-UNCONVERTED-SURFACE-NEXT-OWNER-RESOLVER-TASK-CONTRACT-001
```

## Purpose

Fix the next task shape after the carrier-merge assignment adoption.

The crate-wide unconverted surface report already exposes the remaining Rust
surface inventory and owner clusters. The next executable task is not wider
route expansion and not manual family selection. It is a deterministic resolver
that consumes the report and emits exactly one next owner when the evidence is
unambiguous.

## Inventory

```text
current_blocker = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
source_selfhost_claim = 0
unconverted_report_decision = KeepStopped
unconverted_report_reason = AmbiguousUnconvertedSurfaceCandidates

missing_projection_policy_count = 1396
borrow_policy_needed_count = 113
composite_suspected_count = 1
mapped_to_known_owner_count = 18

largest_cluster = JoinIRPlanCluster
largest_cluster_count = 628
```

This means there are not 1396 direct implementation tasks. The large raw count
is already clustered. The missing piece is a deterministic resolver over those
clusters and reason tokens.

## Next Task

```text
MIRBUILDER-UNCONVERTED-SURFACE-NEXT-OWNER-RESOLVER-001
```

Expected output:

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-unconverted-surface-next-owner-resolution-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_unconverted_surface_next_owner_resolver_guard.sh
```

## Resolver Rules

```text
exclude:
  AlreadyAdopted
  SupportLaneOnly
  TestOnlySurface
  DebugOnlySurface
  BorrowSurfacePolicyKnown

priority:
  MissingRouteOrArtifactEvidence
  MissingProjectionPolicy
  BorrowSurfaceNeedsPolicy
  CompositeNeedsDecomposition
  CompositeSuspected
  MissingVerifierOrOracle
  NativeSeedReady
  ConvertibleLeaf

if exactly one candidate at the highest priority:
  select its next owner card

if multiple candidates tie:
  KeepStopped(reason = AmbiguousNextOwnerCandidates)

if no candidates remain:
  KeepStopped(reason = NoMachineDerivedNextOwner)
```

GeneratedArtifactOnly may become seedable only if the future resolver can prove:

```text
shadow_parity_green = 1
hako_mainline_or_promotion_green = 1
bounded_surface = 1
borrow_policy_resolved = 1
composite_owner = 0
deterministic_regeneration_evidence = 1
verifier_or_oracle_present = 1
generated_artifact_as_edit_authority = 0
```

## Acceptance

```text
task_contract_fixture_exists = 1
resolver_implementation_deferred = 1
report_consumed = 1
manual_family_selection = 0
support_lane_projector_as_hako_adoption_candidate = 0
generated_artifact_as_edit_authority = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no resolver implementation yet
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
no manual family selection
```
