---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow ANF status wording cleanup
Related:
  - src/mir/control_tree/normalized_shadow/anf/README.md
  - src/mir/control_tree/normalized_shadow/anf/mod.rs
  - src/mir/control_tree/normalized_shadow/anf/contract.rs
  - src/mir/control_tree/normalized_shadow/anf/plan_box.rs
  - src/mir/control_tree/normalized_shadow/anf/execute_box.rs
  - src/mir/control_tree/normalized_shadow/common/expr_lowerer_box/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-422-normalized-shadow-anf-status-wording-inventory-card.md
---

# 291x-423: Normalized-Shadow ANF Status Wording Cleanup

## Goal

Remove stale ANF P0-stub/fallback wording from live docs and source comments.

This is a BoxShape cleanup. No ANF behavior changed.

## Change

Updated normalized-shadow ANF wording to match the current route contract:

```text
plan_box builds an ANF plan
execute_box consumes active P1/P2 plans when routing is enabled
Ok(None) means route decline / out-of-scope, not a stub fallback
```

The cleanup touched:

- `anf/README.md`
- `anf/mod.rs`
- `anf/contract.rs`
- `anf/plan_box.rs`
- `anf/execute_box.rs`
- `common/expr_lowerer_box/mod.rs`

## Preserved Behavior

- ANF route selection is unchanged.
- `HAKO_ANF_*` environment-variable behavior is unchanged.
- `[phase145/*]` debug tags are unchanged.
- Hoist logic is unchanged.
- Accepted expression shapes are unchanged.
- Generated JoinIR instructions are unchanged.

## Verification

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "P0 stub|Skeleton implemented|debug logging only|not yet used in execute_box" \
  src/mir/control_tree/normalized_shadow/anf \
  src/mir/control_tree/normalized_shadow/common/expr_lowerer_box
rg -n "graceful fallback|safe fallback|conservative fallback|execute_box is stub|always returns Ok\\(None\\)" \
  src/mir/control_tree/normalized_shadow/anf \
  src/mir/control_tree/normalized_shadow/common/expr_lowerer_box
```

## Next Cleanup

Inventory the next compiler-cleanliness seam after ANF status wording cleanup.
