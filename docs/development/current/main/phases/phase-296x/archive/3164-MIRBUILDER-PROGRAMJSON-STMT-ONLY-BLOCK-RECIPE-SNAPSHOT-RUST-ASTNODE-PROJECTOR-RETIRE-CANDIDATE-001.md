# 3164 - MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-SNAPSHOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark the covered `ProgramJsonStmtOnlyBlockRecipeSnapshotV1` rows from 3163 as
a scoped Rust ASTNode projector retire-candidate.

Covered rows:

```text
local_print
local_assignment
local_return_reject
return_only_reject
```

Deferred rows:

```text
if_no_exit
loop_no_exit
no_exit_block
exit_allowed_block
```

This is a proof checkpoint only. It does not switch the runtime route away from
Rust, remove the bootstrap oracle, execute RecipeMatcher, materialize
RecipeBodies, select backend routes, lower MIR, mutate MIR, allocate IDs, or
claim Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_stmt_only_block_recipe_snapshot_retire_rust_astnode_projector_candidate_guard.sh
```

Expected guard result:

```text
retire_candidate=ProgramJsonStmtOnlyBlockRecipeSnapshotV1
covered_rows=4
deferred_rows=if_no_exit,loop_no_exit,no_exit_block,exit_allowed_block
programjson_stmt_only_block_recipe_snapshot_parity_gate=green
programjson_runtime_parity_green=1
recipe_root_traversal_used=1
stmt_only_reducer_called=1
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
MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-NEXT-CAPABILITY-SELECTION-001
```
