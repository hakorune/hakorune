# 3110 - HAKO-AOT-PROGRAMJSON-PHASE-STATE-RUNE-ATTRS-RESULT-MAP-CONTRACT-001

Status: green

## Scope

Continue the PhaseState parse AOT call contract cleanup after 3109 by moving
the RuneAttrs side path off nullable scanner helper returns.

This card does not claim `ProgramJsonV0PhaseStateBox.parse/2` is AOT-callable.
It proves that the parse blocker moved past the RuneAttrs result-map path and
is now at `_scan_body_rec/8`.

## Implementation

- Added result-map scanner helpers for first string and first array field reads.
- Added `ProgramJsonV0RuneAttrsBox.read_function_runes_map_result/2`.
- Switched PhaseState parse to consume the RuneAttrs total result map.
- Removed the `_out_box` single-hop wrapper from PhaseState parse flow.
- Allowed same-module body support to recognize static self-recursive global
  helper calls, matching the existing method-recursion rule.

## Evidence

```bash
bash tools/checks/hako_aot_programjson_phase_state_rune_attrs_result_map_contract_guard.sh
```

## Remaining Blocker

```text
ProgramJsonV0PhaseStateBox._scan_body_rec/8
```

Next selected card:

```text
HAKO-AOT-PROGRAMJSON-PHASE-STATE-SCAN-BODY-REC-CONTRACT-001
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
