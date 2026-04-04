---
Status: Landed
Date: 2026-04-04
Scope: choose the next source lane after the rust-vm retirement corridor concluded with residual explicit keep.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-63x/README.md
  - docs/development/current/main/phases/phase-63x/63x-90-rust-vm-final-retirement-decision-ssot.md
---

# Phase 64x: Next Source Lane Selection

## Goal

- choose the next source lane after the rust-vm retirement corridor
- respect the new stop-line:
  - rust-vm stays residual explicit keep
  - vm-hako stays reference/conformance
  - mainline remains direct/core

## Candidate Inventory

1. `stage1/selfhost mainline hardening`
   - owner files:
     - `lang/src/runner/stage1_cli_env.hako`
     - `lang/src/runner/stage1_cli.hako`
     - `lang/src/runner/launcher.hako`
     - `tools/selfhost/lib/stage1_contract.sh`
     - `tools/selfhost/lib/identity_routes.sh`
     - `tools/selfhost/build_stage1.sh`
   - read as:
     - current `.hako` compiler / stage1 authority path still carries the highest-leverage mainline source ownership
2. `llvm compare residue cleanup`
   - owner files:
     - `src/host_providers/llvm_codegen/ll_emit_compare_driver.rs`
     - `src/host_providers/llvm_codegen/ll_emit_compare_source.rs`
     - `src/host_providers/llvm_codegen/route.rs`
   - read as:
     - current `cargo check` dead_code warnings point here, so this is real hygiene but not the highest-leverage lane
3. `residual keep reevaluation`
   - owner files:
     - `src/runner/modes/vm.rs`
     - `src/runner/modes/vm_fallback.rs`
     - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
   - read as:
     - deferred until new caller-zero or replacement evidence appears

## Decision

- selected successor lane:
  - `phase-65x stage1/selfhost mainline hardening`
- reason:
  - highest leverage is now the direct `.hako` / Stage1 mainline owner cluster, not another rust-vm revisit
  - current dead_code cleanup under LLVM compare residue is narrower and can follow after mainline owner hardening

## Big Tasks

1. `64xA1` successor lane inventory lock
2. `64xA2` candidate lane ranking
3. `64xB1` successor lane decision
4. `64xD1` proof / closeout

## Result

- `64xA1` landed: successor lane inventory lock
- `64xA2` landed: candidate lane ranking
- `64xB1` landed: successor lane decision
- `64xD1` landed: proof / closeout
- handoff:
  - next lane is `phase-65x stage1/selfhost mainline hardening`
