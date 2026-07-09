# 3389 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001
```

## Purpose

Materialize the first scoped `.hako` route-decision authority pilot for
MapLoadScalarI64Routes.

Only MapLoadScalarI64Routes is in scope. The live Rust fast path now calls the
MapLoad `.hako` authority pilot helper. That helper constructs the route
decision from the checked-in generated typed `.hako` artifact and compares it
against the existing Rust oracle decision with fail-fast mismatch behavior.

## Implementation

```text
authority function:
  mapload_scalar_i64_hako_route_authority_pilot_decision

authority source:
  MAPLOAD_SCALAR_I64_HAKO_POLICY

Rust role:
  oracle / compat checker retained

mismatch policy:
  fail-fast

legacy wrapper retained:
  mapload_scalar_i64_shadow_consumed_decision
```

## Result

```text
mapload_hako_route_decision_authority_pilot = 1
mapload_hako_authority_result_consumed = 1
mapload_rust_oracle_compat_checker = 1
mapload_mismatch_fail_fast = 1
mapload_live_route_calls_authority_pilot = 1
```

## Decision

```text
decision:
  SelectMapLoadAuthorityPilotRerun

reason_token:
  MapLoadHakoRouteDecisionAuthorityPilotMaterialized

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-RERUN-001
```

## Non-Claims

```text
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
  rust_lifecycle_mirbuilder_scalar_known_fastpath_mapload_hako_route_decision_authority_pilot_guard.sh
```
