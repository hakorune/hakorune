---
Status: Landed
Date: 2026-04-27
Scope: next compiler-cleanliness lane selection
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/291x-436-cleanup-burst-closeout-review-card.md
  - docs/development/current/main/design/selfhost-authority-facade-compat-inventory-ssot.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
---

# 291x-437: Next Lane Selection

## Goal

Choose the next compiler-cleanliness lane after closing the normalized-shadow /
normalization cleanup burst.

This card is lane selection only. No behavior changed.

## Candidate Lanes

| Lane | Shape | Decision |
| --- | --- | --- |
| Stage-B adapter thinning | BoxShape | select next |
| CoreMethodContract -> CoreOp / LoweringPlan migration | larger contract lane | defer |
| `.inc` generated enum/table consumer migration | generator/consumer lane | defer until contract manifests are ready |
| MapGet return-shape metadata/proof/lowering | proof/lowering lane | defer until contract/CoreOp lane is active |
| route-entry router compatibility around canonicalizer absence | local routing seam | defer behind Stage-B selection |

## Decision

Select **Stage-B adapter thinning** as the next lane.

Reason:

- The previous cleanup burst is closed and should not keep absorbing small
  wording work.
- Stage-B thinning is a bounded BoxShape lane: it can reduce authority drift
  without adding new accepted source shapes.
- `BuildBox.emit_program_json_v0(...)` is already documented as the source ->
  Program(JSON v0) authority. The entry adapter should move closer to that
  boundary before larger CoreMethodContract/CoreOp work resumes.
- CoreMethodContract/CoreOp, `.inc` enum/table, and MapGet proof/lowering work
  are semantically coupled and should not be mixed into this BoxShape lane.

## Next Card

Create `291x-438-stageb-adapter-thinning-inventory` before touching code.

The inventory must identify which responsibilities still live in
`lang/src/compiler/entry/compiler_stageb.hako` and classify each as one of:

- keep in entry adapter
- already owned by `BuildBox`
- extract behind a small compat/helper box
- defer to CoreMethodContract/CoreOp or another lane

## Guards

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
