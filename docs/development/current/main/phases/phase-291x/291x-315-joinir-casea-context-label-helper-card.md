---
Status: Landed
Date: 2026-04-26
Scope: JoinIR Case-A context-label helper cleanup
Related:
  - src/mir/join_ir/lowering/loop_scope_shape/case_a.rs
  - src/mir/join_ir/lowering/loop_scope_shape/mod.rs
  - src/mir/join_ir/lowering/loop_to_join/case_a_entrypoints.rs
  - docs/development/current/main/phases/phase-291x/291x-314-joinir-casea-context-label-inventory-card.md
---

# 291x-315: JoinIR Case-A Context-label Helper Cleanup

## Goal

Make Case-A wrapper context labels consume the descriptor-owned minimal target
table, while keeping Stage-B bridge labels separate from Case-A minimal
acceptance policy.

This is behavior-preserving BoxShape cleanup.

## Change

Added a descriptor consumer helper:

```text
case_a_minimal_target_name(kind)
```

Updated minimal Case-A wrappers in:

```text
src/mir/join_ir/lowering/loop_to_join/case_a_entrypoints.rs
```

The wrappers now use:

```text
CaseAMinimalTargetKind -> descriptor table -> function label
```

instead of repeating the four exact minimal target strings.

The Stage-B bridge labels remain local constants in `case_a_entrypoints.rs`:

```text
StageBBodyExtractorBox.build_body_src/2
StageBFuncScannerBox.scan_all_boxes/1
```

These are context labels only. They are not Case-A minimal acceptance rows.

## Preserved Behavior

- The accepted Case-A minimal target set is unchanged.
- Stage-B bridge wrappers still pass the same context labels.
- `LoopToJoinLowerer::lower(...)` still receives the same `func_name` values.
- No generic Case-A route behavior changed.

## Non-Goals

- No Case-A target expansion.
- No Case-A target deletion.
- No Stage-B target reclassification.
- No shape-based route change.
- No strict/dev behavior change.

## Validation

```bash
cargo test -q case_a_minimal_target
cargo test -q test_is_loop_lowered_function
```
