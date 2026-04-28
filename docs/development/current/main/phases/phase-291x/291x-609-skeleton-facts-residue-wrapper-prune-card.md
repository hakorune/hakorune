---
Status: Landed
Date: 2026-04-28
Scope: prune skeleton_facts from facts plan_residue
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/plan_residue.rs
  - src/mir/builder/control_flow/verify/observability/flowbox_tags.rs
  - src/mir/builder/control_flow/lower/normalize/canonicalize.rs
  - src/mir/builder/control_flow/joinir/route_entry/router.rs
  - src/mir/builder/control_flow/verify/verifier/tests.rs
---

# 291x-609: Skeleton Facts Residue Wrapper Prune

## Goal

Remove the `skeleton_facts` forwarding surface from `control_flow::facts`
`plan_residue` and make remaining non-plan callers name the plan-side owner
directly.

This is BoxShape-only cleanup. It does not change skeleton classification,
flowbox tagging, canonical fact projection, accepted loop shapes, or lowering
behavior.

## Boundaries

- Keep `SkeletonFacts` and `SkeletonKind` owned by `plan::facts`.
- Do not change `CanonicalLoopFacts` fields or projection semantics.
- Do not modify verifier assertion behavior in this card.

## Result

- Updated flowbox observability and canonicalize code to import skeleton types
  from `plan::facts::skeleton_facts`.
- Updated router/verifier tests to use the plan-side skeleton owner.
- Removed `skeleton_facts` from the `facts/plan_residue.rs` allowlist.

## Verification

```bash
! rg -n "control_flow::facts::skeleton_facts" src tests -g'*.rs'
cargo test -q canonical_projects
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
