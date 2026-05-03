---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P267a, basic local-int inline text table
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P266A-BASIC-LOCAL-IF-TEXT-FAIL-SENTINEL.md
  - lang/src/mir/builder/func_body/basic_lower_box.hako
---

# P267a: Basic Local-Int Inline Text Table

## Problem

After P266a, the source-exe probe first stops at:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._inline_local_ints/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_inline_local_ints/1` is an owner-local compatibility prepass for existing
basic lowerers. It currently builds an `ArrayBox` of `MapBox` records, then
uses `length/get/set/push` to replay Local(Int) bindings as Var substitutions.

That leaks collection plumbing into a string-only prepass and would push Stage0
toward accepting generic collection method calls.

## Decision

Do not widen generic string/i64 classifiers for ArrayBox/MapBox methods.

Keep this prepass owner-local and text-only:

```text
scan Local(Int) -> append "name=value;" to a small text table
replay table -> replace matching {"type":"Var","name":"..."} fragments
```

The table is an internal implementation detail of `FuncBodyBasicLowerBox`; it is
not a general MIR fact store.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic ArrayBox/MapBox method acceptance
- no C shim/body-specific emitter change
- no change to the prepass acceptance shape
- no broader MIR schema or PatternUtil policy change

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p267a_basic_local_int_inline_text_table.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: route metadata no longer classifies
`FuncBodyBasicLowerBox._inline_local_ints/1` as
`generic_string_unsupported_method_call`. A later explicit blocker may remain.

## Result

Accepted.

The source-exe probe no longer stops at
`FuncBodyBasicLowerBox._inline_local_ints/1`:

```text
FuncBodyBasicLowerBox._inline_local_ints/1  generic_pure_string_body  string_handle  DirectAbi
FuncBodyBasicLowerBox._replace_all_text/3   generic_pure_string_body  string_handle  DirectAbi
FuncBodyBasicLowerBox.lower/4               generic_string_or_void_sentinel_body  string_handle_or_null  DirectAbi
```

The next explicit blocker is now the return-call lowerer body itself:

```text
target_shape_blocker_symbol=ReturnCallLowerBox.lower/6
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```
