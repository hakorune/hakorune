---
Status: Landed
Decision: `direct/core follow-up` selected
Date: 2026-04-03
Scope: phase-42x closeout 後の successor source lane を選び、rust-vm を proof/compat keep のまま次の主線へ handoff する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-42x/README.md
  - docs/development/current/main/phases/phase-42x/42x-90-vm-caller-starvation-direct-core-migration-ssot.md
  - docs/development/current/main/phases/phase-42x/42x-91-task-board.md
  - docs/development/current/main/design/optimization/README.md
---

# Phase 43x: Next Source Lane Selection

## Goal

- choose the next source lane after phase-42x closeout
- keep rust-vm as proof/compat keep while the choice is being made
- avoid pulling far-future optimization back into the next lane by accident

## Plain Reading

- 42x proved that caller starvation can shrink vm feature tax.
- 43x is a selection phase, not a broad rewrite.
- direct/core owner routes already exist; the question is which follow-up lane gives the best leverage.
- optimization stays far-future and does not preempt the next source lane choice.

## Success Conditions

- successor lane is chosen and documented
- current docs point at that lane cleanly
- phase-42x stays landed and rust-vm keeps stay narrow
- no broad rewrite starts before the lane choice is fixed

## Failure Patterns

- selection drifts into implementation before the lane is chosen
- optimization gets pulled forward from the far-future lane
- rust-vm proof/compat keep starts growing again

## Fixed Reading

- phase-42x is landed and handed off
- rust-vm remains proof/compat keep, not a mainline owner
- kilo optimization is far-future
- the selected successor lane is `phase-44x stage0 direct/core follow-up`
- current documentation now points at `phase-44x` rather than leaving selection provisional

## Big Tasks

1. shortlist candidate successor lanes
2. compare leverage against rust-vm feature tax
3. select `direct/core follow-up` as the next lane
4. hand off to `phase-44x`
