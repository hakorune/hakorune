# 3112 - HAKO-AOT-PROGRAMJSON-PHASE-STATE-CONSUME-STMT-NON-CONTROL-RESULT-CONTRACT-001

Status: green

## Scope

Continue the PhaseState statement-consumer AOT call contract cleanup after
3111 by moving the non-control statement path to total result-map helpers.

This card deliberately stops before the control and try-cleanup paths. It proves
that the non-control path no longer depends on nullable helper returns and that
the remaining blocker is now the control/try statement dispatch path.

## Implementation

- Switched statement handlers to the result-map `ProgramJsonV0ScannerBox`
  helpers for node type, int field, and string field reads.
- Gave `RecipeFactsBox.from_stmt/4` and its internal helpers explicit result
  shapes. Follow-up cleanup now builds per-stmt name arrays as literals, so no
  public `_push_name` helper boundary is needed for AOT.
- Allowed same-module AOT body proof for known `ArrayBox.push` /
  `RuntimeDataBox.push` side-effect calls.
- Replaced the non-control consumer nullable helpers with total result-map
  helpers:
  - `_handle_non_control_stmt_result/6`
  - `_non_control_handler_out_result/6`
  - `_non_control_handler_state_result/2`
  - `_after_state_from_non_control_result/3`
  - `_emit_handler_error_result_at/3`

## Evidence

```bash
bash tools/checks/hako_aot_programjson_phase_state_consume_stmt_non_control_result_contract_guard.sh
```

## Remaining Blocker

```text
ProgramJsonV0PhaseStateConsumerBox._handle_control_stmt_or_null/6
ProgramJsonV0PhaseStateConsumerBox._handle_try_cleanup_stmt_or_null/6
```

Next selected card:

```text
HAKO-AOT-PROGRAMJSON-PHASE-STATE-CONSUME-STMT-CONTROL-TRY-RESULT-CONTRACT-001
```

## Non-Claims

```text
phase_state_parse_aot_call_fixed = 0
consume_stmt_full_aot_call_fixed = 0
layer4_recipe_dto_parity_green = 0
source_selfhost_claim = 0
mir_mutation = 0
id_allocation = 0
backend_lowering_claim = 0
route_selection_migration = 0
runtime_route_switch = 0
new_backend_route = 0
new_abi = 0
```
