# 3183 - MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-JOIN-THEN-ELSE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark the covered `ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1` row from
3182 as a scoped Rust ASTNode projector retire-candidate.

Covered row:

```text
local_if_then_local_else_print_final_return_var
```

Deferred rows:

```text
additional_exit_allowed_if_modes
recipe_bodies_materialization
```

This is a proof checkpoint only. It does not switch the runtime route away from
Rust, remove the bootstrap oracle, execute RecipeMatcher, materialize
RecipeBodies, select backend routes, lower MIR, mutate MIR, allocate IDs, or
claim Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_no_exit_block_recipe_join_then_else_retire_rust_astnode_projector_candidate_guard.sh
```

Expected guard result:

```text
retire_candidate=ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1
covered_rows=1
programjson_join_then_else_snapshot_parity_gate=green
recipe_root_traversal_used=1
if_stmt_handler_producer_extended=1
no_exit_reducer_called=1
join_then_else_token_projected=1
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
```

## Non-Claims

```text
additional ExitAllowed if-modes
RecipeBodies materialization
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
MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-AFTER-JOIN-THEN-ELSE-NEXT-CONTRACT-SELECTION-001
```
