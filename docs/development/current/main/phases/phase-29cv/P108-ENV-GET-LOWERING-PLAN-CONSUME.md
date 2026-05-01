---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for `EnvGet` ColdRuntime.
Related:
  - docs/development/current/main/phases/phase-29cv/P107-ENV-GET-LOWERING-PLAN-METADATA.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
  - apps/tests/mir_shape_guard/lowering_plan_env_get_coldruntime_min_v1.mir.json
---

# P108 EnvGet LoweringPlan Consume

## Goal

Teach ny-llvmc pure-first to consume the MIR-owned `EnvGet` LoweringPlan entry
from P107.

This is a plan-backed acceptance slice, not a raw extern-call matcher. The
consumer validates the plan fields before emitting:

```text
source=extern_call_routes
source_route_id=extern.env.get
core_op=EnvGet
tier=ColdRuntime
emit_kind=runtime_call
symbol=nyash.env.get
proof=extern_registry
route_proof=extern_registry
arity=1
return_shape=string_handle_or_null
value_demand=runtime_i64_or_handle
```

## Decision

- Add a shared extern-call LoweringPlan view.
- Add a single EnvGet need-kind rule for declaration ownership.
- Emit `nyash.env.get(i64) -> i64` only when the current site has the validated
  plan entry.
- Keep legacy string extern handling unchanged as migration fallback.

## Non-goals

- no raw `.inc` `env.get` / `env.get/1` classifier
- no `EnvSet` expansion
- no hidden compat replay
- no Stage1 artifact or MIR dominance repair

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_env_get_coldruntime_min_v1.mir.json \
  --out /tmp/p108_lowering_plan_env_get_coldruntime.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
