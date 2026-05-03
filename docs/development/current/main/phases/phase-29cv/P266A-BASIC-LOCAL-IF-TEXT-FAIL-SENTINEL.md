---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P266a, basic local-if text failure sentinel
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P264A-BASIC-LOCAL-IF-TEXT-SCALAR-CONTRACT.md
  - lang/src/mir/builder/func_body/basic_lower_box.hako
---

# P266a: Basic Local-If Text Failure Sentinel

## Problem

After P265a, the source-exe probe first stops at:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._try_lower_local_if_return/4
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

`_try_lower_local_if_return/4` is a text-producing try-lower helper, but its
failure path returns `null`. This makes the helper's return flow mix MIR text
with void sentinel values.

## Decision

Do not widen generic string/void sentinel classifier behavior.

Use the text try-lower convention for this helper:

```text
success -> MIR function JSON text
failure -> ""
```

The caller must treat `""` the same as no local-if lowering result.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic classifier acceptance change
- no C shim/body-specific emitter change
- no change to the supported local-if shape

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p266a_basic_local_if_text_fail_sentinel.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: route metadata no longer classifies
`FuncBodyBasicLowerBox._try_lower_local_if_return/4` as
`generic_string_unsupported_void_sentinel_const`. A later explicit blocker may
remain.

## Result

Accepted.

The source-exe probe no longer stops at
`FuncBodyBasicLowerBox._try_lower_local_if_return/4`:

```text
FuncBodyBasicLowerBox._try_lower_local_if_return/4
  target_shape=generic_string_or_void_sentinel_body
  return_shape=string_handle_or_null
  tier=DirectAbi
```

The next explicit blocker is now the owner-local int inline prepass:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._inline_local_ints/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```
