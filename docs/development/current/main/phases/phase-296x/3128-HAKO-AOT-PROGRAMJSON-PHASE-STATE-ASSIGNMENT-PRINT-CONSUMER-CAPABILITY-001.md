# 3128 - HAKO-AOT-PROGRAMJSON-PHASE-STATE-ASSIGNMENT-PRINT-CONSUMER-CAPABILITY-001

Status: landed

## Scope

Close the PhaseState consumer blocker that 3126/3127 deferred for top-level
`Assignment` and `Print` rows.

This card keeps the work below Seq Recipe DTO row adoption.  It proves that the
PhaseState consumer path itself can parse the covered top-level Assignment/Print
Program(JSON v0) rows under AOT runtime.

## Implementation

```text
lang/src/compiler/mirbuilder/stmt_handlers/assignment_stmt_handler.hako
lang/src/compiler/mirbuilder/stmt_handlers/print_stmt_handler.hako
lang/src/compiler/mirbuilder/program_json_v0_phase_state_consumer_box.hako
```

Fixes:

- Preserve scanner string tokens as raw `StringBox` values in Assignment/Print
  handlers.
- Use `BoxHelpers.same_token` for dynamic token comparisons instead of raw `==`
  / `!=` comparisons after `"" + ...` stringification.
- Preserve `assign_rhs_kind` through the PhaseState consumer handoff without
  lossy stringification.

## Covered Rows

```text
local_assignment_int_return_var
local_assignment_add_return_var
local_print_var_return_int
local_print_binary_return_int
```

## Evidence

```bash
bash tools/checks/hako_aot_programjson_phase_state_assignment_print_consumer_capability_guard.sh
```

Expected guard result:

```text
owner=ProgramJsonPhaseStateAssignmentPrintConsumerCapabilityV1
top_level_assignment_runtime_green=1
top_level_print_runtime_green=1
assignment_rhs_kind_preserved_through_phase_state=1
seq_recipe_dto_assignment_print_rows_green=0
runtime_route_switch=0
full_recipe_matcher_execution=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
```

## Non-Claims

```text
Seq Recipe DTO Assignment/Print rows adopted
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
runtime route switch
ProgramJSON full parser
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-ASSIGNMENT-PRINT-PARITY-001
```
