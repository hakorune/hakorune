---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P263a, basic Return(Method) trace side-effect removal
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P261A-BASIC-RETURN-METHOD-DIRECT-TEXT-EMIT.md
  - docs/development/current/main/phases/phase-29cv/P262A-RETURN-CALL-LOWERER-SPLIT.md
  - lang/src/mir/builder/func_body/basic_lower_box.hako
---

# P263a: Basic Return(Method) Trace Side-Effect Removal

## Problem

After P262a, the source-exe probe first stops at:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._try_lower_return_method/4
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```

The MIR for `_try_lower_return_method/4` shows unsupported instructions from
diagnostic-only trace and `break`-based owner-local scans:

```text
env.get("HAKO_SELFHOST_TRACE")
print(...)
keepalive(...)
```

This is not a missing core language shape. It is diagnostic/control plumbing
inside a narrow owner-local lowerer.

## Decision

Do not add generic `print`, `keepalive`, or diagnostic side-effect acceptance to
Stage0 route classifiers.

Remove Return(Method)-path trace prints from `FuncBodyBasicLowerBox` and replace
its generic argument-list scan with a 0/1-argument direct parse, so the helper
stays a pure text/scalar lowering helper.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic `print` or `keepalive` acceptance
- no generic ArrayBox/MapBox method widening
- no C shim/body-specific emitter change
- no supported Return(Method) shape change

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p263a_basic_return_method_trace_side_effect_removal.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: route metadata no longer classifies
`FuncBodyBasicLowerBox._try_lower_return_method/4` as
`generic_string_unsupported_instruction`. A later explicit blocker may remain.

## Result

`cargo build -q --release --bin hakorune` passes.

`FuncBodyBasicLowerBox._try_lower_return_method/4` now routes as:

```text
FuncBodyBasicLowerBox._try_lower_return_method/4  generic_string_or_void_sentinel_body  string_handle_or_null  DirectAbi
```

The source-exe probe still fails explicitly, but moves to the next owner-local
basic-lower blocker:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._emit_local_if/7
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

No Stage0 classifier, `GlobalCallTargetShape`, or C shim emitter was widened.
