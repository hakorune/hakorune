---
Status: Landed
Date: 2026-04-28
Scope: prune empty loop_break contracts module
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/loop_break/mod.rs
  - src/mir/builder/control_flow/plan/loop_break/contracts/mod.rs
---

# 291x-617: Loop Break Empty Contracts Shelf Prune

## Goal

Remove the now-empty `plan::loop_break::contracts` shelf after its last
compatibility wrapper was deleted.

This is BoxShape-only cleanup. It does not change loop-break facts, promotion
logic, accepted loop shapes, or lowering behavior.

## Boundaries

- Keep `loop_break::api` and `loop_break::facts` surfaces intact.
- Do not move any loop-break implementation code in this card.
- Do not change cleanup policy ownership.

## Result

- Removed `mod contracts` from `plan::loop_break`.
- Deleted the empty contracts module file.
- Updated the `loop_break` module overview to list only live shelves.

## Verification

```bash
! rg -n "loop_break::contracts|contracts::|mod contracts" src/mir/builder/control_flow/plan/loop_break -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
