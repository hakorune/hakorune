---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P262a, Return(Call) lowerer split
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P261A-BASIC-RETURN-METHOD-DIRECT-TEXT-EMIT.md
  - lang/src/mir/builder/func_lowering.hako
  - lang/src/mir/builder/func_lowering/return_call_lower_box.hako
---

# P262a: Return(Call) Lowerer Split

## Problem

After P261a, the source-exe probe advances to:

```text
target_shape_blocker_symbol=FuncLoweringBox._lower_return_call/6
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```

`FuncLoweringBox` still owns too many responsibilities:

```text
defs scan
function target table build
body lower dispatch
Return(Int)
Return(Binary)
Return(Call)
call target resolve
function injection
```

The next blocker is inside `Return(Call)`. Adding more logic directly to
`func_lowering.hako` would keep growing the orchestration file.

## Decision

Do not patch `_lower_return_call/6` in place.

Move `Return(Call)` lowering into:

```text
lang/src/mir/builder/func_lowering/return_call_lower_box.hako
```

`FuncLoweringBox` remains the orchestrator:

```text
defs scan
function target table accumulation
body lower dispatch
function injection
```

`ReturnCallLowerBox` owns:

```text
call name extraction
function target text-table append/resolve
argument register materialization
global/method call MIR text emit
```

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic classifier acceptance change
- no C shim/body-specific emitter change
- no change to supported `Return(Call(...))` shapes

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p262a_return_call_lowerer_split.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: behavior remains equivalent. The split is valid if
`FuncLoweringBox._lower_return_call/6` is gone, `ReturnCallLowerBox` owns the
Return(Call) lowering surface, and the source-exe probe still fails explicitly
instead of falling back.

## Result

`cargo build -q --release --bin hakorune` passes.

The source-exe probe still fails explicitly, but no longer attributes the first
backend blocker to `FuncLoweringBox._lower_return_call/6`:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._try_lower_return_method/4
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```

Route metadata confirms the split boundary:

```text
ReturnCallLowerBox.add_target/3            generic_pure_string_body  DirectAbi
ReturnCallLowerBox.resolve_call_target/2   generic_pure_string_body  DirectAbi
ReturnCallLowerBox._param_reg/2            generic_i64_body          DirectAbi
ReturnCallLowerBox.lower/6                 generic_string_unsupported_instruction  Unsupported
```

Next action: keep P262a as a BoxShape-only split commit. Handle the remaining
`FuncBodyBasicLowerBox._try_lower_return_method/4` unsupported instruction in a
separate card/commit so the split does not mix with a new acceptance change.
