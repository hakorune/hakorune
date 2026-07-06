# 3166 - MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-LOOP-NO-EXIT-SNAPSHOT-PARITY-001

Status: landed

## Scope

Extend `ProgramJsonStmtOnlyBlockRecipeSnapshotV1` with a one-row LoopNoExit
projection into the existing HakoAdopted `StmtOnlyBlockRecipeBox` reducer.

Covered row:

```text
local_loop_no_exit
```

Expected summary:

```text
snapshot_kind=ProgramJsonStmtOnlyBlockRecipeSnapshotV1;err=0;accepted=1;block_contract=StmtOnly;stmt_count=2;stmt_kinds=Local,Loop
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_stmt_only_block_recipe_loop_no_exit_snapshot_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonStmtOnlyBlockRecipeSnapshotV1
row_count=1
programjson_traversal_used=1
recipe_root_traversal_used=1
stmt_only_reducer_called=1
loop_no_exit_token_projected=1
prebuilt_token_snapshot_input=0
string_only_facade=0
mir_json_route_green=1
runtime_parity_green=1
source_selfhost_claim=0
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
MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-LOOP-NO-EXIT-SNAPSHOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
