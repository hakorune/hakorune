# Phase 293x Card Archive Manifest

Status: Active
Date: 2026-05-15
Scope: safe-move manifest for phase-293x numbered card archive.
Related:
- docs/development/current/main/design/current-docs-archive-policy-ssot.md
- docs/development/current/main/phases/phase-293x/archive/cards/README.md
- docs/development/current/main/phases/phase-293x/293x-409-DOCS-SLIM-002-ARCHIVE-MANIFEST-PREP.md

## Decision

Do not physically move phase cards in `DOCS-SLIM-002`.

This manifest makes the future archive move mechanical by fixing:

- protected current paths
- bucket counts
- direct-reference risk
- no-move guard for this row

## Snapshot

After `293x-409` lands, phase-293x has 409 numbered root cards, excluding
taskboards:

```text
293x-000-099: 99
293x-100-199: 100
293x-200-299: 100
293x-300-399: 100
293x-400-499: 10
```

The root also contains the phase README and taskboards. They are not card
archive candidates.

## Protected Current Paths

Always keep these live at their current paths:

- `CURRENT_STATE.phase_status`
- `CURRENT_STATE.latest_card_path`
- active taskboard path from `CURRENT_STATE.taskboard`
- active phase README from `CURRENT_STATE.active_phase`
- any card still referenced directly by active guards

## Reference Risk

Before this row, `tools/checks` contained more than 200 direct references to
phase-293x card paths, and current docs plus check scripts referenced more than
200 unique phase-293x card filenames.

That means bulk moving the old cards first would trade document bloat for
broken guard paths. The next cleanup should decouple guard references or leave
forwarding stubs before physical moves.

## Candidate Rule

Archive candidates are numbered root cards that are:

- landed, complete, historical, or superseded
- not protected current paths
- not directly required by a current guard without a stub

## DOCS-SLIM-002 Guard Contract

`tools/checks/docs_slim_002_archive_manifest_guard.sh` fixes this row:

- archive entry files exist
- root card bucket counts match the snapshot
- no phase cards have moved under `archive/cards/` yet
- the direct-reference risk remains visible
- `CURRENT_STATE.toml` points at `293x-409`
