# 3176 - MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-IF-EXIT-ONLY-SNAPSHOT-PARITY-001

Status: landed

## Scope

Implement `ProgramJsonExitAllowedBlockRecipeIfExitOnlySnapshotV1` as the first
ProgramJSON-fed `ExitAllowedBlockRecipeBox` contract capability.

Covered row:

```text
if_then_return_no_else
```

The owner traverses ProgramJSON through `ProgramJsonV0PhaseStateBox`, reads the
`recipe_root`, projects the parseable If row to `IfThenReturnNoElse`, and feeds
that token to the existing `ExitAllowedBlockRecipeBox` reducer.

Held:

```text
JoinThenElse
LoopV0
RecipeBodies materialization
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_exit_allowed_block_recipe_if_exit_only_snapshot_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonExitAllowedBlockRecipeIfExitOnlySnapshotV1
row_count=1
programjson_traversal_used=1
recipe_root_traversal_used=1
exit_allowed_reducer_called=1
if_exit_only_token_projected=1
mir_json_route_green=1
runtime_parity_green=1
join_then_else_contract=0
loop_v0_contract=0
route_selection=0
source_selfhost_claim=0
```

## Non-Claims

```text
JoinThenElse contract
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
MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-IF-EXIT-ONLY-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
