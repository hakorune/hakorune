---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow known-intrinsic comment cleanup
Related:
  - src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs
  - src/mir/control_tree/normalized_shadow/common/known_intrinsics.rs
  - docs/development/current/main/phases/phase-291x/291x-420-normalized-shadow-known-intrinsic-comment-inventory-card.md
---

# 291x-421: Normalized-Shadow Known-Intrinsic Comment Cleanup

## Goal

Remove stale transition wording from the `KnownIntrinsic` contract comment.

This is a BoxShape cleanup. No intrinsic behavior changed.

## Change

Updated the `KnownIntrinsic` comment to describe the current structure:

```text
KnownIntrinsic             = semantic marker enum
KnownIntrinsicRegistryBox  = method name / arity / return metadata owner
```

## Preserved Behavior

- `KnownIntrinsic` variants are unchanged.
- `KnownIntrinsicRegistryBox` is unchanged.
- Lookup behavior is unchanged.
- Intrinsic allowlist contents are unchanged.

## Verification

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "REMOVED|deprecated|method_name\\(\\) and arity\\(\\)" \
  src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs
```

## Next Cleanup

Inventory the next compiler-cleanliness seam after known-intrinsic comment
cleanup.
