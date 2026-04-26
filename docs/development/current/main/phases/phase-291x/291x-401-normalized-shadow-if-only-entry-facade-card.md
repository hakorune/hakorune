---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow if-only entry facade
Related:
  - src/mir/control_tree/normalized_shadow/entry/README.md
  - src/mir/control_tree/normalized_shadow/entry/mod.rs
  - src/mir/control_tree/normalized_shadow/entry/if_only.rs
  - src/mir/control_tree/normalized_shadow/builder.rs
  - docs/development/current/main/phases/phase-291x/291x-400-normalized-shadow-legacy-entry-facade-inventory-card.md
---

# 291x-401: Normalized-Shadow If-Only Entry Facade

## Goal

Remove the route builder's direct dependency on the normalized-shadow legacy
storage module.

This is a BoxShape cleanup. Route order and StepTree acceptance are unchanged.

## Change

Added:

```text
src/mir/control_tree/normalized_shadow/entry/
  README.md
  mod.rs
  if_only.rs
```

Updated `builder.rs` from:

```text
legacy::LegacyLowerer::lower_if_only_to_normalized(...)
```

to:

```text
entry::if_only::lower_if_only_to_normalized(...)
```

The new entry facade delegates to `LegacyLowerer` in this card. Physical
relocation of the implementation remains a separate slice.

## Preserved Behavior

- `builder.rs` route order is unchanged.
- The baseline Phase 123-128 if-only route is unchanged.
- No accepted StepTree shape changed.
- Fail-fast tags and out-of-scope behavior are unchanged.

## Next Cleanup

Inventory the remaining physical storage:

```text
normalized_shadow/legacy/mod.rs
```

Decide whether to move the implementation into `entry/if_only.rs` directly or
rename/quarantine the legacy module more explicitly.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
