# 3249 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-SELECTION-001

Status: landed

## Scope

Select the route-consumed field floor required before any later ProgramJSON
RecipeMatcher authority-switch consultation.

Selected hard matcher fields:

```text
ok
reason_code
matched
contract_kind
has_break
has_continue
has_return
```

Selected route-adjacent facts:

```text
has_nested_loop
loop_cond_return_in_body_present
cond_kind
loop_var
loop_bound_int
update_kind
update_target
step_int
```

## Boundary

This is a selection card only. The selected next card must prove that
`snapshot_summary` and `match_summary` expose these fields with canonical values
for the covered accepted-floor rows.

## Non-Claims

```text
field_floor_parity_green = 0
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_route_consumed_field_floor_selection_guard.sh
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-PARITY-001
```
