# Phase 183x Task Board

## Done

- [x] Define the pure no-dst call pruning contract.
- [x] Extend DCE to remove pure `Call { dst: None, .. }` instructions when otherwise unused.
- [x] Keep `Debug` and terminators outside the cut.
- [x] Add focused regression coverage.
- [x] Update current-task pointers and phase maps.

## Notes

- This slice stays inside the generic no-dst pure cleanup lane.
- Broader effect-sensitive / partial DCE remains backlog after this cut.
