# 187x-91: overwritten local field-set pruning task board

## Landed

- [x] Detect same-block overwritten local `FieldSet` instructions on definitely non-escaping local roots.
- [x] Clear overwrite witnesses on reachable `FieldGet` observers for the same field.
- [x] Clear overwrite witnesses on reachable escape uses for the same local root.
- [x] Keep generic `Store`, generic `Load`, `Debug`, and terminators outside this cut.
- [x] Update phase docs and current pointers to reflect the new local write-overwrite slice.

## Notes

- This is still a local-root DCE slice, not generic memory SSA.
- Broader effect-sensitive / partial DCE remains backlog after the local overwritten-write cut.
