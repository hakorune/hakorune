# Phase 81x SSOT

## Intent

`81x` reruns caller-zero/archive facts after `67x`-`80x` folder and pointer cleanup settled.

## Fixed Facts

- `phase-80x` is landed.
- current docs are thin enough again.
- archive moves must stay limited to true caller-zero surfaces.
- proof-only / compat / reference lanes are not archive candidates by default.

## Acceptance

1. caller inventory is rerun against the current tree
2. `keep-now` vs `archive-ready` is source-backed
3. the lane closes either with a real archive move or an explicit no-op proof
