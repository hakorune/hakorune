---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow legacy physical storage move
Related:
  - src/mir/control_tree/normalized_shadow/entry/README.md
  - src/mir/control_tree/normalized_shadow/entry/if_only.rs
  - src/mir/control_tree/normalized_shadow/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-402-normalized-shadow-legacy-physical-storage-inventory-card.md
---

# 291x-403: Normalized-Shadow Legacy Physical Storage Move

## Goal

Close the normalized-shadow `legacy` module after the route builder was moved to
the if-only entry facade.

This is a BoxShape cleanup. Route order and StepTree acceptance are unchanged.

## Change

Moved the Phase 123-128 baseline if-only implementation into:

```text
src/mir/control_tree/normalized_shadow/entry/if_only.rs
```

Removed:

```text
src/mir/control_tree/normalized_shadow/legacy/mod.rs
pub mod legacy
LegacyLowerer
```

The if-only entry is now a free-function module entry:

```text
entry::if_only::lower_if_only_to_normalized
```

## Preserved Behavior

- `builder.rs` route order is unchanged.
- Baseline if-only lowering behavior is unchanged.
- Existing fail-fast tags are preserved.
- No accepted StepTree shape changed.

## Next Cleanup

Inventory stale "fallback to legacy" wording in normalized-shadow comments and
docs. Many sites now mean "out-of-scope route returns `Ok(None)`", not a real
legacy module fallback.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "normalized_shadow::legacy|legacy::LegacyLowerer|LegacyLowerer|pub mod legacy" src/mir/control_tree/normalized_shadow
```

The final `rg` should produce no output.
