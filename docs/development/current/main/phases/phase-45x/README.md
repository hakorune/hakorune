---
Status: Landed
Date: 2026-04-03
Scope: keep `rust-vm` as proof/oracle/compat keep and shrink the remaining vm-facing owner surfaces without reopening direct/core mainline.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-44x/README.md
  - docs/development/current/main/phases/phase-44x/44x-90-stage0-direct-core-follow-up-ssot.md
  - docs/development/current/main/phases/phase-44x/44x-91-task-board.md
---

# Phase 45x: VM Residual Cleanup

## Goal

- keep `rust-vm` as proof/oracle/compat keep instead of a day-to-day owner
- shrink residual vm-facing owner surfaces without re-opening direct/core mainline
- keep proof-only VM gates explicit and non-growing

## Plain Reading

- `phase-44x` already moved the live route defaults away from VM-backed ownership.
- what remains is residual owner pressure in `vm.rs`, `vm_fallback.rs`, `core.hako`, and explicit proof gates.
- this phase is about shrinking and freezing the remaining vm surfaces, not re-running direct/core follow-up.
- `stage0_capture.rs` is already route-neutral; it is not the next broad target.

## Success Conditions

- `vm.rs` is reduced to proof/oracle keep, not broad execution ownership
- `vm_fallback.rs` stays explicit fallback only
- `core.hako` remains narrow / no-widen compat hold line
- `run_stageb_compiler_vm.sh` stays explicit proof-only keep
- day-to-day defaults do not drift back to `--backend vm`

## Failure Patterns

- new capability work lands in vm-backed helpers because they are still treated as live defaults
- proof-only VM gates become convenient day-to-day routes again
- raw compat grows while residual cleanup is in flight

## Big Tasks

1. inventory residual vm owner surfaces and exact caller edges
2. shrink `vm.rs` to proof/oracle keep
3. drain `vm_fallback.rs` and shared vm helpers
4. freeze `core.hako` compat line and proof-only VM gates
5. prove and close the lane
