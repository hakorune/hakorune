---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AH, loop sum/break/continue text carrier cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381AG-LOOP-MULTI-CARRIER-REGISTER-I64-CONTRACT.md
  - lang/src/mir/builder/internal/lower_loop_sum_bc_box.hako
---

# P381AH: Loop Sum BC Text Carriers

## Problem

After P381AG, the direct Stage1 env EXE route reaches:

```text
LowerLoopSumBcBox.try_lower/1
%r239 = phi i64 [ %r234, ... ], [ %r225, ... ]
%r234 defined with type i1 but expected i64
```

The owner uses `null` as an internal miss sentinel for text carriers such as
`varname` and later merges it with string payloads. That creates a Bool/string
PHI before the owner checks for miss.

## Decision

Use text sentinels for owner-internal text carriers and keep external miss
behavior as `null` only at owner boundaries.

## Boundary

Allowed:

- change `varname`/numeric-text carriers to `""` internally
- keep emitted MIR JSON and accepted loop sum/break/continue shape unchanged

Not allowed:

- add backend null/string PHI repair
- widen loop sum/break/continue acceptance
- add route shapes

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381ah_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `LowerLoopSumBcBox.try_lower/1` no longer merges null sentinel with string
  carriers

## Result

Implemented. `varname`, `limit`, `skip_value`, and `break_value` now use
owner-internal text sentinels. The direct Stage1 env EXE route progressed past
`LowerLoopSumBcBox.try_lower/1` and exposed the next owner-local class-name
sentinel issue in `LowerNewboxConstructorBox.try_lower/1`.
