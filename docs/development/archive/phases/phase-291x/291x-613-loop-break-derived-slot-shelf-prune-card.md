---
Status: Landed
Date: 2026-04-28
Scope: prune unused loop_break derived-slot compatibility shelf
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/loop_break/contracts/mod.rs
  - src/mir/builder/control_flow/plan/loop_break/contracts/derived_slot.rs
---

# 291x-613: Loop Break Derived-Slot Shelf Prune

## Goal

Remove the unused `plan::loop_break::contracts::derived_slot` compatibility
wrapper. The derived-slot detection owner already lives under
`cleanup::policies::body_local_derived_slot`, and live callers use that owner
directly.

This is BoxShape-only cleanup. It does not change body-local derived-slot
detection, loop-break policy inputs, accepted loop shapes, or lowering behavior.

## Boundaries

- Keep derived-slot extraction under `cleanup::policies`.
- Do not move loop-break policy helper code in this card.
- Do not change `LoopBreak` facts, recipes, or emission behavior.

## Result

- Removed `mod derived_slot` from `plan::loop_break::contracts`.
- Deleted the unused compatibility wrapper file.

## Verification

```bash
! rg -n "loop_break::contracts::derived_slot|contracts::derived_slot|loop_break/contracts/derived_slot" src/mir/builder/control_flow -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
