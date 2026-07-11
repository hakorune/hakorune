# 3181 - MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-POST-LOOP-V0-NEXT-CONTRACT-SELECTION-001

Status: landed

## Scope

Select the next ProgramJSON-fed block recipe contract after the scoped LoopV0
retire-candidate proof.

Selected:

```text
ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1
  input: ProgramJSON recipe_root
  reducer: NoExitBlockRecipeBox
  first row:
    local_if_then_local_else_print_final_return_var
```

Held:

```text
ProgramJsonExitAllowedBlockRecipeThenElseModeSnapshotV1
```

The selected row has reducer support already:
`NoExitBlockRecipeBox` maps `IfThenLocalElsePrint` to `IfJoin` with the
`JoinThenElse` shape. The missing boundary is the ProgramJSON producer:
`IfStmtHandler` must produce `If.then Seq([Local])` and
`If.else Seq([Print])` recipe_root rows.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_block_recipe_post_loop_v0_next_contract_selection_guard.sh
```

Expected guard result:

```text
selected_capability=ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1
selected_reducer=NoExitBlockRecipeBox
selected_projection_tokens=IfThenLocalElsePrint
selected_reducer_outputs=IfJoin,JoinThenElse
additional_exit_allowed_if_modes_held=1
route_selection=0
source_selfhost_claim=0
```

## Non-Claims

```text
JoinThenElse parity
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
MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-JOIN-THEN-ELSE-SNAPSHOT-PARITY-001
```
