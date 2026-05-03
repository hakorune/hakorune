---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380A, source-owner cleanup for StringHelpers.to_i64/1
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P164-GENERIC-I64-STRING-ORDERED-COMPARE.md
  - lang/src/shared/common/string_helpers.hako
  - src/mir/global_call_route_plan/generic_i64_body.rs
---

# P380A: StringHelpers.to_i64 Debug Hook Retire

## Problem

After P379A, the direct Stage1 env source-execution probe moved past the MIR
verifier and stopped in pure-first backend selection:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=StringHelpers.to_i64/1
target_shape_blocker_reason=generic_string_unsupported_instruction
```

The first unsupported instruction inside `StringHelpers.to_i64/1` is not part
of numeric scanning. It is a diagnostic hook:

```hako
if env.get("NYASH_TO_I64_DEBUG") == "1" {
  __mir__.log("[string_helpers/to_i64] x", x)
}
```

That hook lowers to MIR `Debug`. Teaching the Stage0 route classifier or C shim
to accept this diagnostic-only instruction would move complexity in the wrong
direction.

## Decision

Retire the `NYASH_TO_I64_DEBUG` hook from `StringHelpers.to_i64/1`.

This is a source-owner cleanup, not a new Stage0 acceptance shape:

- `generic_i64_body` already accepts the numeric scanner surface used by
  `StringHelpers.to_i64/1`, including `env.get/1`, string equality, string
  ordered digit checks, substring, length, arithmetic, and direct calls to
  scalar helpers.
- The debug observer is not required for runtime semantics.
- Stage0 must not learn source-helper diagnostic behavior just to keep the
  helper DirectAbi-eligible.

## Non-Goals

- Do not add a new `GlobalCallTargetShape`.
- Do not add a ny-llvmc body-specific emitter.
- Do not accept arbitrary `Debug` instructions in `generic_i64_body`.
- Do not change `NYASH_TO_I64_FORCE_ZERO` in this card. That flag is legacy
  bring-up behavior and needs a separate inventory before removal.
- Do not treat this as the final `missing_multi_function_emitter` fix. It only
  removes a diagnostic obstacle from the scalar helper body.

## Acceptance

```bash
cargo test --release generic_i64 --lib
cargo build --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Advance-to-next-blocker probe:

```bash
timeout --preserve-status 240s env \
  NYASH_LLVM_SKIP_BUILD=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  bash tools/selfhost_exe_stageb.sh \
  lang/src/runner/stage1_cli_env.hako \
  -o /tmp/p380_stage1_cli_env.exe
```

Expected reading: the helper body should no longer contain MIR `Debug`, and
the first blocker should advance from:

```text
target_shape_blocker_reason=generic_string_unsupported_instruction
```

to a later route fact such as:

```text
target_shape_blocker_reason=generic_string_return_not_string
```

Any remaining `missing_multi_function_emitter` stop is owned by the uniform MIR
function emitter path or a later source-owner cleanup, not by adding another
body shape.

## Result

The refreshed MIR for `StringHelpers.to_i64/1` contains zero `debug`
instructions. The source-execution probe still stops at
`missing_multi_function_emitter`, but the blocker reason moves from
`generic_string_unsupported_instruction` to
`generic_string_return_not_string`.

This means the diagnostic hook is no longer blocking the scanner body. The next
cleanup must handle the scalar-helper/string-profile mismatch separately.
