# 3163 - MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-SNAPSHOT-PARITY-001

Status: landed

## Scope

Add `ProgramJsonStmtOnlyBlockRecipeSnapshotV1` as a ProgramJSON-fed bridge into
the existing HakoAdopted `StmtOnlyBlockRecipeBox` backend-safe token reducer.

This slice consumes ProgramJSON through `ProgramJsonV0PhaseStateBox.parse/2`,
reads the structured `recipe_root` Seq children, projects direct stmt items to
backend-safe tokens, and calls `StmtOnlyBlockRecipeBox.build_summary/2`.

Covered rows:

```text
local_print
local_assignment
local_return_reject
return_only_reject
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_stmt_only_block_recipe_snapshot_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonStmtOnlyBlockRecipeSnapshotV1
row_count=4
programjson_traversal_used=1
recipe_root_traversal_used=1
stmt_only_reducer_called=1
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
MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-SNAPSHOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
