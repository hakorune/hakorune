---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P157, EnvSet extern route and generic string body surface
Related:
  - docs/development/current/main/phases/phase-29cv/P156-STRING-RETURN-PROFILE-CONCAT-ABI-EVIDENCE.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/extern_call_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
---

# P157: EnvSet Extern Route

## Problem

P156 exposed the next real owner boundary inside the Stage1 using resolver:

```text
target_shape_blocker_symbol=Stage1UsingResolverBox.resolve_for_source/1
target_shape_blocker_reason=generic_string_unsupported_extern_call
```

The concrete extern was `env.set/2`, used by the resolver depth guard. The
kernel already exports the ABI leaf:

```text
nyash.env.set(i64 key_handle, i64 value_handle) -> i64
```

Only `EnvGet` existed in the MIR extern route plan and C LoweringPlan consumer,
so the compiler could not prove or emit the existing ABI route.

## Decision

Add exactly one extern route vocabulary:

```text
extern.env.set
core_op=EnvSet
symbol=nyash.env.set
return_shape=scalar_i64
value_demand=runtime_i64
effects=[write.env]
```

Generic string bodies may accept `env.set/2` only when both arguments are
already observed string values. The result is scalar i64 and does not become a
string surface.

This does not add MapBox, ArrayBox, FileBox, or general extern-call support.

## Evidence

MIR JSON now records `EnvSet` rows with both operands:

```text
source_route_id=extern.env.set
core_op=EnvSet
tier=ColdRuntime
symbol=nyash.env.set
arity=2
return_shape=scalar_i64
value_demand=runtime_i64
effects=["write.env"]
```

The top pure-first source-execution stop moved past `env.set/2`:

```text
target_shape_blocker_symbol=Stage1UsingResolverBox.resolve_for_source/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

## Acceptance

```bash
cargo test -q extern_call_route_plan --lib
cargo test -q build_mir_json_root_emits_extern_call_routes_and_lowering_plan --lib
cargo test -q refresh_module_global_call_routes_accepts_string_concat_loop_with_env_set --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p157_env_set_extern_route.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p157_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

The final `--emit-exe` command is accepted as an advance-to-next-blocker
probe, not a full green source-execution gate.
