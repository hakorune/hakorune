---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381G, module generic print SSA temporary naming
Related:
  - docs/development/current/main/phases/phase-29cv/P381F-STAGE1-SHORT-CIRCUIT-NESTED-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P381C-MODULE-GENERIC-PRINT-ARG0.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
---

# P381G: Module Generic Print SSA Site Name

## Problem

P381F removed the Stage1 short-circuit bool PHI type errors. The phase29cg replay
then reached LLVM verification and failed on repeated print result names:

```text
opt: /tmp/p381f_bad.ll:1015:3: error: multiple definition of local value named 'print_call_2'
  %print_call_2 = call i64 @"nyash.console.log_handle"(i64 %r2)
```

The failing function prints the same source register from multiple branch blocks.
The shared module-generic print emitter names the ignored print return as
`%print_call_<arg-reg>`, so repeated prints of the same argument register produce
duplicate LLVM SSA locals.

## Decision

Name print-call temporaries by MIR site instead of argument register:

```text
%print_call_b<block-id>_i<instruction-index>
```

The callsite already has `bid` and `ii`, and MIR block plus instruction index is
the stable owner of one instruction. This fixes LLVM hygiene without changing
print acceptance, adding a body shape, or teaching Stage0 any `.hako` compiler
semantics.

## Non-Goals

- no new `GlobalCallTargetShape`
- no wider print classifier acceptance
- no source rewrite to avoid repeated prints
- no VM fallback
- no boolean PHI repair

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p381g_phase29cg
KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381g_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the replay moves beyond the duplicate `print_call_*` LLVM local-name
error.

## Result

Accepted.

`tools/build_hako_llvmc_ffi.sh` passes. The phase29cg replay no longer reports
the duplicate `print_call_*` local-name error. It moves to the next backend
hygiene blocker:

```text
opt: /tmp/p381g_bad.ll:1208:52: error: use of undefined value '%r12'
  %r14 = call i64 @"Main._run_emit_mir_mode/1"(i64 %r12)
                                                   ^
```

That is a separate direct global-call argument materialization issue: a skipped
string literal used as a function argument must be materialized before the call.
