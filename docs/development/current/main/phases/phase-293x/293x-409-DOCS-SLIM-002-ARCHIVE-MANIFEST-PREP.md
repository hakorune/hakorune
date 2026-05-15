# 293x-409 DOCS-SLIM-002 Archive Manifest Prep

Status: landed
Date: 2026-05-15

## Decision

Prepare the phase-293x card archive without moving old cards yet.

`DOCS-SLIM-001` added the policy and stopped current mirror regrowth. This row
adds the archive entry, bucket protocol, safe-move manifest, and guard so the
next cleanup can move cards without guessing which paths are protected.

## TODO

- [x] Add `phase-293x/archive/` entry docs.
- [x] Add card archive bucket protocol.
- [x] Add a phase-293x archive manifest with bucket counts and protected-path
  rules.
- [x] Add a guard that verifies the manifest and blocks accidental physical
  moves in this row.
- [x] Decouple `DOCS-SLIM-001` guard from a stale latest-card pin.
- [x] Update current pointers and check-script index.

## Scope

- Documentation structure only.
- Archive manifest only.
- Guardrail only.

## Stop Lines

- Do not move numbered cards in this row.
- Do not add forwarding stubs in this row.
- Do not change active blocker `MIMAP-022A`.
- Do not rewrite old guards in this row except for the new manifest guard.
- Do not turn taskboards back into landed-history ledgers.

## Required Evidence

```text
bash tools/checks/docs_slim_002_archive_manifest_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Added `phase-293x/archive/README.md`.
- Added `phase-293x/archive/cards/README.md`.
- Added `phase-293x/archive/cards/phase-293x-card-archive-manifest.md`.
- Added `tools/checks/docs_slim_002_archive_manifest_guard.sh`.
- Removed the `DOCS-SLIM-001` guard's latest-card pin so later docs-slim rows
  can advance `CURRENT_STATE.latest_card` without breaking the older guard.
- Updated the docs layout / archive policy pointers and current state latest
  card fields.

## Evidence

```text
bash tools/checks/docs_slim_002_archive_manifest_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
