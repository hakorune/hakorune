---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P186, generic string body Select value-flow
Related:
  - docs/development/current/main/phases/phase-29cv/P185-STRING-LASTINDEXOF-DIRECTABI-CONSUME.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P186: Generic String Select Flow

## Problem

After P185, the real `stage1_cli_env.hako` probe moved to:

```text
target_shape_blocker_symbol=CallMethodizeBox.methodize_calls_in_mir/1
target_shape_blocker_reason=generic_string_unsupported_instruction
```

The target body already routes its string helper calls through LoweringPlan.
The remaining unsupported instruction surface is MIR `Select`, produced as a
value-flow ternary for scalar/string handle choices.

## Decision

Teach `generic_string_body` to classify `MirInstruction::Select` as a narrow
value-flow instruction:

```text
cond: Bool | I64
then/else: same value class
string/null sentinel mix: StringOrVoid
unknown arm + known arm: infer the unknown arm from the known arm
```

This is not a new body shape and not a JsonFrag-specific rule. It is the MIR
equivalent of PHI-like value flow for a single instruction, and the C shim
already consumes `select` from the active module emitter.

## Acceptance

The real `stage1_cli_env.hako` probe moved past
`CallMethodizeBox.methodize_calls_in_mir/1` and now stops at:

```text
target_shape_blocker_symbol=CallMethodizeBox._find_args_end/2
target_shape_blocker_reason=generic_string_return_not_string
```

```bash
cargo test -q generic_string_select --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p186_select_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
