---
Status: Landed
Date: 2026-04-28
Scope: close out facts plan_residue shelf
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/mod.rs
  - src/mir/builder/control_flow/facts/plan_residue.rs
  - src/mir/builder/control_flow/lower/normalize/canonicalize.rs
  - src/mir/builder/control_flow/joinir/route_entry/router.rs
  - src/mir/builder/control_flow/joinir/route_entry/registry/handlers.rs
  - src/mir/builder/control_flow/joinir/route_entry/registry/handlers/routes.rs
  - src/mir/builder/control_flow/verify/verifier/tests.rs
---

# 291x-610: Plan Residue Shelf Closeout

## Goal

Remove the final `feature_facts` forwarding surface from
`control_flow::facts::plan_residue` and delete the now-empty residue shelf.

This is BoxShape-only cleanup. It does not change feature extraction, nested-loop
detection, canonical fact projection, accepted loop shapes, or lowering
behavior.

## Boundaries

- Keep `feature_facts` owned by `plan::facts`.
- Do not move facts-owned modules out of `control_flow::facts`.
- Do not change route decisions or verifier contracts in this card.

## Result

- Updated remaining `feature_facts` callers to import from
  `plan::facts::feature_facts`.
- Removed `plan_residue` from the `facts` module surface.
- Deleted `src/mir/builder/control_flow/facts/plan_residue.rs`.
- Updated the `facts` module doc comment to make the no-compat-shelf ownership
  rule explicit.

## Verification

```bash
! rg -n "control_flow::facts::feature_facts|plan_residue" src/mir/builder/control_flow -g'*.rs'
cargo test -q canonical_projects
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
