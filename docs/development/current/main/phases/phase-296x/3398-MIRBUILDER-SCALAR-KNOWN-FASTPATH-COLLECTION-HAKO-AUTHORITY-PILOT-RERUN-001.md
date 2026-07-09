# 3398 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-AUTHORITY-PILOT-RERUN-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-AUTHORITY-PILOT-RERUN-001
```

## Purpose

Rerun after the scoped Collection `.hako` route-decision authority pilot.

MapLoad, String, and Collection are now scoped `.hako` route-decision authority
pilots. Rust is still the oracle / compat checker, mismatch remains fail-fast,
and no broader ScalarKnown or runtime authority has moved.

## Result

```text
collection_hako_authority_pilot_rerun = 1
mapload_hako_route_decision_authority_pilot = 1
string_hako_route_decision_authority_pilot = 1
collection_hako_route_decision_authority_pilot = 1
collection_mixed_receiver_domain_guarded = 1
collection_anylength_box_domain_guarded = 1
read_surface_authority_closeout_design_required = 1

scalar_known_hako_runtime_route_authority = 0
source_selfhost_claim = 0
```

## Decision

```text
decision:
  KeepStoppedForReadSurfaceAuthorityCloseoutDesign

reason_token:
  AllReadSurfacesScopedAuthorityPilotsCompleteCloseoutStillConsultationGated

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-AUTHORITY-CLOSEOUT-DESIGN-STOP-001
```

## Non-Claims

```text
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
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_collection_hako_authority_pilot_rerun_guard.sh
```
