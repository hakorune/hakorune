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
4. `docs/development/current/main/phases/phase-277x/README.md`
5. `docs/development/current/main/phases/phase-163x/README.md`
6. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
7. `git status -sb`
8. `tools/checks/dev_gate.sh quick`

## Restart Handoff

- expected worktree:
  - clean
- active lane:
  - `phase-277x optimization lane closeout judgment`
- sibling guardrail:
  - `phase-137x` string corridor / exact-keeper guardrail
- immediate next:
  - `optimization lane closeout judgment`
- immediate follow-on:
  - `post-optimization roadmap refresh`
- current stop-lines:
  - do not mix lane B with lane C (`Debug` / terminator-adjacent operand/control liveness cleanup)
  - do not mix lane B with `generic placement / effect`
  - do not mix parked `phase-96x` backlog into the active optimization lane
- parked corridor:
  - `phase-96x vm_hako LLVM acceptance cutover`
  - only remaining backlog is monitor-policy decision for the frozen `vm-hako-core` pack

## Design Owners

- implementation lane:
  - `docs/development/current/main/phases/phase-277x/README.md`
- next layer landing:
  - `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
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

- latest landed phase:
  - `phase-276x`: PGO now resolves first generate/use artifacts and emits a `.pgo.json` sidecar while keeping LLVM-side instrumentation/use out of scope
- active focus:
  - `phase-277x`: optimization lane closeout judgment after the landed IPO cuts
- pointer rule:
  - `CURRENT_TASK.md` is the only live status pointer
  - `05/10/15` stay thin mirrors only
  - landed detail lives in phase docs, not here

## Execution Queue

1. `optimization lane closeout judgment`
   - IPO lane now has landed build-policy, callable/edge, ThinLTO, and PGO cuts
   - next work is closing the optimization roadmap cleanly
2. `post-optimization roadmap refresh`
