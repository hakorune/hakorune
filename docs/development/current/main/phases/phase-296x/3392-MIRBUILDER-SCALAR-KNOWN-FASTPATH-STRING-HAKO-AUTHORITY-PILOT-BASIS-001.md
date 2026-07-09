# 3392 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-HAKO-AUTHORITY-PILOT-BASIS-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-HAKO-AUTHORITY-PILOT-BASIS-001
```

## Purpose

Select StringScalarI64Routes as the next scoped `.hako` route-decision
authority pilot after MapLoad.

This card is basis-only. It records the consultation-approved proof axis and
selects the implementation card. It does not switch String route-decision
authority yet.

## Proof Axis

```text
PriorScopedReadAuthorityContinuation
+
HomogeneousScalarI64NoPublicationReadSurface
+
RustOracleCompatFailFastRetained
```

String is selected because it is a read-only String receiver-domain surface with
homogeneous ScalarI64 / NoPublication shape across:

```text
StringIndexOf
StringLastIndexOf
StringContains
```

This is not route-count proof and not manual surface selection.

## Decision

```text
decision:
  SelectStringRouteDecisionAuthorityPilotImplementation

reason_token:
  StringHomogeneousScalarI64NoPublicationReadSurface

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001
```

## Result

```text
string_hako_authority_pilot_basis = 1
selected_surface = StringScalarI64Routes
prior_scoped_read_authority_continuation = 1
homogeneous_scalar_i64_no_publication_read_surface = 1
rust_oracle_compat_checker_retained = 1
mismatch_fail_fast_required = 1
basis_only = 1

string_hako_route_decision_authority_pilot = 0
scalar_known_hako_runtime_route_authority = 0
source_selfhost_claim = 0
```

## Non-Claims

```text
string_hako_route_decision_authority_pilot = 0
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
apparent_simplicity_as_proof = 0
manual_surface_selection = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_string_hako_authority_pilot_basis_guard.sh
```
