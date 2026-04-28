---
Status: Landed
Date: 2026-04-28
Scope: reconcile verifier V12 tests and comments with LoopFrame v1 structural loop body contract
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/coreloop-loopframe-v1-ssot.md
  - src/mir/builder/control_flow/verify/verifier/loop_body_validators.rs
  - src/mir/builder/control_flow/verify/verifier/mod.rs
  - src/mir/builder/control_flow/verify/verifier/tests.rs
---

# 291x-621: Verifier Loop Body Contract Reconcile

## Goal

Fix stale verifier V12 tests and comments after LoopFrame v1 made loop bodies
structural.

This is BoxShape-only. It does not change verifier implementation, accepted
control-flow shapes, planner routing, or lowering behavior.

## Evidence

`cargo test -q verifier` failed two stale tests:

- `test_verify_loop_body_if_fails`
- `test_verify_loop_body_exit_fails`

Both expected `CorePlan::If` and `CorePlan::Exit(Return(None))` in
`Loop.body` to fail with V12. Current verifier implementation delegates those
nodes to the normal plan validators, and the LoopFrame v1 SSOT allows
structural loop bodies.

`CorePlan::BranchN` remains rejected in `Loop.body` by V12.

## Boundaries

- Update tests to expect loop-body `If` and `Exit` to verify.
- Add a negative test for loop-body `BranchN`.
- Update stale V12 comments to describe the current structural-body contract.
- Do not change verifier code paths.

## Acceptance

- `cargo test -q verifier` passes.
- `cargo fmt -- --check` passes.
- `cargo check --release --bin hakorune -q` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Reconciled verifier tests with the current LoopFrame v1 contract.
- Preserved V12 coverage for the remaining rejected loop-body vocabulary.
- Removed stale "effect-only" wording from verifier comments.

## Verification

```bash
cargo test -q verifier
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
