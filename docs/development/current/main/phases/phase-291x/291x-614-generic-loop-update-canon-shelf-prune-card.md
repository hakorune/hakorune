---
Status: Landed
Date: 2026-04-28
Scope: prune generic-loop update canon compatibility submodule
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/canon/generic_loop.rs
  - src/mir/builder/control_flow/plan/canon/generic_loop/update.rs
---

# 291x-614: Generic Loop Update Canon Shelf Prune

## Goal

Remove the pure compatibility submodule
`plan::canon::generic_loop::update` while keeping the live top-level
`plan::canon::generic_loop::canon_update_for_loop_var` surface intact.

This is BoxShape-only cleanup. It does not change generic-loop update
canonicalization, accepted shapes, planner decisions, or lowering behavior.

## Boundaries

- Keep the update canon implementation under `facts::canon::generic_loop`.
- Preserve the existing `plan::canon::generic_loop` top-level function name.
- Do not change condition/step canon modules in this card.

## Result

- Removed `mod update` from `plan::canon::generic_loop`.
- Re-exported `canon_update_for_loop_var` directly from the facts owner.
- Deleted the now-empty compatibility submodule file.

## Verification

```bash
! rg -n "plan::canon::generic_loop::update|canon::generic_loop::update|mod update;" src/mir/builder/control_flow/plan -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
