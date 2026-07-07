# 3232 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-EXPANDED-ROWS-001

Status: active

## Scope

Expand the ProgramJSON RecipeMatcher shadow parity coverage from the original
two rows to four rows while keeping Rust as runtime authority.

The covered shape remains the currently supported `LoopWithExit` input:

```text
Program.body = [
  Local(var = Int),
  Loop(cond = Var < Int, body = [
    If(cond = Var < Int, then = [Return(Int)], else = null),
    Assignment(var = Var + Int)
  ]),
  Return(Var)
]
```

This card varies names, initial values, loop bounds, return payloads, and step
sizes inside the accepted shape. It does not add a new ProgramJSON traversal
capability and does not switch runtime authority.

## Acceptance

```text
must require 3231 dual-run shadow guard
must run ProgramJSON matcher result through AOT/EXE for four rows
must compare canonical matcher-result fields against the Rust LoopWithExit oracle
must keep runtime_authority=rust_astnode
must keep programjson_runtime_route_authority=0
must keep runtime_route_switch=0
must keep recipe_matcher_input_authority=0
must keep full_recipe_matcher_execution=0
must keep route_selection/lowering/mutation/id_allocation/runtime_fallback=0
```

## Rows

```text
local_loop_body_if_branch_return
local_loop_body_if_branch_return_alt_names
local_loop_body_if_branch_return_i_wide_step
local_loop_body_if_branch_return_count_wide_step
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_shadow_parity_expanded_rows_gate.sh
```

Expected result:

```text
recipe_matcher_shadow_parity_expanded_rows=1
matcher_result_equal=1
row_count=4
runtime_authority=rust_astnode
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
full_recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-CONSULTATION-002
```
