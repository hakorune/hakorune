---
Status: Landed
Date: 2026-04-26
Scope: JoinIR Case-A target descriptor table split
Related:
  - src/mir/join_ir/lowering/loop_scope_shape/case_a.rs
  - docs/development/current/main/phases/phase-291x/291x-311-joinir-casea-name-policy-inventory-card.md
---

# 291x-312: JoinIR Case-A Target Descriptor Table

## Goal

Move the Case-A minimal target acceptance set into a local descriptor table.

This is behavior-preserving BoxShape cleanup.

## Change

Added:

```text
CaseAMinimalTargetKind
CaseAMinimalTargetDesc
CASE_A_MINIMAL_TARGETS
find_case_a_minimal_target(...)
```

Updated:

```text
is_case_a_minimal_target(...)
```

The accepted Case-A minimal subset remains:

```text
Main.skip/1
FuncScannerBox.trim/1
FuncScannerBox.append_defs/2
Stage1UsingResolverBox.resolve_for_source/5
```

## Preserved Behavior

The Stage-B loop targets remain outside the Case-A minimal subset:

```text
StageBBodyExtractorBox.build_body_src/2
StageBFuncScannerBox.scan_all_boxes/1
```

No fallback dispatch behavior changed in this slice.

## Non-Goals

- No Case-A target expansion.
- No Case-A target deletion.
- No `loop_view_builder::dispatch_by_name(...)` change.
- No bridge target table change.
- No structural Case-A analysis change.

## Validation

```bash
cargo test -q case_a_minimal_target
cargo test -q test_is_loop_lowered_function
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
