---
Status: Landed
Date: 2026-04-28
Scope: prune scan_shapes from facts plan_residue
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/plan_residue.rs
  - src/mir/builder/control_flow/verify/verifier/debug_helpers/mod.rs
  - src/mir/builder/control_flow/lower/normalize/canonicalize.rs
  - src/mir/builder/control_flow/joinir/route_entry/router.rs
  - src/mir/builder/control_flow/verify/verifier/tests.rs
---

# 291x-608: Scan Shapes Residue Wrapper Prune

## Goal

Remove the `scan_shapes` forwarding surface from `control_flow::facts`
`plan_residue` and make remaining non-plan callers name the plan-side owner
directly.

This is BoxShape-only cleanup. It does not change scan-shape extraction,
condition/step shape classification, verifier diagnostics, accepted loop
shapes, or lowering behavior.

## Boundaries

- Keep `scan_shapes` owned by `plan::facts`.
- Do not change `ConditionShape`, `StepShape`, or cond-profile derivation.
- Do not modify verifier assertion behavior in this card.

## Result

- Updated verifier debug helpers to import `StepShape` and cond-profile helpers
  from `plan::facts::scan_shapes`.
- Updated normalize/router/verifier tests to use the plan-side scan-shape owner.
- Removed `scan_shapes` from the `facts/plan_residue.rs` allowlist.

## Verification

```bash
! rg -n "control_flow::facts::scan_shapes" src tests -g'*.rs'
cargo test -q canonical_projects
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
