---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381S, BuilderFallbackAuthority inline tail retire
Related:
  - docs/development/current/main/phases/phase-29cv/P381R-FALLBACK-AUTHORITY-BOOL-GUARD-CLEANUP.md
  - lang/src/mir/builder/internal/fallback_authority_box.hako
---

# P381S: Fallback Inline Tail Retire

## Problem

After P381R, the direct Stage1 env EXE route reaches LLVM type errors inside
old fallback-authority tails:

- `_try_inline_jsonfrag_cases/1`
- the count-param text lowerer call from `_try_boxed_lowerers`

Those tails are legacy Program(JSON) pattern code. The active owner path already
probes boxed lowerers first:

```text
LowerIfCompareBox
LowerReturnBinOpBox
LowerReturnIntBox
...
```

Keeping the inline tail in the active path forces pure-first to compile a large
second copy of old fallback pattern semantics and recreates Bool/i64 PHI issues.

## Decision

Retire `_try_inline_jsonfrag_cases/1` from the active `try_lower` path. If boxed
lowerers do not match, return `null` and let the normal unsupported owner handle
the miss.

Also remove the active count-param text lowerer call from
`BuilderFallbackAuthorityBox`; it has repeatedly reintroduced source-execution
body-shape pressure and is not the right Stage0 unblock seam.

The old function/lowerer text remains for now as a dead shelf. A later cleanup
card can delete it after inventory confirms no direct callers.

## Boundary

Allowed:

- stop calling `_try_inline_jsonfrag_cases/1` from `try_lower`
- stop calling `LowerLoopCountParamBox` from the active fallback authority path
- keep boxed lowerer order unchanged

Not allowed:

- delete public lowerer boxes
- add backend acceptance for the legacy inline tail
- change emitted MIR for matched boxed cases

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381s_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381s_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- pure-first no longer compiles the legacy inline fallback tail as an active
  dependency of Stage1 env EXE
- phase29cg remains green

## Result

Done:

- removed the active `_try_inline_jsonfrag_cases/1` call from
  `BuilderFallbackAuthorityBox.try_lower`
- removed the active `LowerLoopCountParamBox` fallback call from
  `_try_boxed_lowerers`
