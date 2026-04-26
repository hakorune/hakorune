---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow legacy helper privacy prune
Related:
  - src/mir/control_tree/normalized_shadow/legacy/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-398-normalized-shadow-legacy-entry-surface-inventory-card.md
---

# 291x-399: Normalized-Shadow Legacy Helper Privacy Prune

## Goal

Narrow the public surface of the normalized-shadow legacy entry module.

This is a BoxShape cleanup. No behavior changed.

## Change

Kept the single public entry:

```text
LegacyLowerer::lower_if_only_to_normalized
```

Made these helper methods private:

```text
lower_return_from_tree
lower_if_node
verify_branch_is_return_literal
lower_return_value
```

## Preserved Behavior

- The legacy if-only entry route remains available to `builder.rs`.
- Internal recursive lowering calls are unchanged.
- Fail-fast tags and out-of-scope behavior are unchanged.
- No StepTree shape acceptance changed.

## Next Cleanup

Inventory whether the remaining legacy entry path should stay named
`LegacyLowerer` or move behind a narrower entry module/facade.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
