---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381H, direct global-call string literal argument materialization
Related:
  - docs/development/current/main/phases/phase-29cv/P381G-MODULE-GENERIC-PRINT-SSA-SITE-NAME.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
---

# P381H: Direct Global-Call Literal Arg Materialize

## Problem

P381G fixed duplicate print-call SSA temporary names. The phase29cg replay then
reached a new LLVM verifier error:

```text
opt: /tmp/p381g_bad.ll:1208:52: error: use of undefined value '%r12'
  %r14 = call i64 @"Main._run_emit_mir_mode/1"(i64 %r12)
                                                   ^
```

`%r12` is a string literal argument to a direct same-module global call:

```text
Main._run_emit_mir_mode("[freeze:contract][stage1-cli/emit-mir]")
```

The generic lowering prepass may skip hoisted string literal box creation when a
literal looks like a direct raw-ptr/string-window candidate. Direct global-call
emission then formats the argument as `%r12` without ensuring that the skipped
literal has been materialized as a handle.

## Decision

Before emitting a direct global-call from a LoweringPlan route, materialize any
string literal argument whose box has not already been emitted:

```text
if arg is string literal and no %r<arg> box exists:
  emit nyash.box.from_i8_string_const for that arg
emit direct function call with i64 %r<arg>
```

This aligns two existing backend contracts: literal-hoist may skip a box for
string-window lowering, but direct function calls require handle arguments. It
does not add body semantics or widen call acceptance.

## Non-Goals

- no new `GlobalCallTargetShape`
- no `.hako` source rewrite
- no widening of generic call classification
- no VM fallback
- no special case for `Main._run_emit_mir_mode/1`

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p381h_phase29cg
KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381h_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the replay moves beyond the undefined string literal argument
`%r12`/`%r17`/`%r22` errors.

## Result

Accepted.

`tools/build_hako_llvmc_ffi.sh` passes. The phase29cg replay now materializes
the branch-local literal call arguments before the direct calls:

```llvm
%r12 = call i64 @"nyash.box.from_i8_string_const"(...)
%r14 = call i64 @"Main._run_emit_mir_mode/1"(i64 %r12)
```

The replay moves beyond the undefined `%r12`/`%r17`/`%r22` argument errors and
reaches the next backend declaration blocker:

```text
opt: /tmp/p381h_bad.ll:920:18: error: use of undefined value '@nyash.stage1.emit_mir_from_program_json_v0_h'
  %r2 = call i64 @"nyash.stage1.emit_mir_from_program_json_v0_h"(i64 %r1)
                 ^
```
