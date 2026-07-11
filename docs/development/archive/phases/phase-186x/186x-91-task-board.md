# 186x-91: same-root phi local field pruning task board

## Landed

- [x] Extend local-root resolution so DCE recognizes same-root multi-input phi carriers under the current local-box anchor set.
- [x] Allow dead local `FieldGet` pruning through that phi carrier when the read result is otherwise unused.
- [x] Allow dead local `FieldSet` pruning through that phi carrier when the local root remains unobserved.
- [x] Keep mixed-root phi merges, generic `Store`, generic `Load`, `Debug`, and terminators outside this cut.
- [x] Update phase docs and current pointers to reflect the new cross-block effect-sensitive slice.

## Notes

- This is still a local-field DCE slice, not a generic phi-merge relaxation.
- Broader effect-sensitive / partial DCE remains backlog after this cut.
