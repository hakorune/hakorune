---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AB, fold varint local sentinel cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381AA-JSON-NUMBER-CANONICAL-I64-SENTINELS.md
  - lang/src/mir/builder/internal/lower_if_compare_fold_varint_box.hako
---

# P381AB: Fold VarInt Text Sentinel Carriers

## Problem

After P381AA, the direct Stage1 env EXE route reaches:

```text
LowerIfCompareFoldVarIntBox._fold_bin_varint/3
%r581 = phi i64 [ %r569, ... ], [ %r539, ... ]
%r569 defined with type i1 but expected i64
```

The owner initializes `vval`/`ival` as `null` and later assigns parsed integer
values. That mixes null sentinel and numeric payload in one local PHI.

## Decision

Keep external miss behavior as `null`, but use owner-internal text sentinels
for the local carriers:

```text
""       -> unresolved local carrier
"<int>"  -> resolved numeric text
```

## Boundary

Allowed:

- change `vval`/`ival` local carrier sentinel from `null` to `""`
- keep return values and emitted MIR unchanged

Not allowed:

- add backend null/i64 PHI repair
- widen fold-varint acceptance
- add a route shape

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381ab_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `_fold_bin_varint/3` no longer merges null sentinel with i64 payload locals

## Result

Implemented. `LowerIfCompareFoldVarIntBox` now carries optional text state with
`""` sentinels rather than `null`.
