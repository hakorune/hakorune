# 3187 - MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-AFTER-ELSE-ONLY-EXIT-NEXT-CONTRACT-SELECTION-001

Status: landed

## Scope

Select the next ProgramJSON-fed block recipe contract after the scoped
ElseOnlyExit retire-candidate proof.

Selected:

```text
ProgramJsonExitAllowedBlockRecipeThenOnlyExitSnapshotV1
  input: ProgramJSON recipe_root
  reducer: ExitAllowedBlockRecipeBox
  first row:
    local_if_then_return_else_local_final_return_var
```

Held:

```text
ExitAll
RecipeBodies materialization
```

The selected row has reducer support already:
`ExitAllowedBlockRecipeBox` maps `IfThenReturnElseLocal` to `IfExitAllowed`
with the `ThenOnlyExit` mode. The missing boundary is the ProgramJSON producer:
`IfStmtHandler` must produce `If.then Exit(Return)` and
`If.else Seq([Local])` recipe_root rows.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_block_recipe_after_else_only_exit_next_contract_selection_guard.sh
```

Expected guard result:

```text
selected_capability=ProgramJsonExitAllowedBlockRecipeThenOnlyExitSnapshotV1
selected_reducer=ExitAllowedBlockRecipeBox
selected_projection_tokens=IfThenReturnElseLocal
selected_reducer_outputs=IfExitAllowed,ThenOnlyExit
recipe_bodies_design_stop=1
route_selection=0
source_selfhost_claim=0
```

## Non-Claims

```text
ThenOnlyExit parity
ExitAll
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
MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-THEN-ONLY-EXIT-SNAPSHOT-PARITY-001
```
