# 3209 - HAKO-PROGRAMJSON-LOOP-HANDLER-RESULT-MAP-CONTRACT-001

Status: landed

## Scope

Stabilize `LoopStmtHandler.handle/5` before the next RecipeBodies expansion.

This follows the 3208 If handler cleanup. The fix is not an AOT route
exception. The `.hako` handler must publish a clear total result-map contract
and must not rewrap child `err_line` values through dynamic string returns.

## Implementation

- Annotate `LoopStmtHandler` result-map helpers as `MapBox`.
- Annotate `handle_state_values(...): MapBox` and `handle(...): MapBox`.
- Replace the child Return handler dynamic `err_line` rewrap with a stable
  Loop boundary reason token.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_loop_handler_result_map_contract_guard.sh
```

Companion probes:

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_loop_recipe_dto_expanded_return_parity_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_exit_recipe_dto_loop_body_parity_gate.sh
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
HAKO-PROGRAMJSON-RECIPEBODIES-ARRAY-HELPER-TOTAL-MAP-CONTRACT-001
```
