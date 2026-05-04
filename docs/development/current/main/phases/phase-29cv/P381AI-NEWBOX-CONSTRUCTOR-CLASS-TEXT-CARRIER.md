---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AI, newbox constructor class carrier cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381AH-LOOP-SUM-BC-TEXT-CARRIERS.md
  - lang/src/mir/builder/internal/lower_newbox_constructor_box.hako
---

# P381AI: Newbox Constructor Class Text Carrier

## Problem

After P381AH, the direct Stage1 env EXE route reaches:

```text
LowerNewboxConstructorBox.try_lower/1
%r96 = phi i64 [ %r89, ... ], [ %r90, ... ]
%r90 defined with type i1 but expected i64
```

The owner uses `null` as the internal miss sentinel for `cls` and later merges it
with `"ArrayBox"`/`"MapBox"` string payloads.

## Decision

Use a text sentinel for the owner-internal class-name carrier.

## Boundary

Allowed:

- change `cls` internal sentinel from `null` to `""`
- keep accepted constructors and emitted MIR JSON unchanged

Not allowed:

- add backend null/string PHI repair
- widen constructor acceptance
- add route shapes

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381ai_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `LowerNewboxConstructorBox.try_lower/1` no longer merges null sentinel with
  class-name text

## Result

Implemented. `cls` now uses a text sentinel internally. The direct Stage1 env
EXE route progressed past `LowerNewboxConstructorBox.try_lower/1` and exposed
the next owner-local carrier issue in `LowerReturnBinOpVarIntBox.try_lower/1`.
