# 3236 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-COVERAGE-FLOOR-SELECTION-001

Status: landed

## Scope

Select the minimum coverage floor required before any ProgramJSON RecipeMatcher
authority switch can be considered.

This is not an authority switch. Rust ASTNode remains runtime authority.
ProgramJSON remains shadow-only evidence until accepted rows, reject rows, and
route-adjacent field parity are all green.

## Selected Floor

```text
1. accepted floor matrix
2. reject floor
3. limited authority-switch consultation only after both are green
```

Accepted floor axes:

```text
current return-only shape
break present
continue present
break+continue present
return-absent decision row
nested-loop decision row
```

Reject floor axes:

```text
unsupported condition operator / shape
unsupported update operator
update target mismatch
unsupported variable name
malformed or missing verified recipe
extra statement / swapped body order / non-null else
no final return / no in-body return
```

Field floor:

```text
ok / reason_code
matched / contract_kind / has_break / has_continue / has_return
has_nested_loop
loop_cond_return_in_body_present
cond_kind / loop_var / loop_bound_int
update_kind / update_target / step_int
```

## Non-Claims

```text
ProgramJSON does not write PlanBuildOutcome.recipe_contract.
ProgramJSON does not feed route registry predicates.
ProgramJSON does not select routes.
ProgramJSON does not lower or mutate MIR.
ProgramJSON does not allocate IDs.
runtime_route_switch = 0
programjson_runtime_route_authority = 0
Source Selfhost remains unclaimed.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_authority_switch_coverage_floor_selection_guard.sh
```

Expected result:

```text
coverage_floor_selection=1
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ACCEPTED-FLOOR-MATRIX-001
accepted_floor_required=1
reject_floor_required=1
field_floor_required=1
programjson_runtime_route_authority=0
runtime_route_switch=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ACCEPTED-FLOOR-MATRIX-001
```
