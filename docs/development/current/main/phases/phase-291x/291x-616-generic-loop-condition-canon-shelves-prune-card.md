---
Status: Landed
Date: 2026-04-28
Scope: prune generic-loop condition canon compatibility submodules
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/canon/generic_loop/condition.rs
  - src/mir/builder/control_flow/plan/canon/generic_loop/condition/bound.rs
  - src/mir/builder/control_flow/plan/canon/generic_loop/condition/candidates.rs
---

# 291x-616: Generic Loop Condition Canon Shelves Prune

## Goal

Remove unused `plan::canon::generic_loop::condition::{bound,candidates}`
compatibility submodules. Their implementation owner already lives under
`facts::canon::generic_loop::condition`.

This is BoxShape-only cleanup. It does not change generic-loop condition
canonicalization, accepted shapes, planner decisions, or lowering behavior.

## Boundaries

- Keep condition canon implementation under `facts::canon::generic_loop`.
- Preserve the existing `plan::canon::generic_loop::condition` top-level
  re-export surface.
- Do not change generic-loop step or update canon in this card.

## Result

- Removed unused `mod bound` and `mod candidates` from the plan-side condition
  canon shelf.
- Deleted the now-empty compatibility submodule files.

## Verification

```bash
! rg -n "plan::canon::generic_loop::condition::(bound|candidates)|condition::(bound|candidates)::|mod (bound|candidates);" src/mir/builder/control_flow/plan/canon/generic_loop -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
