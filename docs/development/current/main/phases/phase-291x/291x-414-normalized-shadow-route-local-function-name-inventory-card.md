---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow route-local function-name inventory
Related:
  - src/mir/control_tree/normalized_shadow/common/mod.rs
  - src/mir/control_tree/normalized_shadow/if_as_last_join_k.rs
  - src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs
  - src/mir/control_tree/normalized_shadow/post_if_post_k.rs
  - docs/development/current/main/phases/phase-291x/291x-413-normalized-shadow-generic-function-name-cleanup-card.md
---

# 291x-414: Normalized-Shadow Route-Local Function-Name Inventory

## Goal

Pick the next small compiler-cleanliness seam after generic canonical-name
cleanup.

This is a BoxShape inventory. No behavior changed.

## Findings

After `291x-413`, the remaining raw `JoinFunction` name literals in live
normalized-shadow route builders are route-local continuation names:

```text
if_as_last_join_k.rs           "join_k", "k_then", "k_else"
post_if_post_k.rs              "join_k", "k_then", "k_else"
loop_true_if_break_continue.rs "loop_cond_check", "k_then", "k_else"
```

These are not global JoinIR names like `main` or `k_exit`, so putting them in
`join_ir/lowering/canonical_names.rs` would over-broaden that SSOT.

## Decision

Create a normalized-shadow-local function-name vocabulary under
`normalized_shadow/common/`.

Suggested constants:

```text
JOIN_K
K_THEN
K_ELSE
LOOP_COND_CHECK
```

Use that common vocabulary from the three affected route builders.

Do not change:

- route-local function names
- `JoinFuncId` numbering
- route acceptance
- generic `canonical_names` constants

## Next Cleanup

`291x-415`: normalized-shadow route-local function-name cleanup.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n 'JoinFunction::new\([^\n]+\"(join_k|k_then|k_else|loop_cond_check)\"|\"(join_k|k_then|k_else|loop_cond_check)\"\.to_string\(\)' \
  src/mir/control_tree/normalized_shadow -g '*.rs'
```

The final `rg` should produce no output.
