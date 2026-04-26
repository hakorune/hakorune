---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow legacy physical storage inventory
Related:
  - src/mir/control_tree/normalized_shadow/entry/if_only.rs
  - src/mir/control_tree/normalized_shadow/legacy/mod.rs
  - src/mir/control_tree/normalized_shadow/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-401-normalized-shadow-if-only-entry-facade-card.md
---

# 291x-402: Normalized-Shadow Legacy Physical Storage Inventory

## Goal

Decide whether the remaining normalized-shadow `legacy/mod.rs` implementation
should stay quarantined or move into the new if-only entry owner.

This card is inventory-only. No code behavior changed.

## Findings

Remaining normalized-shadow legacy references are local to the if-only entry
bridge:

```text
mod.rs
  pub mod legacy

entry/if_only.rs
  use normalized_shadow::legacy::LegacyLowerer
  LegacyLowerer::lower_if_only_to_normalized(...)

legacy/mod.rs
  pub struct LegacyLowerer
  impl LegacyLowerer
```

No route builder imports `legacy` anymore.

## Decision

Move the implementation physically into:

```text
src/mir/control_tree/normalized_shadow/entry/if_only.rs
```

Then remove:

```text
src/mir/control_tree/normalized_shadow/legacy/mod.rs
pub mod legacy
LegacyLowerer
```

Reason: after `291x-401`, `legacy/mod.rs` has no independent caller boundary.
Keeping it would preserve a storage-history name with no semantic owner value.

## Plan

1. `291x-403`: move the if-only implementation into `entry/if_only.rs` and
   remove the `legacy` module.
2. `291x-404`: inventory stale "fallback to legacy" wording in
   normalized-shadow comments/docs.
3. `291x-405`: replace stale wording with "out-of-scope route" where behavior
   is `Ok(None)`.

Keep these as separate BoxShape cards. Do not alter route order or accepted
StepTree shapes.

## Next Cleanup

Physical move:

```text
legacy/mod.rs -> entry/if_only.rs
```

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "normalized_shadow::legacy|legacy::LegacyLowerer|LegacyLowerer|pub mod legacy" src/mir/control_tree/normalized_shadow
```

The final `rg` should produce no output.
