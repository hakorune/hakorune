---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P234a, LowerLoopSumBc direct MIR JSON owner path
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P233A-LOWER-LOOP-COMPARE-LIMIT-GUARD-SPLIT.md
  - lang/src/mir/builder/internal/lower_loop_sum_bc_box.hako
  - lang/src/shared/mir/loop_form_box.hako
---

# P234a: Lower Loop Sum-BC Direct JSON

## Problem

P233a advances the source-exe probe to:

```text
target_shape_blocker_symbol=LowerLoopSumBcBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```

The unsupported route inventory shows the blocker is not the loop scan itself.
It is the map adapter tail:

```text
LoopOptsBox.new_map/0
LoopOptsBox.put/3
LoopOptsBox.build2/1
```

Teaching Stage0 to understand this adapter would pull MapBox option-building
semantics into the generic string lane.

## Decision

Keep the fix source-owned. `LowerLoopSumBcBox.try_lower/1` already owns the
sum-with-break/continue pattern and has the concrete values:

```text
limit
skip_value
break_value
```

Emit the `LoopFormBox.loop_counter(limit, skip_value, break_value)` MIR JSON
shape directly as string JSON. This removes the map adapter from the active
source-exe path without adding a body shape, a generic MapBox route, or a C
emitter.

Also keep the DirectAbi surface simple:

- split remaining short-circuit guards into nested `if` statements
- keep `skip_value` / `break_value` on the numeric-text lane before JSON concat
- use string default `"2"` for the continue sentinel so the post-null-check PHI
  does not mix scalar i64 with string/null sentinel flow

## Non-Goals

- no `LoopOptsBox` body shape
- no generic MapBox option-builder support
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to the recognized sum_bc source pattern

## Acceptance

Probe result should move past `LowerLoopSumBcBox.try_lower/1`. Observed next
blocker after the source cleanup:

```text
target_shape_blocker_symbol=CompatMirEmitBox._inst_externcall/3
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

Verification:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p234a_loop_sum_bc_direct_json.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
