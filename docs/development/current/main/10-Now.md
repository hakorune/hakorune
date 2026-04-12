---
Status: SSOT
Date: 2026-04-12
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

- `phase221x` is landed:
  - MIR refresh now materializes `metadata.string_kernel_plans`
  - the JSON emitter reads that metadata only
  - family-specific fallbacks remain as compatibility paths

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-221x/README.md`
4. `docs/development/current/main/phases/phase-163x/README.md`
5. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
git diff --check
```
