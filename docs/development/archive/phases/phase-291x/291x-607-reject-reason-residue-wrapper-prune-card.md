---
Status: Landed
Date: 2026-04-28
Scope: prune reject_reason from facts plan_residue
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/plan_residue.rs
  - src/mir/builder/control_flow/mod.rs
  - src/mir/builder/control_flow/joinir/routing.rs
  - src/mir/builder/control_flow/joinir/route_entry/router.rs
---

# 291x-607: Reject Reason Residue Wrapper Prune

## Goal

Remove the `reject_reason` forwarding surface from `control_flow::facts`
`plan_residue` and make the remaining non-plan callers name the plan-side owner
directly.

This is BoxShape-only cleanup. It does not change rejection reasons, diagnostic
storage, routing behavior, accepted loop shapes, or lowering behavior.

## Boundaries

- Keep the `reject_reason` owner under `plan::facts`.
- Do not change the thread-local reject-detail lifecycle.
- Do not merge router, routing, or top-level control-flow entry
  responsibilities in this card.

## Result

- Updated the JoinIR router import to use `plan::facts::reject_reason`.
- Updated routing whitelist-miss detail recording to use the plan-side owner.
- Updated the top-level loop lowering entry to clear/take reject details through
  the plan-side owner.
- Removed `reject_reason` from the `facts/plan_residue.rs` allowlist.

## Verification

```bash
! rg -n "control_flow::facts::reject_reason" src tests -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
