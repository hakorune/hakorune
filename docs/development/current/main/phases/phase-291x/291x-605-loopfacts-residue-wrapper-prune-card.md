---
Status: Landed
Date: 2026-04-28
Scope: prune LoopFacts and loop_types from facts/plan_residue
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/plan_residue.rs
  - src/mir/builder/control_flow/lower/normalize/canonicalize.rs
  - src/mir/builder/control_flow/joinir/route_entry/router.rs
  - src/mir/builder/control_flow/verify/verifier/tests.rs
  - src/mir/builder/control_flow/plan/facts/loop_types.rs
---

# 291x-605: LoopFacts Residue Wrapper Prune

## Goal

Remove `LoopFacts` and `loop_types` from `facts/plan_residue` after moving the
remaining non-plan callers to the plan facts owner path.

This is BoxShape-only cleanup. It does not change loop facts construction,
canonicalization behavior, router behavior, verifier behavior, or lowering.

## Boundaries

- Keep loop facts ownership in `plan/facts/loop_types.rs`.
- Touch only owner-path imports and test fixture type paths.
- Do not move `feature_facts`, `scan_shapes`, `skeleton_facts`, or
  `reject_reason` in this card.

## Result

- Updated canonicalize, route-entry router tests, and verifier tests to use the
  plan facts owner path for `LoopFacts`.
- Removed `LoopFacts` and `loop_types` from the `facts/plan_residue` allowlist.

## Verification

```bash
! rg -n "control_flow::facts::(LoopFacts|loop_types)" src tests -g'*.rs'
cargo test -q canonical_projects
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
