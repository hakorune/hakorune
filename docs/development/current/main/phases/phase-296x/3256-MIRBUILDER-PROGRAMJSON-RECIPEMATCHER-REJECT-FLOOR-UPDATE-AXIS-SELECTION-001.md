# 3256 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-UPDATE-AXIS-SELECTION-001

Status: landed

## Scope

Select the next ProgramJSON RecipeMatcher reject-floor row after the
unsupported condition operator row.

The selected row is `unsupported_update_operator`: a ProgramJSON loop whose
body update assignment uses an unsupported Binary operator. The next
implementation card must prove the CanonicalLoopFacts snapshot fails as
`unsupported_loop_update` and the observe-only matcher reports
`snapshot_not_ok`.

## Decision

```text
selected_next_card =
  MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-UNSUPPORTED-UPDATE-OPERATOR-REJECT-ROW-001

selected_row_id =
  unsupported_update_operator_reject

expected_snapshot_reason =
  unsupported_loop_update

expected_matcher_reason =
  snapshot_not_ok
```

## Non-Claims

```text
unsupported_update_operator_reject_row_green = 0
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_reject_floor_update_axis_selection_guard.sh
```
