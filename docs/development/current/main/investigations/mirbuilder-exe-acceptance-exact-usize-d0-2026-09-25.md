# MIRBUILDER-EXE-ACCEPTANCE-EXACT-USIZE-D0

Status: closed__2026-09-25__decision-accepted
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

## Decision (accepted 2026-09-25)

Worker `f021ceed` audit + main-investigator probes:

```text
Decision: admit `usize` as a parameter-scoped exact arm on
          ExactTrivialParameterAbiV1 — NOT on the shared
          ExactTrivialScalarAbiV1 (a scalar arm would silently widen
          the return ABI, completion seed, callable index, literal
          proofs, and generic metadata classifiers; two of them
          mislabel usize as i64).
Source authority + canonical issuer:
          `usize-semantic-foundation-ssot.md` — usize is a distinct
          exact source type physically executed on the i64 lane
          (MirType::Integer; MirType has no usize variant). Issuer:
          `callable_parameter_contract/issuer.rs:82-102` via
          `ExactTrivialParameterAbiV1::classify`.
Non-authority: `ExactTrivialScalarAbiV1` (stays i64-only),
          `ExactTrivialReturnAbiV1`, literal/expression proofs,
          `builder_metadata.rs` generic-path classifier — all
          untouched so `: usize` returns and usize literals keep
          their existing rejections.
Fail-fast boundary: uncalled usize decls gain dormant contract rows
          (physical_signature OrdinaryScalar lane, tolerant);
          every consumer gate `!= ExactTrivialParameterAbiV1::I64`
          already rejects usize at consumption (dynamic_admission,
          home_prefix_local_flow, coverage, callable_index
          ParameterTypeOutsideProfile, a-prime capability). usize
          CALLS still have no argument lane
          (canonical_direct_call.rs:111-117 is I64-only).
Smallest next slice:
          MIRBUILDER-EXE-ACCEPTANCE-EXACT-USIZE-S0 —
          (i) ExactTrivialParameterAbiV1 gains a param-scoped scalar
          kind {I64, Usize} replacing the shared-enum field;
          (ii) pin the two mislabel sites: formal_contract.rs:144-155
          (usize birth formal must NOT become ExactI64 ->
          ExplicitUnsupported) and instance_function_plan.rs:455-464
          (usize must not enter I64ParameterReturn family);
          (iii) contract tests + README.
Non-claims: no usize call admission, no usize return ABI, no MirType
          variant, no field-storage change (usize fields already
          routed via TypedObjectFieldStorage::USize -> u64 HII lane),
          no suite-green claim — both apps' method-call-dense bodies
          will still stop at the next named boundary on the selfhost
          emit route (documented Invoke terminal class).
```

## Q2/Q3 findings (worker audit, spot-checked)

- The all-declarations sweep is **deliberate**: the contract is a
  complete-batch product; `physical_signature.rs` requires one row
  per selected slot and `selected` ≈ every non-main catalog row.
  Reached-only narrowing is Blocked — no reachability authority
  exists and completeness would break.
- Neither app calls a `*_usize` function; the failure is dead
  imported decls (page_heap_box -> layout_box -> size_class_box).
  usize FIELDS in page_heap_box already route via
  `TypedObjectFieldStorage::USize` -> u64 HII lane.
- After the contract admits usize, both apps' expected next terminal
  on their `--mir` emit route is `unsupported terminator Invoke`
  (method/birth-dense bodies), or an earlier in-package named stop;
  suite stays red for them — this slice fixes a wrong rejection, not
  a green flip.

## Exit

- [x] Exact-usize semantics fixed: distinct exact type, i64 lane
      physically; parameter-scoped admission only.
- [x] Bounded S-card emitted: MIRBUILDER-EXE-ACCEPTANCE-EXACT-USIZE-S0.

## Resolution (S0 landed 2026-09-25)

MIRBUILDER-EXE-ACCEPTANCE-EXACT-USIZE-S0 landed: parameter-scoped
`ExactTrivialParameterScalarV1{I64,Usize}`; shared scalar/return ABI
untouched; all consumption sites verified exact-I64 gated. Suite
rerun: 4 pass / 7 fail — both usize-family entries moved past
`UnsupportedDeclaredType`: boxtorrent -> `main-import-view/
selected-header-missing`, mimalloc -> `ordinary-new/
birth-global-legacy-stopped` (earlier in-pipeline named stops than
the predicted Invoke terminal; consistent with the all-declarations
catalog ordering). Boundary movement recorded in S0 card; neither
app claimed green.
