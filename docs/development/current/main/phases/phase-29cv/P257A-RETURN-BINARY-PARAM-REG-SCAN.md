---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P257a, return-binary param register scan
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P256A-BASIC-LOOP-ENTRY-STRING-SCAN.md
  - lang/src/mir/builder/func_lowering.hako
---

# P257a: Return Binary Param Register Scan

## Problem

After P256a, the source-exe probe advances to:

```text
target_shape_blocker_symbol=FuncLoweringBox._lower_return_binary/5
target_shape_blocker_reason=generic_string_unsupported_method_call
```

`FuncLoweringBox._lower_return_binary/5` only needs to translate parameter names
to their MIR argument registers while lowering a small `Return(Binary(...))`
shape.

The active route currently builds a temporary `MapBox`:

```text
param_map = new MapBox()
param_map.set(param_name, reg)
preg = param_map.get(var_name)
```

This is scalar register lookup, not a reason to widen Stage0 with generic
MapBox semantics.

## Decision

Do not add generic MapBox acceptance and do not add a new body shape.

Replace the temporary map with an owner-local scalar scan:

```text
_param_reg(params_arr, name) -> i64
```

The helper returns `0` when the parameter is not found; valid parameter
registers start at `1`.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic MapBox method widening
- no change to `Return(Binary(...))` accepted shapes
- no C body-specific emitter

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p257a_return_binary_param_reg_scan.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the route should move past the
`FuncLoweringBox._lower_return_binary/5` MapBox method blocker; a later blocker
may remain.

## Result

The owner-local parameter scan routes as:

```text
FuncLoweringBox._param_reg/2  generic_i64_body  DirectAbi
```

The `MapBox.get` method blocker in
`FuncLoweringBox._lower_return_binary/5` is gone. That helper is not yet a final
DirectAbi target, but its current reject is no longer the P257a MapBox method
blocker:

```text
FuncLoweringBox._lower_return_binary/5  generic_string_unsupported_void_sentinel_const
```

The source-exe probe now advances to:

```text
target_shape_blocker_symbol=FuncLoweringBox._lower_return_call/6
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```
