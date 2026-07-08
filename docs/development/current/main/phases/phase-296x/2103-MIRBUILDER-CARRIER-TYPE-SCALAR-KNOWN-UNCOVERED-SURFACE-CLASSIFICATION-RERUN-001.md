# 2103 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-CLASSIFICATION-RERUN-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-CLASSIFICATION-RERUN-001
```

## Purpose

Rerun the ScalarKnown uncovered-surface classification after 2102.

This card selects the next basis card for a narrow string-search scalar i64
typed direct closeout contract. It does not materialize that contract yet.

## Result

```text
classified_surface_count = 3
selection_eligible_surface_count = 1
selected_surface_count = 1
direct_contract_materialized = 0
scalar_known_transport_axis_closeout = 0

selected_surface_id:
  StringScalarI64Routes

selected_contract_id:
  StringSearchScalarI64TypedDirectCloseoutContract

decision:
  SelectStringSearchScalarI64TypedDirectCloseoutContractBasis

reason:
  ExactlyOneScalarKnownUncoveredSurfaceClassified

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001
```

## Rejected Direct Picks

```text
CollectionScalarI64Routes:
  blocked_by = MixedWithAlreadyClosedMapLoadScalarI64

WriteScalarI64Routes:
  blocked_by = WriteResultPolicyRequiredBeforeDirectCloseout
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_uncovered_surface_classification_rerun_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_uncovered_surface_classification_rerun.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_uncovered_surface_classification_rerun_guard.sh
```

## Non-Claims

```text
direct_contract_materialized = 0
scalar_known_transport_axis_closeout = 0
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
manual_axis_selection = 0
manual_carrier_selection = 0
row_count_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
```
