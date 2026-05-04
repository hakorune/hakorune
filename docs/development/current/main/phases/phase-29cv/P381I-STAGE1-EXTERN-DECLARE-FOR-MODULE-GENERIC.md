---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381I, Stage1 extern helper declarations for module-generic definitions
Related:
  - docs/development/current/main/phases/phase-29cv/P381H-DIRECT-GLOBAL-CALL-LITERAL-ARG-MATERIALIZE.md
  - docs/development/current/main/phases/phase-29cv/P381D-STAGE1-EMIT-MIR-PROGRAM-JSON-EXTERN-ROUTE.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
---

# P381I: Stage1 Extern Declare For Module Generic

## Problem

P381H moved phase29cg beyond direct global-call literal argument materialization.
LLVM verification then failed because a Stage1 extern helper was called from a
module-generic function without a declaration:

```text
opt: /tmp/p381h_bad.ll:920:18: error: use of undefined value '@nyash.stage1.emit_mir_from_program_json_v0_h'
  %r2 = call i64 @"nyash.stage1.emit_mir_from_program_json_v0_h"(i64 %r1)
                 ^
```

The generic lowering declaration pass records `needs.*` from the active entry
function. Module-generic function definitions are emitted in the same LLVM
module, but their Stage1 extern calls are not represented in the root `needs`
flags.

## Decision

When module-generic definitions are planned, declare the Stage1 extern helpers
that module-generic definitions may call:

```text
nyash.stage1.emit_program_json_v0_h
nyash.stage1.emit_mir_from_source_v0_h
nyash.stage1.emit_mir_from_program_json_v0_h
```

This is declaration hygiene only. It does not add call acceptance, a body shape,
or a runtime fallback.

## Non-Goals

- no new `GlobalCallTargetShape`
- no C emitter body-specific semantics
- no source rewrite
- no VM fallback
- no change to Stage1 extern route eligibility

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p381i_phase29cg
KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381i_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the replay moves beyond missing `nyash.stage1.emit_*` declaration
errors.

## Result

Accepted.

`tools/build_hako_llvmc_ffi.sh` passes. The phase29cg replay passes end-to-end:

```text
[phase29cg] emit_program_rc=0 emit_mir_rc=0 llvm_rc=0 verify_rc=0 verify_count=
[PASS] phase29cg_stage2_bootstrap_phi_verify
```
