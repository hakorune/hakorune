---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P248a, params ArrayBox return removal from active func lowering
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P247A-DEFS-SCANNER-SCALAR-NAME-END.md
  - lang/src/mir/builder/func_lowering/defs_scanner_box.hako
---

# P248a: Param List String View

## Problem

P247a removes the active `DefsScannerBox.extract_name/2` MapBox path. The next
object-return blocker in `FuncLoweringBox.lower_func_defs/2` is:

```text
target_symbol=DefsScannerBox.extract_params/2
target_shape_reason=generic_string_return_object_abi_not_handle_compatible
tier=Unsupported
```

`extract_params/2` returns an `ArrayBox` of parameter names. That object then
leaks through the active lowering path as `params_arr.length()` and
`params_arr.get(i)` calls in multiple function-body lowerers.

This is not a Stage0 reason to accept generic ArrayBox returns. The active
lowering path only needs a stable parameter-list view:

```text
params JSON string
count(params)
get(params, index)
arity(params) with leading "me" receiver excluded
```

## Decision

Do not add ArrayBox DirectAbi support and do not add generic ArrayBox method
semantics.

Add a small `ParamListBox` helper that treats params as a JSON string view:

```text
as_json(params_json) -> string
count(params_json) -> i64
get(params_json, index) -> string/null
arity(params_json) -> i64
```

Add `DefsScannerBox.extract_params_json/2` for the active path. Keep the legacy
`extract_params/2` wrapper for compatibility, implemented by materializing an
ArrayBox from the string view.

Update the active function-lowering consumers to use `ParamListBox` instead of
`params_arr.length()` / `params_arr.get(i)`.

## Non-Goals

- no new `GlobalCallTargetShape`
- no ArrayBox DirectAbi support
- no generic ArrayBox `length/get` acceptance
- no C body-specific emitter
- no change to parameter scanning semantics

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p248a_param_list_string_view.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the frontier should move past
`DefsScannerBox.extract_params/2`; a later blocker may remain.

## Result

Observed route metadata:

```text
target_symbol=DefsScannerBox.extract_params_json/2
target_shape=generic_pure_string_body
return_shape=string_handle
tier=DirectAbi

target_symbol=ParamListBox.count/1
target_shape=generic_i64_body
return_shape=ScalarI64
tier=DirectAbi

target_symbol=ParamListBox.get/2
target_shape=generic_string_or_void_sentinel_body
return_shape=string_handle_or_null
tier=DirectAbi

target_symbol=ParamListBox.arity/1
target_shape=generic_i64_body
return_shape=ScalarI64
tier=DirectAbi
```

`DefsScannerBox.extract_params/2` no longer appears on the active
source-exe route. The first source-exe frontier moved to:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```
