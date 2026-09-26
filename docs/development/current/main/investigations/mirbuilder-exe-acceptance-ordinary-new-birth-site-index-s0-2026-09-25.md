# MIRBUILDER-EXE-ACCEPTANCE-ORDINARY-NEW-BIRTH-SITE-INDEX-S0

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D3
  (accepted — destination-less birth-recipe site index)
Owner: workstream row H / unified resume gate 1
Authority: D3 Decision. Canonical issuer stays
  `issue_ordinary_source_cohort_v1`; sole physical consumer stays
  `lower_ordinary_raw_new_with_port_v1` (it reads only
  `claim.constructor()`). `OrdinaryNewAdmissionClaimV1` schema is
  untouched.

## Slice

1. `src/mir/normal_callable_semantic_package/ordinary_new_coseal.rs`:
   add `birth_site_index: RefCell<BTreeMap<OwnedExprSiteV1,
   VerifiedOrdinaryNewBirthRecipeV1>>` on `OrdinaryNewClaimLedgerV1`
   plus a crate-visible `take_birth_site_recipe(site, class, arity)`
   that removes the entry (affine, matching `try_take`) and enforces
   class/arity match (`Mismatch`-style named error, mirroring
   `try_take`'s contract).
2. `src/mir/normal_callable_semantic_package/ordinary_new_coseal_issue
   .rs`: after the admission-candidate walk, build the index by
   enumerating `expression_source().constructions` for
   (a) every `batch.declarations()` row reachable via
   `with_lowering_input` and (b) every
   `instance_constructors.rows()` row via
   `batch.with_normal_program_source_loan(|loan|
   row.lowering_input(loan.program()))`. For each `new` site whose
   segments are NOT `[Body, Initializer]` (the claim lane owns
   those, including its `Unavailable` strictness):
   - `ordinary_box_coverage().row_for(class)` → `Err`/missing or
     builtin → skip (index admits only covered user boxes;
     downstream behavior unchanged);
   - `instance_constructors.birth_for(row, arity)` → `None` → skip
     (no verified birth → `use_lowered` stays the honest terminal);
   - apply the same birth checks as `OrdinaryNewCandidate::resolve`
     (box name + source arity match, `published_birth_key`
     namespace `BirthConstructor` with matching owner/arity,
     unit `birth_completion` over the row's own root,
     `OpaqueObservable` `birth_effect`, `BirthAbiHandoffV1::issue`);
     on ANY check failure → skip (the index is additive-only: it
     never mints a new error and never masks the existing
     `birth-global-legacy-stopped` terminal);
   - verified `Birth` recipe → insert; duplicate site →
     `OrdinaryNewCoSealIssueV1::DuplicateSite` (hard error).
3. `src/mir/builder/raw_ordinary_new_claim.rs`: new
   `RawOrdinaryNewClaimPortV1::try_take_ordinary_new_birth_recipe(
   class, argument_count)` default `Ok(None)`; implement on
   `RawInvocationChildPortV1` — build `OwnedExprSiteV1` from
   `callable_owner_v1` + `current_source_site_v1` (both missing →
   `Ok(None)`, same shape as `complete_ordinary_new_expression`)
   and call `take_birth_site_recipe`.
4. `src/mir/builder/normal_callable_semantic_loan_port/ordinary_new
   .rs` + `raw_structured_child_scope.rs`: delegate with
   `check_new_ledger_identity()`.
5. `src/mir/builder/new_expression.rs`: `PreparedRawNewExpressionV1`
   gains the taken recipe; `prepare_ordinary_claim_v1` consults it
   ONLY when `claim.is_none()` (claim precedence preserved);
   `Ordinary` route passes `constructor:
   Option<OrdinaryNewConstructorDispositionV1>` into
   `lower_ordinary_raw_new_with_port_v1` — that function's `claim`
   parameter is replaced by the already-extracted disposition
   (verified: it uses nothing else from the claim).

## Pins (required before close)

- Positive A (field-assign): `me.f = new Inner(n)` inside a
  `birth` body compiles past `birth-global-legacy-stopped`; emitted
  MIR carries `Callee::BirthConstructor` for `Inner.birth/N`
  (MIR-JSON or focused assertion).
- Positive B (return-position): `return new Foo(7)` compiles past
  the same terminal with the typed edge.
- Precedence: a `[Body, Initializer]` site with an admission claim
  still takes the claim path (existing pins stay green);
  init-position covered-class sites without claims keep the
  `Unavailable` strictness (index never consulted there).
- Negative: builtin class (`MapBox`) and no-birth class
  (`BinaryTreeBuilder`-shaped) at non-initializer positions get no
  index entry and keep current behavior (`use_lowered` → bare
  `NewBox` or the unchanged named terminal).
- Negative: arity/class mismatch on take → named error, not a
  silent `None`.

## Guard

- Extend `tools/checks/mirbuilder_qualified_route_scope_guard.sh`
  (or sibling) to pin: `birth_site_index`, the new port method
  name, claim-precedence ordering in `prepare_ordinary_claim_v1`,
  and the non-`[Body, Initializer]` filter in the issuer walk.

## Fail-fast boundary

- `DuplicateSite` on index insert.
- take-side class/arity `Mismatch`.
- Index absence is never an error — it restores today's
  `birth-global-legacy-stopped` verdict exactly.

## Non-claims

- No lifecycle/fault/reclaim tracking for unhomed objects — the
  emitted shape is the legacy `NewBox` + `BirthConstructor` call
  (identical to what R6-S1 intended to preserve via a typed edge).
- Downstream terminals stay honest: `FieldContractUnsupported`
  (`init{}` boxes), qualified-call / non-trivial argument lowering,
  `loop-cond-item-unsupported`, NamedArray, toolchain
  `no_lowering_variant`, inference panic.
- No EXE-green claim; this moves the three `birth-global` entries
  to their next named terminals at best.
- `OrdinaryNewAdmissionClaimV1` destination/declaration schema
  untouched; no claim issued for non-initializer sites; no legacy
  `LegacyCallV0` carrier restored.

## Landed evidence

- Edits landed exactly per the slice. One extra wiring edit beyond
  the card text was required and is recorded here: constructor-body
  lowering (`with_constructor_semantic_scope`) previously did not
  install `ordinary_new_claim_ledger` on the inner port, so the
  index could never be consulted inside `birth` bodies. The scope
  now installs the package ledger for the duration of constructor
  lowering — same scoped-install pattern as `callable_ledger`.
- Pins green (3 new, `ordinary_new_coseal_tests.rs`):
  `birth_site_index_covers_field_assign_and_return_position_sites`
  (index holds exactly the field-assign + return-position sites,
  disjoint from claim sites),
  `birth_site_index_skips_builtin_and_missing_birth_classes`
  (MapBox + no-birth class produce no entries),
  `birth_site_take_enforces_class_arity_and_is_affine`
  (class/arity mismatch → `Mismatch`, affine take, absence is
  `Ok(None)`).
- Compile probes (MIR JSON): `me.inner = new Inner(7)` inside
  `Outer.birth` emits `newbox` + `Callee::BirthConstructor
  {Inner.birth/1}`; `return new Foo(7)` emits the same typed edge
  and `ret`; stored-field default `page: Page = new Page(3)`
  (desugared prologue) likewise; `new Foo(null)` argument lowers
  via the raw path. `birth-global-legacy-stopped` is not reached.
- Guard: `mirbuilder-qualified-route-scope` extended with
  index/port/precedence/filter pins + test-name pins + line limits.
- Fresh `real-apps-exe-boundary` receipt (debug emit + release
  backend): 4 pass / 7 fail — counts unchanged. The three
  `birth-global-legacy-stopped` entries all advanced to
  `[freeze:contract][callable-loop/facts-absent]` (loop-facts
  family). Unchanged terminals: json_stream `loop-cond-item-
  unsupported`, allocator_stress `NamedArray(TextSourceMissing)`,
  typed_object_newbox_min backend `no_lowering_variant`,
  typed_object_untyped_field_min `return_type_strategy` panic.
  `string_substring_in_range` stops at backend `mir_call_no_route`
  for `substring` — pre-existing recipe gap; the fixture contains
  no `new` sites, so this slice is provably non-causal.
- Focused regression: `cargo test --lib ordinary_new` 59 pass;
  `normal_callable` sweep 482 pass / 10 fail — all 10 recorded in
  `cargo_lib_red_baseline.failures.txt` (no new red).

## Exit

- [x] Edits landed; pins green (positive A/B + precedence +
      negatives).
- [x] Guard extended; row guard + pointer guard pass.
- [x] Fresh `real-apps-exe-boundary` receipt recorded with honest
      terminals.
- [x] CURRENT_STATE + workstream row H synced; committed/pushed.
