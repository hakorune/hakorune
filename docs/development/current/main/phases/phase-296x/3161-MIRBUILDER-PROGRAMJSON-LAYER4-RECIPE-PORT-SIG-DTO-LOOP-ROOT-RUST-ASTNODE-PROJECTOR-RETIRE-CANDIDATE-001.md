# 3161 - MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-PORT-SIG-DTO-LOOP-ROOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark the covered `RecipePortSigDtoSnapshotV1` loop-root rows from 3160 as a
scoped Rust ASTNode projector retire-candidate.

Covered rows:

```text
loop_if_then_return_new_stringbox_abc_assignment_final_return_var
loop_if_then_return_call_id0_assignment_final_return_var
loop_if_then_return_call_id1_int9_assignment_final_return_var
loop_if_then_return_call_id1_int7_assignment_final_return_var
loop_if_then_return_method_stringbox_length_abc_assignment_final_return_var
loop_if_then_return_method_stringbox_indexof_b_abc_assignment_final_return_var
```

This is a proof checkpoint only. It does not switch the runtime route away from
Rust, remove the bootstrap oracle, execute RecipeMatcher, select backend routes,
lower MIR, mutate MIR, allocate IDs, or claim Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_port_sig_dto_loop_root_retire_rust_astnode_projector_candidate_guard.sh
```

Expected guard result:

```text
retire_candidate=RecipePortSigDtoSnapshotV1
covered_rows=6
deferred_rows=
recipe_port_sig_loop_root_parity_gate=green
programjson_runtime_parity_green=1
recipe_verifier_used=1
recipe_port_sig_snapshot_used=1
loop_root_children_supported=1
route_selection=0
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
full_recipe_matcher_execution=0
mir_mutation=0
id_allocation=0
backend_lowering=0
source_selfhost_claim=0
programjson_full_parser_claim=0
```

## Non-Claims

```text
runtime route switch
full ASTNode projector retirement
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-001
```
