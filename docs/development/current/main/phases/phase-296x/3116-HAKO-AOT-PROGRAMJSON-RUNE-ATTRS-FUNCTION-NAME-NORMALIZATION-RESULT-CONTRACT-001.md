# 3116 - HAKO-AOT-PROGRAMJSON-RUNE-ATTRS-FUNCTION-NAME-NORMALIZATION-RESULT-CONTRACT-001

Status: green

## Scope

Close the RuneAttrs function-name normalization AOT route-contract blocker.

This card claims only `ProgramJsonV0RuneAttrsBox` normalization routes are
AOT-callable as DirectAbi map-handle routes. Full PhaseState parse remains
unclaimed because the public `ProgramJsonV0PhaseStateBox.parse/2` call is still
the next AOT call boundary.

## Implementation

- Replaced `_normalize_function_name_or_error/4` with
  `_normalize_function_name_result/4`.
- Returned a total result map `{err, err_line, value}` instead of mixing
  normalized names and freeze-tag error strings.
- Removed the caller-side freeze-tag string probe for normalization errors.
- Kept MIR mutation, route selection, lowering, ID allocation, and new ABI out
  of scope.

## Evidence

```bash
bash tools/checks/hako_aot_programjson_rune_attrs_function_name_normalization_result_contract_guard.sh
```

## Remaining Blocker

```text
ProgramJsonV0PhaseStateBox.parse/2 public AOT call readiness
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
