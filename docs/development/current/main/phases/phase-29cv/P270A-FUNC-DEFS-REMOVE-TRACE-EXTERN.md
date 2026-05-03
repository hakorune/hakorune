---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P270a, function defs lowerer trace extern removal
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P269A-COUNT-PARAM-REMOVE-VOID-SCOPE.md
  - lang/src/mir/builder/func_lowering.hako
---

# P270a: Function Defs Remove Trace Extern

## Problem

After P269a, the source-exe probe first stops at:

```text
target_shape_blocker_symbol=FuncLoweringBox.lower_func_defs/2
target_shape_blocker_reason=generic_string_unsupported_extern_call
backend_reason=missing_multi_function_emitter
```

The remaining extern calls in `lower_func_defs/2` are trace-only:

```text
env.get("HAKO_SELFHOST_TRACE")
print("[builder/funcs:skip] ...")
```

They do not participate in the MIR text contract and make the owner lowerer
look effectful to Stage0.

## Decision

Remove the trace env read and skip print from the active source-exe path.

This keeps `lower_func_defs/2` focused on deterministic text lowering. If this
observability is needed again, it should be reintroduced through an explicit
diagnostic lane, not inside the DirectAbi candidate body.

## Non-Goals

- no lowering behavior change for supported function bodies
- no new env variable or logging route
- no generic extern-call acceptance change
- no C shim/body-specific emitter change

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p270a_func_defs_remove_trace_extern.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: route metadata no longer classifies
`FuncLoweringBox.lower_func_defs/2` as `generic_string_unsupported_extern_call`.
A later explicit blocker may remain.

## Result

Accepted.

The source-exe probe no longer classifies `FuncLoweringBox.lower_func_defs/2`
as `generic_string_unsupported_extern_call`. The remaining call list no longer
contains `env.get/1` or `env.console.log`.

The next explicit blocker is now a deeper basic-body helper:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._try_lower_local_if_return/4
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```
