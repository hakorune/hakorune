---
Status: Active
Date: 2026-04-03
Scope: remove the remaining live `--backend vm` helper-route defaults from stage0/runtime and keep `rust-vm` on explicit proof/oracle/compat rails.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-46x/README.md
  - docs/development/current/main/phases/phase-46x/46x-90-next-source-lane-selection-ssot.md
  - docs/development/current/main/phases/phase-46x/46x-91-task-board.md
---

# Phase 47x: Stage0/Runtime Direct-Core Finalization

## Goal

- move day-to-day stage0/runtime ownership from `hakorune --backend vm` to `hakorune` direct/core routes
- keep `.hako` source->MIR production on the producer side and `core_executor` on the terminal owner side
- leave `Program(JSON v0)` and VM gates as explicit compat/proof keeps only

## Plain Reading

- `phase-46x` selected this lane because the highest remaining feature tax lives in helper-route defaults, not in the already-shrunk VM core tail.
- the first job is to stop `tools/selfhost/lib/selfhost_run_routes.sh` from using `--backend vm` as the day-to-day runtime default.
- the second job is to make Stage-A source->MIR first, so compat `Program(JSON v0)` stays explicit fallback only.
- the third job is to drain `run_stageb_compiler_vm.sh` out of default Stage-B callers without deleting the proof gate.

## Success Conditions

- runtime default no longer executes `hakorune --backend vm` for day-to-day selfhost runs
- Stage-A first path is source->MIR first and `Program(JSON v0)` remains explicit compat fallback
- default Stage-B callers no longer depend on `run_stageb_compiler_vm.sh`
- `run_stageb_compiler_vm.sh` remains explicit proof-only keep
- `cargo check --bin hakorune` stays green

## Failure Patterns

- moving VM defaults around without actually draining them from helper routes
- widening `core.hako` or raw compat to cover new direct/core work
- treating proof-only VM gates as default producers again
- trying to archive `vm.rs` before helper-route defaults are drained

## Big Tasks

1. lock direct/core-first contracts for runtime and Stage-A routes
2. cut runtime default over from `--backend vm` to source->MIR->direct/core
3. make Stage-A source->MIR first and keep `Program(JSON v0)` explicit compat fallback
4. drain default Stage-B callers away from the VM proof gate
5. prove and close the lane cleanly
