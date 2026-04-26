---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow k-exit naming inventory
Related:
  - src/mir/control_tree/normalized_shadow/loop_true_break_once.rs
  - src/mir/join_ir/lowering/canonical_names.rs
  - docs/development/current/main/phases/phase-291x/291x-407-normalized-shadow-support-contract-wording-cleanup-card.md
---

# 291x-408: Normalized-Shadow K-Exit Naming Inventory

## Goal

Inventory the remaining `K_EXIT_LEGACY` naming before changing source code.

This is a BoxShape inventory. No behavior changed.

## Findings

`src/mir/control_tree/normalized_shadow/loop_true_break_once.rs` builds the
loop exit continuation with `JoinFuncId::new(2)`.

```text
JoinFuncId(2) -> "join_func_2"
```

The string value is not stale by itself: it is the normalized-shadow
compatibility name used for that fixed function id. The stale part is the
`K_EXIT_LEGACY` constant name and comments that describe this as a "legacy
variant".

The live references are:

```text
src/mir/join_ir/lowering/canonical_names.rs
src/mir/control_tree/normalized_shadow/loop_true_break_once.rs
```

Historical phase notes may still mention `K_EXIT_LEGACY`; those remain archive
history and should not be rewritten by this cleanup.

## Decision

Do not rename the emitted function string in this card or the next one.

Replace the source-facing constant name with a compatibility-specific name:

```text
NORMALIZED_SHADOW_K_EXIT = "join_func_2"
```

This keeps the bridge-visible function key stable while removing the misleading
"legacy" terminology from active source.

## Next Cleanup

`291x-409`: normalized-shadow k-exit naming cleanup.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "K_EXIT_LEGACY|legacy variant for normalized shadow" \
  src/mir/control_tree/normalized_shadow/loop_true_break_once.rs \
  src/mir/join_ir/lowering/canonical_names.rs
```

The final `rg` should produce no output.
