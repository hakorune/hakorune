# 3118 - HAKO-AOT-PROGRAMJSON-PHASE-STATE-PARSE-AOT-CALL-READINESS-001

Status: active-progress

## Scope

Make the public `ProgramJsonV0PhaseStateBox.parse/2` route callable from an
imported `.hako` AOT app before resuming Layer4 Recipe DTO parity.

This card follows the scanner result-map contract direction selected in 3109:
fix `.hako` result contracts and narrow AOT/MIR metadata contracts instead of
widening `void` signatures into object or mixed-runtime returns.

## Task Plan

1. [done] Normalize PhaseState append helpers so every parse failure/success path
   returns the same top-level result `MapBox`.
2. [done] Prove in emitted MIR JSON that `ProgramJsonV0PhaseStateBox.parse/2`
   publishes `DirectAbi` with `return_shape=map_handle`.
3. [done] Update the PhaseState AOT blocker fixture/guard once the old
   `missing_multi_function_emitter` blocker is no longer the first failure.
4. [done] If the next blocker is PHI value typing, handle it as a triggered AOT/MIR
   typing debt slice, not as a scanner-library or ProgramJSON traversal claim.
5. [pending-heavy] Re-run the imported-app full AOT executable probe and keep it
   as a heavy readiness item. The daily guard for resuming Layer4 is the emitted
   MIR JSON route contract, not the minutes-long `emit-exe` path.

## Current Triggered Debt

The active probe may expose this known parked debt:

```text
HAKO-AOT-PHI-DST-TYPE-SCALAR-BOOL-I64-CONTRACT-001
```

Triggered investigation target:

```text
function = BoxTypeInspectorBox.is_map/1
symptom = LLVM mem2reg PHI type mismatch
example = phi i64 incoming value defined as i1
```

Allowed fix shape:

```text
publish or normalize PHI dst_type from proven scalar bool/i64 incoming values
add a focused AOT regression row for the triggering PhaseState parse app
```

Applied fix shape:

```text
normalize the triggered BoxTypeInspector is_map/is_array source shape
coerce integer-width PHI incoming values in the Python LLVM PHI wiring path
```

Forbidden fix shape:

```text
generic void object return widening
mixed_runtime_i64_or_handle for scanner out-map helpers
scanner source-string contains/regex proof
new backend route
new ABI
Layer4 Recipe DTO parity claim before the imported parse app is green
```

## Acceptance

```text
phase_state_parse_route = DirectAbi
phase_state_parse_return_shape = map_handle
imported_phase_state_parse_mir_json_readiness = green
old_missing_multi_function_emitter_blocker = closed
full_imported_phase_state_parse_aot_executable = unclaimed_heavy_probe_pending
```

Required guards:

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_phase_state_aot_call_blocker_guard.sh
bash tools/checks/hako_aot_programjson_phase_state_scan_body_local_result_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

If the first guard is converted from a blocker guard into a readiness guard,
its fixture must name the exact new first blocker or report the imported parse
route-readiness contract as green.

The converted guard proves the lightweight route-readiness contract via
`emit-mir-json`. It intentionally does not claim the full imported parse
executable as green because that path can spend minutes in LLVM `opt` for this
large imported closure.

## Resume Target

```text
MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001
```

Resume after `ProgramJsonV0PhaseStateBox.parse/2` publishes the imported
`DirectAbi` / `map_handle` route in emitted MIR JSON. Keep the full executable
probe as a separate heavy readiness item.

## Non-Claims

```text
layer4_recipe_dto_parity_green = 0
source_selfhost_claim = 0
mir_mutation = 0
id_allocation = 0
backend_lowering_claim = 0
route_selection_migration = 0
runtime_route_switch = 0
new_backend_route = 0
new_abi = 0
generic_void_object_return_widening = 0
mixed_runtime_i64_or_handle_for_scanner_out_map = 0
```
