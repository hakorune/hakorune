# 2107 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-RUST-TYPED-DIRECT-CLOSEOUT-CONTRACT-BOUNDARY-REFACTOR-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-RUST-TYPED-DIRECT-CLOSEOUT-CONTRACT-BOUNDARY-REFACTOR-001
```

## Purpose

Repackage existing ScalarKnown Rust owner evidence into a data-only typed
direct closeout contract boundary before continuing Hako migration.

This is a BoxShape refactor. It does not change route selection, return-shape
semantics, publication semantics, effect semantics, lowering paths, or runtime
behavior.

## Rust Boundary

```text
source:
  src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs

struct:
  ScalarKnownTypedDirectCloseoutContract

status enum:
  ScalarKnownContractStatus
    AcceptedScopedCloseout
    CandidateNeedsPolicy
```

## Accepted Scoped Closeouts

```text
MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract
StringSearchScalarI64TypedDirectCloseoutContract
```

## Candidate Surfaces

```text
CollectionScalarI64Routes
WriteScalarI64Routes
```

## Result

```text
rust_contract_boundary_refactor = 1
scalar_known_typed_direct_closeout_contract_boundary = 1
accepted_scoped_closeout_contract_count = 2
remaining_candidate_surface_count = 2
behavior_preserved = 1
existing_rust_owner_evidence_repackaged = 1

decision:
  SelectRemainingSurfaceBoundaryInventoryRerunAfterRustBoundaryRefactor

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-RERUN-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_rust_typed_direct_closeout_contract_boundary_refactor_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-rust-typed-direct-closeout-contract-boundary-refactor-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_rust_typed_direct_closeout_contract_boundary_refactor.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_rust_typed_direct_closeout_contract_boundary_refactor_guard.sh
```

## Non-Claims

```text
direct_contract_selection = 0
collection_direct_closeout_ready = 0
write_direct_closeout_ready = 0
scalar_known_transport_axis_closeout = 0
hako_adoption = 0
source_selfhost_claim = 0
new_route_authority = 0
behavior_change = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
new_python_semantic_projector = 0
```
