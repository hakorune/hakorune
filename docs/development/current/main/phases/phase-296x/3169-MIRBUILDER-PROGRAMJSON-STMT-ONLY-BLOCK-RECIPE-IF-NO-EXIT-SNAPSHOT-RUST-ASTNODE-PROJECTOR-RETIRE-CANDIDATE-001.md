# 3169 - MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-IF-NO-EXIT-SNAPSHOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark the covered `ProgramJsonStmtOnlyBlockRecipeIfNoExitSnapshotV1` row from
3168 as a scoped Rust ASTNode projector retire-candidate.

Covered row:

```text
local_if_assignment_no_exit
```

Deferred rows:

```text
then_local_no_else_if
no_exit_block
exit_allowed_block
```

This is a proof checkpoint only. It does not switch the runtime route away from
Rust, remove the bootstrap oracle, execute RecipeMatcher, materialize
RecipeBodies, select backend routes, lower MIR, mutate MIR, allocate IDs, or
claim Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_stmt_only_block_recipe_if_no_exit_snapshot_retire_rust_astnode_projector_candidate_guard.sh
```

Expected guard result:

```text
retire_candidate=ProgramJsonStmtOnlyBlockRecipeIfNoExitSnapshotV1
covered_rows=1
deferred_rows=then_local_no_else_if,no_exit_block,exit_allowed_block
programjson_if_no_exit_snapshot_parity_gate=green
programjson_runtime_parity_green=1
recipe_root_traversal_used=1
stmt_only_reducer_called=1
if_no_exit_token_projected=1
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
recipe_bodies_materialization=0
route_selection=0
source_selfhost_claim=0
programjson_full_parser_claim=0
```

## Non-Claims

```text
then-local/no-else If support
NoExit block contract
ExitAllowed block contract
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
MIRBUILDER-PROGRAMJSON-IF-HANDLER-THEN-LOCAL-NO-ELSE-CAPABILITY-001
```
