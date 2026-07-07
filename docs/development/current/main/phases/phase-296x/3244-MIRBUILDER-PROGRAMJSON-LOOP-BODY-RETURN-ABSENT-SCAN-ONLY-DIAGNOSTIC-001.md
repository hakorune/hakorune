# 3244 - MIRBUILDER-PROGRAMJSON-LOOP-BODY-RETURN-ABSENT-SCAN-ONLY-DIAGNOSTIC-001

Status: landed

## Scope

Add a scan-only ProgramJSON diagnostic proving that a final top-level `Return`
does not set loop-body `has_return`.

Covered row:

```text
Program.body = [
  Local(Int),
  Loop(body=[
    If(then=[Break], else=null),
    If(then=[Continue], else=null),
    Assignment(AddVarInt)
  ]),
  Return(Var)
]
```

Expected diagnostic:

```text
loop_body_has_break = 1
loop_body_has_continue = 1
loop_body_has_return = 0
final_top_level_return_present = 1
final_top_level_return_used_for_loop_body_has_return = 0
```

## Boundary

This is not an accepted-floor row. It does not call RecipeMatcher and does not
compare matcher results.

## Non-Claims

```text
return_absent_green = 0
return_absent_accepted_floor = 0
matcher_result_equal = 0
recipe_matcher_accepted_floor = 0
runtime_route_switch = 0
programjson_runtime_route_authority = 0
recipe_matcher_input_authority = 0
route_selection = 0
MIR lowering/mutation = 0
ID allocation = 0
Source Selfhost remains unclaimed.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_loop_body_return_absent_scan_only_diagnostic_gate.sh
```
