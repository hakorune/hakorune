# 186x-90: same-root phi local field pruning SSOT

Status: SSOT
Date: 2026-04-12
Scope: extend the landed local field read/write DCE slice across same-root multi-input phi carriers, while keeping generic `Store` / `Load`, `Debug`, terminators, and mixed-root phi merges outside this cut.

## Goal

- keep canonical MIR and generic `phi_query` as the owner of same-base reasoning
- widen DCE across cross-block local carriers without inventing a new local-memory vocabulary
- keep the effect-sensitive cut narrow: same-root local phi only

## Decision

- DCE local-root resolution may use `phi_query` to recognize a multi-input phi as `SameBase(root)` under the current local-box anchor set
- such a phi stays non-escaping for this local-field DCE slice
- dead `FieldGet { base, .. }` may disappear when `base` resolves to that same-root definitely non-escaping local box and the read result is otherwise unused
- dead `FieldSet { base, .. }` may disappear when `base` resolves to that same-root definitely non-escaping local box and the root has no reachable `FieldGet` observers
- mixed-root phi merges stay outside this cut
- generic `Store`, generic `Load`, `Debug`, and control-flow terminators stay outside this cut

## Acceptance

- dead local field reads/writes disappear through same-root multi-input phi carriers
- the dead phi and dead copy chain feeding that carrier disappear too when nothing else keeps them live
- mixed-root phi merges stay intact
- the existing local-box read/write slices keep their previous behavior
- `tools/checks/dev_gate.sh quick` stays green

## Exit

- same-root phi local field pruning is landed
- the remaining DCE backlog is broader effect-sensitive / partial widening after the local field carrier cut
