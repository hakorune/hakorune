---
Status: Landed
Date: 2026-04-04
Scope: decide whether rust-vm can retire fully or must remain as a residual explicit keep after the 60x->62x corridor.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-62x/README.md
  - docs/development/current/main/phases/phase-62x/62x-90-rust-vm-delete-ready-removal-wave-ssot.md
---

# Phase 63x: Rust-VM Final Retirement Decision

## Goal

- decide whether full rust-vm retirement is now defensible
- if not, define the residual explicit keep set and stop-line clearly
- keep `vm-hako` out of scope as reference/conformance

## Decision Inputs

- `60x`: proof/compat keep pruning continuation
- `61x`: caller-zero audit rerun
- `62x`: delete-ready removal wave (no-op)

## Evidence Lock

- mainline retirement: already achieved
  - day-to-day runtime/default routes no longer depend on `--backend vm`
- full source retirement: not yet defensible
  - `vm.rs` still has `route_orchestrator` caller
  - `vm_fallback.rs` still has `route_orchestrator` caller
  - `stage_a_compat_bridge.rs` still has `stage_a_route` caller
  - `core.hako` still owns raw compat callers
  - `run_stageb_compiler_vm.sh` still has proof callers
  - `dispatch.rs` / `route_orchestrator.rs` still expose explicit keep/reference backend override handling
- dead_code reading:
  - current `cargo check` dead_code warnings point at unrelated LLVM compare residue, not rust-vm core surfaces
  - dead_code therefore does not support broad rust-vm deletion today

## Decision

- `63xA2` decision:
  - full rust-vm retirement: `no`
  - residual explicit keep: `yes`

## Residual Keep Stop-Line

- residual explicit keep set:
  - `src/runner/modes/vm.rs`
  - `src/runner/modes/vm_fallback.rs`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - `lang/src/runner/stage1_cli/core.hako`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`
- stop-line:
  - keep explicit and non-growing
  - do not restore day-to-day ownership to these surfaces
  - revisit retirement only after new caller-zero facts or explicit route replacement

## Big Tasks

1. decision inventory
   - `63xA1` retirement-decision evidence lock
   - `63xA2` retire-vs-residual decision
2. stop-line definition
   - `63xB1` residual keep stop-line or retirement plan freeze
3. prove and close
   - `63xD1` proof / closeout

## Result

- `63xA1` landed: retirement-decision evidence lock
- `63xA2` landed: retire-vs-residual decision
- `63xB1` landed: residual explicit keep stop-line freeze
- `63xD1` landed: proof / closeout
- handoff:
  - next lane is `phase-64x next source lane selection`
