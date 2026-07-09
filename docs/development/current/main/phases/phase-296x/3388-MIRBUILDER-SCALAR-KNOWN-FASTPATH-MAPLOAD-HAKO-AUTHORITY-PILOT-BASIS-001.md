# 3388 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-BASIS-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-BASIS-001
```

## Purpose

Define the basis for the first scoped MapLoad `.hako` route-decision authority
pilot.

This card is basis-only. It defines the authority source and Rust oracle /
compat-checker contract. It does not switch authority.

## Basis

```text
surface:
  MapLoadScalarI64Routes

route_kind:
  MapLoadScalarI64

authority source:
  MAPLOAD_SCALAR_I64_HAKO_POLICY

authority result fields:
  route_kind
  core_op
  lowering_tier
  return_shape
  value_demand
  publication_policy
  effect_class
  proof_family
  allowed_proofs
  role

Rust role:
  oracle / compat checker retained

mismatch policy:
  fail-fast
```

## Decision

```text
decision:
  SelectMapLoadRouteDecisionAuthorityPilotImplementation

reason_token:
  MapLoadAuthorityPilotBasisDefinedRustOracleRetained

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001
```

## Result

```text
mapload_hako_authority_pilot_basis = 1
mapload_authority_scope_defined = 1
hako_artifact_result_authority_source_defined = 1
rust_oracle_compat_checker_contract_defined = 1
mismatch_fail_fast_contract_defined = 1
basis_only = 1

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
  rust_lifecycle_mirbuilder_scalar_known_fastpath_mapload_hako_authority_pilot_basis_guard.sh
```
