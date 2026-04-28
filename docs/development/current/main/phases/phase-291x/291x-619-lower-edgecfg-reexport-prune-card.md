---
Status: Landed
Date: 2026-04-28
Scope: prune lower/plan EdgeCFG type re-export hops after edgecfg_facade retirement
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-584-edgecfg-facade-boundary-review-card.md
  - src/mir/builder/control_flow/lower/mod.rs
  - src/mir/builder/control_flow/lower/planner_compat.rs
  - src/mir/builder/control_flow/plan/mod.rs
  - src/mir/builder/control_flow/edgecfg/api/mod.rs
---

# 291x-619: Lower EdgeCFG Re-export Prune

## Goal

Close the remaining EdgeCFG compatibility export hops after `plan::edgecfg_facade`
was retired.

This stays BoxShape-only. It does not change EdgeCFG definitions, plan
construction, verifier behavior, or lowering behavior.

## Evidence

`291x-584` removed the `plan::edgecfg_facade` file and moved callers to
`control_flow::edgecfg::api`, but two small compatibility hops remained:

- `plan::Frag` re-exported `edgecfg::api::Frag`.
- `lower::{Frag, ExitKind}` re-exported EdgeCFG types through the lower surface.

The only live `lower::{Frag, ExitKind}` consumers are verifier modules, where
the direct owner path is `edgecfg::api`.

## Boundaries

- Move verifier imports for `Frag` and `ExitKind` to `edgecfg::api`.
- Remove the now-unused `lower::{Frag, ExitKind}` re-exports.
- Remove the now-unused `plan::Frag` re-export.
- Do not add a replacement facade.

## Acceptance

- No caller imports `Frag` or `ExitKind` through `control_flow::lower`.
- No caller imports `Frag` through `control_flow::plan`.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo fmt -- --check` passes.
- `cargo check --release --bin hakorune -q` passes.
- `git diff --check` passes.

## Result

- Moved verifier `Frag` / `ExitKind` imports to `edgecfg::api`.
- Removed `lower::{Frag, ExitKind}` from the lower compatibility surface.
- Removed the unused `plan::Frag` re-export.

## Verification

```bash
rg -n -U "control_flow::lower::\{[^}]*\b(Frag|ExitKind)\b|control_flow::lower::(Frag|ExitKind)|lower::(Frag|ExitKind)" src tests -g'*.rs'
rg -n -U "control_flow::plan::\{[^}]*\bFrag\b|control_flow::plan::Frag|plan::Frag" src tests -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
```
