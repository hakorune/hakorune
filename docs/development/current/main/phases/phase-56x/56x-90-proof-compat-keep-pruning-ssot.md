---
Status: SSOT
Date: 2026-04-04
Scope: prune explicit proof/compat keep surfaces after phase-55x retired route/default/help exposure.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-55x/README.md
---

# 56x-90 Proof/Compat Keep Pruning SSOT

## Intent

- shrink the remaining proof/compat keep surfaces to their minimal explicit ownership
- avoid delete-ready conclusions until `phase-57x`
- preserve `vm-hako` as reference/conformance keep outside this lane

## Canonical Reading

- `phase-56x` starts after route-surface retirement is landed.
- this lane is about pruning keep surfaces, not removing rust-vm.
- if a surface is still needed for explicit proof/compat operation, keep it but narrow it.

## Target Surfaces

- `tools/selfhost/lib/selfhost_run_routes.sh` (`stage-a`)
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
- `src/runner/modes/vm_fallback.rs`
- `lang/src/runner/stage1_cli/core.hako`
- `tools/selfhost/run_stageb_compiler_vm.sh`
- proof smoke wrappers that still overstate rust-vm ownership

## Inventory Lock

- proof-only keep:
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `tools/selfhost/bootstrap_selfhost_smoke.sh`
  - `tools/selfhost/selfhost_smoke.sh`
  - `tools/selfhost/selfhost_stage3_accept_smoke.sh`
  - `tools/plugins/plugin_v2_smoke.sh`
- compat keep:
  - `tools/selfhost/lib/selfhost_run_routes.sh` (`stage-a`)
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - `src/runner/modes/vm_fallback.rs`
  - `lang/src/runner/stage1_cli/core.hako`
- keep-now but out of pruning scope:
  - `src/runner/modes/vm.rs`
  - `src/runner/modes/vm_hako.rs`
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`

## Boundaries

- do not delete `vm.rs` in this lane
- do not touch `vm-hako` reference/conformance payloads here
- do not widen `stage-a` or fallback ownership while pruning
- keep `cargo check --bin hakorune` and `git diff --check` green

## Success Conditions

- proof-only and compat keeps are explicitly bounded and non-growing
- stale wrapper/comment/helper residue is removed from keep surfaces
- the lane hands off to `phase-57x rust-vm delete-ready audit / removal wave`
