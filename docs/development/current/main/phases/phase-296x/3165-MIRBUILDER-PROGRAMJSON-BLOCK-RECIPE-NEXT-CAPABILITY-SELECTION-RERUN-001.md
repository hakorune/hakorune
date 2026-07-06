# 3165 - MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-NEXT-CAPABILITY-SELECTION-RERUN-001

Status: landed

## Scope

Select the next ProgramJSON-fed block recipe capability after the direct-stmt
`ProgramJsonStmtOnlyBlockRecipeSnapshotV1` rows were marked as a scoped Rust
ASTNode projector retire-candidate.

This is a selection card only. It does not implement the LoopNoExit projection,
switch runtime routes, execute RecipeMatcher, materialize RecipeBodies, select
backend routes, lower MIR, mutate MIR, allocate IDs, or claim Source Selfhost.

## Selected Capability

```text
ProgramJsonStmtOnlyBlockRecipeLoopNoExitSnapshotV1
```

Next card:

```text
MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-LOOP-NO-EXIT-SNAPSHOT-PARITY-001
```

Reason:

```text
ProgramJSON Loop rows already produce structured recipe_root and
RecipeStmtSeqDtoSnapshotV1 observes Local>Loop. The existing
StmtOnlyBlockRecipeBox reducer already accepts LoopNoExit, so the smallest next
bridge is a one-row LoopNoExit projection.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_block_recipe_next_capability_selection_rerun_001_guard.sh
```

Expected guard result:

```text
selected_capability=ProgramJsonStmtOnlyBlockRecipeLoopNoExitSnapshotV1
selected_next_card=MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-LOOP-NO-EXIT-SNAPSHOT-PARITY-001
source_rows=local_loop_no_exit
must_traverse_recipe_root=1
must_call_existing_stmt_only_reducer=1
implementation_done=0
parity_gate_green=0
source_selfhost_claim=0
```

## Non-Claims

```text
implementation done
parity gate green
RecipeBodies materialization
runtime route switch
full ASTNode projector retirement
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-LOOP-NO-EXIT-SNAPSHOT-PARITY-001
```
