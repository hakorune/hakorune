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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_loop_recipe_dto_parity_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_phase_state_aot_call_blocker_guard.sh
```

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
do not claim full RecipeMatcher execution
do not add MIR mutation, lowering, ID allocation, route switch, or new ABI
do not replace ProgramJSON-derived DTO traversal with prebuilt token strings
```

## Acceptance

```text
3119 lightweight guard = green
heavy emit-exe probe = green or exact first blocker documented
runtime_parity_green = 1 only if executable output matches canonical DTO summaries
source_selfhost_claim = 0
```

If the exact first blocker is an opt-cost timeout with no semantic failure, keep
it as a heavy readiness performance task instead of widening compiler contracts.
