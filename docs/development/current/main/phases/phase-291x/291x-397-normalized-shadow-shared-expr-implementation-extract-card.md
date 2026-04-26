---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow shared expression implementation extraction
Related:
  - src/mir/control_tree/normalized_shadow/support/expr_lowering.rs
  - src/mir/control_tree/normalized_shadow/support/README.md
  - src/mir/control_tree/normalized_shadow/legacy/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-396-normalized-shadow-shared-expr-facade-card.md
---

# 291x-397: Normalized-Shadow Shared Expr Implementation Extract

## Goal

Move the shared assignment and minimal-compare helper implementation out of the
normalized-shadow legacy entry owner.

This is a BoxShape cleanup. It does not change accepted StepTree shapes or
lowering semantics.

## Change

Moved implementation ownership for:

```text
lower_assign_stmt
parse_minimal_compare
```

from:

```text
normalized_shadow/legacy/mod.rs
```

to:

```text
normalized_shadow/support/expr_lowering.rs
```

`LegacyLowerer` now calls the support owner for its old entry path instead of
owning those helper implementations itself.

## Preserved Behavior

- Fail-fast tags are preserved.
- Route lowerers continue using `support::expr_lowering`.
- The old if-only legacy entry route remains in `LegacyLowerer`.
- No StepTree acceptance or route priority changed.

## Next Cleanup

Inventory the remaining `LegacyLowerer` entry surface:

```text
lower_if_only_to_normalized
lower_return_from_tree
lower_if_node
lower_return_value
verify_branch_is_return_literal
```

Prefer another small BoxShape slice before moving the full legacy entry path.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
