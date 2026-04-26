---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow route-local function-name cleanup
Related:
  - src/mir/control_tree/normalized_shadow/common/route_function_names.rs
  - src/mir/control_tree/normalized_shadow/common/mod.rs
  - src/mir/control_tree/normalized_shadow/if_as_last_join_k.rs
  - src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs
  - src/mir/control_tree/normalized_shadow/post_if_post_k.rs
  - docs/development/current/main/phases/phase-291x/291x-414-normalized-shadow-route-local-function-name-inventory-card.md
---

# 291x-415: Normalized-Shadow Route-Local Function-Name Cleanup

## Goal

Move normalized-shadow route-local function-name literals behind a
normalized-shadow-local vocabulary.

This is a BoxShape cleanup. No emitted function names changed.

## Change

Added:

```text
src/mir/control_tree/normalized_shadow/common/route_function_names.rs
```

with route-local constants:

```text
JOIN_K          = "join_k"
K_THEN          = "k_then"
K_ELSE          = "k_else"
LOOP_COND_CHECK = "loop_cond_check"
```

Updated affected route builders to consume that vocabulary.

## Preserved Behavior

- Route-local function names are unchanged.
- `JoinFuncId` numbering is unchanged.
- Generic JoinIR canonical-name SSOT is unchanged.
- Route acceptance is unchanged.

## Verification

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n 'JoinFunction::new\([^\n]+\"(join_k|k_then|k_else|loop_cond_check)\"|\"(join_k|k_then|k_else|loop_cond_check)\"\.to_string\(\)' \
  src/mir/control_tree/normalized_shadow -g '*.rs'
```

## Next Cleanup

Inventory the next compiler-cleanliness seam after route-local function-name
cleanup.
