---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P277a, count-param step text normalizer split
Related:
  - docs/development/current/main/phases/phase-29cv/P272A-COUNT-PARAM-TEXT-CONTROL-CLEANUP.md
  - docs/development/current/main/phases/phase-29cv/P276A-STAGE1-MIR-DEBUG-PRINT-RETURN-CONTRACT.md
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P277a: Count-Param Step Text Normalizer

## Problem

After P276a, the active source-execution route reaches the count-param loop
lowerer again:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

P272a removed boolean-negation noise and split final JSON emission, but
`try_lower_text/1` still normalizes subtraction by mutating `step` inside a
late branch:

```text
if bop_minus == 1 {
  step = StringHelpers.int_to_str(0 - JsonFragBox._str_to_int(step))
}
```

That keeps the main lowerer carrying a late PHI for the final string argument.

## Decision

Move step normalization into an owner-local text helper:

```text
_normalize_step_text(step, bop_minus) -> String
```

`try_lower_text/1` remains orchestration plus direct JSON handoff. The helper
owns the subtraction conversion and always returns text.

## Non-Goals

- no count-loop accepted-shape expansion
- no new `GlobalCallTargetShape`
- no generic void-sentinel classifier change
- no C shim/body-specific emitter change

## Acceptance

- `LowerLoopCountParamBox.try_lower_text/1` no longer reports
  `generic_string_unsupported_void_sentinel_const`.
- The source-execution probe advances to the next blocker.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Accepted.

The count-param lowerer now stays on the generic string DirectAbi route:

```text
LowerLoopCountParamBox._normalize_step_text/2  generic_pure_string_body  string_handle  DirectAbi
LowerLoopCountParamBox.try_lower_text/1        generic_pure_string_body  string_handle  DirectAbi
```

The source-execution probe now reaches the backend recipe boundary:

```text
reason=no_lowering_variant
```

Route metadata still records later unsupported object-return owners such as
`MirBuilderSourceCompatBox.emit_root_from_source_v0/2 ->
MirRootHydratorBox._parse_object/1`, but the immediate probe failure is no
longer `LowerLoopCountParamBox.try_lower_text/1`.
