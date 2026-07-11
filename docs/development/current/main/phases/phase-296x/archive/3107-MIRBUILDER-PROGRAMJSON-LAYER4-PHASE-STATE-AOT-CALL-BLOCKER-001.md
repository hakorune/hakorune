# 3107 - MIRBUILDER-PROGRAMJSON-LAYER4-PHASE-STATE-AOT-CALL-BLOCKER-001

Status: landed

## Scope

Record the first blocker found while opening
`MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001`.

The selected Layer4 pilot is still `ProgramJsonLoopRecipeDtoPilotV1`, but the
AOT gate cannot yet call `ProgramJsonV0PhaseStateBox.parse/2` from an imported
`.hako` app.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_phase_state_aot_call_blocker_guard.sh
```

Expected blocker:

```text
callee_symbol=ProgramJsonV0PhaseStateBox.parse/2
owner_hint=backend_lowering
reason=module_generic_prepass_failed
first_op=mir_call
```

## Decision

```text
selected_next_card:
  HAKO-AOT-PROGRAMJSON-PHASE-STATE-PARSE-CALL-CONTRACT-001

resume_after_green:
  MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001
```

## Non-Claims

```text
layer4_recipe_dto_parity_green = 0
phase_state_aot_call_fixed = 0
source_selfhost_claim = 0
mir_mutation = 0
id_allocation = 0
backend_lowering_claim = 0
route_selection_migration = 0
runtime_route_switch = 0
new_backend_route = 0
new_abi = 0
```
