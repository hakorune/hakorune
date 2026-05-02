---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P148, generic string return param passthrough
Related:
  - docs/development/current/main/phases/phase-29cv/P147-GLOBAL-CALL-UNKNOWN-RETURN-VOID-SENTINEL-BODY.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - src/mir/global_call_route_plan/tests/shape_reasons.rs
  - lang/src/runner/stage1_cli_env.hako
---

# P148: Global-Call String Return Param Passthrough

## Problem

After P147, the pure-first source-execution stop moved back to the mode
contract:

```text
Stage1ModeContractBox.resolve_mode/0
  target_shape_blocker_symbol=Stage1ModeContractBox._normalize_mode_alias/1
  target_shape_blocker_reason=generic_string_return_not_string
```

`_normalize_mode_alias/1` has a string return signature and only returns either
canonical string constants or the input `mode` unchanged. The generic string
scanner rejected the fallback return because the input parameter has unknown
MIR value class. Broadly seeding all unknown parameters as strings is forbidden
because it would hide real type holes.

## Decision

Add a narrow exact-flow rule for generic pure string bodies:

- only functions with a string return ABI (`String` / `StringBox`) may use it
- only parameters with `String`, `StringBox`, or `Unknown` parameter ABI are
  eligible
- the value must reach `return` through exact `copy` / all-param `phi` flow
- the parameter is not reclassified as string for arbitrary operations
- `Unknown` return signatures remain rejected by the normal
  `generic_string_return_not_string` path

This keeps the P145 rule intact: no default parameter-string inference.

## Evidence

After rebuilding and emitting `lang/src/runner/stage1_cli_env.hako`:

```text
Stage1ModeContractBox.resolve_mode/0 -> Stage1ModeContractBox._normalize_mode_alias/1
  tier=DirectAbi
  target_shape=generic_pure_string_body
  proof=typed_global_call_generic_pure_string
```

The full pure-first trace now passes `resolve_mode/0` and stops later:

```text
first_block=14989 first_inst=0 first_op=mir_call
reason=missing_multi_function_emitter
target_shape_blocker_symbol=Stage1SourceProgramAuthorityBox.emit_program_from_source/2
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_string_return_param_passthrough
cargo test -q refresh_module_global_call_routes_uses_direct_child_route_over_void_metadata
cargo test -q global_call_routes
cargo fmt --check
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p148_stage1_cli_env.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p148_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
```
