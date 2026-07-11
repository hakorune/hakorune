# 3131 - MIRBUILDER-PROGRAMJSON-LAYER4-EXIT-RECIPE-DTO-PARITY-001

Status: landed

## Scope

Add `ProgramJsonExitRecipeDtoSnapshotV1` as the next concrete Layer4 ProgramJSON
Recipe DTO capability after the expanded Seq retire-candidate checkpoint.

This owner consumes `ProgramJsonV0PhaseStateBox.parse/2`, reads `recipe_root`,
and extracts the covered `If.then_item = Exit` DTO payload.  It does not execute
RecipeMatcher, select routes, lower MIR, mutate MIR, or allocate IDs.

## Covered Rows

```text
local_if_then_return_int_final_return_int
local_if_then_return_int_final_return_var
local_if_then_else_assignment_no_exit_reject
```

Loop-body Exit remains explicitly deferred because that route currently reaches
`parse_error` for this DTO path.

## Implementation

```text
lang/src/compiler/mirbuilder/program_json_exit_recipe_dto_snapshot.hako
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-exit-recipe-dto-parity-v0.json
tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_exit_recipe_dto_parity_gate.sh
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_exit_recipe_dto_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonExitRecipeDtoSnapshotV1
programjson_traversal_used=1
structured_recipe_dto_constructed=1
mir_json_route_green=1
runtime_parity_green=1
loop_exit_dto_green=0
runtime_route_switch=0
full_recipe_matcher_execution=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
```

## Non-Claims

```text
loop-body Exit DTO
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
MIRBUILDER-PROGRAMJSON-LAYER4-EXIT-RECIPE-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
