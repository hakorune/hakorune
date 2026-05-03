---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P246a, DefsScanner scalar bounds API
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P245A-STRING-OR-VOID-CHILD-RETURN-PROFILE.md
  - lang/src/mir/builder/func_lowering/defs_scanner_box.hako
---

# P246a: Defs Scanner Scalar Bounds

## Problem

P245a advances the source-exe probe to:

```text
target_shape_blocker_symbol=DefsScannerBox.find_defs_bounds/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

`find_defs_bounds/1` returns an `ArrayBox` only so the caller can read
`defs_start` and `defs_end`.

The active caller does not need object semantics; it needs two scalar indices.

## Decision

Do not add object-return support to Stage0.

Add scalar APIs:

```text
find_defs_start(program_json) -> i64, -1 on no-match
find_defs_end_from(program_json, defs_start) -> i64, -1 on no-match
```

Update `FuncLoweringBox.lower_func_defs/2` to use these scalar APIs. Keep the
legacy `find_defs_bounds/1` wrapper available for compatibility, but remove it
from the active source-exe path.

## Non-Goals

- no new `GlobalCallTargetShape`
- no ArrayBox DirectAbi support
- no C body-specific emitter
- no change to defs scanning semantics

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p246a_defs_scanner_scalar_bounds.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the frontier should move past
`DefsScannerBox.find_defs_bounds/1`; a later blocker may remain.

## Result

Observed route metadata:

```text
target_symbol=DefsScannerBox.find_defs_start/1
target_shape=generic_i64_body
return_shape=ScalarI64
tier=DirectAbi

target_symbol=DefsScannerBox.find_defs_end_from/2
target_shape=generic_i64_body
return_shape=ScalarI64
tier=DirectAbi
```

The active source-exe frontier moved past `DefsScannerBox.find_defs_bounds/1`.
The next frontier is:

```text
target_shape_blocker_symbol=FuncLoweringBox.lower_func_defs/2
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```
