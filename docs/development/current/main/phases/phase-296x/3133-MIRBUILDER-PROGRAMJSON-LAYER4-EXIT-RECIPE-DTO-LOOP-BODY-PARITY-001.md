# 3133 - MIRBUILDER-PROGRAMJSON-LAYER4-EXIT-RECIPE-DTO-LOOP-BODY-PARITY-001

Status: landed

## Scope

Promote the previously deferred loop-body Exit Recipe DTO row for
`ProgramJsonExitRecipeDtoSnapshotV1`.

This is a Layer4 ProgramJSON Recipe DTO parity expansion only.  It fixes the
covered Loop body path to use stable token equality for dynamic scanner strings
and proves the resulting `Loop.body[0].If.then_item=Exit` snapshot at MIR JSON
and AOT runtime.

## Changed

```text
LoopStmtHandler dynamic string equality:
  op / body separator / body_kind checks now use BoxHelpers.same_token

Promoted row:
  local_loop_if_then_return_int_assignment_final_return_var
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_exit_recipe_dto_loop_body_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonExitRecipeDtoSnapshotV1
programjson_traversal_used=1
structured_recipe_dto_constructed=1
mir_json_route_green=1
runtime_parity_green=1
loop_exit_dto_green=1
legacy_root_exit_parity_guard_still_green=1
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
MIRBUILDER-PROGRAMJSON-LAYER4-EXIT-RECIPE-DTO-EXPANDED-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
