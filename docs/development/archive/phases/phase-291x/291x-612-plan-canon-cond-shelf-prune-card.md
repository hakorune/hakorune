---
Status: Landed
Date: 2026-04-28
Scope: prune unused plan canon cond compatibility shelf
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/canon/mod.rs
  - src/mir/builder/control_flow/plan/canon/cond.rs
---

# 291x-612: Plan Canon Cond Shelf Prune

## Goal

Remove the unused `plan::canon::cond` compatibility shelf. The condition canon
owner already lives under `control_flow::facts::canon::cond`, and no live caller
uses the plan-side shelf.

This is BoxShape-only cleanup. It does not change condition canonicalization,
accepted shapes, planner decisions, or lowering behavior.

## Boundaries

- Keep condition canon implementation in `facts::canon::cond`.
- Do not migrate the many direct `CondBlockView` users in this card.
- Do not change generic-loop canon surfaces.

## Result

- Removed `mod cond` from `plan::canon`.
- Deleted `src/mir/builder/control_flow/plan/canon/cond.rs`.

## Verification

```bash
! rg -n "plan::canon::cond|control_flow::plan::canon::cond|mod cond;" src/mir/builder/control_flow/plan -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
