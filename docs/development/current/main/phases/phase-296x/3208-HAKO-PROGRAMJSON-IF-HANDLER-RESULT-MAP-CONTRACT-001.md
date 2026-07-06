# 3208 - HAKO-PROGRAMJSON-IF-HANDLER-RESULT-MAP-CONTRACT-001

Status: landed

## Scope

Close the `IfStmtHandler.handle/5` AOT return-shape cleanup before expanding
the next RecipeBodies arena contract.

The trigger was an AOT route that disliked a dynamic string return path through
`IfStmtHandler.handle/5`. The fix is not an AOT exception. The fix is to make
the `.hako` handler publish a clear total result-map contract.

## Implementation

- Annotate `IfStmtHandler` result-map helpers as `MapBox`.
- Annotate recipe item helpers as `MapBox`.
- Annotate `handle(...): MapBox`.
- Replace dynamic `err_line` rewrapping from child result maps with stable
  reason-token strings at the handler boundary.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_if_handler_result_map_contract_guard.sh
```

Companion probes for wider regression checks:

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_if_handler_then_local_no_else_capability_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_if_branch_multi_body_arena_parity_gate.sh
```

## Non-Claims

```text
aot_dynamic_string_return_widening=0
by_name_aot_exception=0
programjson_new_shape=0
recipe_bodies_materialization=0
runtime_recipe_bodies_arena=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
runtime_route_switch=0
source_selfhost_claim=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-LOOP-BODY-ARENA-NEXT-CONTRACT-SELECTION-001
```
