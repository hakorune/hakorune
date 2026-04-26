---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow if-only fossil boundary note
Related:
  - src/mir/control_tree/normalized_shadow/entry/README.md
  - src/mir/control_tree/normalized_shadow/entry/if_only.rs
  - src/mir/control_tree/normalized_shadow/builder.rs
  - docs/development/current/main/phases/phase-291x/291x-428-normalized-shadow-if-only-fossil-boundary-inventory-card.md
---

# 291x-429: Normalized-Shadow If-Only Fossil Boundary Note

## Goal

Fence the Phase 123-128 if-only baseline as a historical route boundary.

This is a BoxShape cleanup. No behavior changed.

## Change

Added fossil-boundary notes to:

- `entry/README.md`
- `entry/if_only.rs`
- `builder.rs`

The notes make the historical baseline explicit:

```text
newer normalized routes run first
if_only is the final Phase 123-128 baseline route
placeholder compare/then-branch simplification must not be silently fixed
```

## Preserved Behavior

- Compare LHS placeholder behavior is unchanged.
- Then-branch-only simplified emission is unchanged.
- `[phase123/*]` and `[phase125/*]` decline tags are unchanged.
- Route priority is unchanged.
- Accepted StepTree shapes are unchanged.

## Verification

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
RUN_TIMEOUT_SECS=30 bash tools/smokes/v2/profiles/integration/apps/archive/phase123_if_only_normalized_semantics_vm.sh
RUN_TIMEOUT_SECS=30 bash tools/smokes/v2/profiles/integration/apps/archive/phase128_if_only_partial_assign_normalized_vm.sh
```

Note: the archive smoke default `RUN_TIMEOUT_SECS=10` timed out in this WSL
session. With a 30s timeout, both archive smokes passed.

## Next Cleanup

Inventory the next compiler-cleanliness seam after normalized-shadow if-only
fossil boundary note.
