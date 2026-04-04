# Phase 82x SSOT

## Intent

`82x` selects the next source lane after `81x` confirmed there are still no true archive-ready wrappers.

## Facts to Keep Stable

- `81x` landed with a no-op archive sweep.
- top-level selfhost wrappers remain explicit façade or proof surfaces, not archive payload.
- top-level `.hako` wrappers remain compatibility surfaces, not archive payload.
- `src/runner/modes/mod.rs` remains a compatibility re-export surface.
- current pointers are thin enough again after `80x`.

## Candidate Ranking

1. `phase-83x selfhost top-level facade pressure thinning continuation`
   - target: top-level shell façades that still carry visible surface pressure despite the folder split
2. `phase-84x runner wrapper/source contract thinning`
   - target: remaining top-level `.hako` wrapper pressure and stub/snapshot coupling
3. `phase-85x phase index / current mirror hygiene`
   - target: stale registry surfaces such as `phases/README.md`

## Acceptance

1. the next lane is selected once
2. the selected lane is ranked against at least two alternatives
3. the closeout hands off cleanly to the chosen successor
