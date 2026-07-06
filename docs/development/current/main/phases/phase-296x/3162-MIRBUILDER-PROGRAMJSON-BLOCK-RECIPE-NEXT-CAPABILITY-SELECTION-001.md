# 3162 - MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-NEXT-CAPABILITY-SELECTION-001

Status: landed

## Scope

Select the next concrete ProgramJSON-fed block recipe capability after the
covered `RecipePortSigDtoSnapshotV1` loop-root rows were marked as a scoped
Rust ASTNode projector retire-candidate.

This is a selection card only. It does not implement the bridge, switch runtime
routes, execute RecipeMatcher, materialize RecipeBodies, select backend routes,
lower MIR, mutate MIR, allocate IDs, or claim Source Selfhost.

## Selected Capability

```text
ProgramJsonStmtOnlyBlockRecipeSnapshotV1
```

Next card:

```text
MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-SNAPSHOT-PARITY-001
```

Reason:

```text
StmtOnlyBlockRecipeBox is already HakoAdopted for backend-safe token snapshots.
The next small movement is to feed that reducer from ProgramJSON/recipe_root
instead of a prebuilt token snapshot.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_block_recipe_next_capability_selection_guard.sh
```

Expected guard result:

```text
selected_capability=ProgramJsonStmtOnlyBlockRecipeSnapshotV1
selected_next_card=MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-SNAPSHOT-PARITY-001
source_rows=4
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
MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-SNAPSHOT-PARITY-001
```
