# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D1

Status: open__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D0 (accepted; S0 landed)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  post-S0 terminal map on the D0 card.

## Slice

Select ONE bounded failure class from the remaining 8 red entries of
`real-apps-exe-boundary` (post-S0 receipt, 3 pass / 8 fail) with a
complete authority/caller/delete/evidence tuple, and emit one bounded
execution card. No implementation in this mode.

## Fresh class map (post-S0)

| candidate family | entries | known owner hints |
|---|---|---|
| ordinary-new lane blockers | birth_param_min (`local-commit/root-call-entry-missing`), binary_trees (`birth-global-legacy-stopped`) | ordinary-new claim authority / local-commit contract; highest weight — 2 entries |
| phase84-5 untyped-field panic | untyped_field_min (`ValueId(15)`) | return_type_strategy + untyped storage; reconcile vs parked Invoke/untyped-storage classifications |
| `ParameterContract UnsupportedDeclaredType` | boxtorrent_mini, mimalloc_lite | `usize` in cataloged `selfhost.hako_alloc` decls; multi-owner BoxCount |
| `NamedArray(TextSourceMissing)` | allocator_stress | semantic boundary decision |
| `main-import-view/selected-header-missing` | json_stream_aggregator | non-i64 qualified method result ABI widening |
| `no_lowering_variant` + stale opt | newbox_min | env/toolchain — verify with `opt-18` before treating as semantic work |

## Questions to answer

1. Which single class has a complete tuple (source authority +
   canonical issuer + fail-fast boundary + smallest slice + non-claims)?
2. For the ordinary-new pair: are `root-call-entry-missing` and
   `birth-global-legacy-stopped` one shared owner edge (ordinary-new
   claim for `Class.birth/N` on the exe lane) or two?
3. For untyped_field_min: is the panic a *moved* boundary from S0
   (lexical receiver route change) or pre-existing? `ValueId(15)` must
   be traced before classifying.

## Boundary

- Includes: class inventory re-verification, owner audit, tuple
  assembly, next S-card emission.
- Excludes: implementation, grammar/ABI changes, fallback.

## Exit

- [ ] One accepted Decision with complete tuple.
- [ ] One bounded S-card emitted; pointers synced.
