---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380U, module generic legacy externcall consumer
Related:
  - docs/development/current/main/phases/phase-29cv/P380T-MODULE-GENERIC-STRUCTURED-CALL-CONSUME.md
  - docs/development/current/main/phases/phase-29cv/P380S-GENERIC-BODY-ENV-GET-CANONICAL-SPELLING.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P380U: Module Generic Legacy `externcall` Consumer

## Problem

P380T made module generic definition emission consume structured `op:"call"`.
The phase29cg blocker moved to the parent function:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=Stage1ModeContractBox.resolve_mode/0
```

Inspection shows `resolve_mode/0` still contains legacy `op:"externcall"`
instructions for `env.get`. The MIR already publishes exact
`extern_call_routes` entries for those sites, but the module generic prepass and
body emitter do not consume legacy `externcall`.

## Decision

Make module generic definition emission consume legacy `op:"externcall"` only
through existing `extern_call_routes` facts.

Prepass may assign value class for valid extern route views. Body emission must
reuse the existing extern lowering plan emitter. This keeps Stage0 on the
LoweringPlan route and does not add raw extern semantics to the generic body.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic acceptance of arbitrary `externcall`
- no direct name-based `env.get` lowering in module generic code
- no fallback when `extern_call_routes` is missing
- no change to source Program(JSON)

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p380u_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380u_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the blocker should move beyond legacy `externcall` prepass failure in
`Stage1ModeContractBox.resolve_mode/0`.

## Result

Implemented.

Module generic prepass now accepts legacy `op:"externcall"` only when the site
has a valid `extern_call_routes` view for the existing supported extern routes.
Body emission calls the existing extern lowering plan emitter, so the concrete
lowering remains owned by LoweringPlan metadata.

Validation:

```text
bash tools/build_hako_llvmc_ffi.sh
-> PASS

KEEP_OUT_DIR=1 OUT_DIR=/tmp/p380u_phase29cg \
  STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
  NYASH_LLVM_SKIP_BUILD=1 \
  bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
-> emit_program_rc=0 emit_mir_rc=0 llvm_rc=4
```

Relevant progress:

```text
consumer=mir_call_extern_emit site=b0.i1 route=extern.env.get core_op=EnvGet
consumer=mir_call_extern_emit site=b1.i1 route=extern.env.get core_op=EnvGet
consumer=mir_call_extern_emit site=b6.i1 route=extern.env.get core_op=EnvGet
consumer=mir_call_extern_emit site=b9.i1 route=extern.env.get core_op=EnvGet
consumer=mir_call_extern_emit site=b12.i1 route=extern.env.get core_op=EnvGet
```

The blocker moved beyond `resolve_mode/0` extern reads. The next stop is a
same-module global call from the entry body:

```text
reason=missing_multi_function_emitter
target_shape_reason=generic_string_global_target_shape_unknown
target_shape_blocker_symbol=Stage1InputContractBox.resolve_emit_program_source_text/0
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

The next card should investigate the target body/source contract before adding
any new Stage0 body shape.
