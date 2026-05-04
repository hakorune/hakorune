---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380T, module generic structured call consumer
Related:
  - docs/development/current/main/phases/phase-29cv/P380S-GENERIC-BODY-ENV-GET-CANONICAL-SPELLING.md
  - docs/development/current/main/phases/phase-29cv/P380P-STRUCTURED-CALL-GENERIC-MIR-OP.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P380T: Module Generic Structured `call` Consumer

## Problem

P380S moved phase29cg past `Stage1ModeContractBox.resolve_mode/0` route
classification. The next blocker is inside same-module generic definition
emission:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=Stage1InputContractBox.clean_env_value/1
```

`Stage1InputContractBox.clean_env_value/1` is already classified through
MIR-owned route facts, but its body uses structured Program(JSON) `op:"call"`.
The entry pure lowering path accepts structured `call`, while the module generic
definition prepass/body emitter still reads only legacy wrapped `op:"mir_call"`.

## Decision

Make the module generic prepass and body emitter consume structured `op:"call"`
through the same call payload path as legacy `op:"mir_call"`.

This is a consumer alignment only. Permission still comes from the existing
LoweringPlan entries and module generic definition registry.

## Non-Goals

- no new `GlobalCallTargetShape`
- no new body-specific emitter
- no generic MapBox/ArrayBox semantics
- no externalization of same-module global calls
- no duplicate target-shape inference in C

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p380t_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380t_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the blocker should move beyond `module_generic_prepass_failed` for
`Stage1InputContractBox.clean_env_value/1`. Any remaining blocker must be
reported from existing LoweringPlan/module emission facts, not from a new body
shape.

## Result

Implemented.

Module generic prepass/body emission now reads both legacy wrapped
`op:"mir_call"` and structured Program(JSON) `op:"call"` through one call
payload extractor. The existing LoweringPlan site lookup and module generic
definition registry remain the authority for whether a call may be emitted.

Validation:

```text
bash tools/build_hako_llvmc_ffi.sh
-> PASS

KEEP_OUT_DIR=1 OUT_DIR=/tmp/p380t_phase29cg \
  STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
  NYASH_LLVM_SKIP_BUILD=1 \
  bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
-> emit_program_rc=0 emit_mir_rc=0 llvm_rc=4
```

The blocker moved beyond
`Stage1InputContractBox.clean_env_value/1`. The next stop is still a module
generic prepass failure, but now in the parent function:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=Stage1ModeContractBox.resolve_mode/0
```

Inspection shows `resolve_mode/0` still contains legacy `op:"externcall"`
`env.get` instructions with valid `extern_call_routes` entries. The next card
should make module generic prepass/body emission consume legacy `externcall`
through those existing extern route facts. It must not add a new body shape.
