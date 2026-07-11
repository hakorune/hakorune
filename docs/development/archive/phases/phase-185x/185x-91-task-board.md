# 185x-91: dead local field-set write pruning task board

## Landed

- [x] Extend DCE to remove dead `FieldSet` writes on definitely non-escaping local boxes when otherwise unobserved.
- [x] Keep `Store`, `Load`, `Debug`, and terminators outside the cut.
- [x] Keep live `FieldGet` observers intact so writes that are still observed do not disappear.
- [x] Update the phase/task docs and current pointers to reflect the new DCE slice.

## Notes

- This slice stays inside the broader effect-sensitive DCE lane.
- Broader effect-sensitive / partial DCE remains backlog after this cut.
