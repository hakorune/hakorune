---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AK, return logical text carrier cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381AJ-RETURN-BINOP-VARINT-TEXT-CARRIERS.md
  - lang/src/mir/builder/internal/lower_return_logical_box.hako
---

# P381AK: Return Logical Text Carriers

## Problem

After P381AJ, the direct Stage1 env EXE route reaches:

```text
LowerReturnLogicalBox.try_lower/1
%r358 = phi i64 [ %r340, ... ], [ %r352, ... ]
%r352 defined with type i1 but expected i64
```

The owner uses `null` as an internal miss sentinel for logical value carriers
(`lhs_true`/`rhs_true`) and later merges it with Bool/i64 payloads before
emitting MIR JSON text.

## Decision

Use text sentinels for logical value carriers and Bool predicates for debug
guards.

## Boundary

Allowed:

- change owner-internal logical carriers from `null` to `""`
- change local debug flag from numeric `0/1` to Bool
- keep accepted logical shapes and emitted MIR JSON unchanged

Not allowed:

- add backend null/Bool PHI repair
- widen return logical acceptance
- add route shapes

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381ak_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `LowerReturnLogicalBox.try_lower/1` no longer merges null sentinel with logical
  value carriers

## Result

Implemented. Logical value carriers now use text sentinels and `debug_on` is a
Bool predicate. The direct Stage1 env EXE route progressed past
`LowerReturnLogicalBox.try_lower/1` and exposed the next numeric-seed issue in
`MirJsonEmitBox._emit_phi_incoming_rec/3`.
