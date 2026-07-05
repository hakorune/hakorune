# 3119 - MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001

Status: selected-next

## Scope

Resume the first Layer4 ProgramJSON structured Plan/Recipe DTO parity slice
after the PhaseState parse route-readiness contract landed in 3118.

This is an implementation-capability card, not another guard-only detour.

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
