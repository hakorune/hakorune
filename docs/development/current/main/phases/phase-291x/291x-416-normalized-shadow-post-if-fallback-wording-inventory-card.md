---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow post-if fallback wording inventory
Related:
  - src/mir/control_tree/normalized_shadow/post_if_post_k.rs
  - docs/development/current/main/phases/phase-291x/291x-415-normalized-shadow-route-local-function-name-cleanup-card.md
---

# 291x-416: Normalized-Shadow Post-If Fallback Wording Inventory

## Goal

Pick the next small compiler-cleanliness seam after route-local function-name
cleanup.

This is a BoxShape inventory. No behavior changed.

## Findings

`post_if_post_k.rs` still has stale fallback wording:

```text
legacy fallback
Fall back to the Phase 129 baseline minimal compare route
```

The code is not a legacy path. It first tries the shared expression lowerer and
then uses the route-local baseline minimal compare path when the expression is
out of scope for that shared lowerer.

Broader `fallback` wording in ANF and `loop_if_exit_contract.rs` is out of
scope for this card. Those comments describe explicit out-of-scope behavior and
need a separate inventory if we decide to rename that vocabulary.

## Decision

Clean only the two stale comments in `post_if_post_k.rs`.

Do not change:

- lowering order
- error/Ok(None) behavior
- ANF fallback wording
- loop-if-exit contract wording

## Next Cleanup

`291x-417`: normalized-shadow post-if fallback wording cleanup.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "legacy fallback|Fall back to the Phase 129 baseline minimal compare route" \
  src/mir/control_tree/normalized_shadow/post_if_post_k.rs
```

The final `rg` should produce no output.
