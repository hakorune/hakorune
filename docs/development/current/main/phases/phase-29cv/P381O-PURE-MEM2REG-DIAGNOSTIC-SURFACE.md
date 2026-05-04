---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381O, pure-first mem2reg diagnostic surface
Related:
  - docs/development/current/main/phases/phase-29cv/P381N-STAGE1-ENV-MIR-SHAPE-GUARD-SSOT.md
  - lang/c-abi/shims/hako_llvmc_ffi_common.inc
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
---

# P381O: Pure Mem2reg Diagnostic Surface

## Problem

The bad route is now narrowed to:

```text
full stage1_cli_env.hako
  -> direct --emit-exe
  -> ny-llvmc pure-first generic module emit
  -> opt -passes=mem2reg
  -> unsupported_pure_shape
```

The failure is not the Program(JSON)->MIR bridge keeper. It is also not a new
source contract issue from P381M/P381N.

The current C shim hides the useful failure:

- `opt -passes=mem2reg` redirects stderr to `/dev/null`
- the raw generated `.ll` is removed before the caller can inspect it
- `NYASH_LLVM_DUMP_IR` only dumps the canonicalized IR after mem2reg succeeds

That turns the real LLVM verifier reason into the generic
`unsupported pure shape` message.

## Decision

Make pure-first mem2reg failure fail-fast with evidence:

- capture `opt` stderr into a temp file
- print the first diagnostic lines under a stable tag only on failure
- copy the raw generated `.ll` to `NYASH_LLVM_DUMP_IR` on mem2reg failure

This is diagnostic surface only. It does not add a backend body shape, does not
change lowering acceptance, and does not touch Program(JSON)->MIR bridge routing.

## Boundary

Allowed:

- improve `mem2reg` failure diagnostics
- preserve raw IR only when the caller explicitly sets `NYASH_LLVM_DUMP_IR`
- keep success behavior unchanged

Not allowed:

- add a new `GlobalCallTargetShape`
- bypass mem2reg
- accept malformed IR
- route through VM fallback

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh

rm -f /tmp/p381o_stage1_env.ll
NYASH_DISABLE_PLUGINS=1 \
NYASH_LLVM_DUMP_IR=/tmp/p381o_stage1_env.ll \
target/release/hakorune --emit-exe /tmp/p381o_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- the direct Stage1 env EXE route still fails until the real IR bug is fixed
- stderr includes `[llvm-mem2reg/error]`
- `/tmp/p381o_stage1_env.ll` contains the raw generated IR on failure

## Result

Done:

- mem2reg failure now preserves the raw IR through `NYASH_LLVM_DUMP_IR`
- mem2reg failure now prints the hidden `opt` diagnostic under
  `[llvm-mem2reg/error]`
