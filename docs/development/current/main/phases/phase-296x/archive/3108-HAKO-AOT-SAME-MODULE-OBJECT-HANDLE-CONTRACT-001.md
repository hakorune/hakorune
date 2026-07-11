# 3108 - HAKO-AOT-SAME-MODULE-OBJECT-HANDLE-CONTRACT-001

Status: landed

## Scope

Fix the first AOT route metadata contract gap found while opening
`HAKO-AOT-PROGRAMJSON-PHASE-STATE-PARSE-CALL-CONTRACT-001`.

Same-module helper bodies that return collection handles through `NewBox`,
collection `birth()`, or a proven object-handle child route now publish a
uniform MIR object-handle contract instead of falling through to
`missing_multi_function_emitter`.

## Evidence

```bash
bash tools/checks/hako_aot_same_module_object_handle_contract_guard.sh
```

Expected green routes:

```text
RecipeItemBox._array_or_empty/1 -> object_handle / DirectAbi
RecipeItemBox.seq/1             -> map_handle / DirectAbi
```

## Decision

```text
selected_next_card:
  HAKO-AOT-PROGRAMJSON-PHASE-STATE-DEEP-OBJECT-HELPER-CALL-CONTRACT-001

resume_after_green:
  MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001
```

## Remaining Blocker

`ProgramJsonV0PhaseStateBox.parse/2` is still not fixed. The next blockers are
deeper object-helper calls under phase-state parse, including
`ProgramJsonV0RuneAttrsBox.read_function_runes_map/2` and
`ProgramJsonV0PhaseStateBox._scan_body_rec/8`.

## Non-Claims

```text
phase_state_parse_aot_call_fixed = 0
layer4_recipe_dto_parity_green = 0
source_selfhost_claim = 0
mir_mutation = 0
id_allocation = 0
backend_lowering_claim = 0
new_backend_route = 0
new_abi = 0
```
