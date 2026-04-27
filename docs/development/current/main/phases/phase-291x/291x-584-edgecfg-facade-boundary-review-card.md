---
Status: Landed
Date: 2026-04-28
Scope: review plan::edgecfg_facade and delete the pure pass-through facade in favor of direct edgecfg::api owner paths
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-575-plan-compat-residue-inventory-card.md
  - src/mir/builder/control_flow/plan/mod.rs
  - src/mir/builder/control_flow/plan/edgecfg_facade.rs
  - src/mir/builder/control_flow/edgecfg/api/mod.rs
  - src/mir/builder/control_flow/lower/planner_compat.rs
---

# 291x-584: EdgeCFG Facade Boundary Review

## Goal

Determine whether `plan::edgecfg_facade` is a real boundary seam or just a
compatibility shim, then land the smallest safe cleanup.

This remains BoxShape-only. It does not change EdgeCFG behavior or move
implementation ownership.

## Evidence

`plan/edgecfg_facade.rs` contained only direct re-exports of
`control_flow::edgecfg::api::{BlockParams, BranchStub, EdgeStub, ExitKind, Frag,
FragEmitSession}` and added no validation, translation, or policy.

Live plan/lower callers already treat `edgecfg::api` as the true owner surface,
so the facade was only an extra import hop.

## Boundaries

- Rewrite callers from `plan::edgecfg_facade` to `edgecfg::api`.
- Remove the dead facade module and its plan-level `Frag` re-export hop.
- Do not change EdgeCFG API definitions or any lowering behavior.

## Acceptance

- No `edgecfg_facade` imports remain under `src/mir/builder/control_flow`.
- `plan::mod` no longer declares the facade module.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Rewired plan and lower imports to the direct `edgecfg::api` owner path.
- Removed `plan/edgecfg_facade.rs`.
- Re-exported `Frag` directly from `edgecfg::api` at the plan surface where
  that convenience still mattered.

## Verification

```bash
rg -n "edgecfg_facade" src/mir/builder/control_flow -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
cargo check --release --bin hakorune
git diff --check
```
