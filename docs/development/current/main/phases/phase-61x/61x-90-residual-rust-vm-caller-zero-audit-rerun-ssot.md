---
Status: SSOT
Date: 2026-04-04
Scope: rerun caller-zero facts after phase-60x pruning narrowed the explicit keep bucket.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-60x/README.md
---

# 61x-90 Residual Rust-VM Caller-Zero Audit Rerun SSOT

## Intent

- verify whether any residual `rust-vm` keep has become delete-ready after `60x`
- keep the audit source-backed with `rg`/callers/smokes, not wording-only
- avoid mixing `vm-hako` reference/conformance work into the rust-vm retirement corridor

## Audit Targets

- `src/runner/modes/vm.rs`
- `src/runner/modes/vm_fallback.rs`
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
- `lang/src/runner/stage1_cli/core.hako`
- `tools/selfhost/run_stageb_compiler_vm.sh`
- `src/runner/dispatch.rs`
- `src/runner/route_orchestrator.rs`

## Inventory Lock

- `src/runner/modes/vm.rs`
  - caller: `src/runner/route_orchestrator.rs` -> `VmRouteAction::Vm`
  - classification: `keep-now`
- `src/runner/modes/vm_fallback.rs`
  - caller: `src/runner/route_orchestrator.rs` -> `VmRouteAction::CompatFallback`
  - classification: `keep-now`
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - caller: `src/runner/modes/common_util/selfhost/stage_a_route.rs`
  - classification: `keep-now`
- `lang/src/runner/stage1_cli/core.hako`
  - callers: `_mode_run(...)`, `_run_raw_request(...)`
  - classification: `keep-now`
- `tools/selfhost/run_stageb_compiler_vm.sh`
  - callers: `tools/selfhost/selfhost_smoke.sh`, `tools/selfhost/lib/selfhost_run_routes.sh`, Stage-B proof smokes
  - classification: `keep-now`
- `src/runner/dispatch.rs`
  - current role: explicit backend override surface for `vm` / `vm-hako`
  - classification: `keep-now`
- `src/runner/route_orchestrator.rs`
  - current role: explicit keep/reference route owner for `Vm`, `CompatFallback`, `VmHako`
  - classification: `keep-now`
- rerun result:
  - `delete-ready`: none in the first pass
  - `archive-later`: none newly promoted in `61xA1`

## Classification Freeze

- `keep-now`
  - `src/runner/modes/vm.rs`
  - `src/runner/modes/vm_fallback.rs`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - `lang/src/runner/stage1_cli/core.hako`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`
- `delete-ready`
  - none
- `archive-later`
  - none newly promoted in `61x`
- frozen reading:
  - `61xB` collects proof bundles only
  - `62x` stays gated on new caller-zero or replacement evidence

## Boundary

- `61x` reruns facts; it does not remove broad sources by itself
- any delete-ready claim must show caller-zero or explicit replacement evidence
- successor lanes stay fixed:
  1. `62x rust-vm delete-ready removal wave`
  2. `63x rust-vm final retirement decision`
