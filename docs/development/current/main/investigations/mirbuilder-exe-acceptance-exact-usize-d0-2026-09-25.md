# MIRBUILDER-EXE-ACCEPTANCE-EXACT-USIZE-D0

Status: open__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D1 (closed) +
  final-pipeline-ssot ordering item (4): "select an exact `usize`
  callable owner only when its issuer, consumer, caller, fail-fast
  terminal, and exclusive delete-set exist."
Owner: workstream row H / unified resume gate 1

## Slice

Design only. Decide whether an exact-`usize` declared parameter type
gets a sanctioned callable-contract kind, and if so name the issuer,
consumer, caller, fail-fast terminal, and exclusive delete-set. Emit
one bounded S-card or a ParkedSealed/NoSafeSlice disposition.

## The edge (from D1 audit)

- `boxtorrent_mini` and `mimalloc_lite` fail at
  `ParameterContract { UnsupportedDeclaredType { declaration: 6,
  parameter: 0 } }`, raised by
  `callable_parameter_contract/issuer.rs:82-102`, invoked over ALL
  batch declarations (`normal_callable_semantic_package/issuer.rs:483-485`)
  — including imported-but-uncalled catalog decls.
- `usize` parameters come from `lang/src/hako_alloc/memory/
  size_class_box.hako` (`size_to_bin_usize`, `good_size_usize`,
  `bin_size_usize`, `accepts_usize`) via the `page_heap_box` ->
  `layout_box` -> `size_class_box` freestatic batch;
  `page_heap_box.hako` also declares `usize` FIELDS.
- Admitted today: `None`->OpaqueHandle, `StringBox`->ExactText,
  `i64`->ExactTrivial, `MapBox`, `ArrayBox`->DeclaredHandle.
  `usize` fits none.
- Do NOT coerce `usize` to i64/OpaqueHandle without a source issuer,
  consumer, and production cutover (SSOT).

## Questions to answer

1. Does the language treat `usize` as a distinct exact type or an
   alias of i64? Check the language type SSOT and `lang/src/
   hako_alloc/memory/NUMERIC_FIELDS.md` policy before designing.
2. If exact: which module issues the `usize` contract kind — a new
   `ExactTrivialParameterAbiV1` scalar arm, or a separate kind? Name
   the consumer (physical signature/ABI emission) and the runtime
   ABI (usize is pointer-width; lifecycle C ABI is i64-shaped).
3. Scope narrowing alternative: should the contract iterate only
   *reached* declarations instead of the whole batch — is the
   all-declarations sweep deliberate (exact membership) or
   incidental? If incidental, is a bounded "reached-only" narrowing
   the honest fix instead of admitting usize?
4. usize fields in `page_heap_box` — do they hit a separate storage
   edge beyond parameters? Record whether fixing parameters alone
   flips the entries or merely moves the terminal.

## Boundary

- Includes: `callable_parameter_contract/*`, `exact_trivial_scalar_abi`,
  `size_class_box.hako`/`page_heap_box.hako` decls, language type SSOT,
  NUMERIC_FIELDS policy.
- Excludes: implementation; NamedArray; String-result ABI; loop facts;
  newbox/Invoke transport.

## Exit

- [ ] Exact-usize semantics fixed (distinct type vs alias) with
      authority.
- [ ] One accepted Decision + bounded S-card, or ParkedSealed with
      reopen trigger.
