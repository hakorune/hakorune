---
Status: Accepted
Decision: accepted
Date: 2026-03-10
Scope: `phase-29ch` の docs-first checklist。
Related:
  - docs/development/current/main/phases/phase-29ch/README.md
  - docs/development/current/main/phases/phase-29cg/README.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
---

# 29ch-10 MIR-Direct Bootstrap Unification Checklist

## 1) Boundary lock

- [x] `MDB-01` `phase-29ch` is authority migration, not JSON v0 deletion
- [x] `MDB-02` `phase-29cg` solved bucket is keep-closed
- [x] `MDB-03` reduced proof source remains the first target

## 2) Current authority handoff

- [x] `MDB-04` `phase-29cg` reduced-case authority is pinned as input evidence
- [x] `MDB-05` exact MIR-direct owner/route inventory is fixed
- [x] `MDB-06` one reduced proof source is moved to MIR-direct authority

## 3) Done judgment

- [x] reduced bootstrap can be explained without JSON v0 route authority
- [x] bridge is documented as `temporary bootstrap boundary` only
- [ ] next separate JSON v0 retirement phase can be cut without mixing concerns

## 4) Restart quick entry

- final goal: `parser -> selfhost mirbuilder -> MIR(JSON) -> backend/VM`
- current phase: `phase-29ch` is MIR-direct bootstrap unification only
- non-goal in this phase: `Program(JSON v0)` deletion
- first task: fix exact MIR-direct owner/route inventory for one reduced proof source
