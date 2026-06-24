# 296x-947 PHI-IMPORT-BOUNDARY-CLOSEOUT-001

Status: Landed
Date: 2026-06-16
Scope: PHI lifecycle closeout.

## Purpose

Close the current PHI lifecycle migration lane by classifying the remaining
direct PHI construction sites.

JSON MIR import is an explicit serialized-MIR reconstruction boundary. It is
not semantic PHI lowering and should not route through builder lifecycle APIs.

## Boundary

```text
semantic_phi_definition_owner=phi_lifecycle
existing_phi_transform_owner=remap_existing_phi_block_ids
json_import_phi_owner=json_v1_bridge_parse_instruction
test_fixture_phi_owner=test_only
```

## Remaining Allowed Direct PHI Sites

```text
src/runner/json_v1_bridge/parse/instruction.rs:
  serialized MIR import boundary

src/mir/join_ir_vm_bridge/block_finalizer.rs:
  test-only existing-PHI preservation fixture

src/mir/builder/record_helper_args.rs:
  test/helper inference fixture

src/mir/builder/ssa/phi_input_materializer.rs:
  unit tests / fixture construction
```

## Contract

```text
output_contract=phi_import_boundary_closeout_v0
semantic_joinir_vm_bridge_direct_phi_construction=0
loop_header_direct_phi_construction=0
simple_coreplan_raw_phi_emit=0
existing_phi_transform_boundary_named=1
json_import_boundary_allowed=1
json_import_routed_through_builder_lifecycle=0
summary=ok
```

## Stop Line

```text
do_not_route_json_import_through_builder_lifecycle=1
do_not_treat_import_as_semantic_lowering=1
do_not_migrate_test_fixture_phi_builders=1
```

## Proof

```bash
cargo fmt --check
cargo check --bin hakorune
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

