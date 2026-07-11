# 3168 - MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-IF-NO-EXIT-SNAPSHOT-PARITY-001

Status: landed

## Scope

Extend `ProgramJsonStmtOnlyBlockRecipeSnapshotV1` with a one-row IfNoExit
projection for the parseable then/else assignment If shape. The projection
routes the `IfNoExit` token through the existing HakoAdopted
`StmtOnlyBlockRecipeBox` reducer.

Covered row:

```text
local_if_assignment_no_exit
```

Expected summary:

```text
snapshot_kind=ProgramJsonStmtOnlyBlockRecipeSnapshotV1;err=0;accepted=1;block_contract=StmtOnly;stmt_count=2;stmt_kinds=Local,If
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_stmt_only_block_recipe_if_no_exit_snapshot_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonStmtOnlyBlockRecipeSnapshotV1
row_count=1
programjson_traversal_used=1
recipe_root_traversal_used=1
stmt_only_reducer_called=1
if_no_exit_token_projected=1
prebuilt_token_snapshot_input=0
string_only_facade=0
mir_json_route_green=1
runtime_parity_green=1
then_local_no_else_if=0
no_exit_block_contract=0
exit_allowed_block_contract=0
source_selfhost_claim=0
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
MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-IF-NO-EXIT-SNAPSHOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
