# 3248 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-MISSING-VERIFIED-RECIPE-REJECT-ROW-001

Status: landed

## Scope

Add the first ProgramJSON RecipeMatcher reject-floor row selected by 3247.

Covered row:

```text
row_id = missing_verified_recipe_reject
program_json = {}
```

Expected boundary:

```text
CanonicalLoopFacts snapshot:
  ok = 0
  reason = verified_recipe_missing
  matcher_input_present = 0

RecipeMatcher boundary:
  ok = 0
  reason = snapshot_not_ok
  matched = 0
```

## Boundary

This is still shadow-only. ProgramJSON does not write
`PlanBuildOutcome.recipe_contract`, does not feed route predicates, and does not
select/release routes.

## Non-Claims

```text
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_missing_verified_recipe_reject_row_gate.sh
```
