---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: add the ny-llvmc direct-target validator for plan-backed
  same-module global calls without enabling call emission.
Related:
  - docs/development/current/main/phases/phase-29cv/P116-GLOBAL-CALL-TARGET-SYMBOL-DECLS.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
---

# P117 Global Call Direct Target Validator

## Stop Line

P116 gave `global_call_routes` a `target_symbol` and emitted module function
declarations. The next dangerous shortcut would be to emit a call as soon as a
raw callee name appears.

P117 adds the validator that a real emitter must use:

```text
LoweringPlanGlobalCallView
  route_id == global.user_call
  core_op == UserGlobalCall
  target_symbol present
  target_exists == true
  target_arity == arity
  arity_matches == true
  quoted symbol is valid
```

The validator is intentionally not an emitter. `UserGlobalCall` remains
`tier=Unsupported` until the function-body emitter exists.

## Behavior

When route tracing is enabled and a global-call site has a direct target ready
by plan, ny-llvmc emits a `lowering_plan` trace with:

```text
consumer=global_call_direct_target_pending
```

Then it still fail-fasts with the plan reason:

```text
reason=missing_multi_function_emitter
```

This keeps the next card honest: it must either add function-body emission or
leave the shape unsupported. It must not silently externalize same-module user
calls.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/hakorune --emit-mir-json /tmp/p117_stage1_cli_env.mir.json \
  lang/src/runner/stage1_cli_env.hako
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
NYASH_LLVM_ROUTE_TRACE=1 \
target/release/ny-llvmc --in /tmp/p117_stage1_cli_env.mir.json \
  --emit obj --out /tmp/p117_stage1_cli_env.o
```

Expected stderr contains both:

```text
consumer=global_call_direct_target_pending
reason=missing_multi_function_emitter
```

