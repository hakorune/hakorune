---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow shared expression lowering facade
Related:
  - src/mir/control_tree/normalized_shadow/support/README.md
  - src/mir/control_tree/normalized_shadow/support/mod.rs
  - src/mir/control_tree/normalized_shadow/support/expr_lowering.rs
  - src/mir/control_tree/normalized_shadow/if_as_last_join_k.rs
  - src/mir/control_tree/normalized_shadow/post_if_post_k.rs
  - src/mir/control_tree/normalized_shadow/loop_true_break_once.rs
  - docs/development/current/main/phases/phase-291x/291x-395-normalized-shadow-legacy-lowerer-inventory-card.md
---

# 291x-396: Normalized-Shadow Shared Expr Facade

## Goal

Stop route lowerers from importing the normalized-shadow legacy entry owner for
shared expression helper behavior.

This is a BoxShape cleanup. StepTree acceptance and lowering semantics are
unchanged.

## Change

Added:

```text
src/mir/control_tree/normalized_shadow/support/
  README.md
  mod.rs
  expr_lowering.rs
```

The facade currently exposes:

```text
expr_lowering::lower_assign_stmt
expr_lowering::parse_minimal_compare
```

Migrated these route lowerers from `LegacyLowerer` helper imports to the new
support facade:

```text
if_as_last_join_k.rs
post_if_post_k.rs
loop_true_break_once.rs
```

`builder.rs` still calls `LegacyLowerer::lower_if_only_to_normalized` because
that is the legacy entry path, not the shared helper seam.

## Preserved Behavior

- The facade delegates to the existing implementation in this card.
- Existing fail-fast tags and out-of-scope behavior are unchanged.
- No new StepTree shape is accepted.

## Next Cleanup

Physically move the shared expression helper implementation out of
`legacy/mod.rs` and into `support::expr_lowering`, then leave the legacy entry
path depending on the support implementation.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
