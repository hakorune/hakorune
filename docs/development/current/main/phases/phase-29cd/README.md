---
Status: Active
Scope: compiler cleanliness aftercare (archive replay lane / live compat contract / dust)
Related:
- CURRENT_TASK.md
- docs/development/current/main/design/joinir-smoke-legacy-stem-retirement-ssot.md
- docs/development/current/main/design/joinir-legacy-fixture-pin-inventory-ssot.md
- docs/development/current/main/design/joinir-frontend-legacy-fixture-key-retirement-ssot.md
- docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md
---

# Phase 29cd: compiler cleanliness aftercare

## Goal

current compiler/docs lane を保ったまま、残っている compat/historical residue を
`archive replay lane` / `live compat contract lane` / `dust lane` に分離して閉じる。

ここでいう aftercare は「本線設計を作り直す」ことではなく、すでに semantic-first に
揃った current lane を汚さずに、残件の keep/retire authority を固定する作業を指す。

## Non-goals

- route/recipe/facts の current architecture を再設計する
- archived replay stem を caller inventory 無しで eager delete する
- `docs/private` nested repo を top-level cleanup と混ぜる

## Workstreams

1. archive replay lane
   - archive-backed current wrapper 6本の `fixed keep / retire when` を固定する
   - archived replay basename は manual/archive lane にのみ残す
2. live compat contract lane
   - `SMOKES_SELFHOST_FILTER`
   - by-name fixture key
   - semantic fixture alias
   の責務を分離し、exact historical token は inventory-only に閉じる
3. dust lane
   - stale comment
   - orphan helper
   - dead code / dead note
   の low-risk cleanup を行う

## Exit criteria

- active phase README の `PatternN / patternN_` residue は `0 hit` のまま維持する
- current gate / selfhost how-to は semantic wrapper / semantic fixture alias / semantic route substring を先頭に置く
- archive-backed six-route wrappers の keep authority が SSOT で一意に読める
- `cargo check --tests` と fast gate / probe が緑を維持する

## Gate

- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
- `cargo check --tests`

## Instructions

- P0: `docs/development/current/main/phases/phase-29cd/P0-AFTERCARE-CLOSEOUT-INSTRUCTIONS.md`
