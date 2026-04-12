# 188x-91: cross-block overwritten local field-set pruning task board

## Landed

- [x] Propagate overwritten local field-set witnesses across one reachable linear edge.
- [x] Exclude loop backedges with dominator-based guard.
- [x] Keep successor-block reads and root escape uses as witness clear points.
- [x] Keep merges, generic `Store`, generic `Load`, `Debug`, and terminators outside this cut.
- [x] Update phase docs and current pointers for the linear-edge partial DCE slice.

## Notes

- This is still local-root DCE, not generic memory SSA.
- Broader effect-sensitive / partial DCE remains backlog after the linear-edge overwrite cut.
