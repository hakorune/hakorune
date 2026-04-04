---
Status: Landed
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

## Caller-Zero Proof Bundle

- `src/runner/modes/vm.rs`
  - evidence:
    - `src/runner/route_orchestrator.rs:179` -> `VmRouteAction::Vm => runner.execute_vm_mode(filename)`
- `src/runner/modes/vm_fallback.rs`
  - evidence:
    - `src/runner/route_orchestrator.rs:181` -> `VmRouteAction::CompatFallback => runner.execute_vm_fallback_interpreter(filename)`
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - evidence:
    - `src/runner/modes/common_util/selfhost/stage_a_route.rs:81` -> `resolve_program_payload_to_mir(...)`
    - `src/runner/selfhost.rs:176` -> `stage_a_route::try_capture_stage_a_module(...)`
- `lang/src/runner/stage1_cli/core.hako`
  - evidence:
    - `core.hako:163` -> `_mode_run(...) -> run_program_json(...)`
    - `core.hako:257` -> `_run_raw_request(...) -> run_program_json(...)`
- `tools/selfhost/run_stageb_compiler_vm.sh`
  - evidence:
    - `tools/selfhost/selfhost_smoke.sh:10`
    - `tools/selfhost/lib/selfhost_run_routes.sh:9`
    - `tools/smokes/v2/profiles/integration/selfhost/*stageb*_vm.sh`
    - `tools/smokes/v2/profiles/integration/parser/parser_rune_decl_local_attrs_selected_entry_trace.sh:246`
- `src/runner/dispatch.rs`
  - evidence:
    - explicit backend override surface still documents/accepts `vm` / `vm-hako`
- `src/runner/route_orchestrator.rs`
  - evidence:
    - keeps `Vm`, `CompatFallback`, and `VmHako` explicit route actions live

## Removal Candidate Shortlist

- shortlist: none
- reason:
  - every audited surface still has at least one explicit caller, route owner, or compat contract role
- implication:
  - `62x` should only remove anything if `61xD1` or later prep adds new caller-zero facts

## Proof

- `cargo check --bin hakorune` PASS
- `git diff --check` PASS

## Handoff

- `61x` closes with no newly proven delete-ready rust-vm core surfaces.
- successor lane:
  - `62x rust-vm delete-ready removal wave`

## Boundary

- `61x` reruns facts; it does not remove broad sources by itself
- any delete-ready claim must show caller-zero or explicit replacement evidence
- successor lanes stay fixed:
  1. `62x rust-vm delete-ready removal wave`
  2. `63x rust-vm final retirement decision`
