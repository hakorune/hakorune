---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P302a, LowerLoopCountParam text boundary split
Related:
  - docs/development/current/main/phases/phase-29cv/P301A-MIR-JSON-CALLEE-FIELD-GET-CONSUME.md
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P302a: LowerLoopCountParam Text Boundary Split

## Problem

After P301a, the source-execution probe advances past the MIR JSON emitter
field-read sequence and stops at:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

`LowerLoopCountParamBox.try_lower_text/1` is a source-side text lowerer. It
already uses `""` as the failed-probe sentinel, but it keeps compare
normalization and step extraction inside one large function with branch-local
assignments to `cmp`, `limit`, and `step`.

That shape makes the route classifier see string flows through broad
branch-join PHIs and can reintroduce void/null sentinel classification pressure
even though the owner contract is textual.

## Decision

Split the source owner into smaller text helpers:

```text
_cmp_limit_lhs_text(...)
_cmp_limit_rhs_text(...)
_step_text(...)
_finish_count_param_text(...)
```

Each helper returns `""` on failed probe and avoids carrying partially-filled
string locals across wide control-flow joins.

This keeps the cleanup in the `.hako` owner and avoids adding any new Stage0
C shim/body classifier semantics.

## Non-Goals

- no C shim acceptance widening
- no new `GlobalCallTargetShape`
- no new runtime helper route
- no parser/loop pattern policy change
- no fallback or externalization

## Acceptance

- `LowerLoopCountParamBox.try_lower_text/1` no longer reports
  `generic_string_unsupported_void_sentinel_const`.
- The probe advances to the next blocker or emits successfully.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Accepted.

`LowerLoopCountParamBox.try_lower_text/1` no longer carries compare `cmp` /
`limit` and body `step` partial state through one wide body. The source owner
now resolves compare limits and step text through smaller helpers that keep the
failed-probe sentinel as `""`.

The source-execution probe advances past
`LowerLoopCountParamBox.try_lower_text/1` and stops at the next module generic
prepass blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_effects_rec/3
target_shape_blocker_reason=-
```
