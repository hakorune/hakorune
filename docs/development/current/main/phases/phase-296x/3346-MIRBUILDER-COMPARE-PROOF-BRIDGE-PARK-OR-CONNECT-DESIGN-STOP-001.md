# 3346 - MIRBUILDER-COMPARE-PROOF-BRIDGE-PARK-OR-CONNECT-DESIGN-STOP-001

## Token

```text
MIRBUILDER-COMPARE-PROOF-BRIDGE-PARK-OR-CONNECT-DESIGN-STOP-001
```

## Purpose

Stop before changing the compare proof bridge cluster.

The reachability inventory proved that the compare bridge files are declared in
`src/mir/builder.rs`, but the live compare lowering path still goes through
`src/mir/builder/ops/comparison.rs` directly.

## Evidence

```text
proof_only_rust_bridge:
  src/mir/builder/compare_branch_emission_bridge.rs
  src/mir/builder/compare_localssa_finalize_compare_bridge.rs
  src/mir/builder/compare_mir_compare_emission_bridge.rs
  src/mir/builder/compare_rhs_symbolref_contract.rs
  src/mir/builder/compare_rhs_symbolref_lookup_bridge.rs
  src/mir/builder/compare_rhs_valueid_resolution_bridge.rs

inventory result:
  compare_proof_bridge_file_count = 6
  compare_proof_bridge_total_lines = 974
  compare_proof_bridge_production_connected = 0

live compare path:
  src/mir/builder/ops/comparison.rs
  ssa::local::finalize_compare
  emission::compare::emit_to
```

## Consultation Question

The compare bridge cluster is currently proof-only Rust code. It is compiled
through module declarations but is not the live compare lowering path.

Which next move is allowed?

```text
A. Delete the compare proof bridge cluster.
   - remove the six Rust files and builder.rs module declarations
   - archive or remove the dependent guards/fixtures/cards
   - requires a cargo/test proof that live compare lowering still works

B. Park the compare proof bridge cluster as proof-only.
   - move or mark it as historical/proof-only
   - keep historical guards callable by explicit path
   - keep it out of live fast-path / authority claims

C. Connect a minimal subset to production.
   - refactor proof response-heavy bridge code into thin production helpers
   - first candidate: LocalSSA finalize compare + MIR compare emission
   - no route/runtime authority change
   - requires behavior-preserving compare lowering tests
```

Please decide the first allowed action and the non-claims that must remain
zero while the action is performed.

## Result

```text
compare_proof_bridge_design_stop = 1
consultation_required = 1
compare_bridge_deleted = 0
compare_bridge_parked = 0
compare_bridge_production_connected = 0
selected_next_card = CONSULTATION_REQUIRED
source_selfhost_claim = 0
```

## Non-Claims

```text
rust_fastpath_rewired = 0
compare_lowering_behavior_changed = 0
route_selection_authority = 0
hako_runtime_route_authority = 0
hako_backend_lowering_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```
