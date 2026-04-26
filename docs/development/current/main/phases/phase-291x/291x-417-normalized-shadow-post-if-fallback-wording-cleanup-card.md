---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow post-if fallback wording cleanup
Related:
  - src/mir/control_tree/normalized_shadow/post_if_post_k.rs
  - docs/development/current/main/phases/phase-291x/291x-416-normalized-shadow-post-if-fallback-wording-inventory-card.md
---

# 291x-417: Normalized-Shadow Post-If Fallback Wording Cleanup

## Goal

Remove stale fallback wording from the post-if normalized-shadow route.

This is a BoxShape cleanup. No lowering behavior changed.

## Change

Updated comments in `post_if_post_k.rs` from legacy/fallback terminology to:

```text
try the shared expression lowerer first
route-local Phase 129 baseline minimal compare path
```

## Preserved Behavior

- Lowering order is unchanged.
- `Ok(None)` handling is unchanged.
- Error behavior is unchanged.
- ANF and loop-if-exit fallback wording is untouched.

## Verification

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "legacy fallback|Fall back to the Phase 129 baseline minimal compare route" \
  src/mir/control_tree/normalized_shadow/post_if_post_k.rs
```

## Next Cleanup

Inventory the next compiler-cleanliness seam after post-if fallback wording
cleanup.
