# Phase 293x Archive

Status: Active
Date: 2026-05-15
Scope: phase-293x historical execution archive entry.
Related:
- docs/development/current/main/design/current-docs-archive-policy-ssot.md
- docs/development/current/main/phases/phase-293x/archive/cards/README.md
- docs/development/current/main/phases/phase-293x/archive/cards/phase-293x-card-archive-manifest.md

## Purpose

This directory holds historical phase-293x execution docs after they are no
longer active restart surfaces.

The current owner remains:

```text
docs/development/current/main/CURRENT_STATE.toml
```

Do not use this directory as the current lane pointer.

## Contents

- `cards/README.md`: card archive protocol and bucket layout.
- `cards/phase-293x-card-archive-manifest.md`: safe-move manifest and current
  inventory snapshot.

## Current Stop Line

`DOCS-SLIM-002` creates the archive entry and manifest only. It does not move
phase cards yet. Physical moves start in a later row after guard/card direct
references are decoupled or forwarding stubs are planned.
