# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-13
Scope: current lane / next lane / restart order only.

## Purpose

- root から active lane / next lane に最短で戻る
- landed history は phase docs を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/15-Workstream-Map.md`
4. `docs/development/current/main/phases/phase-260x/README.md`
5. `docs/development/current/main/phases/phase-163x/README.md`
6. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
7. `git status -sb`
8. `tools/checks/dev_gate.sh quick`

## Restart Handoff

- expected worktree:
  - clean
- active lane:
  - `phase-163x primitive and user-box fast path`
- sibling guardrail:
  - `phase-137x` string corridor / exact-keeper guardrail
- immediate next:
  - `escape / barrier -> LLVM attrs`
- immediate follow-on:
  - `numeric loop / SIMD`
- current stop-lines:
  - do not mix lane B with lane C (`Debug` / terminator-adjacent operand/control liveness cleanup)
  - do not mix lane B with `generic placement / effect`
  - do not mix parked `phase-96x` backlog into the active optimization lane
- parked corridor:
  - `phase-96x vm_hako LLVM acceptance cutover`
  - only remaining backlog is monitor-policy decision for the frozen `vm-hako-core` pack

## Design Owners

- implementation lane:
  - `docs/development/current/main/phases/phase-163x/README.md`
- next layer landing:
  - `docs/development/current/main/phases/phase-260x/README.md`
- roadmap SSOT:
  - `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
- string guardrail owner:
  - `docs/development/current/main/phases/phase-137x/README.md`
- generic memory lane-B contract owner:
  - `docs/development/current/main/design/generic-memory-dce-observer-owner-contract-ssot.md`
- observer/control lane-C contract owner:
  - `docs/development/current/main/design/observer-control-dce-owner-contract-ssot.md`
- concurrency manual owner:
  - `docs/reference/concurrency/semantics.md`
- concurrency runtime-plan owner:
  - `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`

## Current Notes

- concurrency manuals are now re-pointed to the current `task_scope` / `joinAll()` / `failureReport()` owners
- `phase-255x` is landed: `joinAll()` now returns `Err(TaskJoinTimeout: timed out after Nms)` when its bounded join hits deadline without a latched first failure
- `phase-256x` is landed: `SimplifyCFG` now threads a branch arm through an empty jump trampoline into a final block when its PHIs can be trivially rewritten from the trampoline predecessor to the branching block
- `phase-257x` is landed: `SimplifyCFG` now threads a branch arm through an empty jump trampoline even when the threaded arm carried edge-args, but only when those edge-args are dead for a PHI-free final target
- `phase-258x` is landed: `SimplifyCFG` now propagates constant conditions through single-input PHIs before folding compare / branch conditions
- `phase-259x` is landed: SimplifyCFG closeout judgment hands the remaining optimization lane to memory-effect work
- `phase-260x` is landed: the memory-effect owner seam and stats surface now sit on their own top-level pass, and the same-block private-carrier slices are fully landed
- explicit scope-exit timeout surfacing is parked while the optimization lane hands off to `memory-effect layer`
- the next code lane is now `memory-effect layer`
- `CURRENT_TASK.md` is the only live status pointer; `05/10/15` are thin mirrors only
- if this file grows again, move the detail back into the phase docs

## Execution Queue

1. `memory-effect layer`
   - hand off to `escape / barrier -> LLVM attrs`
   - do not bury the next memory slices inside `dce/memory.rs` without a top-level owner
