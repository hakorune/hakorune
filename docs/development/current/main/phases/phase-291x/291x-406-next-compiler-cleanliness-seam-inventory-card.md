---
Status: Landed
Date: 2026-04-27
Scope: next compiler-cleanliness seam inventory after normalized-shadow legacy cleanup
Related:
  - src/mir/control_tree/normalized_shadow/support/README.md
  - src/mir/control_tree/normalized_shadow/support/expr_lowering.rs
  - src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs
  - src/mir/control_tree/normalized_shadow/loop_true_break_once.rs
  - src/mir/join_ir/lowering/canonical_names.rs
  - docs/development/current/main/phases/phase-291x/291x-405-normalized-shadow-stale-legacy-wording-cleanup-card.md
---

# 291x-406: Next Compiler-Cleanliness Seam Inventory

## Goal

Pick the next small BoxShape cleanup seam after normalized-shadow legacy storage
and fallback wording cleanup.

This card is inventory-only. No code behavior changed.

## Candidate Review

| Candidate | Current shape | Decision |
| --- | --- | --- |
| `normalized_shadow/support/*` wording | still says route lowerers and the "legacy entry path" depend on support, even though `legacy/mod.rs` is removed | pick next |
| `common/expr_lowering_contract.rs` wording | still says fallback and "legacy lowering path" in a contract that now participates in route decline semantics | include next |
| `loop_true_break_once.rs` `K_EXIT_LEGACY` comments | mentions "legacy variant for normalized shadow" because the actual canonical constant is `K_EXIT_LEGACY` | defer; needs canonical-name compatibility inventory |
| ANF "graceful fallback" wording | generic out-of-scope terminology, not tied to removed legacy storage | keep for now |
| broader `src/mir/builder/calls/*` legacy call lane | larger builder-call migration surface | defer; not same card |

## Selected Next Seam

Clean stale support/contract wording in:

```text
src/mir/control_tree/normalized_shadow/support/README.md
src/mir/control_tree/normalized_shadow/support/expr_lowering.rs
src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs
```

Use route-owner terminology:

```text
baseline if-only entry
route lowerers
route decline / Ok(None)
baseline lowering behavior
```

Do not touch `K_EXIT_LEGACY` in the same card.

## Next Cleanup

`291x-407`: normalized-shadow support/contract wording cleanup.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "legacy entry path|inside `legacy`|legacy lowering path" \
  src/mir/control_tree/normalized_shadow/support \
  src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs
```

The final `rg` should produce no output.
