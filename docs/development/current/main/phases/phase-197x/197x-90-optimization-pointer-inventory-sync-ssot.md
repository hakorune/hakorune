# 197x-90 Optimization Pointer Inventory Sync SSOT

Status: SSOT

## Goal

- keep optimization pointer docs aligned after `phase196x`
- remove stale wording that still points at lane A2 or old feature-pilot blockers

## In Scope

- `CURRENT_TASK.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/15-Workstream-Map.md`
- `docs/development/current/main/phases/README.md`
- `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`

## Fixed Decisions

- immediate next remains `semantic simplification bundle lane B0`
- `Front` / `Blocker` wording should describe the active layer/lane, not old feature-pilot backlog
- historical phase docs may preserve old sequencing, but if they mention a former "current next" it must be clearly past-tense

## Out of Scope

- any DCE code widening
- `generic placement / effect` implementation
- string guardrail or sum-route behavior changes
