---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow legacy entry surface inventory
Related:
  - src/mir/control_tree/normalized_shadow/legacy/mod.rs
  - src/mir/control_tree/normalized_shadow/builder.rs
  - docs/development/current/main/phases/phase-291x/291x-397-normalized-shadow-shared-expr-implementation-extract-card.md
---

# 291x-398: Normalized-Shadow Legacy Entry Surface Inventory

## Goal

Inventory the remaining `LegacyLowerer` public surface after shared expression
helpers moved to `support::expr_lowering`.

This is inventory-only. No code behavior changed.

## Findings

Only one production caller remains outside `legacy/mod.rs`:

```text
builder.rs
  LegacyLowerer::lower_if_only_to_normalized
```

The remaining public methods are internally used by `LegacyLowerer` only:

```text
lower_return_from_tree
lower_if_node
verify_branch_is_return_literal
lower_return_value
```

No external caller uses those helpers directly.

## Decision

Keep the old if-only entry path in `LegacyLowerer` for now, but prune the
helper visibility surface first.

Next slice:

```text
pub fn lower_return_from_tree          -> fn lower_return_from_tree
pub fn lower_if_node                   -> fn lower_if_node
pub fn verify_branch_is_return_literal -> fn verify_branch_is_return_literal
pub fn lower_return_value              -> fn lower_return_value
```

This narrows the legacy module boundary without moving entry lowering or
changing StepTree acceptance.

## Next Cleanup

Make the remaining helper methods private and keep the single public entry:

```text
LegacyLowerer::lower_if_only_to_normalized
```

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Non-Goals

- Do not move the full legacy entry path yet.
- Do not change route priority.
- Do not change fail-fast tags or out-of-scope behavior.
