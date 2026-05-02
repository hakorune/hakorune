---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P187, generic i64 body Select value-flow
Related:
  - docs/development/current/main/phases/phase-29cv/P186-GENERIC-STRING-SELECT-FLOW.md
  - src/mir/global_call_route_plan/generic_i64_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P187: Generic I64 Select Flow

## Problem

After P186, the real `stage1_cli_env.hako` probe moved past the string
methodize body and stopped at the scalar helper:

```text
target_shape_blocker_symbol=CallMethodizeBox._find_args_end/2
target_shape_blocker_reason=generic_string_return_not_string
```

The target body is not a string-return body. It scans a string with
`length()`/`substring()` and returns an integer index or `-1`:

```text
method _find_args_end(s, args_start) -> i64-like scalar
```

Its remaining value-flow surface includes MIR `Select` instructions with i1
conditions and i64 arms. Routing this through `generic_string_body` would make
the string classifier wider again, so the scalar route must own the fact.

## Decision

Teach `generic_i64_body` to classify `MirInstruction::Select` as a narrow
PHI-like scalar value-flow instruction:

```text
cond: Bool | I64
then/else: same value class
unknown arm + known arm: infer the unknown arm from the known arm
```

The accepted classes remain the existing generic-i64 body vocabulary. This is
not a new body shape and not a fallback; it is the scalar counterpart of P186's
string `Select` handling. The C shim already emits `select` from the active
module generic function emitter and consumes the route from LoweringPlan.

## Result

`CallMethodizeBox._find_args_end/2` classifies through the generic scalar i64
route when its scan loop is represented with MIR `Select` value flow.

The real `stage1_cli_env.hako` probe moved past the previous blocker and now
stops at the next methodize helper:

```text
target_shape_blocker_symbol=CallMethodizeBox._find_func_name/3
target_shape_blocker_reason=generic_string_unsupported_method_call
```

## Acceptance

```bash
cargo test -q generic_i64_select --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p187_i64_select_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
