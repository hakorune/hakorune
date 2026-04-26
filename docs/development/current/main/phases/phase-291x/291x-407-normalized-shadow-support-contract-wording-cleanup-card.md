---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow support/expr contract wording cleanup
Related:
  - src/mir/control_tree/normalized_shadow/support/README.md
  - src/mir/control_tree/normalized_shadow/support/expr_lowering.rs
  - src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs
  - docs/development/current/main/phases/phase-291x/291x-406-next-compiler-cleanliness-seam-inventory-card.md
---

# 291x-407: Normalized-Shadow Support Contract Wording Cleanup

## Goal

Remove stale support/contract wording that still referred to a legacy entry or
legacy lowering path after normalized-shadow legacy storage was removed.

This is a BoxShape cleanup. No behavior changed.

## Change

Updated wording in:

```text
normalized_shadow/support/README.md
normalized_shadow/support/expr_lowering.rs
normalized_shadow/common/expr_lowering_contract.rs
```

New terminology:

```text
baseline if-only entry
route decline
baseline path
```

## Preserved Behavior

- No route order changed.
- No accepted StepTree shape changed.
- No fail-fast tag changed.
- `K_EXIT_LEGACY` naming was intentionally left for a separate canonical-name
  inventory.

## Next Cleanup

Inventory `K_EXIT_LEGACY` naming in normalized-shadow loop routes and
`join_ir/lowering/canonical_names.rs`.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "legacy entry path|inside `legacy`|legacy lowering path" \
  src/mir/control_tree/normalized_shadow/support \
  src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs
```
