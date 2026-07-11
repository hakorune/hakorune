# 3173 - MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-IF-JOIN-SNAPSHOT-PARITY-001

Status: landed

## Scope

Implement `ProgramJsonNoExitBlockRecipeIfJoinSnapshotV1` as the first
ProgramJSON-fed recursive block recipe contract capability after the StmtOnly
bridge.

Covered row:

```text
if_then_local_no_else
```

The owner traverses ProgramJSON through `ProgramJsonV0PhaseStateBox`, reads the
`recipe_root`, projects the parseable If row to `IfThenLocalNoElse`, and feeds
that token to the existing `NoExitBlockRecipeBox` reducer.

Held:

```text
JoinThenElse
ExitAllowed
LoopV0
RecipeBodies materialization
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_no_exit_block_recipe_if_join_snapshot_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonNoExitBlockRecipeIfJoinSnapshotV1
row_count=1
programjson_traversal_used=1
recipe_root_traversal_used=1
no_exit_reducer_called=1
if_join_token_projected=1
mir_json_route_green=1
runtime_parity_green=1
exit_allowed_contract=0
join_then_else_contract=0
loop_v0_contract=0
route_selection=0
source_selfhost_claim=0
```

## Non-Claims

```text
JoinThenElse contract
ExitAllowed contract
LoopV0 contract
RecipeBodies materialization
full RecipeMatcher execution
runtime route switch
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-IF-JOIN-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
