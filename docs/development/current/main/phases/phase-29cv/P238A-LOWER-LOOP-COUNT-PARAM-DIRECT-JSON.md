---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P238a, LowerLoopCountParam direct MIR JSON owner path
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P237A-LOWER-IF-NESTED-INT-OR-VAR-TEXT-ABI.md
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
  - lang/src/shared/mir/loop_form_box.hako
---

# P238a: Lower Loop Count-Param Direct JSON

## Problem

P237a advances the source-exe probe to:

```text
target_shape_blocker_symbol=LoopOptsBox.new_map/0
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

The active propagation includes `LowerLoopCountParamBox.try_lower/1`, which
builds a MapBox options object and delegates to `LoopOptsBox.build2/1`.

Teaching Stage0 to accept `LoopOptsBox.new_map/0` would pull the LoopForm
option-map adapter into the generic string lane.

## Decision

Keep the fix source-owned. `LowerLoopCountParamBox.try_lower/1` already owns
the complete count-param loop facts:

```text
init
limit
step
cmp
```

Emit the `LoopFormBox.build_loop_count_param_ex(init, limit, step, cmp)` MIR
JSON shape directly as a string and remove the `LoopOptsBox` adapter from this
owner path.

## Non-Goals

- no `LoopOptsBox` body shape
- no generic MapBox option-builder support
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to the recognized count-param loop pattern

## Acceptance

Probe result should move past the `LowerLoopCountParamBox.try_lower/1` path to
`LoopOptsBox.new_map/0`; a later blocker may remain:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p238a_lower_loop_count_param_direct_json.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Implemented in `LowerLoopCountParamBox.try_lower/1`:

- removed the `LoopOptsBox` option-map adapter from this owner path
- emitted the count-param loop MIR JSON directly from owner-local facts
- kept `start`, `limit`, and `step` on a numeric text ABI before final string
  emission
- removed nullable local accumulators and compound guard expressions that made
  the generic string lane see a void-sentinel local flow

Observed probe:

```text
target_shape_blocker_symbol=LoopOptsBox.new_map/0
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

This confirms the probe moved past `LowerLoopCountParamBox.try_lower/1`. The
remaining `LoopOptsBox.new_map/0` is a later owner path, not the count-param
path changed by this card.
