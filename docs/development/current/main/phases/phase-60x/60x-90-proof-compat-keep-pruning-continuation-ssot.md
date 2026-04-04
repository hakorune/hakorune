---
Status: SSOT
Date: 2026-04-04
Scope: continue pruning explicit proof/compat keeps after route/default/help retirement narrowed the outer rust-vm surface.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-59x/README.md
---

# 60x-90 Proof/Compat Keep Pruning Continuation SSOT

## Intent

- keep attacking the explicit proof/compat keep bucket without reopening route/default affordances
- prefer wording/contract/seam narrowing over premature source deletion
- leave `vm-hako` reference/conformance surfaces untouched in this lane

## Highest-Leverage Keep Surfaces

- `tools/selfhost/lib/selfhost_run_routes.sh`
  - `stage-a-compat` remains an explicit compat-only route and should stay narrow
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - still owns Program(JSON v0) compat bridge logic
- `src/runner/modes/vm_fallback.rs`
  - explicit compat fallback interpreter keep
- `lang/src/runner/stage1_cli/core.hako`
  - raw compat hold line
- proof smoke wrappers
  - `tools/selfhost/bootstrap_selfhost_smoke.sh`
  - `tools/selfhost/selfhost_smoke.sh`
  - `tools/selfhost/selfhost_stage3_accept_smoke.sh`
  - `tools/plugins/plugin_v2_smoke.sh`

## Boundaries

- do not reopen CLI/default/backend selection work from phase-59x
- do not mix `vm-hako` reference/conformance cleanup into this lane
- do not claim delete-ready broad rust-vm source until caller-zero facts materially change
