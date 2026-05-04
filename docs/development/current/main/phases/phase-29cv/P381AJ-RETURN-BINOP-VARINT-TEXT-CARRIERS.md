---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AJ, return binop var/int text carrier cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381AI-NEWBOX-CONSTRUCTOR-CLASS-TEXT-CARRIER.md
  - lang/src/mir/builder/internal/lower_return_binop_varint_box.hako
---

# P381AJ: Return BinOp VarInt Text Carriers

## Problem

After P381AI, the direct Stage1 env EXE route reaches:

```text
LowerReturnBinOpVarIntBox.try_lower/1
%r726 = phi i64 [ %r722, ... ], [ %r710, ... ]
%r722 defined with type i1 but expected i64
```

The owner uses `null` as an internal miss sentinel for `var_name` and numeric
text carriers, then merges those carriers with string or integer-text payloads.

## Decision

Use text sentinels for owner-internal carriers in both `Var + Int` and
`Int + Var` paths.

## Boundary

Allowed:

- change internal carrier sentinel from `null` to `""`
- keep accepted binary shapes and emitted MIR JSON unchanged

Not allowed:

- add backend null/string or null/i64 PHI repair
- widen return-binop acceptance
- add route shapes

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381aj_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `LowerReturnBinOpVarIntBox.try_lower/1` no longer merges null sentinel with
  var-name or numeric-text carriers

## Result

Implemented. Var-name and numeric-text carriers now use `""` internally in both
`Var + Int` and `Int + Var` branches. The direct Stage1 env EXE route progressed
past `LowerReturnBinOpVarIntBox.try_lower/1` and exposed the next owner-local
logical carrier issue in `LowerReturnLogicalBox.try_lower/1`.
