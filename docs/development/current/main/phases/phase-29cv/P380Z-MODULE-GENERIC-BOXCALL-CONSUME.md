---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380Z, module generic legacy boxcall consume
Related:
  - docs/development/current/main/phases/phase-29cv/P380Y-GENERIC-I64-PRINT-DEAD-DST.md
  - docs/development/current/main/phases/phase-29cv/P380T-MODULE-GENERIC-STRUCTURED-CALL-CONSUME.md
  - docs/development/current/main/phases/phase-29cv/P380U-MODULE-GENERIC-LEGACY-EXTERNCALL-CONSUME.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P380Z: Module Generic Boxcall Consume

## Problem

P380Y advanced the phase29cg replay beyond the Program(JSON) result validator.
The next failure is no longer a classifier failure:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=Stage1InputContractBox._debug_len_inline/1
```

`Stage1InputContractBox._debug_len_inline/1` is already classified as
`generic_pure_string_body`. Its MIR contains a legacy `boxcall` for the
metadata-owned string length operation:

```json
{"op":"boxcall","box":5,"method":"length","args":[],"dst":6}
```

The module generic definition prepass/body emitter consumes structured
`op:"call"` and legacy wrapped `op:"mir_call"`, but not legacy `op:"boxcall"`.
That makes a valid LoweringPlan site fail before emission.

## Decision

Teach the module generic definition prepass/body emitter to read legacy
`boxcall` as a method-call surface for existing LoweringPlan generic method
routes.

This is a JSON surface normalization in the C shim:

```text
boxcall(box=recv, method=m, args=a)
  -> Method(recv, m, args) for route-plan consumption
```

No method is accepted by name alone. Emission must still be proven by the
existing generic method route / LoweringPlan entry.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic `boxcall` fallback
- no raw method-name lowering without LoweringPlan
- no source-side rewrite
- no VM fallback

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p380z_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380z_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the blocker moves beyond
`Stage1InputContractBox._debug_len_inline/1` module generic prepass failure.

## Result

Accepted.

Verified:

```bash
bash tools/build_hako_llvmc_ffi.sh
```

The phase29cg replay now consumes the legacy `boxcall` sites inside module
generic function definitions. `_debug_len_inline/1` is no longer the module
generic prepass blocker; the replay advances through the debug len/preview
chain and stops at the next route-surface gap:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1
```

That next blocker is not a `boxcall` issue. It is the P380X
`Stage1EmitProgramJson` extern route needing the same module generic prepass
consumer support that the entry/body emitter path already has.
