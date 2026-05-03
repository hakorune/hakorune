---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P247a, DefsScanner scalar function-name extraction
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P246A-DEFS-SCANNER-SCALAR-BOUNDS.md
  - lang/src/mir/builder/func_lowering/defs_scanner_box.hako
---

# P247a: Defs Scanner Scalar Name End

## Problem

P246a advances the source-exe probe past `DefsScannerBox.find_defs_bounds/1`.
The next frontier is:

```text
target_shape_blocker_symbol=FuncLoweringBox.lower_func_defs/2
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

Route metadata for `FuncLoweringBox.lower_func_defs/2` shows the active
function-name path still goes through an object-return helper:

```text
target_symbol=DefsScannerBox.extract_name/2
target_shape_reason=generic_string_return_object_abi_not_handle_compatible
tier=Unsupported
```

The caller only needs two facts:

```text
name_end: i64
func_name: substring(defs_str, name_idx + 8, name_end)
```

The MapBox return and `name_info.get(...)` calls are unnecessary on the active
source-exe path.

## Decision

Do not add generic `MapBox.get` or object-return support.

Add a scalar helper:

```text
find_name_end(defs_str, name_idx) -> i64, -1 on malformed
```

Update `FuncLoweringBox.lower_func_defs/2` to use `find_name_end/2` and derive
`func_name` with a local substring. Keep the legacy `extract_name/2` wrapper for
compatibility, implemented through `find_name_end/2`.

## Non-Goals

- no new `GlobalCallTargetShape`
- no MapBox DirectAbi support
- no generic `MapBox.get` acceptance
- no C body-specific emitter
- no change to function-name scanning semantics

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p247a_defs_scanner_scalar_name_end.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the frontier should move past
`DefsScannerBox.extract_name/2`; a later blocker may remain.

## Result

Observed route metadata:

```text
target_symbol=DefsScannerBox.find_name_end/2
target_shape=generic_i64_body
return_shape=ScalarI64
tier=DirectAbi
```

`DefsScannerBox.extract_name/2` no longer appears on the active
`FuncLoweringBox.lower_func_defs/2` route. The next object-return blocker in
the same function is:

```text
target_symbol=DefsScannerBox.extract_params/2
target_shape_reason=generic_string_return_object_abi_not_handle_compatible
tier=Unsupported
```

The source-exe frontier remains at `FuncLoweringBox.lower_func_defs/2` until
the remaining active method/object paths are removed:

```text
target_shape_blocker_symbol=FuncLoweringBox.lower_func_defs/2
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```
