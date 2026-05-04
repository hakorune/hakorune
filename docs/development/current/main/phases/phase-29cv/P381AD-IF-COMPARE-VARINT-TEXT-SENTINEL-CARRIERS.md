---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AD, if.compare varint local sentinel cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381AB-FOLD-VARINT-TEXT-SENTINEL-CARRIERS.md
  - lang/src/mir/builder/internal/lower_if_compare_varint_box.hako
---

# P381AD: If Compare VarInt Text Sentinel Carriers

## Problem

After P381AC, the direct Stage1 env EXE route reaches:

```text
LowerIfCompareVarIntBox.try_lower/1
%r392 = phi i64 [ %r367, ... ], [ %r361, ... ]
%r367 defined with type i1 but expected i64
```

The owner initializes `aval`/`bval` as `null` and later assigns parsed integer
payloads. That mixes null sentinel and numeric payload in one local PHI.

## Decision

Use text sentinels for owner-internal resolved value carriers and keep external
misses as `null`.

## Boundary

Allowed:

- change `aval`/`bval` local carrier sentinel from `null` to `""`
- keep accepted shapes and emitted MIR unchanged

Not allowed:

- add backend null/i64 PHI repair
- widen varint acceptance
- add route shapes

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381ad_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `LowerIfCompareVarIntBox.try_lower/1` no longer merges null sentinel with
  i64 payload locals

## Result

Implemented. `LowerIfCompareVarIntBox` now carries optional text state with
`""` sentinels rather than `null`.
