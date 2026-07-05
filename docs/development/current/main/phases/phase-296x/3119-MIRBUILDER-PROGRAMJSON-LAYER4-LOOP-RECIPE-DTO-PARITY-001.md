# 3119 - MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001

Status: active-progress

## Scope

Resume the first Layer4 ProgramJSON structured Plan/Recipe DTO parity slice
after the PhaseState parse route-readiness contract landed in 3118.

This is an implementation-capability card, not another guard-only detour.

## Progress

Implemented:

```text
ProgramJsonLoopRecipeDtoSnapshotV1
  ProgramJSON -> PhaseState parse -> recipe_root -> canonical Loop DTO summary
```

Guard:

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_loop_recipe_dto_parity_gate.sh
```

Green claims from this guard:

```text
programjson_traversal_used = 1
structured_recipe_dto_constructed = 1
snapshot_route = DirectAbi
snapshot_return_shape = string_handle
phase_state_parse_route = DirectAbi
phase_state_parse_return_shape = map_handle
mir_json_route_green = 1
```

Not claimed:

```text
runtime_parity_green = 0
full_emit_exe_status = unclaimed_heavy_timeout_pending
```

The full `emit-exe` probe for this imported closure timed out at 120 seconds,
so runtime parity is split to the selected heavy readiness follow-up:

```text
HAKO-AOT-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-HEAVY-EXE-READINESS-001
```

## Prerequisites

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_phase_state_aot_call_blocker_guard.sh
bash tools/checks/hako_aot_programjson_phase_state_scan_body_local_result_contract_guard.sh
```

Required state:

```text
ProgramJsonV0PhaseStateBox.parse/2 route = DirectAbi
ProgramJsonV0PhaseStateBox.parse/2 return_shape = map_handle
old missing_multi_function_emitter blocker = closed
full imported parse executable green = not required for this lightweight slice
```

## Implementation Target

Build one covered loop Recipe DTO parity path from ProgramJSON structure:

```text
ProgramJSON Loop node
  -> PhaseState parse / scan body result maps
  -> Loop Recipe DTO fields
  -> Rust oracle parity fields
```

The card must include:

```text
.hako implementation update
fixture rows for the covered loop Recipe DTO shape
guard that compares canonical DTO fields against the Rust oracle
retire-candidate note only for the covered projector slice, if parity is green
```

## Stop Conditions

```text
STOP if the implementation only adds another string-only facade.
STOP if it accepts prebuilt token strings instead of ProgramJSON-derived result maps.
STOP if it widens void object returns or scanner out-map helpers in AOT.
STOP if full RecipeMatcher execution, MIR mutation, lowering, ID allocation, or route switch is required.
STOP if the only remaining blocker is full emit-exe opt cost; split that as a heavy readiness card.
```

## Required Non-Claims

```text
full_recipe_matcher_execution = 0
mir_mutation = 0
id_allocation = 0
backend_lowering_claim = 0
runtime_route_switch = 0
source_selfhost_claim = 0
programjson_all_shapes_supported = 0
full_imported_parse_executable_green = 0
```

## Next Decision

If the covered DTO parity row is green, select the next Layer4 capability batch
or mark the matching Rust ASTNode projector slice as retire-candidate. If parity
cannot run without full emit-exe, open a heavy AOT opt/readiness card instead of
widening scanner or route contracts.
