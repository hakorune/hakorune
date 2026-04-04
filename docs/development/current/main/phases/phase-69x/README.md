---
Status: Active
Date: 2026-04-04
Scope: recut `src/runner` so product / keep / reference reading is visible in the tree.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-68x/README.md
  - docs/development/current/main/phases/phase-68x/68x-90-hako-runner-authority-compat-facade-recut-ssot.md
---

# Phase 69x: Rust Runner Product/Keep/Reference Recut

## Goal

- turn the current rust runner layout into explicit folder lanes
- make `product`, `keep`, and `reference` reading obvious from paths
- keep the phase tree-first like `67x` and `68x`

## Big Tasks

1. `69xA1` runner folder inventory lock
2. `69xA2` target layout ranking
3. `69xB1` product/reference split
4. `69xB2` keep split
5. `69xC1` alias/module cleanup
6. `69xD1` proof / closeout

## Current Read

- `68x` has landed and clarified `.hako` runner authority / compat / facade / entry reading
- current front:
  - `69xA1 runner folder inventory lock`
- current intent:
  - `src/runner` should stop relying on comments alone for product vs keep vs reference reading
  - `vm-hako` should read as reference, not as rust-vm residue
  - explicit rust-vm keep should stay explicit while imports and module paths are recut safely
