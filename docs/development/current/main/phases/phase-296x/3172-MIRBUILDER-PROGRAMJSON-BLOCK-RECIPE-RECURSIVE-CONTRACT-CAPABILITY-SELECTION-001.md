# 3172 - MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-RECURSIVE-CONTRACT-CAPABILITY-SELECTION-001

Status: landed

## Scope

Select the next ProgramJSON-fed Layer4 block recipe contract after the
StmtOnly bridge and then-local/no-else If retire-candidate proof.

Selected:

```text
ProgramJsonNoExitBlockRecipeIfJoinSnapshotV1
  input: ProgramJSON recipe_root
  reducer: NoExitBlockRecipeBox
  first rows:
    if_then_local_no_else
    if_then_local_else_print
```

Held:

```text
ProgramJsonExitAllowedBlockRecipeSnapshotV1
ProgramJsonNoExitBlockRecipeLoopV0SnapshotV1
```

The selection intentionally enters the NoExit block contract first because it
exercises recursive block structure through IfJoin while avoiding explicit
non-local exits, LoopV0 handoff, RecipeBodies materialization, route selection,
MIR mutation/lowering, ID allocation, runtime route switching, and Source
Selfhost claims.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_block_recipe_recursive_contract_capability_selection_guard.sh
```

Expected guard result:

```text
selected_capability=ProgramJsonNoExitBlockRecipeIfJoinSnapshotV1
selected_reducer=NoExitBlockRecipeBox
held_capability=ProgramJsonExitAllowedBlockRecipeSnapshotV1
no_exit_if_join_rows=2
exit_allowed_held=1
recursive_recipe_bodies_materialization=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering=0
source_selfhost_claim=0
```

## Non-Claims

```text
ProgramJSON NoExit IfJoin parity
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
MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-IF-JOIN-SNAPSHOT-PARITY-001
```
