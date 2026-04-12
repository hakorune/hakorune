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
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-223x/README.md`
4. `docs/development/current/main/phases/phase-224x/README.md`
5. `docs/development/current/main/phases/phase-225x/README.md`
6. `docs/development/current/main/phases/phase-163x/README.md`
7. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
8. `git status -sb`
9. `tools/checks/dev_gate.sh quick`

## Restart Handoff

- expected worktree:
  - clean
- active lane:
  - `phase-163x primitive and user-box fast path`
- sibling guardrail:
  - `phase-137x` string corridor / exact-keeper guardrail; `phase-219x` / `phase-220x` / `phase-221x` / `phase-222x` / `phase-223x` / `phase-224x` / `phase-225x` are landed
- immediate next:
  - `generic placement / effect`
- immediate follow-on:
  - `semantic simplification bundle`
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
  - `docs/development/current/main/phases/phase-225x/README.md`
- roadmap SSOT:
  - `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
- string guardrail owner:
  - `docs/development/current/main/phases/phase-137x/README.md`
- generic memory lane-B contract owner:
  - `docs/development/current/main/design/generic-memory-dce-observer-owner-contract-ssot.md`
- observer/control lane-C contract owner:
  - `docs/development/current/main/design/observer-control-dce-owner-contract-ssot.md`

## Current Notes

- `phase-225x` is landed: optimizer pre/post-DCE placement/effect hooks now run through one generic transform owner seam, with the landed string corridor sink delegated underneath
- the next code lane remains `generic placement / effect`
- if this file grows again, move the detail back into the phase docs
