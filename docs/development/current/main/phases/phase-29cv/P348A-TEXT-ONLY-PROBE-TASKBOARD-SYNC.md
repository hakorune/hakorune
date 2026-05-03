---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: sync delete-last taskboard after P347A text-only probe archive
Related:
  - docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md
  - docs/development/current/main/phases/phase-29cv/P347A-PHASE29CH-TEXT-ONLY-PROBE-ARCHIVE.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P348A: Text-Only Probe Taskboard Sync

## Intent

Keep the phase-29cv delete-last taskboard aligned with the P347A archive.

P347A moved `phase29ch_program_json_text_only_probe.sh` out of active
`tools/dev/`, but the active blocker board still listed only the two earlier
archived phase29ch diagnostics probes. This card updates that taskboard so the
current keeper map does not drift.

## Boundary

Documentation-only sync.

No tool movement, route behavior, or keeper policy changes.

## Implementation

- Added the archived text-only probe to the Stage1 contract keeper bucket in
  `P33-DELETE-LAST-BLOCKERS-ONLY.md`.
- Updated `CURRENT_STATE.toml` latest-card pointer.

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
