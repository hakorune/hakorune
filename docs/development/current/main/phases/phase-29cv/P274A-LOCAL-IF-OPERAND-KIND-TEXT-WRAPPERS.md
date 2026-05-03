---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P274a, local-if operand-kind text wrappers
Related:
  - docs/development/current/main/phases/phase-29cv/P271A-BASIC-LOCAL-IF-TEXT-WRAPPERS.md
  - docs/development/current/main/phases/phase-29cv/P273A-FUNC-DEFS-TEXT-WRAPPERS.md
  - lang/src/mir/builder/func_body/basic_lower_box.hako
---

# P274a: Local-If Operand-Kind Text Wrappers

## Problem

After P273a, the source-exe probe first stops at:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._try_lower_local_if_return/4
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

P271a converted most nullable local-if helper results to the `""` text
sentinel, but `lhs_kind` and `rhs_kind` still read
`_extract_operand_kind(...)` directly. That helper is
`string_handle_or_null`, so comparing the result directly keeps a nullable
value inside the local-if body.

## Decision

Keep `_try_lower_local_if_return/4` text-only by wrapping operand-kind helper
results as well:

```text
_extract_operand_kind(...) -> _text_or_empty(...) -> compare with "var"/"int"
```

## Non-Goals

- no local-if accepted-shape expansion
- no new `GlobalCallTargetShape`
- no generic void-sentinel classifier change
- no C shim/body-specific emitter change

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p274a_local_if_operand_kind_text_wrappers.exe lang/src/runner/stage1_cli_env.hako
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
FuncBodyBasicLowerBox._try_lower_local_if_return/4  generic_pure_string_body  string_handle  DirectAbi
FuncLoweringBox.lower_func_defs/2                   generic_pure_string_body  string_handle  DirectAbi
```

The next explicit blocker is now an object-return ABI boundary:

```text
target_shape_blocker_symbol=BuildBundleInputBox.collect/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```
