---
Status: Active
Date: 2026-04-04
Scope: archive or retire live aliases/docs/wrappers whose callers fell to zero after the folder-separated recuts.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-69x/README.md
  - docs/development/current/main/phases/phase-69x/69x-90-rust-runner-product-keep-reference-recut-ssot.md
---

# Phase 70x: Caller-Zero Archive Sweep

## Goal

- identify alias/docs/wrapper surfaces that no longer have live callers
- move only caller-zero historical surfaces into archive
- keep proof/reference/current routes out of archive

## Big Tasks

1. `70xA1` caller-zero inventory lock
2. `70xA2` archive-ready ranking
3. `70xB1` live alias/archive sweep
4. `70xC1` current pointer cleanup
5. `70xD1` proof / closeout

## Current Read

- `69x` has landed and the runner tree now reads `product / keep / reference`
- current front:
  - `70xB1 live alias/archive sweep`
- current intent:
  - only caller-zero live aliases/wrappers move
  - proof-only keep and reference routes stay live
  - archive should collect history, not current explicit keep
  - the first inventory pass found no archive-ready live wrappers yet
  - this sweep is expected to close as a no-op unless a doc-only alias drains to zero
