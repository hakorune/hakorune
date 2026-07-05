# 3111 - HAKO-AOT-PROGRAMJSON-PHASE-STATE-SCAN-BODY-LOCAL-RESULT-CONTRACT-001

Status: green

## Scope

Continue the PhaseState parse AOT call contract cleanup after 3110 by removing
the local nullable helper returns inside `ProgramJsonV0PhaseStateBox._scan_body_rec/8`.

This card does not claim `ProgramJsonV0PhaseStateBox.parse/2` is AOT-callable.
It proves the scan-body local helper blockers moved to DirectAbi map-handle
routes and that the remaining blocker is now the statement consumer path.

## Implementation

- Added `ProgramJsonV0ScannerBox.read_node_type_at_result/2` as a total
  result-map helper.
- Switched PhaseState and PhaseStateConsumer node-type reads to the result-map
  scanner helper.
- Replaced `_append_recipe_item_or_error` / `_append_recipe_children_rec` with
  total result-map helpers:
  - `_append_recipe_item_result/5`
  - `_append_recipe_children_result/6`
- Kept the fix `.hako`-local; no Rust route widening or new backend route.

## Evidence

```bash
bash tools/checks/hako_aot_programjson_phase_state_scan_body_local_result_contract_guard.sh
```

## Remaining Blocker

```text
ProgramJsonV0PhaseStateConsumerBox.consume_stmt/4
```

Next selected card:

```text
HAKO-AOT-PROGRAMJSON-PHASE-STATE-CONSUME-STMT-RESULT-CONTRACT-001
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
