---
Status: Landed
Date: 2026-04-04
Scope: make the final decision on full rust-vm retirement versus residual explicit keep.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-61x/61x-90-residual-rust-vm-caller-zero-audit-rerun-ssot.md
  - docs/development/current/main/phases/phase-62x/62x-90-rust-vm-delete-ready-removal-wave-ssot.md
---

# 63x-90 Rust-VM Final Retirement Decision SSOT

## Intent

- collect the corridor evidence in one place
- decide whether full retirement is supported by source-backed facts
- otherwise freeze a residual explicit keep set and stop-line

## Current Starting Read

- mainline retirement is already achieved
- full source retirement is not yet proven
- `62x` removal wave was a no-op because delete-ready candidates did not materialize

## Evidence Lock

- route/default evidence:
  - mainline no longer treats `--backend vm` as a default owner lane
- caller evidence:
  - `src/runner/modes/vm.rs`
    - `src/runner/route_orchestrator.rs:179` still dispatches `VmRouteAction::Vm => runner.execute_vm_mode(filename)`
  - `src/runner/modes/vm_fallback.rs`
    - `src/runner/route_orchestrator.rs:181` still dispatches `VmRouteAction::CompatFallback => runner.execute_vm_fallback_interpreter(filename)`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
    - `src/runner/modes/common_util/selfhost/stage_a_route.rs:81` still calls `resolve_program_payload_to_mir(...)`
  - `lang/src/runner/stage1_cli/core.hako`
    - `core.hako:163` and `core.hako:257` still route through `run_program_json(...)`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
    - still referenced by `tools/selfhost/selfhost_smoke.sh`, `tools/selfhost/lib/selfhost_run_routes.sh`, and Stage-B proof smokes
  - `src/runner/dispatch.rs` / `src/runner/route_orchestrator.rs`
    - still own explicit backend override handling for `vm` / `vm-hako`
- dead_code evidence:
  - current `cargo check --bin hakorune` dead_code warnings point to LLVM compare residue, not rust-vm core
  - therefore dead_code does not currently open a rust-vm deletion wave

## Decision Boundary

- full retirement requires:
  - no broad rust-vm source remaining in active route ownership
  - no compat/proof keep that still has unavoidable callers
  - no required backend override surface for `vm`
- otherwise:
  - declare residual explicit keep
  - stop widening it
  - hand off a later reevaluation point instead of forcing deletion

## Decision

- final decision:
  - `mainline retirement`: achieved
  - `full rust-vm source retirement`: not yet defensible
  - `residual explicit keep`: required

## Residual Explicit Keep Stop-Line

- fixed keep set:
  - `src/runner/modes/vm.rs`
  - `src/runner/modes/vm_fallback.rs`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - `lang/src/runner/stage1_cli/core.hako`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`
- freeze:
  - no new mainline capability work lands here
  - keep-only callers stay explicit proof/compat/reference
  - next retirement attempt requires new caller-zero or explicit replacement evidence

## Proof

- `cargo check --bin hakorune` PASS
- `git diff --check` PASS

## Handoff

- retirement corridor outcome:
  - mainline retirement: complete
  - full source retirement: deferred
  - residual explicit keep: frozen
- next lane:
  - `phase-64x next source lane selection`
