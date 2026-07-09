# 3396 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-AUTHORITY-PILOT-BASIS-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-AUTHORITY-PILOT-BASIS-001
```

## Purpose

Select CollectionScalarI64Routes as the next scoped `.hako` route-decision
authority pilot after MapLoad and String.

This card is basis-only. It declares the mixed receiver-domain boundary and the
AnyLength / Box row boundary before implementation.

## Proof Axis

```text
PriorScopedReadAuthorityContinuation
+
LenSurfacePolicyHomogeneousScalarI64NoPublicationObserve
+
ExplicitEnumeratedMixedReceiverDomainBoundary
+
AnyLengthBoxDomainIsExplicitRowNotWildcardSelector
+
GeneratedTypedArtifactMismatchGateCurrent
+
RustOracleCompatFailFastRetained
```

The homogeneous axes are:

```text
return_shape = ScalarI64
value_demand = ScalarI64
publication_policy = NoPublication
effect_class = observe
proof_or_policy_source = LenSurfacePolicy
lowering_tier = WarmDirectAbi
```

Receiver domain is intentionally mixed and is not used as selection proof.

## Route Rows

```text
MapEntryCount -> MapLen, receiver_domain = MapBox
ArraySlotLen -> ArrayLen, receiver_domain = ArrayBox
StringLen -> StringLen, receiver_domain = StringBox
AnyLength -> AnyLen, receiver_domain = Box
```

`AnyLength / Box` is explicit row metadata only. It is not a wildcard selector,
global Box authority, runtime Box-domain fallback, or receiver-domain widening
authority.

## Decision

```text
decision:
  SelectCollectionRouteDecisionAuthorityPilotImplementation

reason_token:
  CollectionLenSurfacePolicyHomogeneousAxesMixedDomainBoundaryDeclared

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001
```

## Result

```text
collection_hako_authority_pilot_basis = 1
selected_surface = CollectionScalarI64Routes
selected_route_family = MapEntryCount_ArraySlotLen_StringLen_AnyLength
prior_scoped_read_authority_continuation = 1
len_surface_policy_homogeneous_scalar_i64_no_publication_observe = 1
generated_typed_artifact_mismatch_gate_current = 1
mixed_receiver_domain_boundary_declared = 1
explicit_mixed_receiver_domain_enumeration = 1
receiver_domain_not_used_as_selection_proof = 1
any_length_box_domain_is_explicit_row_not_wildcard_selector = 1
rust_oracle_compat_checker_retained = 1
mismatch_fail_fast_required = 1
basis_only = 1
authority_pilot_implementation_deferred = 1

collection_hako_route_decision_authority_pilot = 0
collection_hako_authority_result_consumed = 0
collection_live_route_calls_authority_pilot = 0
scalar_known_hako_runtime_route_authority = 0
source_selfhost_claim = 0
```

## Non-Claims

```text
collection_hako_route_decision_authority_pilot = 0
collection_hako_authority_result_consumed = 0
collection_live_route_calls_authority_pilot = 0
collection_anylength_global_box_authority = 0
receiver_domain_authority_switch = 0
receiver_domain_widening_authority = 0
receiver_domain_projection = 0
any_length_wildcard_selector = 0
runtime_box_domain_fallback = 0
read_surface_authority_closeout = 0
scalar_known_hako_runtime_route_authority = 0
scalar_known_transport_axis_authority_switch = 0
rust_fastpath_rewired = 0
route_selection_authority_switch = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
caller_orientation_runtime_path = 0
build_rs_hako_compiler_invocation = 0
live_hako_authority = 0
new_backend_route = 0
new_abi = 0
runtime_fallback = 0
source_selfhost_claim = 0

route_count_as_proof = 0
owner_name_as_proof = 0
source_path_as_authority = 0
route_membership_alone_as_proof = 0
manual_surface_selection = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_collection_hako_authority_pilot_basis_guard.sh
```
