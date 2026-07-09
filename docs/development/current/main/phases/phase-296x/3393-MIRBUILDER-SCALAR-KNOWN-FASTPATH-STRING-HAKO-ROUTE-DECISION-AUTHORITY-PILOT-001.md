# 3393 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001
```

## Purpose

Materialize the scoped `.hako` route-decision authority pilot for
StringScalarI64Routes.

Only StringScalarI64Routes is in scope. The live Rust string fast path now calls
the String `.hako` authority pilot helper. That helper constructs the route
decision from the checked-in generated typed `.hako` artifact and compares it
against the existing Rust oracle decision with fail-fast mismatch behavior.

## Implementation

```text
authority function:
  string_scalar_i64_hako_route_authority_pilot_decision

authority source:
  STRING_SEARCH_SCALAR_I64_HAKO_POLICIES

route family:
  StringIndexOf
  StringLastIndexOf
  StringContains

Rust role:
  oracle / compat checker retained

mismatch policy:
  fail-fast

legacy wrapper retained:
  string_scalar_i64_shadow_consumed_decision
```

## Result

```text
string_hako_route_decision_authority_pilot = 1
string_hako_authority_result_consumed = 1
string_rust_oracle_compat_checker = 1
string_mismatch_fail_fast = 1
string_live_route_calls_authority_pilot = 1
```

## Decision

```text
decision:
  SelectStringAuthorityPilotRerun

reason_token:
  StringHakoRouteDecisionAuthorityPilotMaterialized

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-HAKO-AUTHORITY-PILOT-RERUN-001
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
  rust_lifecycle_mirbuilder_scalar_known_fastpath_string_hako_route_decision_authority_pilot_guard.sh
```
