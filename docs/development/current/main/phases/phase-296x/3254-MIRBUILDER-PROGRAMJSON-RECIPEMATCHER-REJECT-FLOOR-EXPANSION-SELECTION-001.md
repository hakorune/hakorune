# 3254 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-EXPANSION-SELECTION-001

Status: landed

## Scope

Select the next ProgramJSON RecipeMatcher reject-floor row after the missing
verified recipe row and the nested-loop reject boundary.

The selected row is `unsupported_condition_operator`: a ProgramJSON loop whose
condition operator is unsupported by the current CanonicalLoopFacts snapshot.
The next implementation card must prove the snapshot fails as
`unsupported_loop_cond` and the observe-only matcher reports
`snapshot_not_ok`.

## Decision

```text
selected_next_card =
  MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-UNSUPPORTED-CONDITION-OPERATOR-REJECT-ROW-001

selected_row_id =
  unsupported_condition_operator_reject

expected_snapshot_reason =
  unsupported_loop_cond

expected_matcher_reason =
  snapshot_not_ok
```

## Non-Claims

```text
unsupported_condition_operator_reject_row_green = 0
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_reject_floor_expansion_selection_guard.sh
```
