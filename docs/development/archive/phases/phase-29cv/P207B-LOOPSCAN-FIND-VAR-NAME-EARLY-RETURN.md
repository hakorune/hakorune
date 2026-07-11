---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P207b, LoopScanBox.find_loop_var_name source-flow cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/README.md
  - docs/development/current/main/CURRENT_STATE.toml
  - lang/src/mir/builder/internal/loop_scan_box.hako
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P207b: LoopScan find_loop_var_name Early Return

## Problem

P206 moved the source-execution probe to:

```text
target_shape_blocker_symbol=LoopScanBox.find_loop_var_name/2
target_shape_blocker_reason=generic_string_return_not_string
```

The function scans a Compare fragment and returns the first loop variable name:

1. prefer the `lhs` Var name
2. otherwise use the `rhs` Var name
3. otherwise return `null`

The current source writes through a mutable `varname` local initialized to
`null`. That creates a string/null PHI in MIR and hides the existing
string-or-void return shape from the route classifier.

## Decision

Do not add a new body shape, target shape, or C shim rule for this blocker.

Make the source flow explicit instead:

```text
lhs match -> return read_string_after(...)
rhs match -> return read_string_after(...)
no match   -> return null
```

This keeps the behavior unchanged while removing the mutable string/null PHI
that triggered `generic_string_return_not_string`.

## Boundary

This card may only touch the control flow of
`LoopScanBox.find_loop_var_name/2`.

It must not:

- change `LoopScanBox.extract_ne_else_sentinel_value/4`
- change JsonFrag helper behavior
- add or widen `GenericPureStringBody`
- add `GlobalCallTargetShape` variants
- add ny-llvmc body-specific semantics

If a later probe still needs more support, prefer MIR-owned facts or the shared
uniform MIR function emitter before another body classifier.

## Probe Contract

Before this card, the stage probe stopped at:

```text
LoopScanBox.find_loop_var_name/2
generic_string_return_not_string
```

After this card, that blocker should disappear. The probe may still fail on a
later symbol; that later stop is the next card's blocker, not a regression.

## Probe Result

The `--emit-exe` probe no longer stops at
`LoopScanBox.find_loop_var_name/2`. The next observed stop is:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox.is_map/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

That later stop is outside this card's boundary.

## Acceptance

```bash
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p207b_loopscan_find_var_name_early_return.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
