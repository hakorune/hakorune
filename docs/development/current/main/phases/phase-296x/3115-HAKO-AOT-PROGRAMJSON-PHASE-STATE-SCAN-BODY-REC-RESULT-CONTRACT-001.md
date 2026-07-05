# 3115 - HAKO-AOT-PROGRAMJSON-PHASE-STATE-SCAN-BODY-REC-RESULT-CONTRACT-001

Status: green

## Scope

Close the PhaseState scan-body AOT route-contract blocker exposed after the
RecipeVerifier / RecipePortSig cleanup.

This card claims only the scan-body helper-call blocker is removed from
`ProgramJsonV0PhaseStateBox.parse/2`. Full PhaseState parse remains unclaimed
because RuneAttrs function-name normalization is still blocked.

## Implementation

- Moved the scan-body loop into `parse/2`.
- Retired the `_scan_body_rec` helper call from the AOT path.
- Kept the scan body loop local so AOT no longer needs a string-heavy
  MapBox-returning same-module helper call.
- Kept MIR mutation, route selection, lowering, ID allocation, and new ABI out
  of scope.

## Evidence

```bash
bash tools/checks/hako_aot_programjson_phase_state_scan_body_rec_result_contract_guard.sh
```

## Remaining Blocker

```text
ProgramJsonV0RuneAttrsBox function-name normalization
```

Next selected card:

```text
HAKO-AOT-PROGRAMJSON-RUNE-ATTRS-FUNCTION-NAME-NORMALIZATION-RESULT-CONTRACT-001
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
