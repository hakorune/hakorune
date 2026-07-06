# 3175 - MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-NEXT-CONTRACT-CAPABILITY-SELECTION-001

Status: landed

## Scope

Select the next ProgramJSON-fed block recipe contract after the scoped NoExit
IfJoin retire-candidate proof.

Selected:

```text
ProgramJsonExitAllowedBlockRecipeIfExitOnlySnapshotV1
  input: ProgramJSON recipe_root
  reducer: ExitAllowedBlockRecipeBox
  first row:
    if_then_return_no_else
```

Held:

```text
ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1
ProgramJsonNoExitBlockRecipeLoopV0SnapshotV1
```

The selected row uses an existing PhaseState producer: parseable If rows with
then Return and no else already become `RecipeItemBox.If` with `then_item=Exit`
and `else_item=Seq([])`.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_block_recipe_next_contract_capability_selection_guard.sh
```

Expected guard result:

```text
selected_capability=ProgramJsonExitAllowedBlockRecipeIfExitOnlySnapshotV1
selected_reducer=ExitAllowedBlockRecipeBox
selected_projection_tokens=IfThenReturnNoElse
selected_reducer_outputs=IfExitOnly,ExitIf
join_then_else_held=1
loop_v0_held=1
route_selection=0
source_selfhost_claim=0
```

## Non-Claims

```text
ExitAllowed parity
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
MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-IF-EXIT-ONLY-SNAPSHOT-PARITY-001
```
