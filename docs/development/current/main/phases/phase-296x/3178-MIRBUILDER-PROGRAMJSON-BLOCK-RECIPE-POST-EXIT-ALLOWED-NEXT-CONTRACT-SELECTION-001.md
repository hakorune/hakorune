# 3178 - MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-POST-EXIT-ALLOWED-NEXT-CONTRACT-SELECTION-001

Status: landed

## Scope

Select the next ProgramJSON-fed block recipe contract after the scoped
ExitAllowed IfExitOnly retire-candidate proof.

Selected:

```text
ProgramJsonNoExitBlockRecipeLoopV0SnapshotV1
  input: ProgramJSON recipe_root
  reducer: NoExitBlockRecipeBox
  first row:
    local_loop_if_then_return_assignment_final_return_var
```

Held:

```text
ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1
ProgramJsonExitAllowedBlockRecipeThenElseModeSnapshotV1
```

The selected row uses the existing `LoopStmtHandler` producer for
`Loop.body = Seq([If.then Exit, Assignment])` and the already-proven
`IfThenReturnNoElse` body contract.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_block_recipe_post_exit_allowed_next_contract_selection_guard.sh
```

Expected guard result:

```text
selected_capability=ProgramJsonNoExitBlockRecipeLoopV0SnapshotV1
selected_reducer=NoExitBlockRecipeBox
selected_projection_tokens=LoopExitAllowedBody
selected_reducer_outputs=LoopV0,ExitAllowed
join_then_else_held=1
additional_exit_allowed_if_modes_held=1
route_selection=0
source_selfhost_claim=0
```

## Non-Claims

```text
LoopV0 parity
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
MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-LOOP-V0-SNAPSHOT-PARITY-001
```
