---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AE, if.compare varvar name sentinel cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381AD-IF-COMPARE-VARINT-TEXT-SENTINEL-CARRIERS.md
  - lang/src/mir/builder/internal/lower_if_compare_varvar_box.hako
---

# P381AE: If Compare VarVar Name Text Sentinels

## Problem

After P381AD, the direct Stage1 env EXE route reaches:

```text
LowerIfCompareVarVarBox.try_lower/1
%r148 = phi i64 [ %r142, ... ], [ %r134, ... ]
%r142 defined with type i1 but expected i64
```

The owner initializes `lhs_name`/`rhs_name` as `null` and later assigns string
name payloads. That mixes null sentinel and string-handle payload in one local
PHI.

## Decision

Use text sentinels for owner-internal name carriers and keep external miss
behavior as `null`.

## Boundary

Allowed:

- change `lhs_name`/`rhs_name` local sentinel from `null` to `""`
- keep accepted shapes and emitted MIR unchanged

Not allowed:

- add backend null/string PHI repair
- widen varvar acceptance
- add route shapes

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381ae_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `LowerIfCompareVarVarBox.try_lower/1` no longer merges null sentinel with
  name string locals

## Result

Implemented. `lhs_name` and `rhs_name` now use text sentinels internally and
only return external miss as `null`. The direct Stage1 env EXE route progressed
past `LowerIfCompareVarVarBox.try_lower/1` and exposed the next owner-local
Bool/numeric guard in `LowerIfNestedBox._read_cmp_side_int/4`.
