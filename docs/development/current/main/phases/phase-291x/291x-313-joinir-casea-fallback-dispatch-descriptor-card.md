---
Status: Landed
Date: 2026-04-26
Scope: JoinIR Case-A fallback dispatch descriptor consumer
Related:
  - src/mir/join_ir/lowering/loop_scope_shape/case_a.rs
  - src/mir/join_ir/lowering/loop_scope_shape/mod.rs
  - src/mir/join_ir/lowering/loop_view_builder.rs
  - docs/development/current/main/phases/phase-291x/291x-312-joinir-casea-target-descriptor-table-card.md
---

# 291x-313: JoinIR Case-A Fallback Dispatch Descriptor Consumer

## Goal

Make `LoopViewBuilder` consume the Case-A target descriptor instead of
duplicating exact function-name policy in fallback dispatch.

This is behavior-preserving BoxShape cleanup.

## Change

Re-exported the Case-A descriptor lookup from `loop_scope_shape`:

```text
find_case_a_minimal_target(...)
CaseAMinimalTargetKind
```

Updated:

```text
LoopViewBuilder::dispatch_by_name(...)
```

The fallback dispatch now:

```text
function name -> find_case_a_minimal_target(...) -> kind -> lowerer
```

instead of matching the same exact function names again.

## Preserved Behavior

The fallback lowerer mapping remains:

```text
SkipWhitespace       -> lower_case_a_skip_ws_with_scope
Trim                 -> lower_case_a_trim_with_scope
AppendDefs           -> lower_case_a_append_defs_with_scope
Stage1UsingResolver  -> lower_case_a_stage1_usingresolver_with_scope
```

Unsupported names still return `None` from fallback dispatch.

## Non-Goals

- No Case-A target expansion.
- No Case-A target deletion.
- No shape-based route change.
- No bridge target table change.
- No cleanup of lowerer context-label strings in `case_a_entrypoints.rs`.

## Validation

```bash
cargo test -q case_a_minimal_target
cargo test -q test_is_loop_lowered_function
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
