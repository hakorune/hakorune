# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D1

Status: closed__2026-09-25__NoSafeSlice
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

## Decision (accepted 2026-09-25): NoSafeSlice — open the untyped-storage authority D0

Worker `0b3a5cac` audit + main-investigator spot-checks:

```text
Decision: NoSafeSlice — no remaining class satisfies the full
          (authority, issuer, caller, boundary, bounded slice) tuple.
          Open a design card for the highest-weight family: untyped /
          inferred-storage Home-end and result-type authority
          (covers birth_param_min + untyped_field_min, and partially
          gates binary_trees downstream).
Source authority + canonical issuer:
          destruction/end authority = object_definition.rs:107-132 ->
          local_entry.rs:82-91 -> root_home.rs:167-184; field-read
          result type publication = fields/post_success.rs:84-92 ->
          return_type_strategy.rs:173-227. No co-seal issuer exists
          for inferred (non-i64-declared) storage.
Non-authority: issuer.rs:82-102 parameter contract (usize = BoxCount),
          named_array_requirement.rs:122-140, model.rs:306-336
          qualified ABI — each a separate design decision.
Fail-fast boundary: all 8 terminals are already named stops or a
          debug-assertions panic on the same edge; none are silent.
Smallest next slice:
          MIRBUILDER-EXE-ACCEPTANCE-UNTYPED-STORAGE-AUTHORITY-D0
          (design) — not an implementation slice.
Non-claims: does not select usize params, NamedArray, String-result
          ABI, loop Facts, or the newbox toolchain question.
```

Key audit results:

- Q1: `binary_trees` vs `birth_param_min` = **TwoOwners**. binary_trees
  lacks an ordinary-new claim for non-initializer `new` sites (Return
  position) plus a `null` argument kind
  (`ordinary_new_coseal_issue.rs`, `ordinary_new_arguments.rs:9-13`).
  birth_param lacks Home-end/destruction authority for inferred
  storage: `Page` fields via `init{}` have `declared_type_name ==
  None` -> `Destruction::Unavailable(FieldType)` -> Home not endable
  -> no `RootHomeExitEntry::Call` -> `root-call-entry-missing`.
- Q2: `untyped_field_min` panic is **pre-existing debt**, not
  S0-exposed — its `main` has zero method calls (route predicate never
  applied). The debug panic masks the release-path
  `unsupported terminator Invoke` terminal; same semantic edge.
  Verified `git log main_root.rs` — S0 is the only route change.
- Q3 spot-check: `opt` on PATH = LLVM 14.0.0; `opt-18` exists
  (`/usr/lib/llvm-18/bin/opt`). newbox_min = EnvironmentDebt confirmed.
- Q4: ranking — highest-weight family = untyped-storage authority
  (2 direct entries + binary_trees downstream edge). `usize`/NamedArray/
  String-ABI/loop-Facts each need their own D-card.

## Exit

- [x] Decision recorded: NoSafeSlice -> untyped-storage authority D0.
- [x] Next card emitted; pointers synced.
