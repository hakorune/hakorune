# 3247 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-SELECTION-001

Status: landed

## Scope

Select the first ProgramJSON RecipeMatcher reject-floor row after the accepted
floor reached Return-absent coverage.

Selected first row:

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-MISSING-VERIFIED-RECIPE-REJECT-ROW-001
row_id = missing_verified_recipe_reject
```

## Why This Row

`ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot/1` first requires
a verified ProgramJSON recipe. Current unsupported condition operators fail at
the PhaseState/Verifier boundary before CanonicalLoopFacts can publish
`unsupported_loop_cond`, so the first reachable reject-floor row is the missing
verified-recipe boundary.

Expected boundary:

```text
snapshot.ok = 0
snapshot.reason = verified_recipe_missing
matcher_result.ok = 0
matcher_result.reason = snapshot_not_ok
matched = 0
```

## Boundary

This is a selection card only. The selected row must be implemented as its own
AOT/EXE reject-row gate next.

## Non-Claims

```text
reject_row_green = 0
programjson_runtime_route_authority = 0
runtime_route_switch = 0
recipe_matcher_input_authority = 0
route_selection = 0
MIR lowering/mutation = 0
ID allocation = 0
Source Selfhost remains unclaimed.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_reject_floor_selection_guard.sh
```
