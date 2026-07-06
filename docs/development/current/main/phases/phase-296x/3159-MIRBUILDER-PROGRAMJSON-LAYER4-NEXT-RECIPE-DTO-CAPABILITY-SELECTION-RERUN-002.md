# 3159 - MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-RERUN-002

Status: landed

## Scope

Select the next concrete ProgramJSON Layer4 Recipe DTO capability after the
covered `SeqRecipeDtoSnapshotV1` loop-root rows were marked as a scoped Rust
ASTNode projector retire-candidate.

This is a selection card only. It does not implement a new `.hako` owner,
switch runtime routes, execute RecipeMatcher, select backend routes, lower MIR,
mutate MIR, allocate IDs, or claim Source Selfhost.

## Selected Capability

```text
ProgramJsonRecipePortSigLoopRootV1
```

Next card:

```text
MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-PORT-SIG-DTO-LOOP-ROOT-PARITY-001
```

Reason:

```text
RecipePortSigDtoSnapshotV1 already has base Layer4 parity, but its loop-root
rows were deferred behind Seq DTO root-child coverage. SeqRecipeDtoSnapshotV1
now proves the covered Local>Loop>Return rows, so the next metadata boundary is
RecipeVerifierBox.verify/2 -> RecipePortSigBox.snapshot/1 over the same
structured recipe_root.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_next_recipe_dto_capability_selection_rerun_002_guard.sh
```

Expected guard result:

```text
selected_capability=ProgramJsonRecipePortSigLoopRootV1
selected_next_card=MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-PORT-SIG-DTO-LOOP-ROOT-PARITY-001
source_rows=6
expected_port_sig=def_count=1;update_count=2
must_construct_structured_recipe_dto=1
must_use_recipe_verifier=1
must_use_recipe_port_sig_snapshot=1
implementation_done=0
parity_gate_green=0
source_selfhost_claim=0
```

## Non-Claims

```text
new .hako implementation
parity gate green
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
MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-PORT-SIG-DTO-LOOP-ROOT-PARITY-001
```
