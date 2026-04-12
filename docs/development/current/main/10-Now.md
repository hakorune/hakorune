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
- immediate next: `semantic simplification bundle`
- immediate follow-on: `memory-effect layer`

## Landing Snapshot

- `phase246x` is landed:
  - `Cancelled(reason)` now exists as a narrow scope-owned future path with stable `scope-cancelled` reason
- latest semantic simplification cut:
  - copied-constant `Branch` terminators, constant `Compare` instructions, and empty trampoline jump-threading now fold before CFG merge

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-246x/README.md`
4. `docs/development/current/main/phases/phase-163x/README.md`
5. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
git diff --check
```
