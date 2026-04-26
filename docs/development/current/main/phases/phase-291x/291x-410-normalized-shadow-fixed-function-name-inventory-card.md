---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow fixed function-name inventory
Related:
  - src/mir/control_tree/normalized_shadow/loop_true_break_once.rs
  - src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs
  - src/mir/control_tree/normalized_shadow/post_if_post_k.rs
  - src/mir/join_ir/lowering/canonical_names.rs
  - docs/development/current/main/phases/phase-291x/291x-409-normalized-shadow-k-exit-naming-cleanup-card.md
---

# 291x-410: Normalized-Shadow Fixed Function-Name Inventory

## Goal

Pick the next small compiler-cleanliness seam after normalized-shadow k-exit
naming cleanup.

This is a BoxShape inventory. No behavior changed.

## Candidate Review

| Candidate | Current shape | Decision |
| --- | --- | --- |
| `loop_true_break_once.rs` fixed names | raw `"join_func_0"`, `"join_func_1"`, `"join_func_3"`, and `"join_func_4"` remain next to fixed `JoinFuncId` values | pick next |
| `loop_true_if_break_continue.rs` generic names | raw `"main"`, `"loop_step"`, and `"k_exit"` can use canonical names but also has route-local `k_then`/`k_else` | defer; separate multi-route cleanup |
| `post_if_post_k.rs` generic names | raw `"main"` and `"post_k"` can use canonical names but route-local join names remain | defer; separate multi-route cleanup |
| broad `join_ir/lowering/*` raw names | many older route lowerers still carry raw `"loop_step"`/`"k_exit"` strings | defer; larger lowering cleanup surface |

## Decision

Normalize only the fixed-name surface in `loop_true_break_once.rs`.

Add explicit normalized-shadow compatibility names to the canonical-name SSOT:

```text
NORMALIZED_SHADOW_MAIN      = "join_func_0"
NORMALIZED_SHADOW_LOOP_STEP = "join_func_1"
NORMALIZED_SHADOW_K_EXIT    = "join_func_2"
NORMALIZED_SHADOW_LOOP_BODY = "join_func_3"
NORMALIZED_SHADOW_POST_K    = "join_func_4"
```

Then update `loop_true_break_once.rs` to consume those constants.

Do not change:

- `JoinFuncId` numbering
- emitted function keys
- route acceptance
- `loop_true_if_break_continue.rs`
- `post_if_post_k.rs`

## Next Cleanup

`291x-411`: normalized-shadow fixed function-name cleanup.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n '"join_func_[0134]"' \
  src/mir/control_tree/normalized_shadow/loop_true_break_once.rs
```

The final `rg` should produce no output.
