Status: SSOT mirror
Date: 2026-06-24
Scope: one-screen current dashboard. Do not store landed history here.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md

# Now

## Current

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- workstream card: read `latest_workstream_card` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`
- selfhost roadmap: `docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md`
  now narrows the remaining work to family-by-family `HakoAdopted` decisions,
  the Python SemanticProjector freeze, and consultation-gated ABI / syntax
  boundaries.
- ProgramJSON migration policy:
  `docs/development/current/main/design/mirbuilder-programjson-capability-batch-migration-policy-ssot.md`

## Rule

This file is only a mirror. Implementation details, acceptance, landed history,
and parked tasks belong in the active card, the workstream SSOT, phase cards,
or git history.
