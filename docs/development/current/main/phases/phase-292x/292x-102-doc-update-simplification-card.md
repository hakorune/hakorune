---
Status: Active
Date: 2026-04-22
Scope: documentation-update policy for Phase 292x.
Related:
  - docs/development/current/main/phases/phase-292x/292x-STATUS.toml
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/CURRENT_STATE.toml
  - tools/checks/current_state_pointer_guard.sh
---

# 292x-102: Phase Docs Update Simplification

## Problem

Phase 292x updates currently require touching too many human-written mirrors:

- `CURRENT_TASK.md`
- `10-Now.md`
- `05-Restart-Quick-Resume.md`
- `15-Workstream-Map.md`
- `CURRENT_STATE.toml`
- phase README
- task board
- debt ledger
- active slice card

That makes small `.inc` cleanup slices carry a large docs synchronization cost.
It also increases the chance that one mirror drifts from the actual lane state.

## Decision

Use `292x-STATUS.toml` as the compact phase status SSOT.

Every landed `.inc` thinning slice must update:

1. `292x-STATUS.toml`
2. the active slice card, only with the decision/result that future readers need
3. `292x-92-inc-codegen-analysis-debt-ledger.md`, only when the guard baseline
   changes

Current mirrors are only updated when one of these changes:

- active lane
- active blocker token
- next recommended slice
- guard baseline
- restart pointer list

## Human Update Rule

Do not copy the full slice result into every current mirror.

Allowed mirror text is short:

```text
phase-292x status SSOT: docs/development/current/main/phases/phase-292x/292x-STATUS.toml
latest landed slice: <id>
next recommended slice: <id>
```

Detailed per-slice facts belong in the phase status file and the active card.

## Future Automation

The intended follow-up is a generated-region sync helper:

```bash
python3 tools/docs/sync_current_lane_docs.py --check
python3 tools/docs/sync_current_lane_docs.py --write
```

The helper should read:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/phases/phase-292x/292x-STATUS.toml`

and update only marked generated regions in:

- `CURRENT_TASK.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `docs/development/current/main/15-Workstream-Map.md`

Until that helper exists, keep the rule manual but strict: status first, mirrors
only when the summary actually changes.

## Acceptance

- `292x-STATUS.toml` exists and records:
  - guard baseline
  - landed slices
  - remaining seed matcher backlog
  - next recommended slice
- Phase README points readers to the status SSOT before historical cards.
- Current mirrors point to the status SSOT instead of duplicating full slice
  ledgers.
- `tools/checks/current_state_pointer_guard.sh` still passes.
