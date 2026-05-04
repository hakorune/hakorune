---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380R, structured `call` LoweringPlan diagnostic consumer
Related:
  - docs/development/current/main/phases/phase-29cv/P380Q-PROGRAM-JSON-BRIDGE-SEMANTIC-REFRESH.md
  - docs/development/current/main/phases/phase-29cv/P380P-STRUCTURED-CALL-GENERIC-MIR-OP.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
---

# P380R: Structured `call` Plan Diagnostic

## Problem

P380Q makes the Program(JSON)->MIR bridge publish `global_call_routes` and
LoweringPlan facts. The first phase29cg structured call now has MIR-owned
diagnostic facts:

```text
callee_name=Stage1ModeContractBox.resolve_mode/0
reason=missing_multi_function_emitter
target_shape_reason=generic_string_unsupported_extern_call
```

But ny-llvmc still reports the P380P placeholder:

```text
target_shape_reason=structured_call_global_user_target
target_shape_blocker_reason=same_module_global_call_requires_uniform_emitter
```

That keeps diagnosis in the C structured-call shim instead of the published
LoweringPlan.

## Decision

Make structured `op:"call"` reuse the same unsupported `global_call_routes`
diagnostic consumer as `op:"mir_call"`.

This is a diagnostic/ownership fix only:

- no new emitter
- no new target shape
- no raw callee-name classification
- no permission to emit same-module user calls

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p380r_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380r_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: phase29cg still fails, but the structured `call` unsupported-shape
line surfaces the MIR-owned plan facts. In particular:

```text
target_shape_reason=generic_string_unsupported_extern_call
```

The next implementation owner is then the extern-call canonicalization/emitter
gap for the target function, not a new body shape.

## Result

Implemented.

Structured `op:"call"` now uses the same unsupported `global_call_routes`
LoweringPlan diagnostic consumer as `op:"mir_call"`. The old structured-call
placeholder remains only as a fallback for missing plan facts.

Remeasure:

```text
KEEP_OUT_DIR=1 OUT_DIR=/tmp/p380r_phase29cg \
  STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
  NYASH_LLVM_SKIP_BUILD=1 \
  bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
-> emit_program_rc=0 emit_mir_rc=0 llvm_rc=4
```

Relevant stderr:

```text
[llvm-route/trace] stage=lowering_plan result=hit reason=mir_lowering_plan_v0 extra=consumer=global_call_direct_target_pending site=b0.i1 route=global.user_call core_op=UserGlobalCall tier=Unsupported symbol=-
[llvm-pure/unsupported-shape] recipe=pure-first first_block=0 first_inst=1 first_op=call owner_hint=mir_normalizer reason=missing_multi_function_emitter target_return_type=? target_shape_reason=generic_string_unsupported_extern_call target_shape_blocker_symbol=- target_shape_blocker_reason=-
```

The next blocker is now explicit:

```text
Stage1ModeContractBox.resolve_mode/0
target_shape_reason=generic_string_unsupported_extern_call
```

That should be handled by canonical extern-call lowering/normalization, not by
adding another body shape.
