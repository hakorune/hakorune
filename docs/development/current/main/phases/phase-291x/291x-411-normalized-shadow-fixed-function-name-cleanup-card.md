---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow fixed function-name cleanup
Related:
  - src/mir/control_tree/normalized_shadow/loop_true_break_once.rs
  - src/mir/join_ir/lowering/canonical_names.rs
  - docs/development/current/main/phases/phase-291x/291x-410-normalized-shadow-fixed-function-name-inventory-card.md
---

# 291x-411: Normalized-Shadow Fixed Function-Name Cleanup

## Goal

Move the fixed `loop_true_break_once.rs` `join_func_*` names behind the
canonical-name SSOT.

This is a BoxShape cleanup. No emitted function names changed.

## Change

Added normalized-shadow compatibility constants:

```text
NORMALIZED_SHADOW_MAIN      = "join_func_0"
NORMALIZED_SHADOW_LOOP_STEP = "join_func_1"
NORMALIZED_SHADOW_K_EXIT    = "join_func_2"
NORMALIZED_SHADOW_LOOP_BODY = "join_func_3"
NORMALIZED_SHADOW_POST_K    = "join_func_4"
```

Updated `loop_true_break_once.rs` to use those constants instead of raw
`"join_func_*"` literals.

## Preserved Behavior

- `JoinFuncId` numbering is unchanged.
- Bridge-visible function keys are unchanged.
- StepTree acceptance is unchanged.
- `loop_true_if_break_continue.rs` and `post_if_post_k.rs` are untouched.

## Verification

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n '"join_func_[0134]"' \
  src/mir/control_tree/normalized_shadow/loop_true_break_once.rs
```

## Next Cleanup

Inventory the next compiler-cleanliness seam. Keep generic canonical-name
cleanup for other normalized-shadow routes separate from fixed
`loop_true_break_once` function-id cleanup.
