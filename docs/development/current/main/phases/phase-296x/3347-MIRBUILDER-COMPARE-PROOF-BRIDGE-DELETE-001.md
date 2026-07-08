# 3347 - MIRBUILDER-COMPARE-PROOF-BRIDGE-DELETE-001

## Token

```text
MIRBUILDER-COMPARE-PROOF-BRIDGE-DELETE-001
```

## Purpose

Delete the proof-only compare bridge cluster from the live Rust builder module
surface.

The live compare lowering path remains:

```text
src/mir/builder/ops/comparison.rs
  -> ssa::local::finalize_compare
  -> emission::compare::emit_to
```

## Deleted Rust Files

```text
src/mir/builder/compare_branch_emission_bridge.rs
src/mir/builder/compare_localssa_finalize_compare_bridge.rs
src/mir/builder/compare_mir_compare_emission_bridge.rs
src/mir/builder/compare_rhs_symbolref_contract.rs
src/mir/builder/compare_rhs_symbolref_lookup_bridge.rs
src/mir/builder/compare_rhs_valueid_resolution_bridge.rs
```

The corresponding `src/mir/builder.rs` module declarations are removed.

## Result

```text
compare_proof_bridge_deleted = 1
compare_proof_bridge_deleted_file_count = 6
builder_mod_declarations_removed = 1
live_compare_path_preserved = 1
compare_lowering_behavior_changed = 0
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_compare_proof_bridge_delete_guard.sh
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-SET-MAPSTORE-I64-001
```

## Non-Claims

```text
route_selection_authority = 0
hako_runtime_route_authority = 0
hako_backend_lowering_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```
