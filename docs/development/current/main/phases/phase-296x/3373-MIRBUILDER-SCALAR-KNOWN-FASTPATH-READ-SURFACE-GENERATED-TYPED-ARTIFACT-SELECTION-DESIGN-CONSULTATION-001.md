# 3373 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-GENERATED-TYPED-ARTIFACT-SELECTION-DESIGN-CONSULTATION-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-GENERATED-TYPED-ARTIFACT-SELECTION-DESIGN-CONSULTATION-001
```

## Purpose

Consume the 3372 design stop and select the first ScalarKnown read surface for a
checked-in generated typed `.hako` artifact shadow-consume pilot.

This card is basis-only. It approves the read-surface priority proof axis and
selects `MapLoadScalarI64Routes` first. It does not create a generated artifact
and does not wire the Rust fast path.

## Proof Axis

```text
ReadSurfaceGeneratedArtifactMinimalityAxis:
  artifact_shape_complexity
  live_decision_insertion_locality
  policy_homogeneity
  semantic_authority_non_broadening
```

Forbidden proof sources:

```text
route_count_as_proof = 0
owner_name_as_proof = 0
source_path_as_authority = 0
route_membership_alone_as_proof = 0
manual_surface_selection = 0
```

## Selection

```text
selected_surface:
  MapLoadScalarI64Routes

selected_reason:
  Narrowest read generated artifact family that mirrors an existing
  scalar-map proof branch without broadening authority.

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-MAPLOAD-SCALAR-I64-001
```

## MapLoad Scope

```text
route_kind:
  MapLoadScalarI64

core_op:
  MapGet

return/value/publication/effect:
  ScalarI64OrMissingZero / ScalarI64 / NoPublication / read

proof_family:
  ScalarI64MapGetStoreFact

allowed_existing_proofs:
  MapSetScalarI64SameKeyNoEscape
  MapSetScalarI64DominatesNoEscape
  MapSetScalarI64CoveredDynamicI64KeyNoEscape
```

## Result

```text
read_surface_generated_typed_artifact_selection_consultation = 1
read_surface_generated_artifact_minimality_axis = 1
mapload_scalar_i64_routes_selected_first = 1
mapload_generated_artifact_basis_selected = 1
basis_only = 1
implementation_deferred_to_next_card = 1
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_read_surface_generated_typed_artifact_selection_design_consultation_guard.sh
```

## Non-Claims

```text
generated_typed_hako_artifact_created = 0
mapload_fastpath_shadow_consumed = 0
read_surface_connection_complete = 0
fastpath_connected_closeout = 0
hako_runtime_route_authority = 0
rust_fastpath_rewired = 0
route_selection_authority_switch = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
build_rs_hako_compiler_invocation = 0
live_hako_authority = 0
caller_orientation_runtime_path = 0
new_backend_route = 0
new_abi = 0
runtime_fallback = 0
source_selfhost_claim = 0
```
