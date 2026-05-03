---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P292a, TypeOp Check accept-shape text search route
Related:
  - docs/development/current/main/phases/phase-29cv/P291A-TYPEOP-CAST-ACCEPT-SHAPE-INDEXOF.md
  - lang/src/mir/builder/internal/lower_typeop_check_box.hako
---

# P292a: TypeOp Check Accept-Shape Text Search Route

## Problem

After P291a, the source-execution probe advances to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=LowerTypeOpCheckBox._accept_shape/1
target_shape_blocker_reason=-
```

`LowerTypeOpCheckBox._accept_shape/1` is the Check sibling of the Cast shape
probe fixed in P291a. It returns scalar 0/1, but every `s.indexOf(pattern)`
method call in the probe has no `LoweringPlan` row and is observed as
`RuntimeDataBox.indexOf`.

## Decision

Route all accept-shape text searches through the existing DirectAbi helper:

```hako
StringHelpers.index_of(s, 0, pattern)
```

This keeps the helper scalar-only and avoids widening generic method
acceptance.

## Non-Goals

- no C shim method rediscovery
- no generic `indexOf` acceptance widening
- no new body shape
- no TypeOp Check lowering semantic change
- no fallback or compat route change

## Acceptance

- `_accept_shape/1` no longer has missing `indexOf` method routes.
- `LowerTypeOpCheckBox._accept_shape/1` is DirectAbi-compatible as a generic i64
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
symbol=LowerTypeOpCheckBox._accept_shape/1
```

The probe advances to the next blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirBuilderBox._try_emit_registry_program_json/2
target_shape_blocker_reason=-
```
