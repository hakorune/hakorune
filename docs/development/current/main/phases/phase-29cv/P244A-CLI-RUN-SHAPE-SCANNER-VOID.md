---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P244a, CliRunShapeScanner observability return contract
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P243A-LOWER-LOOP-COUNT-PARAM-INLINE-EMIT.md
  - lang/src/mir/builder/func_body/cli_run_shape_box.hako
---

# P244a: CLI Run Shape Scanner Void

## Problem

P243a advances the source-exe probe to:

```text
target_shape_blocker_symbol=CliRunShapeScannerBox.scan/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

`CliRunShapeScannerBox.scan/1` is observability-only. Its return value is not
used by `FuncLoweringBox.lower_func_defs/2`, but the scanner currently builds
and returns a `MapBox` containing `has_run` and `branches`.

That makes Stage0 see an object-return body even though no lowering decision
consumes that object.

## Decision

Do not add object-return semantics to Stage0.

Keep the scanner as an observability leaf:

```text
scan match/no-match -> null
trace branch count -> scalar local only
```

This preserves the optional trace tag while removing the unused MapBox/ArrayBox
return boundary.

## Non-Goals

- no new `GlobalCallTargetShape`
- no MapBox/ArrayBox DirectAbi support
- no C body-specific emitter
- no change to actual CLI run lowering
- no new trace tag

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p244a_cli_run_shape_scanner_void.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the frontier should move past
`CliRunShapeScannerBox.scan/1`; a later blocker may remain.

## Result

Observed probe:

```text
target_shape=generic_string_void_logging_body
target_symbol=CliRunShapeScannerBox.scan/1
return_shape=void_sentinel_i64_zero
tier=DirectAbi
```

The next source-exe frontier is:

```text
target_shape_blocker_symbol=LowerLoopLocalReturnVarBox._read_step_int/4
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```
