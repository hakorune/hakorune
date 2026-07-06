# 3156 - MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-RERUN-001

Status: landed

## Scope

Select the next concrete ProgramJSON Layer4 Recipe DTO capability after the
covered `RecipeShapeKindDtoSnapshotV1` loop-root rows were marked as a scoped
Rust ASTNode projector retire-candidate.

This is a selection card only. It does not implement the parity gate, switch
runtime routes, execute RecipeMatcher, select backend routes, lower MIR, mutate
MIR, allocate IDs, or claim Source Selfhost.

## Selected Capability

```text
ProgramJsonSeqRecipeDtoLoopRootV1
```

Next card:

```text
MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-LOOP-ROOT-PARITY-001
```

Reason:

```text
SeqRecipeDtoSnapshotV1 is the remaining parent DTO still limited to stmt-only
root sequence children. 3151 and 3154 already proved Local>Loop>Return root
sequence and shape_kind parity in sibling parent DTOs.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_next_recipe_dto_capability_selection_rerun_001_guard.sh
```

Expected guard result:

```text
selected_capability=ProgramJsonSeqRecipeDtoLoopRootV1
selected_next_card=MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-LOOP-ROOT-PARITY-001
source_rows=6
expected_seq_sig=Local>Loop>Return
must_construct_structured_recipe_dto=1
must_use_root_sequence_scanner=1
implementation_done=0
parity_gate_green=0
source_selfhost_claim=0
```

## Non-Claims

```text
implementation done
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
MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-LOOP-ROOT-PARITY-001
```
