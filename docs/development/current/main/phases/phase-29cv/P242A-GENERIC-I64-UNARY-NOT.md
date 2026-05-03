---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P242a, generic i64 scalar Unary Not fact
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P241A-VOID-LOGGING-RETURN-PROFILE.md
  - src/mir/global_call_route_plan/generic_i64_body.rs
---

# P242a: Generic I64 Unary Not

## Problem

P241a advances the source-exe probe to:

```text
target_shape_blocker_symbol=BuilderProgramJsonInputContractBox._program_json_header_present/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

The target body is a scalar validation helper:

```text
if !(program_json_text.contains("\"version\"")) { return 0 }
if !(program_json_text.contains("\"kind\"")) { return 0 }
return 1
```

`contains/1` is already part of the generic i64 scalar surface. The missing
piece is `UnaryOp::Not` over a Bool/I64 scalar result, so the function falls
through to generic string classification and reports the method-call blocker.

## Decision

Do not add a body shape and do not widen generic string.

Add the scalar fact:

```text
Bool|I64 --not--> Bool
```

Only `generic_i64_body` changes. This is a value-level scalar fact, not a
compiler body semantic clone.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic string method expansion
- no ny-llvmc body emitter
- no source workaround in `BuilderProgramJsonInputContractBox`

## Acceptance

```bash
cargo test -q global_call_route_plan::tests::generic_i64 --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p242a_generic_i64_unary_not.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the frontier should move past
`BuilderProgramJsonInputContractBox._program_json_header_present/1`; a later
blocker may remain.

## Result

Observed probe:

```text
target_shape=generic_i64_body
target_symbol=BuilderProgramJsonInputContractBox._program_json_header_present/1
return_shape=ScalarI64
tier=DirectAbi
```

The next source-exe frontier is:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```
