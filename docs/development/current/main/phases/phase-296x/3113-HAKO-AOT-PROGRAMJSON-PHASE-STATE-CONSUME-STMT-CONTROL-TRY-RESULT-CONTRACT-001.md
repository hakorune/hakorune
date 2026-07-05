# 3113 - HAKO-AOT-PROGRAMJSON-PHASE-STATE-CONSUME-STMT-CONTROL-TRY-RESULT-CONTRACT-001

Status: green

## Scope

Close the PhaseState statement-consumer AOT call blocker by moving the control
and try-cleanup paths from nullable helper returns to total result-map helpers.

This card claims only the `ProgramJsonV0PhaseStateConsumerBox` call chain is
AOT-callable as DirectAbi map-handle routes. Full PhaseState parse remains
unclaimed; this card only exposes RecipeVerifier / RecipePortSig as the next
consumer-probe result-contract blocker family.

## Implementation

- Replaced `_handle_try_cleanup_stmt_or_null/6` with
  `_handle_try_cleanup_stmt_result/6`.
- Replaced `_control_handler_out_or_null/6` and
  `_handle_control_stmt_or_null/6` with result-map helpers.
- Annotated `consume_stmt/4`, `_dispatch_or_unsupported/6`, and control
  recipe helpers as `MapBox`.
- Kept MIR mutation, route selection, lowering, ID allocation, and new ABI out
  of scope.

## Evidence

```bash
bash tools/checks/hako_aot_programjson_phase_state_consume_stmt_control_try_result_contract_guard.sh
```

## Remaining Blocker

```text
RecipeVerifierBox / RecipePortSigBox result contract cleanup
```

Next selected card:

```text
HAKO-AOT-PROGRAMJSON-RECIPE-VERIFIER-PORT-SIG-RESULT-CONTRACT-001
```

## Non-Claims

```text
phase_state_parse_aot_call_fixed = 0
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
