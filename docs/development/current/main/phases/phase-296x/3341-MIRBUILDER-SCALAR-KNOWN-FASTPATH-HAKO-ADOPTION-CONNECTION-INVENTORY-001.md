# 3341 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-ADOPTION-CONNECTION-INVENTORY-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-ADOPTION-CONNECTION-INVENTORY-001
```

## Purpose

Stop the ScalarKnown Write closeout chain and inventory whether the adopted
`.hako` policy mirrors are actually connected to the Rust fast-path execution
path.

The inventory records that the current Rust fast-path truth is still:

```text
write_routes / collection_read_routes / string_routes
  -> generic_method_routes
  -> lowering_plan
  -> C shim generic method match
```

The `scalar_known_typed_direct_closeout_contract.rs` module is declared in the
Rust route-plan module, but the contract table is not consumed by the fast-path
decision path. The `.hako` policy classifiers are guard-executed parity mirrors,
not runtime route authority.

## Evidence

```text
rust_execution_path:
  src/mir/generic_method_route_plan.rs
  src/mir/generic_method_route_plan/write_routes.rs
  src/mir/route_fixpoint.rs
  src/runner/mir_json_emit/route_json.rs
  lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc

contract_inventory:
  src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs

hako_policy_mirrors:
  lang/src/compiler/lib/write_push_surface_policy_classifier.hako
  lang/src/compiler/lib/write_delete_surface_policy_classifier.hako
  lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako
  lang/src/compiler/lib/write_set_mapstore_any_policy_classifier.hako
  lang/src/compiler/lib/generic_method_route_fact_token_formatter.hako
```

## Acceptance

```text
scalar_known_fastpath_hako_adoption_connection_inventory = 1
rust_fastpath_owner_still_write_routes = 1
contract_module_declared = 1
contract_external_rust_reference_count = 0
contract_fastpath_connected = 0
hako_policy_mirror_guard_only = 1
hako_fastpath_runtime_connection = 0
hako_adopted_as_runtime_authority = 0
source_selfhost_claim = 0
closeout_chain_pause_required = 1
```

## Decision

```text
decision:
  DesignConsultationRequired

reason_token:
  HakoAdoptionMirrorNotConnectedToRustFastpath

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CONNECTION-DESIGN-CONSULTATION-001
```

## Consultation Draft

```text
ScalarKnown `.hako` adoption is currently a guard-executed Rust-oracle mirror,
while Rust fast-path truth remains write_routes / collection_read_routes /
string_routes -> generic_method_routes -> lowering_plan / C shim.

Should the next move be:

A. Redefine HakoAdopted as executable mirror only and stop closeout claims until
   the Rust execution path consumes a `.hako` artifact.

B. Choose one narrow surface, such as SetSurfacePolicy / MapStoreI64, and connect
   a generated or compiled `.hako` artifact to the Rust fast-path decision point.

Please decide the first allowed connection mechanism, the minimum safe surface,
and which claims remain forbidden until the Rust execution path consumes the
`.hako` artifact.
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_adoption_connection_inventory_guard.sh
```

## Non-Claims

```text
rust_fastpath_rewired = 0
hako_runtime_route_authority = 0
hako_backend_lowering_authority = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```
