# Phase 184x Task Board

## Done

- [x] Define the dead local field-get read pruning contract.
- [x] Extend DCE to remove dead `FieldGet` reads on definitely non-escaping local boxes when otherwise unused.
- [x] Keep `Load`, `Debug`, and terminators outside the cut.
- [x] Add focused regression coverage.
- [x] Update current-task pointers and phase maps.

## Notes

- This slice stays inside the broader effect-sensitive DCE lane.
- Broader effect-sensitive / partial DCE remains backlog after this cut.
