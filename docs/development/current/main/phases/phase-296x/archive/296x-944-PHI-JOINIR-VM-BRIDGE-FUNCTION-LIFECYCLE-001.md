# 296x-944 PHI-JOINIR-VM-BRIDGE-FUNCTION-LIFECYCLE-001

Status: Landed
Date: 2026-06-16
Scope: BoxShape-only PHI lifecycle migration.

## Purpose

Migrate the semantic JoinIR VM bridge PHI construction site to a
function-level lifecycle API.

This row targets the semantic conditional-method-call bridge sites only:

- `src/mir/join_ir_vm_bridge/handlers/conditional_method_call.rs`
- `src/mir/join_ir_vm_bridge/joinir_block_converter/handlers.rs`

JSON import/deserialization and existing-PHI transform sites remain separate
boundaries.

## Changes

- Expose the PHI lifecycle module within `crate::mir`.
- Add `define_phi_final_fn_with_type_hint_and_tag`.
- Route conditional method-call merge PHIs through that lifecycle helper.
- Extend `coreplan_phi_binding_boundary_guard.sh` to scan
  `src/mir/join_ir_vm_bridge`.

## Contract

```text
output_contract=phi_joinir_vm_bridge_function_lifecycle_v0
target_file_count=2
conditional_method_call_handler_direct_phi_construction=0
joinir_block_converter_conditional_method_call_direct_phi_construction=0
function_level_lifecycle_phi_with_type_hint=1
json_import_migration_started=0
phi_block_remapper_migration_started=0
instruction_rewrite_migration_started=0
guard_updated=1
summary=ok
```

## Stop Line

```text
do_not_route_json_import_through_builder_lifecycle=1
do_not_migrate_phi_block_remapper_in_this_row=1
do_not_migrate_instruction_rewrite_in_this_row=1
```

## Proof

```bash
cargo fmt --check
cargo check --bin hakorune
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
