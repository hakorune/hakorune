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

- current implementation lane: `phase163x primitive and user-box fast path`
- sibling guardrail lane: `phase137x main kilo reopen selection`
- immediate next: `generic placement / effect`
- immediate follow-on: `semantic simplification bundle`

## Landing Snapshot

- `phase224x` is landed:
  - publication/materialization helper proof lookup reads folded `placement_effect_routes` string proof first
  - legacy `string_corridor_candidates` stay as a compatibility fallback
  - retained and same-block route-window sinks from `phase222x` / `phase223x` remain intact

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-224x/README.md`
4. `docs/development/current/main/phases/phase-163x/README.md`
5. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
git diff --check
```
