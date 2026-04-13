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

- current implementation lane: `phase29bq selfhost mirbuilder failure-driven`
- sibling guardrail lane: `phase137x main kilo reopen selection`
- immediate next: `compiler expressivity first`
- immediate follow-on: `phase29bq failure-driven blocker capture`
- top queued cut: `phase29bq selfhost mirbuilder failure-driven`
- Compiler lane: `phase-29bq`（JIR-PORT-00..08 done / active blocker=`none` / next=`none`）
- JoinIR port mode（lane A）: monitor-only（failure-driven）

## Landing Snapshot

- latest landed:
  - `phase277x`: optimization lane closeout judgment froze the landed optimization roadmap and handed the mainline back to compiler expressivity / selfhost entry
- active:
  - `phase29bq`: failure-driven selfhost mirbuilder lane under compiler-expressivity-first policy
- detail owner:
  - landed history stays in phase docs and roadmap SSOT

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/reference/concurrency/semantics.md`
4. `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
5. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
6. `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
git diff --check
```
