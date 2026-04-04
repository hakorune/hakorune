---
Status: Active
Date: 2026-04-04
Scope: choose the next source lane after phase-65x landed.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-65x/65x-90-stage1-selfhost-mainline-hardening-ssot.md
---

# 66x-90 Next Source Lane Selection SSOT

## Intent

- pick the next source lane after `phase-65x`
- prefer the lane with the highest leverage on current source clarity
- keep known blockers explicit instead of smearing them across unrelated lanes

## Starting Read

- stage1/selfhost mainline owner cleanup is landed
- stable green proof bundle is preserved
- focused `emit_mir_mainline` parse red at `lang/src/compiler/build/build_box.hako:4` remains tracked

## Big Tasks

1. `66xA1` successor lane inventory lock
2. `66xA2` candidate lane ranking
3. `66xB1` successor lane decision
4. `66xD1` proof / closeout
