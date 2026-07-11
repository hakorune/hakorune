---
Status: Landed
Date: 2026-04-28
Scope: prune the LoopBreakFacts forwarding type from facts/plan_residue
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/plan_residue.rs
  - src/mir/builder/control_flow/joinir/route_entry/registry/utils.rs
  - src/mir/builder/control_flow/plan/loop_break/facts/mod.rs
---

# 291x-602: Loop-Break Facts Residue Wrapper Prune

## Goal

Remove the `LoopBreakFacts` forwarding type from `facts/plan_residue` after
moving the lone route-entry caller to the `loop_break` facts owner path.

This is BoxShape-only cleanup. It does not change loop-break extraction,
routing, recipe adoption, or lowering.

## Boundaries

- Keep `LoopBreakFacts` ownership in `plan/loop_break/facts`.
- Do not move loop-break extraction modules.
- Do not change planner-first tags or strict/dev routing behavior.

## Result

- Updated `joinir/route_entry/registry/utils.rs` to import `LoopBreakFacts`
  from `plan::loop_break::facts`.
- Removed the now-unused `LoopBreakFacts` forwarding type from
  `facts/plan_residue.rs`.

## Verification

```bash
! rg -n "control_flow::facts::LoopBreakFacts" src tests -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
