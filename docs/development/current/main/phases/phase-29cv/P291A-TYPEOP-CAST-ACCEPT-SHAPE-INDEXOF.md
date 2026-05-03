---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P291a, TypeOp Cast accept-shape text search route
Related:
  - docs/development/current/main/phases/phase-29cv/P290A-JSONFRAG-CONST-VALUE-SIG-TEXT-INT.md
  - lang/src/mir/builder/internal/lower_typeop_cast_box.hako
---

# P291a: TypeOp Cast Accept-Shape Text Search Route

## Problem

After P290a, the source-execution probe advances to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=LowerTypeOpCastBox._accept_shape/1
target_shape_blocker_reason=-
```

`LowerTypeOpCastBox._accept_shape/1` is an i64 shape probe. It returns scalar
0/1, but every `s.indexOf(pattern)` method call in the probe has no
`LoweringPlan` row. This is a source-side text-search routing issue, not a
reason to widen generic method acceptance or teach the C shim to rediscover
method semantics.

## Decision

Route all accept-shape text searches through the existing DirectAbi helper:

```hako
StringHelpers.index_of(s, 0, pattern)
```

This keeps the helper scalar-only and avoids method-route inference from a
PHI/unknown receiver.

## Non-Goals

- no C shim method rediscovery
- no generic `indexOf` acceptance widening
- no new body shape
- no TypeOp Cast lowering semantic change
- no fallback or compat route change

## Acceptance

- `_accept_shape/1` no longer has missing `indexOf` method routes.
- `LowerTypeOpCastBox._accept_shape/1` is DirectAbi-compatible as a generic i64
  helper.
- The source-execution probe advances to the next blocker or produces the exe.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Done.

The source-execution probe now records all seven `_accept_shape/1` text-search
sites as explicit `StringHelpers.index_of/3` DirectAbi routes:

```text
symbol=StringHelpers.index_of/3
symbol=LowerTypeOpCastBox._accept_shape/1
```

The probe advances to the next blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=LowerTypeOpCheckBox._accept_shape/1
target_shape_blocker_reason=-
```
