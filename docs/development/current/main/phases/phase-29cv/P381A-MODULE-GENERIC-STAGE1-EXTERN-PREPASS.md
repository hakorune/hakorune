---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381A, module generic stage1 extern prepass
Related:
  - docs/development/current/main/phases/phase-29cv/P380X-STAGE1-EMIT-PROGRAM-JSON-EXTERN-ROUTE.md
  - docs/development/current/main/phases/phase-29cv/P380Z-MODULE-GENERIC-BOXCALL-CONSUME.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381A: Module Generic Stage1 Extern Prepass

## Problem

P380Z advanced the phase29cg replay beyond legacy `boxcall` consumption. The
next blocker is:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1
```

The function body is already canonicalized to the P380X extern route:

```json
{"op":"externcall","func":"nyash.stage1.emit_program_json_v0_h","args":[0],"dst":1}
```

Its metadata has an explicit LoweringPlan route:

```text
source=extern_call_routes
source_route_id=extern.stage1.emit_program_json_v0
core_op=Stage1EmitProgramJson
return_shape=string_handle
```

The module generic body emitter can already emit this route through
`emit_extern_call_lowering_plan_mir_call`, but the module generic prepass only
marks env/hostbridge extern routes. It therefore rejects the function before
body emission.

## Decision

Teach the module generic prepass to consume the existing
`Stage1EmitProgramJson` extern route and mark its result as a string handle.

This is not a new route and not a body shape. It is the prepass counterpart of
the P380X emitter path:

```text
LoweringPlan Stage1EmitProgramJson -> T_I64 + ORG_STRING
```

## Non-Goals

- no new `GlobalCallTargetShape`
- no new extern symbol
- no raw extern-name fallback
- no source-side rewrite
- no VM fallback

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p381a_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p381a_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the blocker moves beyond
`Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1` module
generic prepass failure.

## Result

Accepted. The phase29cg replay moved beyond
`Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1`.

The next blocker is:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=Stage1EmitMirDispatchBox.run_emit_mir_mode/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

This confirms the Stage1 emit-program-json extern route is now consumed by the
module generic prepass without adding a new body shape.
