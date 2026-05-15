# Phase 293x Card Archive

Status: Active
Date: 2026-05-15
Scope: archive protocol for phase-293x numbered cards.
Related:
- docs/development/current/main/design/current-docs-archive-policy-ssot.md
- docs/development/current/main/phases/phase-293x/archive/cards/phase-293x-card-archive-manifest.md

## Bucket Layout

Use range buckets so archived cards stay scannable:

```text
293x-000-099/
293x-100-199/
293x-200-299/
293x-300-399/
```

Future buckets can be added as the phase grows.

## Move Protocol

Move a numbered card only when all are true:

- it is not `CURRENT_STATE.phase_status`
- it is not `CURRENT_STATE.latest_card_path`
- it is not the active row for the current taskboard
- no current guard depends on its old path, or a forwarding stub is left behind
- the archive ledger or manifest keeps the card discoverable

## Stub Protocol

If a current doc, guard, or script still references the old path, keep a short
stub at the old path:

```text
# Moved

Status: Historical
Moved to: docs/development/current/main/phases/phase-293x/archive/cards/<bucket>/<file>
```

Do not leave both full copies live.
