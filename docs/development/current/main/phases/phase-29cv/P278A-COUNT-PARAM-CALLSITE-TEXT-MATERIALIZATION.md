---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P278a, count-param callsite text materialization
Related:
  - docs/development/current/main/phases/phase-29cv/P277A-COUNT-PARAM-STEP-TEXT-NORMALIZER.md
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P278a: Count-Param Callsite Text Materialization

## Problem

P277a split subtraction step normalization, but the active source-execution
probe still reports:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

The remaining issue is not new count-loop semantics. `try_lower_text/1`
compares many owner-local helper results against the `""` sentinel. Those
helper calls are DirectAbi string helpers, but call result metadata can still be
observed as nullable/void-like before the parent body proves the value as a
plain String.

## Decision

Materialize helper call results at the callsite before sentinel comparisons:

```text
local value = "" + me._helper(...)
if value == "" { return "" }
```

This keeps the text sentinel contract local to the owner and avoids widening
generic string comparison semantics.

## Non-Goals

- no count-loop accepted-shape expansion
- no generic StringOrVoid-vs-String compare widening
- no new `GlobalCallTargetShape`
- no C shim/body-specific emitter change

## Acceptance

- `LowerLoopCountParamBox.try_lower_text/1` no longer reports
  `generic_string_unsupported_void_sentinel_const`.
- The source-execution probe advances to the next blocker.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Done.

The active source-execution probe now reports:

```text
LowerLoopCountParamBox.try_lower_text/1	generic_pure_string_body	string_handle	DirectAbi
```

The whole source-execution compile still stops later at the pure-first boundary:

```text
[llvm-pure/unsupported-shape] ... reason=no_lowering_variant
```

This card intentionally does not change that backend boundary. It only removes
the count-param helper-call sentinel blocker without widening generic string or
collection semantics.
