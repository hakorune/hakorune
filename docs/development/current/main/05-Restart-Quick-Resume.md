---
Status: Active
Date: 2026-04-13
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

- lane: `phase-163x primitive and user-box fast path`
- guardrail: `phase-137x main kilo reopen selection`
- immediate next: `semantic simplification bundle`
- immediate follow-on: `memory-effect layer`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/15-Workstream-Map.md`
4. `docs/development/current/main/phases/phase-242x/README.md`
5. `docs/development/current/main/phases/phase-163x/README.md`
6. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
7. `docs/development/current/main/phases/phase-137x/README.md`

## Current Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
git diff --check
```
