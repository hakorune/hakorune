# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D2

Status: selected__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-EXACT-USIZE-S0 (landed)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  post-S0 terminal map on the exact-usize S0 card.

## Slice

Select ONE bounded failure class from the remaining 7 red entries of
`real-apps-exe-boundary` (post-usize-S0 receipt, 4 pass / 7 fail)
with a complete authority/caller/delete/evidence tuple, and emit one
bounded execution card. No implementation in this mode.

## Fresh class map (post-exact-usize-S0)

| candidate family | entries | known owner hints |
|---|---|---|
| `main-import-view/selected-header-missing` | json_stream_aggregator, boxtorrent_mini | qualified main-import ABI; json_stream needs non-i64 result — boxtorrent terminal newly reached here after usize admission; likely same owner edge |
| `ordinary-new/birth-global-legacy-stopped` | binary_trees, mimalloc_lite | ordinary-new claim authority for `Class.birth/N`; binary_trees also has return-position new + null-arg gaps (per D1 audit); mimalloc newly reached here after usize admission |
| `NamedArray(TextSourceMissing)` | allocator_stress | computed-handle vs admitted text-source; semantic boundary decision |
| `return_type_strategy` panic rc=101 | untyped_field_min | untyped/inferred storage — ParkedSealed per MIRBUILDER-UNTYPED-OBJECT-STORAGE-D0; sentinel only |
| `no_lowering_variant` + stale `opt` | newbox_min | env/toolchain debt (opt=LLVM14, opt-18 exists); not semantic work |

## Questions to answer

1. Which single class has a complete tuple (source authority +
   canonical issuer + fail-fast boundary + smallest slice +
   non-claims)?
2. For the `main-import-view` pair: do json_stream and boxtorrent
   share one owner edge (qualified-import selected-header/ABI), or
   does boxtorrent stop for a different reason under the same label?
3. For the `birth-global-legacy-stopped` pair: is mimalloc's
   `Class.birth/N` shape identical to binary_trees' (return-position
   new + null args), or a narrower subset that admits a bounded
   slice alone?
4. Do any of the moved terminals hide a *different* family behind
   the same named stop (masking)?

## Boundary

- Includes: class inventory re-verification, owner audit, tuple
  assembly, next S-card emission.
- Excludes: implementation, grammar/ABI changes, fallback.

## Exit

- [ ] One accepted Decision with complete tuple.
- [ ] One bounded S-card emitted; pointers synced.
