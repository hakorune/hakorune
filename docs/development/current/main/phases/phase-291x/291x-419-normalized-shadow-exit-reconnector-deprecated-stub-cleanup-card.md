---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow exit-reconnector deprecated stub cleanup
Related:
  - src/mir/control_tree/normalized_shadow/exit_reconnector.rs
  - docs/development/current/main/phases/phase-291x/291x-418-normalized-shadow-exit-reconnector-deprecated-stub-inventory-card.md
---

# 291x-419: Normalized-Shadow Exit-Reconnector Deprecated Stub Cleanup

## Goal

Remove the unused deprecated extraction stub from `ExitReconnectorBox`.

This is a BoxShape cleanup. No reconnection behavior changed.

## Change

Removed:

```text
ExitReconnectorBox::extract_k_exit_jump_args(...)
```

and the commented old extraction implementation next to it.

## Preserved Behavior

- `ExitReconnectorBox::reconnect(...)` is unchanged.
- Direct variable-map reconnection tests are unchanged.
- `MergeResult.remapped_exit_values` remains the SSOT for remapped exit values.
- Module export shape is unchanged.

## Verification

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "extract_k_exit_jump_args|Deprecated - boundary approach|extract_k_exit_jump_args_old" \
  src/mir/control_tree/normalized_shadow/exit_reconnector.rs
```

## Next Cleanup

Inventory the next compiler-cleanliness seam after the exit-reconnector
deprecated stub cleanup.
