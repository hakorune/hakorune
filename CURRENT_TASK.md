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
4. `docs/development/current/main/phases/phase-246x/README.md`
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
  - `semantic simplification bundle`
- immediate follow-on:
  - `memory-effect layer`
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
  - `docs/development/current/main/phases/phase-230x/README.md`
- roadmap SSOT:
  - `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
- string guardrail owner:
  - `docs/development/current/main/phases/phase-137x/README.md`
- generic memory lane-B contract owner:
  - `docs/development/current/main/design/generic-memory-dce-observer-owner-contract-ssot.md`
- observer/control lane-C contract owner:
  - `docs/development/current/main/design/observer-control-dce-owner-contract-ssot.md`

## Current Notes

- `phase-246x` is landed: `Cancelled(reason)` now exists as a narrow scope-owned future path with stable `scope-cancelled` reason
- current semantic simplification cut now includes empty-trampoline jump threading after copied-constant `Branch` and constant `Compare` folds
- the next code lane is now `semantic simplification bundle`
- `CURRENT_TASK.md` is the only live status pointer; `05/10/15` are thin mirrors only
- if this file grows again, move the detail back into the phase docs
