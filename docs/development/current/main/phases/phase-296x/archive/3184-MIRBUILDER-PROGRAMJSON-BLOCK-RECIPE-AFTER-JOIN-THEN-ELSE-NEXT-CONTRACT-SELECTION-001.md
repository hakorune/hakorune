# 3184 - MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-AFTER-JOIN-THEN-ELSE-NEXT-CONTRACT-SELECTION-001

Status: landed

## Scope

Select the next ProgramJSON-fed block recipe contract after the scoped
JoinThenElse retire-candidate proof.

Selected:

```text
ProgramJsonExitAllowedBlockRecipeElseOnlyExitSnapshotV1
  input: ProgramJSON recipe_root
  reducer: ExitAllowedBlockRecipeBox
  first row:
    local_if_then_local_else_return_final_return_var
```

Held:

```text
ThenOnlyExit
ExitAll
RecipeBodies materialization
```

The selected row has reducer support already:
`ExitAllowedBlockRecipeBox` maps `IfThenLocalElseReturn` to `IfExitAllowed`
with the `ElseOnlyExit` mode. The missing boundary is the ProgramJSON
producer: `IfStmtHandler` must produce `If.then Seq([Local])` and
`If.else Exit(Return)` recipe_root rows.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_block_recipe_after_join_then_else_next_contract_selection_guard.sh
```

Expected guard result:

```text
selected_capability=ProgramJsonExitAllowedBlockRecipeElseOnlyExitSnapshotV1
selected_reducer=ExitAllowedBlockRecipeBox
selected_projection_tokens=IfThenLocalElseReturn
selected_reducer_outputs=IfExitAllowed,ElseOnlyExit
recipe_bodies_design_stop=1
route_selection=0
source_selfhost_claim=0
```

## Non-Claims

```text
ElseOnlyExit parity
ThenOnlyExit
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
MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-ELSE-ONLY-EXIT-SNAPSHOT-PARITY-001
```
