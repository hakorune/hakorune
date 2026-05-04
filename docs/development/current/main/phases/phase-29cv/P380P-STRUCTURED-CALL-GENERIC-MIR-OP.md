---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380P, structured `call` generic MIR op support
Related:
  - docs/development/current/main/phases/phase-29cv/P380O-PHASE29CG-LLVM-DIAGNOSTIC-SURFACE.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
---

# P380P: Structured `call` Generic MIR Op

## Problem

P380O exposed the next phase29cg keeper boundary:

```text
[llvm-pure/unsupported-shape] recipe=pure-first first_block=0 first_inst=1 first_op=call owner_hint=mir_normalizer reason=unknown_op
```

The MIR is not an arbitrary source helper body. It is canonical v0 call JSON:

```json
{"op":"call","callee":{"type":"Global","name":"Stage1ModeContractBox.resolve_mode/0"},"args":[],"dst":2}
```

`CallMethodizeBox` must remain identity for already-structured canonical calls
per P337A/P338A. Therefore this should not be fixed by rewriting the source
owner output into `mir_call`.

## Decision

Teach the pure lowering generic walk to recognize structured `op:"call"` as a
MIR op and route it through the same dispatch surface as `op:"mir_call"` when
it has a `callee` object.

Important boundary:

- supported method/extern/constructor/global runtime surfaces may use existing
  `mir_call` dispatch
- same-module global user calls must not be emitted as unresolved externs
- same-module global user calls remain a `missing_multi_function_emitter`
  stop until the uniform multi-function emitter lands

This is generic MIR op support, not a new body shape.

## Non-Goals

- no `GlobalCallTargetShape`
- no C shim body-specific emitter
- no source helper meaning
- no unresolved externalization of same-module user calls
- no change to `CallMethodizeBox` identity behavior

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p380p_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380p_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/smokes/v2/profiles/integration/core/phase2034/mirbuilder_call_methodize_canonical_identity_canary_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: phase29cg no longer reports `reason=unknown_op` for structured
`op:"call"`. If the first structured call is a same-module global user call, it
must stop as `missing_multi_function_emitter`.

## Result

Implemented.

The pure lowering generic walk now recognizes structured `op:"call"` with a
`callee` object and routes supported surfaces through the existing `mir_call`
dispatch. It does not externalize same-module global user calls.

Remeasure output now reaches the intended backend boundary:

```text
[llvm-route/trace] stage=mir_call result=seen reason=enter extra=ii=1 dst=2 recv=0 ctype=Global bname=- mname=Stage1ModeContractBox.resolve_mode/0 a0=0 a1=0
[llvm-pure/unsupported-shape] recipe=pure-first first_block=0 first_inst=1 first_op=call owner_hint=mir_normalizer reason=missing_multi_function_emitter target_return_type=? target_shape_reason=structured_call_global_user_target target_shape_blocker_symbol=Stage1ModeContractBox.resolve_mode/0 target_shape_blocker_reason=same_module_global_call_requires_uniform_emitter
```

This removes the earlier `reason=unknown_op` stop. The next blocker is the
uniform multi-function emitter, not call normalization or a new body shape.

Validation:

```text
bash tools/build_hako_llvmc_ffi.sh
-> done

KEEP_OUT_DIR=1 OUT_DIR=/tmp/p380p_phase29cg \
  STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
  NYASH_LLVM_SKIP_BUILD=1 \
  bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
-> emit_program_rc=0 emit_mir_rc=0 llvm_rc=4
-> reason=missing_multi_function_emitter

bash tools/smokes/v2/profiles/integration/core/phase2034/mirbuilder_call_methodize_canonical_identity_canary_vm.sh
-> PASS
```
