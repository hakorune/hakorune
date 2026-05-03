---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P280a, count-param final emit text materialization
Related:
  - docs/development/current/main/phases/phase-29cv/P278A-COUNT-PARAM-CALLSITE-TEXT-MATERIALIZATION.md
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P280a: Count-Param Final Emit Text Materialization

## Problem

Fresh source-execution metadata after P279a still reports:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

P278a materialized intermediate helper results before sentinel comparisons, but
the final return still directly returns a helper call:

```text
return me._emit_count_param_json(start_text, limit, step, cmp)
```

The helper itself is DirectAbi string, but the parent body still has many early
`""` sentinel returns. Keep the final return boundary parent-local too.

## Decision

Materialize the final emit result before returning:

```text
local emitted = "" + me._emit_count_param_json(start_text, limit, step, cmp)
return emitted
```

This keeps the text contract in the `.hako` owner and avoids widening generic
string / StringOrVoid handling.

## Non-Goals

- no new accepted count-loop shape
- no generic StringOrVoid widening
- no C shim/body-specific emitter change
- no new `GlobalCallTargetShape`

## Acceptance

- `LowerLoopCountParamBox.try_lower_text/1` no longer reports
  `generic_string_unsupported_void_sentinel_const`.
- The source-execution probe advances to the next blocker.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Done.

`LowerLoopCountParamBox.try_lower_text/1` now routes as:

```text
target_shape=generic_pure_string_body
return_shape=string_handle
tier=DirectAbi
```

Fresh source-execution advanced past the count-param blocker and now stops at
the next module generic prepass blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=BuilderDelegateFinalizeBox._build_user_box_decls_from_program_json/1
```
