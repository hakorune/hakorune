---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow legacy entry facade inventory
Related:
  - src/mir/control_tree/normalized_shadow/builder.rs
  - src/mir/control_tree/normalized_shadow/legacy/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-399-normalized-shadow-legacy-helper-privacy-prune-card.md
---

# 291x-400: Normalized-Shadow Legacy Entry Facade Inventory

## Goal

Decide how to remove the remaining direct `legacy::LegacyLowerer` dependency
from the normalized-shadow route builder.

This card is inventory-only. No code behavior changed.

## Current Shape

After `291x-399`, only one public legacy entry remains:

```text
LegacyLowerer::lower_if_only_to_normalized
```

The only caller outside `legacy/mod.rs` is:

```text
builder.rs
  LegacyLowerer::lower_if_only_to_normalized(step_tree, &env_layout)
```

This is already narrow, but the route builder still imports a storage/compat
name (`legacy`) instead of a semantic entry owner.

## Decision

Add a narrow if-only entry facade and make `builder.rs` depend on it:

```text
normalized_shadow::entry::if_only
  lower_if_only_to_normalized(step_tree, env_layout)
```

The first implementation may delegate to `LegacyLowerer`; the point of the next
card is to move the public dependency out of `builder.rs`. Physical relocation
of the old implementation is a separate slice.

## Plan

1. `291x-401`: add `entry/if_only.rs` + README and migrate `builder.rs` from
   `legacy::LegacyLowerer` to the entry facade.
2. `291x-402`: inventory whether the remaining implementation should move from
   `legacy/mod.rs` into the entry module or stay quarantined.
3. `291x-403`: clean stale "fallback to legacy" wording where the behavior is
   actually "out-of-scope route returns Ok(None)".

Keep these as separate BoxShape cards. Do not mix with StepTree acceptance or
route priority changes.

## Next Cleanup

Implement the if-only entry facade:

```text
src/mir/control_tree/normalized_shadow/entry/README.md
src/mir/control_tree/normalized_shadow/entry/mod.rs
src/mir/control_tree/normalized_shadow/entry/if_only.rs
```

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Non-Goals

- Do not move the full legacy implementation in the facade-add card.
- Do not change route order in `builder.rs`.
- Do not change StepTree shape acceptance.
- Do not change fail-fast tags or out-of-scope behavior.
