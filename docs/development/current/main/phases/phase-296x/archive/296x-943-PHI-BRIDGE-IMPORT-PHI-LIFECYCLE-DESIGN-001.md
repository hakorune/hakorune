# 296x-943 PHI-BRIDGE-IMPORT-PHI-LIFECYCLE-DESIGN-001

Status: Landed
Date: 2026-06-16
Scope: BoxShape-only PHI lifecycle design.

## Purpose

Classify the remaining non-test direct PHI construction sites after the
loop-header batch/prepend migration.

`bridge` and `import` are not the same owner:

- semantic bridge code that creates new MIR behavior should route through a
  lifecycle API;
- JSON import/deserialization code reconstructs serialized MIR and should stay
  an explicit import boundary, not masquerade as semantic PHI lowering.

## Inventory

```text
semantic_bridge_phi_site:
  src/mir/join_ir_vm_bridge/handlers/conditional_method_call.rs
  shape=function_level_new_merge_block_phi
  migration_candidate=1

json_import_phi_site:
  src/runner/json_v1_bridge/parse/instruction.rs
  shape=serialized_mir_import
  migration_candidate=0
  import_boundary_allowed=1

joinir_rewrite_phi_site:
  src/mir/builder/control_flow/joinir/merge/rewriter/stages/plan/instruction_rewrite.rs
  shape=existing_instruction_transform
  migration_candidate=defer

phi_block_remapper_site:
  src/mir/builder/control_flow/joinir/merge/phi_block_remapper.rs
  shape=existing_phi_block_id_transform
  migration_candidate=defer
```

## Decision

Open a narrow semantic bridge row first.

```text
next_task=PHI-JOINIR-VM-BRIDGE-FUNCTION-LIFECYCLE-001
target_file=src/mir/join_ir_vm_bridge/handlers/conditional_method_call.rs
required_api=function_level_phi_final_with_type_hint_and_span
json_import_migration_started=0
joinir_rewrite_migration_started=0
phi_block_remapper_migration_started=0
```

The semantic bridge cannot use the builder-level facade because it operates on
`MirFunction` and constructs standalone blocks. The lifecycle owner should grow
a function-level typed/spanned final PHI helper rather than allowing the bridge
to construct `MirInstruction::Phi` directly.

## Stop Line

```text
do_not_route_json_import_through_builder_lifecycle=1
do_not_mix_import_with_semantic_bridge=1
do_not_migrate_phi_block_remapper_in_bridge_row=1
do_not_migrate_instruction_rewrite_in_bridge_row=1
```

## Guard Direction

After the semantic bridge row lands, `coreplan_phi_binding_boundary_guard.sh`
should remove `conditional_method_call.rs` from the direct PHI construction
allowlist while keeping explicit import/test/transform boundaries separate.

