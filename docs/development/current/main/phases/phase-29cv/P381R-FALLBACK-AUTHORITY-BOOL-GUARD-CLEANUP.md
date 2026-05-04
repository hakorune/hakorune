---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381R, fallback authority bool guard cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381Q-BOX-INTROSPECT-TRACE-FLAG-TEXT.md
  - lang/src/mir/builder/internal/fallback_authority_box.hako
---

# P381R: Fallback Authority Bool Guard Cleanup

## Problem

After P381Q, the direct Stage1 env EXE route advances to another LLVM type
error:

```text
BuilderFallbackAuthorityBox._try_inline_jsonfrag_cases/1
%r282 = icmp ne i64 %r269, 1
```

The source guard builds `op_supported` as a scalar flag and then compares it
with `1`. The pure-first route has already classified that flag as Bool (`i1`),
so the generated IR compares an `i1` as if it were `i64`.

## Decision

Use the flag as a Bool condition directly:

```hako
if !op_supported { ... }
```

This keeps the old fallback policy unchanged while avoiding a false i64
comparison.

## Boundary

Allowed:

- replace `flag != 1` with Bool-condition guards for local support flags

Not allowed:

- change fallback matching policy
- add backend bool-to-i64 rescue for this owner code
- add a new lowering shape

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381r_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- direct Stage1 env EXE route no longer fails on fallback authority Bool/i64
  support-flag comparisons

## Result

Done:

- converted fallback support-flag checks to Bool-condition guards
