---
Status: Active
Date: 2026-04-15
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/10-Now.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
```

## Current

- lane: `phase-29bq selfhost mirbuilder failure-driven`
- guardrail: `phase-137x string corridor / exact-keeper guardrail`
- immediate next: `compiler expressivity first`
- immediate follow-on: `phase-29bq loop owner seam cleanup`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/15-Workstream-Map.md`
4. `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
5. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
6. `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
7. `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`
8. `docs/development/current/main/phases/phase-137x/README.md`

## Current Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
git diff --check
```
