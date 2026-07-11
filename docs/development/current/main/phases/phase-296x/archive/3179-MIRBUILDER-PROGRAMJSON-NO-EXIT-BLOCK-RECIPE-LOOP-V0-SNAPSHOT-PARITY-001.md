# 3179 - MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-LOOP-V0-SNAPSHOT-PARITY-001

Status: landed

## Scope

Implement `ProgramJsonNoExitBlockRecipeLoopV0SnapshotV1` as a ProgramJSON-fed
`NoExitBlockRecipeBox` LoopV0 contract capability.

Covered row:

```text
local_loop_if_then_return_assignment_final_return_var
```

The owner traverses ProgramJSON through `ProgramJsonV0PhaseStateBox`, reads the
`recipe_root`, recognizes `Loop.body = Seq([If.then Exit, Assignment])`, projects
the loop to `LoopExitAllowedBody`, and feeds that token to the existing
`NoExitBlockRecipeBox` reducer.

Held:

```text
JoinThenElse
additional ExitAllowed if-modes
RecipeBodies materialization
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_no_exit_block_recipe_loop_v0_snapshot_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonNoExitBlockRecipeLoopV0SnapshotV1
row_count=1
programjson_traversal_used=1
recipe_root_traversal_used=1
no_exit_reducer_called=1
loop_v0_token_projected=1
exit_allowed_body_shape_observed=1
mir_json_route_green=1
runtime_parity_green=1
route_selection=0
source_selfhost_claim=0
```

## Non-Claims

```text
JoinThenElse contract
additional ExitAllowed if-modes
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
MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-LOOP-V0-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
