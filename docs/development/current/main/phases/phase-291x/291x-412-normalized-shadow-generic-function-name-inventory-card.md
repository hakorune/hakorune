---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow generic function-name inventory
Related:
  - src/mir/control_tree/normalized_shadow/entry/if_only.rs
  - src/mir/control_tree/normalized_shadow/if_as_last_join_k.rs
  - src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs
  - src/mir/control_tree/normalized_shadow/post_if_post_k.rs
  - src/mir/join_ir/lowering/canonical_names.rs
  - docs/development/current/main/phases/phase-291x/291x-411-normalized-shadow-fixed-function-name-cleanup-card.md
---

# 291x-412: Normalized-Shadow Generic Function-Name Inventory

## Goal

Pick the next small compiler-cleanliness seam after fixed `join_func_*` name
cleanup.

This is a BoxShape inventory. No behavior changed.

## Findings

The remaining normalized-shadow raw generic function names that already have
canonical-name constants are:

```text
entry/if_only.rs                    "main"
if_as_last_join_k.rs                "main"
post_if_post_k.rs                   "main", "post_k"
loop_true_if_break_continue.rs      "main", "loop_step", "k_exit"
```

These can use existing `canonical_names` constants:

```text
MAIN
LOOP_STEP
K_EXIT
POST_K
```

Route-local names are intentionally out of scope:

```text
k_then
k_else
join_k
```

## Decision

Normalize only the generic names that already have constants. Do not introduce
new route-local constants in the same card.

## Next Cleanup

`291x-413`: normalized-shadow generic function-name cleanup.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n 'JoinFunction::new\([^\n]+\"(main|loop_step|k_exit|post_k)\"|\"(main|loop_step|k_exit|post_k)\"\.to_string\(\)' \
  src/mir/control_tree/normalized_shadow -g '*.rs'
```

The final `rg` should produce no output.
