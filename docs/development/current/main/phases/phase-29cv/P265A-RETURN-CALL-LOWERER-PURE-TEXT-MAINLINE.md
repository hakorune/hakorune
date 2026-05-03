---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P265a, Return(Call) pure text mainline
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P262A-RETURN-CALL-LOWERER-SPLIT.md
  - docs/development/current/main/phases/phase-29cv/P264A-BASIC-LOCAL-IF-TEXT-SCALAR-CONTRACT.md
  - lang/src/mir/builder/func_lowering/return_call_lower_box.hako
---

# P265a: Return(Call) Pure Text Mainline

## Problem

After P264a, the source-exe probe first stops at:

```text
target_shape_blocker_symbol=ReturnCallLowerBox.lower/6
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```

The body still contains dev/debug side-effect plumbing and `break`-based scans:

```text
env.get("HAKO_MIR_BUILDER_METHODIZE")
env.get("HAKO_MIR_BUILDER_DEBUG")
print(...)
break
```

This is not a reason to widen Stage0 generic instruction acceptance. The
Return(Call) owner-local lowerer should stay a pure text/scalar mainline helper.

## Decision

Do not add generic `env.get`, `print`, or `keepalive` acceptance.

Keep only the current default Global-call MIR text emit path in
`ReturnCallLowerBox.lower/6`:

```text
const resolved_with_arity
call func_reg(args)
ret result
```

Remove the methodize debug branch from this helper. If methodized Return(Call)
lowering is needed again, it must return as a separate documented card with an
explicit owner and acceptance fixture.

Rewrite local scans without `break`, using scalar cursor state.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic classifier acceptance change
- no C shim/body-specific emitter change
- no ArrayBox/MapBox method widening
- no new supported Return(Call) shapes

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p265a_return_call_lowerer_pure_text_mainline.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the first backend blocker moves past
`ReturnCallLowerBox.lower/6`. A later explicit blocker may remain. Residual
non-first route metadata for `ReturnCallLowerBox.lower/6` is tracked separately
if it reappears as a blocking route.

## Result

`cargo build -q --release --bin hakorune` passes.

`ReturnCallLowerBox.lower/6` no longer owns the first source-exe backend
blocker. The probe now advances to:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._try_lower_local_if_return/4
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

The methodize/debug side-effect path was removed from `ReturnCallLowerBox.lower/6`;
the remaining mainline emit is the default Global call text path.

Route metadata still contains residual non-first unsupported metadata for
`ReturnCallLowerBox.lower/6`. It is not the active source-exe blocker after this
card and should only be handled if it returns to the first-blocker path.
