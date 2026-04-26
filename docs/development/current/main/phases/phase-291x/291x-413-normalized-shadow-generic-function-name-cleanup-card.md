---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow generic function-name cleanup
Related:
  - src/mir/control_tree/normalized_shadow/entry/if_only.rs
  - src/mir/control_tree/normalized_shadow/if_as_last_join_k.rs
  - src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs
  - src/mir/control_tree/normalized_shadow/post_if_post_k.rs
  - src/mir/join_ir/lowering/canonical_names.rs
  - docs/development/current/main/phases/phase-291x/291x-412-normalized-shadow-generic-function-name-inventory-card.md
---

# 291x-413: Normalized-Shadow Generic Function-Name Cleanup

## Goal

Replace normalized-shadow raw generic function names with existing
`canonical_names` constants.

This is a BoxShape cleanup. No emitted function names changed.

## Change

Updated normalized-shadow routes to use:

```text
cn::MAIN
cn::LOOP_STEP
cn::K_EXIT
cn::POST_K
```

instead of direct string literals:

```text
"main"
"loop_step"
"k_exit"
"post_k"
```

## Preserved Behavior

- Route-local names `k_then`, `k_else`, and `join_k` remain route-local.
- `JoinFuncId` numbering is unchanged.
- Bridge-visible function keys are unchanged.
- Route acceptance is unchanged.

## Verification

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n 'JoinFunction::new\([^\n]+\"(main|loop_step|k_exit|post_k)\"|\"(main|loop_step|k_exit|post_k)\"\.to_string\(\)' \
  src/mir/control_tree/normalized_shadow -g '*.rs'
```

## Next Cleanup

Inventory the next compiler-cleanliness seam. Keep route-local function-name
policy separate from generic canonical-name cleanup.
