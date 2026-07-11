# 3190 - MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-AFTER-THEN-ONLY-EXIT-NEXT-CONTRACT-SELECTION-001

Status: landed

## Scope

Select the next ProgramJSON-fed block recipe contract after the scoped
ThenOnlyExit retire-candidate proof.

Selected:

```text
ProgramJsonExitAllowedBlockRecipeExitAllSnapshotV1
  input: ProgramJSON recipe_root
  reducer: ExitAllowedBlockRecipeBox
  first row:
    local_if_then_return_else_break_final_return_var
```

Held:

```text
RecipeBodies materialization
```

The selected row has reducer support already:
`ExitAllowedBlockRecipeBox` maps `IfThenReturnElseBreak` to `IfExitOnly` with
the `ExitAll` mode. The missing boundary is the ProgramJSON producer:
`IfStmtHandler` must produce `If.then Exit(Return)` and `If.else Exit(Break)`
recipe_root rows.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_block_recipe_after_then_only_exit_next_contract_selection_guard.sh
```

Expected guard result:

```text
selected_capability=ProgramJsonExitAllowedBlockRecipeExitAllSnapshotV1
selected_reducer=ExitAllowedBlockRecipeBox
selected_projection_tokens=IfThenReturnElseBreak
selected_reducer_outputs=IfExitOnly,ExitAll
recipe_bodies_design_stop=1
route_selection=0
source_selfhost_claim=0
```

## Non-Claims

```text
ExitAll parity
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
MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-EXIT-ALL-SNAPSHOT-PARITY-001
```
