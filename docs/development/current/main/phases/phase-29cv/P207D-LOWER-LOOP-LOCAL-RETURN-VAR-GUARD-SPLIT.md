---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P207d, LowerLoopLocalReturnVarBox.try_lower varname guard split
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P207C-BOX-TYPE-INSPECTOR-IS-MAP-DIRECT-SCALAR.md
  - docs/development/current/main/CURRENT_STATE.toml
  - lang/src/mir/builder/internal/lower_loop_local_return_var_box.hako
---

# P207d: LowerLoopLocalReturnVar varname Guard Split

## Problem

P207c moved the source-execution probe to:

```text
target_shape_blocker_symbol=LowerLoopLocalReturnVarBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

`try_lower/1` is a string-or-null lowering helper. The current var-name guard
uses a combined condition:

```text
if varname == null || varname == "" { return null }
```

In MIR this keeps the null side of the `||` expression alive through join PHIs
even though that path immediately returns. The later body then looks like a
string flow that may still carry a void sentinel.

## Decision

Do not expand `generic_string_body` for this blocker.

Split the guard in source:

```text
if varname == null { return null }
if varname == "" { return null }
```

This preserves behavior while making the non-null path explicit before the
rest of the loop-lowering scan proceeds.

## Boundary

This card may only change the `varname` guard in
`LowerLoopLocalReturnVarBox.try_lower/1`.

It must not:

- change `_read_compare_limit/4`
- change `_read_step_int/4`
- change emitted MIR JSON
- add or widen a body classifier
- add ny-llvmc semantics

If a later stop remains in the same function, handle it as a separate guard
cleanup or MIR-owned fact card.

## Probe Contract

Before this card, the stage probe stopped at:

```text
LowerLoopLocalReturnVarBox.try_lower/1
generic_string_unsupported_void_sentinel_const
```

After this card, that blocker should either disappear or move to a more
specific later value-flow site.

## Probe Result

The `--emit-exe` probe no longer stops at
`LowerLoopLocalReturnVarBox.try_lower/1`. The next observed stop is:

```text
target_shape_blocker_symbol=JsonFragBox.read_bool_from/2
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

That later stop is outside this card's boundary.

## Acceptance

```bash
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p207d_lower_loop_local_return_var_guard_split.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
