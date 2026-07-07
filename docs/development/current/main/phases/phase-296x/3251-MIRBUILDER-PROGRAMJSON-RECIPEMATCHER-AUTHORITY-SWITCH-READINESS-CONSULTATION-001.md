# 3251 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-READINESS-CONSULTATION-001

Status: landed

## Scope

Record the authority-switch readiness decision after the 3250 route-consumed
field-floor parity gate.

This card does not switch runtime authority. It explicitly keeps ProgramJSON as
shadow-only evidence and selects the next consultation row needed before any
limited authority switch can be reconsidered.

## Readiness State

```text
accepted floor:
  current_return_only_shape = green
  continue_present = green
  break_present = green
  break_and_continue_present = green
  return_absent_decision_row = green
  nested_loop_decision_row = decision_required

reject floor:
  missing_verified_recipe = green
  remaining reject axes = pending

field floor:
  route_consumed_field_floor_parity = green
```

## Decision

```text
selected:
  B_NESTED_LOOP_DECISION_ROW_NEXT

rejected for now:
  A_LIMITED_AUTHORITY_SWITCH_NOW

reason:
  Authority switch is not ready until the nested-loop decision axis and reject
  floor expansion are resolved.
```

Selected next:

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-DECISION-ROW-CONSULTATION-001
```

## Non-Claims

```text
authority_switch_ready = 0
programjson_runtime_route_authority = 0
runtime_route_switch = 0
recipe_matcher_input_authority = 0
route_selection = 0
MIR lowering/mutation = 0
ID allocation = 0
runtime_fallback = 0
Source Selfhost remains unclaimed.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_authority_switch_readiness_consultation_guard.sh
```
