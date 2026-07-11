# 3147 - MIRBUILDER-PROGRAMJSON-LAYER4-IF-RECIPE-DTO-EXPANDED-RETURN-PARITY-001

Status: landed

## Scope

Expand `ProgramJsonIfRecipeDtoSnapshotV1` with `then_item=Exit` payload rows
for the expanded Return-only vocabulary already proven by Exit DTO:

```text
NewBoxStringAbc
CallId0
CallId1Int9
CallId1Int7
MethodStringBoxLengthAbc
MethodStringBoxIndexOfBAbc
```

This is parent If DTO parity only. It does not switch runtime routes, execute
RecipeMatcher, select backend routes, lower MIR, mutate MIR, allocate IDs, or
claim Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_if_recipe_dto_expanded_return_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonIfRecipeDtoSnapshotV1
expanded_rows=6
programjson_traversal_used=1
structured_recipe_dto_constructed=1
mir_json_route_green=1
runtime_parity_green=1
legacy_if_parity_guard_still_green=1
expanded_exit_payload_prerequisite_green=1
runtime_route_switch=0
full_recipe_matcher_execution=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
```

## Non-Claims

```text
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
MIRBUILDER-PROGRAMJSON-LAYER4-IF-RECIPE-DTO-EXPANDED-RETURN-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
