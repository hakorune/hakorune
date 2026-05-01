---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P131, generic i64 same-module global-call body
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P130-GLOBAL-CALL-VOID-CONST-REJECT-REASON.md
  - src/mir/global_call_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P131: Global Call Generic I64 Body

## Problem

P130 made the env flag helper blocker precise:

```text
Stage1InputContractBox._env_flag_enabled/1
target_shape_reason=generic_string_unsupported_void_sentinel_const
```

That helper is not a string body. It is a narrow i64 body that reads `env.get/1`,
checks a `void` sentinel, compares string handles, and returns `0` or `1`.
Leaving it as an unsupported string-body reject keeps `_stage1_debug_on/0`
blocked even though the existing generic function emitter can lower this MIR
vocabulary when MIR grants a typed proof.

## Decision

Add one lowerable same-module target shape:

```text
target_shape=generic_i64_body
proof=typed_global_call_generic_i64
return_shape=ScalarI64
value_demand=scalar_i64
```

MIR owns the shape. ny-llvmc consumes it only through
`LoweringPlanGlobalCallView`, emits the target as a same-module definition, and
then emits direct calls to that definition.

## Rules

Allowed:

- `env.get/1` as the existing extern lowering-plan route
- same-module calls only when their target shape is already lowerable
- `null`/`void` constants as i64 zero sentinels inside this i64 body shape
- string equality/presence comparisons and i64 returns

Forbidden:

- treating `generic_i64_body` as `generic_pure_string_body`
- by-name acceptance for `_env_flag_enabled/1`
- emitting a declaration-only direct call
- widening method-call support or backend-global `print`

## Expected Evidence

After this card, the env flag helper and its thin debug wrapper become direct
same-module calls:

```text
callee=Stage1InputContractBox._env_flag_enabled/1
target_shape=generic_i64_body
proof=typed_global_call_generic_i64
return_shape=ScalarI64
```

The next blocker for `resolve_emit_program_source_text/0` should move past
`_stage1_debug_on/0` to the debug helper / print surface.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `tools/build_hako_llvmc_ffi.sh` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `git diff --check` succeeds.
- `tools/checks/current_state_pointer_guard.sh` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` emits
  `generic_i64_body` for `_env_flag_enabled/1` and `_stage1_debug_on/0`.
