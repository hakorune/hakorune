# 3250 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-PARITY-001

Status: landed

## Scope

Prove the selected route-consumed field floor is present in ProgramJSON
CanonicalLoopFacts snapshots and observe-only RecipeMatcher results for covered
accepted-floor rows.

Rows:

```text
break_present_fields
break_continue_present_fields
```

Checked field floor:

```text
ok / reason_code / matched / contract_kind / has_break / has_continue / has_return
has_nested_loop / loop_cond_return_in_body_present
cond_kind / loop_var / loop_bound_int
update_kind / update_target / step_int
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_route_consumed_field_floor_parity_gate.sh
```
