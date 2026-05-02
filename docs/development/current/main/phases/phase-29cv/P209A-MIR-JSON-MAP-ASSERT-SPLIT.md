---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P209a, split MirJsonEmitBox map assertion from map-value passthrough
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P208C-GENERIC-I64-SCHEMA-FIELD-PROOF-CONSUME.md
  - lang/src/shared/mir/json_emit_box.hako
---

# P209a: MIR JSON Map Assert Split

## Problem

P208c advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._expect_map/2
target_shape_blocker_reason=generic_string_return_not_string
backend_reason=missing_multi_function_emitter
```

`_expect_map/2` is an identity-return dev assertion:

```hako
if BoxHelpers.is_map(val) { return val }
print(...)
return val
```

Treating this as a new map-return body shape would widen Stage0 with another
selfhost helper capsule. The value passthrough is not semantic lowering; only
the dev assertion is useful.

## Decision

Keep the map value at the caller and split the assertion into a scalar/void
helper:

```text
local callee = me._map_get_soft(...)
me._assert_map(callee, "mir_call.callee")
```

`_assert_map/2` returns `0` and preserves the existing print side effect on
invalid input. It must classify as an ordinary scalar/logging helper, not as a
map-return body.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic map-return classifier
- no C body-specific emitter
- no removal of `_expect_map/2` in this card

## Acceptance

Probe result after the split:

```text
target_shape_blocker_symbol=MirJsonEmitBox._map_get_soft_rec/6
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_expect_map/2` is no longer the first blocker.

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p209a_map_assert.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
