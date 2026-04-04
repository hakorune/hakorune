---
Status: Active
Date: 2026-04-04
Scope: rerun caller-zero facts after phase-60x narrowed the remaining proof/compat keep bucket.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-60x/README.md
  - docs/development/current/main/phases/phase-60x/60x-90-proof-compat-keep-pruning-continuation-ssot.md
  - docs/development/current/main/phases/phase-60x/60x-91-task-board.md
---

# Phase 61x: Residual Rust-VM Caller-Zero Audit Rerun

## Goal

- rerun source-backed caller-zero facts after `60x` pruning
- separate `keep-now` from anything that is now actually `delete-ready`
- keep `vm-hako` reference/conformance surfaces out of scope

## Focus Surfaces

- `src/runner/modes/vm.rs`
- `src/runner/modes/vm_fallback.rs`
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
- `lang/src/runner/stage1_cli/core.hako`
- `tools/selfhost/run_stageb_compiler_vm.sh`
- `src/runner/dispatch.rs`
- `src/runner/route_orchestrator.rs`

## Inventory Lock

- `src/runner/modes/vm.rs`
  - still called only from `src/runner/route_orchestrator.rs` via `VmRouteAction::Vm`
  - not caller-zero
- `src/runner/modes/vm_fallback.rs`
  - still called only from `src/runner/route_orchestrator.rs` via `VmRouteAction::CompatFallback`
  - not caller-zero
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - still called from `src/runner/modes/common_util/selfhost/stage_a_route.rs`
  - narrowed in `60x`, but not caller-zero
- `lang/src/runner/stage1_cli/core.hako`
  - `run_program_json(...)` still owns raw compat calls from `_mode_run(...)` and `_run_raw_request(...)`
  - not caller-zero
- `tools/selfhost/run_stageb_compiler_vm.sh`
  - still called by `tools/selfhost/selfhost_smoke.sh`, `tools/selfhost/lib/selfhost_run_routes.sh`, and Stage-B proof smokes
  - not caller-zero
- `src/runner/dispatch.rs`
  - still accepts explicit backend override surfaces for `vm` / `vm-hako`
  - not a removal target in `61x`
- `src/runner/route_orchestrator.rs`
  - still owns explicit keep/reference dispatch for `Vm`, `CompatFallback`, and `VmHako`
  - not a removal target in `61x`
- first-pass result:
  - `delete-ready`: none
  - `keep-now`: all listed focus surfaces

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
- freeze rule:
  - `61x` does not manufacture removal candidates from wording-only cleanup
  - `62x` may remove only surfaces that gain caller-zero or explicit replacement proof after `61xB`

## Success Conditions

- caller-zero facts are source-backed, not inferred from wording
- delete-ready claims remain narrow and auditable
- `cargo check --bin hakorune` and `git diff --check` stay green

## Big Tasks

1. rerun inventory and classification
   - `61xA1` residual caller inventory rerun
   - `61xA2` keep/delete-ready classification freeze
2. audit and prepare
   - `61xB1` caller-zero proof bundle
   - `61xB2` removal candidate shortlist
3. prove and close
   - `61xD1` proof / closeout
