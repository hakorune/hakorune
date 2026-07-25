Status: SSOT mirror
Date: 2026-07-25
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
- current semantic authority:
  `docs/reference/language/function-exit-and-entry-result.md`
- priority: follow the exact active task. Do not reopen the superseded App
  any-statement-tail S0 as canonical work.
- closed row: RAW-VM-REFERENCE-SUPPORT0-S0 — typed profile handoff, bounded
  diagnostics, supported opt-in surface, and renamed conformance proof
- active decision: NORMAL-ENTRY-CUTOVER-D1-PARK-AND-FORGE-NORMAL-FILE-VM0 —
  immediate cutover is parked; NormalFileNoImportVmReferenceV1 is the one
  bounded family to forge
- next executable row: NORMAL-FILE-VM0-FAMILY-D0 (read-only evidence); D2 must
  accept before any normal production caller is added
- closed immediately before this frontier: passive Canonical/NarrowV1/
  VM-reference profile, explicit early `--backend raw-vm-reference` canary,
  mandatory feature-enabled/disabled parity family, and the D0 decision that
  found no safe bounded normal caller
- parked: normal/default cutover, general VM/LLVM, JSON, executor, selfhost,
  fastmem, and CUT0

## Rule

This file is only a mirror. Implementation details, acceptance, landed history,
and parked tasks belong in the active card, the workstream SSOT, phase cards,
or git history.
