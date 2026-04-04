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
- prefer tasks where tree/layout changes are the primary artifact

## Starting Read

- stage1/selfhost mainline owner cleanup is landed
- stable green proof bundle is preserved
- focused `emit_mir_mainline` parse red at `lang/src/compiler/build/build_box.hako:4` remains tracked
- current split opportunities are:
  - `tools/selfhost/**` still mixes mainline/proof/compat/lib surfaces at the top level
  - `lang/src/runner/**` already has partial structure, but authority vs compat reading is still file-name heavy
  - `src/runner/modes/**` still mixes product / residual keep / reference lanes in one bucket

## Candidate Ranking

1. `phase-67x selfhost folder split`
   - split `tools/selfhost/**` into `mainline/`, `proof/`, `compat/`, and `lib/`
   - keep top-level `tools/selfhost/` facade-only
2. `phase-68x runner authority/compat folder recut`
   - make `.hako` runner authority/compat/facade/entry reading visible in tree shape
3. `phase-69x rust runner product/keep/reference recut`
   - separate `src/runner` residual keep/reference surfaces after shell/.hako split settles
4. `phase-70x caller-zero archive sweep`
   - archive only after folder/caller boundaries are easier to read

## Corridor

- `67x` selfhost folder split
- `68x` `.hako` runner authority/compat/facade recut
- `69x` rust runner product/keep/reference recut
- `70x` caller-zero archive sweep

## Big Tasks

1. `66xA1` successor lane inventory lock
2. `66xA2` candidate lane ranking
3. `66xB1` successor lane decision
4. `66xB2` folder-separation corridor lock
5. `66xD1` proof / closeout
