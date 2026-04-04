---
Status: SSOT
Date: 2026-04-04
Scope: continue rust-vm route-surface retirement after phase-58x selected this lane over another keep-pruning or delete-ready rerun.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-58x/README.md
---

# 59x-90 Rust-VM Route-Surface Retirement Continuation SSOT

## Intent

- keep attacking the route/default/help layer that can still re-grow rust-vm pressure
- do not broaden deletion claims while the keep-now cores remain necessary
- treat explicit compat/proof access as bounded keeps, not owner lanes

## Highest-Leverage Surfaces

- `src/cli/args.rs`
  - explicit backend help still advertises `vm` / `vm-hako`
- `src/runner/dispatch.rs`
  - explicit compat/proof and reference route banners remain visible
- `src/runner/route_orchestrator.rs`
  - explicit `vm` / `vm-hako` plan selection still defines the live affordance seam
- `tools/selfhost/lib/selfhost_run_routes.sh`
  - `stage-a` compat branch still executes `--backend vm`
- `tools/selfhost/run.sh`
  - still fronts the explicit compat route surface

## Boundaries

- do not delete `vm.rs`, `vm_fallback.rs`, `stage_a_compat_bridge.rs`, `core.hako`, or `run_stageb_compiler_vm.sh` in this lane
- do not mix `vm-hako` reference/conformance cleanup into this lane
- prefer narrowing labels, defaults, and explicit route affordances over touching keep-now source cores

## Inventory Lock

- `src/cli/args.rs`
  - explicit backend help still advertises `vm` and `vm-hako`
- `src/runner/dispatch.rs`
  - explicit compat/proof and reference route banners remain visible
- `src/runner/route_orchestrator.rs`
  - explicit `vm` / `vm-hako` plan selection and reasons remain the owner seam
- `tools/selfhost/lib/selfhost_run_routes.sh`
  - `runtime_mode=stage-a` still executes the explicit compat branch on `--backend vm`
- `tools/selfhost/run.sh`
  - still fronts the explicit compat route through help/usage surface
- supporting live docs/examples
  - `README.md`
  - `README.ja.md`
  - `tools/selfhost/README.md`
