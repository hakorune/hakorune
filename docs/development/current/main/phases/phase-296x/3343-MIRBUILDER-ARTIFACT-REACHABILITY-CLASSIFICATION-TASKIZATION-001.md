# 3343 - MIRBUILDER-ARTIFACT-REACHABILITY-CLASSIFICATION-TASKIZATION-001

## Token

```text
MIRBUILDER-ARTIFACT-REACHABILITY-CLASSIFICATION-TASKIZATION-001
```

## Purpose

Convert the post-3342 reachability review into an explicit task order before
continuing the ScalarKnown fast-path `.hako` shadow-consumption handoff.

The review found three different artifact classes mixed together:

```text
live fast-path owners
guard/parity executable mirrors
proof-only Rust bridge modules
unreached rust_lifecycle guard scripts
```

These must be classified before claiming that a `.hako` mirror or Rust proof
bridge is connected to the real compiler path.

## Review Findings

```text
compare proof bridge cluster:
  src/mir/builder/compare_branch_emission_bridge.rs
  src/mir/builder/compare_localssa_finalize_compare_bridge.rs
  src/mir/builder/compare_mir_compare_emission_bridge.rs
  src/mir/builder/compare_rhs_symbolref_contract.rs
  src/mir/builder/compare_rhs_symbolref_lookup_bridge.rs
  src/mir/builder/compare_rhs_valueid_resolution_bridge.rs

finding:
  modules are declared by src/mir/builder.rs
  normal compare lowering still flows through src/mir/builder/ops/comparison.rs
  production fast-path connection is unproven

hako mirror library:
  lang/src/compiler/lib/*.hako policy/classifier/formatter mirrors

finding:
  compiler entry transitive using graph does not reach the mirror library
  parity gates may import mirrors through temporary guard apps
  HakoAdopted currently means executable Rust-oracle mirror, not route authority

rust_lifecycle guards:
  many individual guards exist
  dev_gate quick and CI do not run them as an active guard set

finding:
  running every rust_lifecycle guard by default is too heavy
  active guard execution must be latest/current-blocker scoped
```

## Task Order

```text
1. MIRBUILDER-ARTIFACT-REACHABILITY-CLASSIFICATION-INVENTORY-001
   classify artifacts as live_fastpath, shadow_mirror, proof_only, or
   unreached_guard; no deletion or connection.

2. MIRBUILDER-CURRENT-ACTIVE-RUST-LIFECYCLE-GUARD-RESOLVER-001
   design and implement a light resolver that maps CURRENT_STATE latest/current
   blocker to at most a few active guards; do not run all historical guards.

3. MIRBUILDER-COMPARE-PROOF-BRIDGE-PARK-OR-CONNECT-DESIGN-STOP-001
   decide whether the compare proof bridge cluster is deleted, parked as
   proof-only, or refactored into production helpers.

4. MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-SET-MAPSTORE-I64-001
   resume the selected ScalarKnown fast-path shadow-consumption handoff after
   reachability and active-guard boundaries are explicit.
```

## Result

```text
artifact_reachability_review_taskized = 1
run_all_rust_lifecycle_guards_by_default = 0
active_guard_resolver_required = 1
compare_proof_bridge_fastpath_connection_unproven = 1
hako_mirror_fastpath_connection_unproven = 1
selected_next_card = MIRBUILDER-ARTIFACT-REACHABILITY-CLASSIFICATION-INVENTORY-001
source_selfhost_claim = 0
```

## Non-Claims

```text
rust_fastpath_rewired = 0
hako_runtime_route_authority = 0
hako_backend_lowering_authority = 0
compare_bridge_deleted = 0
compare_bridge_production_connected = 0
all_rust_lifecycle_guards_in_ci = 0
all_rust_lifecycle_guards_in_dev_gate = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```
