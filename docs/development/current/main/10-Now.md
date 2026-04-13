---
Status: SSOT
Date: 2026-04-13
Scope: current lane / blocker / next pointer だけを置く薄い mirror。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Self Current Task — Now (main)

## Current

- current implementation lane: `phase276x IPO PGO generate/use first cut`
- sibling guardrail lane: `phase137x main kilo reopen selection`
- immediate next: `IPO / build-time optimization`
- immediate follow-on: `optimization lane closeout judgment`
- top queued cut: `IPO / build-time optimization`

## Landing Snapshot

- latest landed:
  - `phase275x`: PGO scaffold now has a dedicated owner seam while generate/use behavior stays disabled
- active:
  - `phase276x`: first PGO generate/use cut now sits on top of the landed build-policy, callable/edge, ThinLTO, and PGO seams
- detail owner:
  - landed history stays in phase docs and roadmap SSOT

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/reference/concurrency/semantics.md`
4. `docs/development/current/main/phases/phase-276x/README.md`
5. `docs/development/current/main/phases/phase-163x/README.md`
6. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
git diff --check
```
