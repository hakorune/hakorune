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

- current implementation lane: `phase274x IPO ThinLTO first cut`
- sibling guardrail lane: `phase137x main kilo reopen selection`
- immediate next: `IPO / build-time optimization`
- immediate follow-on: `PGO scaffold`
- top queued cut: `IPO / build-time optimization`

## Landing Snapshot

- latest landed:
  - `phase273x`: IPO now owns callable-node facts and call-edge facts before any ThinLTO wiring
- active:
  - `phase274x`: ThinLTO first cut now sits on top of the landed build-policy and callable/edge contract seams
- detail owner:
  - landed history stays in phase docs and roadmap SSOT

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/reference/concurrency/semantics.md`
4. `docs/development/current/main/phases/phase-274x/README.md`
5. `docs/development/current/main/phases/phase-163x/README.md`
6. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
git diff --check
```
