# 3344 - MIRBUILDER-ARTIFACT-REACHABILITY-CLASSIFICATION-INVENTORY-001

## Token

```text
MIRBUILDER-ARTIFACT-REACHABILITY-CLASSIFICATION-INVENTORY-001
```

## Purpose

Classify the artifacts called out by the post-3342 review before continuing the
ScalarKnown fast-path `.hako` shadow-consumption handoff.

The goal is to make closed-world proof/mirror artifacts visibly different from
live compiler fast-path owners.

## Classification

```text
live_fastpath:
  src/mir/generic_method_route_plan/write_routes.rs
  src/mir/generic_method_route_plan/collection_read_routes.rs
  src/mir/generic_method_route_plan/string_routes.rs
  src/mir/route_value_type_publication.rs
  src/mir/builder/control_flow/joinir/route_entry/runtime_adjacent_shadow_guard.rs
  src/mir/global_call_route_plan/same_module_static_helper_contract.rs

proof_only_rust_bridge:
  src/mir/builder/compare_branch_emission_bridge.rs
  src/mir/builder/compare_localssa_finalize_compare_bridge.rs
  src/mir/builder/compare_mir_compare_emission_bridge.rs
  src/mir/builder/compare_rhs_symbolref_contract.rs
  src/mir/builder/compare_rhs_symbolref_lookup_bridge.rs
  src/mir/builder/compare_rhs_valueid_resolution_bridge.rs

shadow_mirror_library:
  lang/src/compiler/lib/*.hako

unreached_guard_ecosystem:
  tools/checks/rust_lifecycle*.sh
```

## Result

```text
artifact_reachability_classification_inventory = 1
live_fastpath_owner_examples_count = 6
compare_proof_bridge_file_count = 6
compare_proof_bridge_total_lines = 974
compare_proof_bridge_production_connected = 0
hako_lib_compiler_reachable_count = 0
hako_mirror_library_fastpath_connected = 0
rust_lifecycle_guard_script_count = 1044
run_all_rust_lifecycle_guards_by_default = 0
active_guard_resolver_required = 1
source_selfhost_claim = 0
```

## Decision

```text
decision:
  SelectActiveGuardResolverBeforeShadowConsume

reason_token:
  ReachabilityMixedClosedWorldArtifacts

selected_next_card:
  MIRBUILDER-CURRENT-ACTIVE-RUST-LIFECYCLE-GUARD-RESOLVER-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_artifact_reachability_classification_inventory_guard.sh
```

## Non-Claims

```text
compare_bridge_deleted = 0
compare_bridge_production_connected = 0
hako_runtime_route_authority = 0
hako_backend_lowering_authority = 0
all_rust_lifecycle_guards_in_ci = 0
all_rust_lifecycle_guards_in_dev_gate = 0
rust_fastpath_rewired = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```
