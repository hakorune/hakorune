---
Status: Active
Decision: provisional
Date: 2026-04-03
Scope: day-to-day caller を vm-gated route から外し、stage0/bootstrap mainline owner を `hakorune` binary の direct/core route 側へ寄せる。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-41x/README.md
  - docs/development/current/main/phases/phase-41x/41x-90-stage0-direct-core-route-hardening-ssot.md
  - docs/development/current/main/phases/phase-41x/41x-91-task-board.md
---

# Phase 42x: VM Caller Starvation / Direct-Core Owner Migration

## Goal

- stop day-to-day bootstrap/selfhost callers from feeding live `--backend vm` routes
- keep proof-only VM gates frozen and non-growing
- move owner pressure toward `hakorune` binary direct/core seams
- leave `vm.rs` / raw compat as proof-oracle/compat keep instead of feature surfaces

## Plain Reading

- 41x hardened the direct/core route and trimmed the obvious facades.
- 42x does not delete `rust-vm`; it starves the callers that still make `rust-vm` a feature tax.
- new capability work must not land on `run_stageb_compiler_vm.sh`, `vm.rs`, `core.hako`, or other proof/compat keeps.
- the active migration targets are the route facades and helpers that still funnel day-to-day work into vm-gated execution.

## Success Conditions

- `selfhost_build.sh` and `run.sh` are read as direct/core-first facades, not mixed day-to-day owners
- live caller families are drained away from vm-gated producer paths where direct/core routes already exist
- proof-only VM gates stay explicit and small
- `vm.rs` shrinks further toward proof/oracle keep after caller drain proves the route stable
- raw backend default/token remains deferred

## Failure Patterns

- `run_stageb_compiler_vm.sh` drifts back into day-to-day bootstrap mainline
- `stage1` compat or raw routes absorb new capability work
- `selfhost_build.sh` / `run.sh` keep acting as convenience wrappers around vm-gated execution
- caller starvation is postponed while new feature work keeps landing in vm-facing surfaces

## Fixed Reading

- `phase-41x` is landed; route hardening and `vm.rs` proof/oracle shrink are complete enough for handoff
- `42xA1` landed: caller-starvation targets are locked to `selfhost_build.sh` / `run.sh` / `child.rs` / `vm.rs` / `vm_fallback.rs`
- `42xA2` landed: proof-only VM keeps are frozen as explicit do-not-grow surfaces
- `42xB1` landed: `selfhost_build.sh` downstream caller pressure is moved into helper-owned route-main code
- `42xB2` landed: `run.sh` route-only facade migration keeps route path constants helper-owned and the top level parser/dispatch-only
- `42xC1` landed: `child.rs` shell-only drain moved the stage0 child capture implementation into `stage0_capture.rs`
- `42xC2` landed: `vm.rs` preflight/source-prepare split moved source read / source_hint / safety gate into `vm_source_prepare.rs`
- `run_stageb_compiler_vm.sh`, `selfhost_vm_smoke.sh`, and `selfhost_stage3_accept_smoke.sh` remain proof-only keeps
- `lang/src/runner/stage1_cli/core.hako` remains compat keep and must stay no-widen
- `src/runner/core_executor.rs` remains the direct owner for in-proc/direct MIR execution
- `tools/selfhost/run_stage1_cli.sh` and `tools/selfhost/stage1_mainline_smoke.sh` remain the direct proof home
- `tools/selfhost/selfhost_build.sh`, `src/runner/modes/vm.rs`, and `src/runner/modes/vm_fallback.rs` remain active starvation/migration surfaces
- `src/runner/modes/vm.rs` and `src/runner/modes/vm_fallback.rs` remain engineering keep until caller drain proves they can shrink again

## Big Tasks

1. lock caller-starvation targets and do-not-grow keeps exactly
2. move `selfhost_build.sh` caller pressure toward direct/core routes
3. drain `vm.rs` / `vm_fallback.rs` / compat keep from day-to-day route ownership
4. close out with focused proof and handoff

## Exact Next

1. `docs/development/current/main/15-Workstream-Map.md`
2. `CURRENT_TASK.md`
3. `docs/development/current/main/phases/phase-42x/42x-90-vm-caller-starvation-direct-core-migration-ssot.md`
4. `docs/development/current/main/phases/phase-42x/42x-91-task-board.md`
5. `tools/selfhost/selfhost_build.sh`
6. `src/runner/modes/vm.rs`
7. `src/runner/modes/common_util/vm_source_prepare.rs`
8. `lang/src/runner/stage1_cli/core.hako`

- current active micro task: `42xD1 proof / closeout`
- next micro task: `next source lane selection`
