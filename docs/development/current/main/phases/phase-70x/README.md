---
Status: Landed
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
## Result

- `70xA1` landed: caller-zero inventory lock
- `70xA2` landed: archive-ready ranking
- `70xB1` landed: live alias/archive sweep
- `70xC1` landed: current pointer cleanup
- `70xD1` landed: proof / closeout
- outcome:
  - the first caller-zero sweep closed as a no-op
  - no live wrapper or alias was proven archive-ready
  - the next move is `phase-71x next source lane selection`
