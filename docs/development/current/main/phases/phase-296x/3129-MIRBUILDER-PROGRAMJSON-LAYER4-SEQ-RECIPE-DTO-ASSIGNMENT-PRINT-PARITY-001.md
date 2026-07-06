# 3129 - MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-ASSIGNMENT-PRINT-PARITY-001

Status: landed

## Scope

Expand `ProgramJsonSeqRecipeDtoSnapshotV1` parity after 3128 made the covered
top-level Assignment/Print PhaseState consumer rows runtime-green.

This card adds Assignment/Print rows to the existing Seq Recipe DTO parity
fixture and gate.  It remains a DTO snapshot parity card only.

## Covered Rows Added

```text
local_assignment_int_return_var
local_assignment_add_return_var
local_print_var_return_int
local_print_binary_return_int
```

The prior rows remain covered:

```text
return_int
return_new_box
local_return_var
empty_body_reject
```

## Implementation

```text
lang/src/compiler/mirbuilder/program_json_seq_recipe_dto_snapshot.hako
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-seq-recipe-dto-parity-v0.json
tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_seq_recipe_dto_parity_gate.sh
```

`ProgramJsonSeqRecipeDtoSnapshotV1` already handled Assignment and Print Var
state.  This card adds the missing `BinaryVarInt` print shape classification and
updates the fixture/gate to treat 3128 as the explicit prerequisite.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_seq_recipe_dto_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonSeqRecipeDtoSnapshotV1
programjson_traversal_used=1
structured_recipe_dto_constructed=1
mir_json_route_green=1
runtime_parity_green=1
runtime_route_switch=0
full_recipe_matcher_execution=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
```

## Non-Claims

```text
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
runtime route switch
ProgramJSON full parser
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-EXPANDED-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
