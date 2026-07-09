# 3387 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-DESIGN-CONSULTATION-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-DESIGN-CONSULTATION-001
```

## Purpose

Select MapLoadScalarI64Routes as the first scoped `.hako` route-authority pilot
surface after the all-surface ScalarKnown mismatch gate was hardened.

This is design consultation / basis-only. It selects the pilot basis and defines
the authority boundary. It does not implement an authority switch.

## Selected Surface

```text
surface:
  MapLoadScalarI64Routes

route_kind:
  MapLoadScalarI64

shape:
  MapGet / WarmDirectAbi / ScalarI64OrMissingZero / ScalarI64 /
  NoPublication / read

proof_family:
  ScalarI64MapGetStoreFact
```

## Decision

```text
decision:
  SelectMapLoadHakoAuthorityPilotBasis

reason_token:
  MapLoadIsSmallestReadNoPublicationAuthorityPilotSurface

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-BASIS-001
```

## Result

```text
mapload_hako_route_authority_pilot_basis = 1
selected_surface = MapLoadScalarI64Routes
selected_route_kind = MapLoadScalarI64
hako_generated_typed_artifact_authority_candidate = 1
rust_oracle_compat_checker_retained = 1
mismatch_fail_fast_required = 1
basis_only = 1
authority_switch_implementation_deferred = 1

mapload_hako_route_decision_authority_pilot = 0
scalar_known_hako_runtime_route_authority = 0
source_selfhost_claim = 0
```

## Non-Claims

```text
mapload_hako_route_decision_authority_pilot = 0
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
  rust_lifecycle_mirbuilder_scalar_known_fastpath_mapload_hako_authority_pilot_design_consultation_guard.sh
```
