# 3117 - HAKO-AOT-PROGRAMJSON-RECIPE-FACTS-NAME-ARRAY-LITERAL-CONTRACT-001

Status: green

## Scope

Remove the `RecipeFactsBox._push_name/2` AOT helper boundary from the
ProgramJSON PhaseState parse path.

The statement facts currently carry at most one name per `defs`, `updates`, or
`uses` slot. Build those arrays as literals inside `RecipeFactsBox` instead of
calling a side-effect helper that mutates an ArrayBox.

## Implementation

- Added `_new_facts_with/4` to construct the facts map from explicit arrays.
- Replaced `_push_name` calls with literal name arrays for Print, Local,
  Assignment, and Return facts.
- Removed `_contains_name/2` and `_push_name/2`.
- Kept RecipeFacts output as data-only `MapBox` / RecipeItem payloads.

## Evidence

```bash
bash tools/checks/hako_aot_programjson_phase_state_consume_stmt_non_control_result_contract_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_phase_state_aot_call_blocker_guard.sh
bash tools/bin/hako --backend mir --verify lang/src/compiler/mirbuilder/recipe/recipe_facts_box.hako
```

## Result

```text
recipe_facts_name_arrays_use_literals = 1
recipe_facts_push_name_helper_removed = 1
current_first_blocker = ProgramJsonV0PhaseStateBox.parse/2
current_first_blocker_reason = missing_multi_function_emitter
```

Next selected card:

```text
HAKO-AOT-PROGRAMJSON-PHASE-STATE-PARSE-AOT-CALL-READINESS-001
```

## Non-Claims

```text
phase_state_parse_aot_call_fixed = 0
layer4_recipe_dto_parity_green = 0
source_selfhost_claim = 0
mir_mutation = 0
id_allocation = 0
backend_lowering_claim = 0
route_selection_migration = 0
runtime_route_switch = 0
new_backend_route = 0
new_abi = 0
```
