---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow k-exit naming cleanup
Related:
  - src/mir/control_tree/normalized_shadow/loop_true_break_once.rs
  - src/mir/join_ir/lowering/canonical_names.rs
  - docs/development/current/main/phases/phase-291x/291x-408-normalized-shadow-k-exit-naming-inventory-card.md
---

# 291x-409: Normalized-Shadow K-Exit Naming Cleanup

## Goal

Remove misleading `K_EXIT_LEGACY` terminology from active source while keeping
the normalized-shadow bridge-visible function key unchanged.

This is a BoxShape cleanup. No StepTree acceptance or lowering behavior changed.

## Change

Renamed the active canonical-name constant:

```text
K_EXIT_LEGACY -> NORMALIZED_SHADOW_K_EXIT
```

Preserved the value:

```text
NORMALIZED_SHADOW_K_EXIT = "join_func_2"
```

Updated normalized-shadow loop metadata and tests to use the new name.

## Preserved Behavior

- `JoinFuncId::new(2)` remains the normalized-shadow exit continuation.
- The emitted function key remains `"join_func_2"`.
- `K_EXIT` remains `"k_exit"` for canonical loop routes.
- `JoinFragmentMeta.continuation_funcs` still records the exact function key
  used by the normalized-shadow module.

## Verification

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "K_EXIT_LEGACY|legacy variant for normalized shadow" \
  src/mir/control_tree/normalized_shadow/loop_true_break_once.rs \
  src/mir/join_ir/lowering/canonical_names.rs
```

## Next Cleanup

Inventory the next compiler-cleanliness seam after the normalized-shadow
k-exit naming cleanup. Keep this as a separate BoxShape card before touching
any new code surface.
