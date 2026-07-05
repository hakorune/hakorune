# 3120 - HAKO-AOT-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-HEAVY-EXE-READINESS-001

Status: selected-next

## Scope

Turn the 3119 lightweight Layer4 loop Recipe DTO route proof into runtime
parity by making the generated `.hako` probe emit and run as an executable in a
bounded readiness gate.

This card exists because 3119 proved the `emit-mir-json` route but the full
`emit-exe` probe timed out at 120 seconds while compiling the imported closure.

## Prerequisites

```bash
bash tools/checks/hako_programjson_scanner_result_map_return_contract_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_loop_recipe_dto_parity_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_phase_state_aot_call_blocker_guard.sh
```

Required scanner contract:

```text
3109 HAKO-PROGRAMJSON-SCANNER-RESULT-MAP-RETURN-CONTRACT-001 is green.
ProgramJsonV0ScannerBox new field-read helpers return total result maps.
Legacy null-sentinel helpers are no-new-consumer compatibility helpers.
Generic void signature object return rejection remains in force.
Body proof alone cannot publish object or mixed-runtime handle returns.
```

If this prerequisite regresses, stop this heavy-readiness card and repair the
3109 result-map contract first. Do not unblock 3120 by widening AOT return
publication.

## Allowed Work

```text
reduce heavy AOT compile cost for the 3119 probe
split the probe if a smaller executable row can prove runtime parity
add a bounded heavy-readiness guard that runs only at milestones
record exact timeout / first unsupported route if the executable path fails
```

## Forbidden Work

```text
do not widen scanner void returns
do not use mixed_runtime_i64_or_handle for scanner out-map helpers
do not use body proof alone to publish object returns from void signatures
do not add a nullable out-map bridge inside this card
do not claim full RecipeMatcher execution
do not add MIR mutation, lowering, ID allocation, route switch, or new ABI
do not replace ProgramJSON-derived DTO traversal with prebuilt token strings
```

Temporary bridge rule, if a future blocker absolutely needs it:

```text
only a separate DeclaredNullableOutMapReturn bridge card may allow it
bridge card must declare remove_after and new_consumers_allowed = false
bridge card must stay narrower than mixed_runtime_i64_or_handle
3120 does not create that bridge
```

## Acceptance

```text
3119 lightweight guard = green
3109 scanner result-map contract guard = green
heavy emit-exe probe = green or exact first blocker documented
runtime_parity_green = 1 only if executable output matches canonical DTO summaries
void_signature_object_return_widening = 0
source_selfhost_claim = 0
```

If the exact first blocker is an opt-cost timeout with no semantic failure, keep
it as a heavy readiness performance task instead of widening compiler contracts.
