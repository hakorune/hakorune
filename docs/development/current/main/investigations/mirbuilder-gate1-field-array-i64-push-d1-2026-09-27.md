# MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-D1

Status: closed__2026-09-27__decision-accepted
Date: 2026-09-27
Parent: MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-S0 (landed
  `26216df32b`; this card is the exact prerequisite discovered inside
  that series — per D22, an exact prerequisite takes precedence over
  the queued caller-switch row).
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-exe-acceptance-suite-red-disposition-d21-2026-09-27.md
  §MIRBUILDER-GATE1-DEPENDENCY-PLAN-D22

## Slice

Design only. Settle how the sole construction-store owner
(`issue_construction_plan` -> `ConstructionState` ->
`Invoke FieldSet`) admits `me.<field> = new <builtin>` provider
stores so that the field-residence provider's physical home passes
the artifact lane for `HakoAllocPage`'s production field forms.

This was design work: one read-only worker (`2ccbe061`) produced the
end-to-end construction-plan census; the primary verified the
load-bearing contracts directly (`instance_construction.rs`,
`normal_callable_construction_state.rs`, `statement_surface`,
`instance_constructor_semantic.rs`, README contract paragraph).
No code, fixtures, fallback, production switch, or new semantic
receipts were issued.

## Finite source tuple

`lang/src/hako_alloc/memory/page_heap_box.hako:39-42` (production
provider shape):

```hako
box HakoAllocPage {
    free_stack: ArrayBox = new ArrayBox()        // stored-field-initializer
    block_used: ArrayBox = new ArrayBox()
    use_counts: ArrayBox = new ArrayBox()
    requested_sizes: ArrayBox = new ArrayBox()
    birth(page_id, block_size, capacity) { ... } // generated stores are
                                                 // prepended by normalization
}
```

`StoredFieldInitializer` normalization generates `me.<field> = <init>`
stores prepended to every `birth` body (`property_emit.rs:331-357`),
so the provider stores the plan sees are real `FieldWrite` +
`construction_source` rows — the same rows S0's
`resolve_birth_provider` already proves.

## Observed terminal (post-S0)

- Document lane (`apps/boxtorrent-mini --emit-mir-json`): the
  `seedBlocks` `SourceCallOutsideSelectedFamily` is closed; the next
  terminal is `[freeze:contract][static-result-ingress/foreign-lineage]`
  on `Main.main` — a different named family, outside this slice.
- Artifact lane: `[freeze:contract][construction-store/
  artifact-source-unavailable]` — `issue_construction_plan` marks the
  enclosing box `RetainedUnavailable` because (a)
  `has_stored_field_initializer` -> `FieldContractUnsupported`
  (instance_construction.rs:126-128), (b) every `field_decl` must be
  `i64` (:162-166), and (c) `new` RHS -> `BodyCoverageUnsupported`
  (:282-309). The provider's physical home is unreachable on the
  artifact lane for ANY box carrying non-i64 fields today.

## Verified census (worker `2ccbe061` + primary)

- `ConstructionStoreRhsV1` = `LiteralI64 | Parameter{site,binding}`
  only; every birth statement must be a Plain `me.<field>` assignment
  to an AST `fields` name; `init {}` names do not populate `fields`.
- Physical split today: plan stores emit canonical
  `InvokeOperation::FieldSet` under one shared fault frame justified
  by "every initialized field Trivial — no field release owed"
  (normal_callable_construction_state.rs:293-297). Non-plan stores
  emit plain name-keyed `MirInstruction::FieldSet` via
  `fields/assignment.rs`. `RetainedUnavailable` tolerates everything
  until `validate_artifact_after_compiler_finishing` hard-rejects it.
- The `new` value already has its owner:
  `lower_ordinary_raw_new_expression_with_port_v1` +
  `RawOrdinaryNewClaimPortV1` produce `MirInstruction::NewBox` for
  builtin classes (`OrdinaryNewCandidate::resolve` returns `Ok(None)`
  for `is_builtin_box`), and S0's `named_array_field_provider_recording`
  hooks the same route. `take/emit_construction_store` at
  `statement_surface.rs:456-461` is the only point where the port,
  the exact statement site, and the taken store coexist.
- `field_demands` is issued all-`Trivial` and asserted all-`Trivial`
  at install (`source-or-cleanup-contract`); `callable_home_demands`
  maps handle classes to `HomeDemandV1::Handle` already.
- `has_stored_field_initializer` is a sealed parser trigger recorded
  at the field-member site (constructor_source.rs:241-280). Generated
  stores are `ASTNode::assign_me_field(field, expr)` prepended in
  declaration order; the generated `new` keeps its resolver
  `construction_source` row — proven inside S0's provider join.
- `Destruction` already reports `FieldType`-unavailable for non-i64
  field types (object_definition.rs); `unowned-birth-call` rejects
  `Callee::BirthConstructor` inside a Selected birth; `emission-count`
  counts `Invoke`s only — builtin `new` emits a `NewBox` instruction,
  so both validators stay correct untouched.
- No `HomeRelease` exists for builtin ArrayBox allocations anywhere
  in this lane; `ReclaimUnpublished` is outer-storage-only.
- Positive fixture today: `apps/typed-object-birth-min` — i64-typed
  `field_decls` + parameter stores only; every eligible shape is
  scalar.

## Decision (accepted 2026-09-27)

```text
Decision: admit provider construction stores inside the single
  construction-store owner — do not fork a second store lane and do
  not change ordinary-new eligibility. `me.<field> = new <builtin>`
  becomes a coverage-only provider arm of ConstructionStoreRhsV1;
  the existing ordinary-new port produces the value; the plan's sole
  Invoke FieldSet emission wraps it.
Source authority + canonical issuer: `issue_construction_plan` in
  the same constructor loan, consuming resolver `construction_source`
  at the store's `value_site` (bare `new` of a builtin class — zero
  args, zero field initializers, class has no user Birth row — the
  same predicate S0's resolve_birth_provider seals). Declared field
  type must equal the constructed class name; an untyped field_decl
  is admitted only when the provider class supplies it.
Non-authority: MIR value types, name/spelling checks, and any
  synthesized provider identity. The positional prepended shape of
  generated stores is observed but not authority — every store is
  checked by the same arm rules whether generated or handwritten.
Fail-fast boundary: `new <UserBox>` RHS, mismatched declared type,
  non-builtin provider class, arbitrary initializer expressions,
  duplicate/missing stores — all stay typed ConstructionUnavailableV1
  rejections. `has_stored_field_initializer` stops being an
  unconditional reject and becomes informational: stores are checked
  by arm rules, so an excluded default expression still rejects
  through its RHS arm.
Smallest next slice: MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-S1 —
  provider RHS arm + field-type/demand reissue + statement-surface
  port routing + canonical FieldSet emission + focused positive
  (production decl-init shape on the artifact lane) + negatives +
  the two existing pins whose `FieldContractUnsupported` expectation
  changes under the widened contract.
Non-claims: no `init {}`-field admission (fixture-only form, not the
  production shape); no `new <UserBox>` provider (unowned-birth-call
  stays the correct guard); no release-suffix work on the shared
  fault landing; no ordinary-new eligibility change (the caller-side
  `new HakoAllocPage` admission remains the D18-successor row 6
  scope); no caller switch, retirement, Gate-1 green, or
  static-result-ingress family work.
```

## Settled answers

1. **Provider arm**: `ConstructionStoreRhsV1` gains a coverage-only
   arm carrying the provider's `value_site`; value production is
   deferred to the statement-surface port route — the plan never
   allocates. The arm admits `construction_source` = bare `new` of a
   builtin class (no args, no field initializers, no user Birth row).
2. **Field-type gate**: `field_decls` are re-checked per field
   against that field's store: `i64` typed -> LiteralI64/Parameter
   RHS only; `<Class>` typed -> provider RHS of the same named class;
   untyped -> provider RHS supplies the type. Anything else is
   `FieldContractUnsupported`. The per-field check moves after store
   classification (two-phase inside `issue_construction_plan`).
3. **`has_stored_field_initializer`**: the trigger stops being an
   unconditional reject. Generated stores satisfy the same arm rules;
   the `StoredFieldInitializer` membership still proves the source
   role at the S0 residence join — no new trigger↔statement relation
   is needed because the provider `construction_source` row is shared.
   An excluded default expression rejects through its RHS arm.
4. **`field_demands`**: provider fields carry the provider class's
   existing home-demand mapping (`callable_home_demands` family —
   builtin box class -> `HomeDemandV1::Handle`); `install_construction`
   accepts a non-Trivial demand iff that field's store is the
   provider arm, keeping `source-orleanup-contract` closed for every
   other shape.
5. **Fault landing**: the shared birth landing stays a bare
   `ReturnFault`. Bounded claim: provider values are builtin
   `new <CoreBox>` homes; no `HomeRelease` instruction exists for
   them anywhere in this lane, so the "no field release owed" claim
   holds exactly as for every other builtin `new` — the reclaim
   question for user-box providers is deferred with them.
   `ReclaimUnpublished` remains outer-storage-only, unchanged.
6. **Statement surface**: `statement_surface.rs:456-461` is the
   extension point — after `take_construction_store` returns the
   provider-arm store, the surface lowers the `new` through the
   existing `lower_prepared_raw_new_expression_with_port_v1` /
   `RawOrdinaryNewClaimPortV1` route (the same route S0's provider
   recording hooks), then hands the produced `ValueId` to the
   canonical `Invoke FieldSet` emission. One allocation owner, one
   commit-per-emission check, unchanged.
7. **Init fields stay out**: `init {}` names do not populate the
   plan's field inventory and this slice does not admit them — the
   production shape is `field: ArrayBox = new ArrayBox()` decl-init;
   the `init`-listed fixture form remains a document-lane-only test
   shape from S0.

## Focused acceptance specification (for S1)

- Positive (artifact lane, `compile_normal_with_published`):
  `box Holder { free_stack: ArrayBox = new ArrayBox()
    block_used: ArrayBox = new ArrayBox() capacity: i64 = 0
    birth() {} seed() { ... four alias pushes ... } }` compiles
  end-to-end — canonical `Invoke FieldSet` stores for the provider
  and scalar arms, `named_array_field_allocations` recorded for each
  provider site, four validated named-array rows, and
  `validate_artifact_after_compiler_finishing` passes.
- Negative: `new <UserBox>` provider RHS; declared type `ArrayBox`
  with `new StringBox()` provider (type/class mismatch); untyped
  field with a non-builtin provider; arbitrary initializer
  (`field: i64 = f()`); explicit store duplicating a decl-init field;
  missing store for a declared field; `i64` field carrying a provider
  `new` RHS. All typed `ConstructionUnavailableV1`/residence rejects.
- Existing pins: `construction_plan_keeps_unavailable_dependencies_
  out_of_empty_cleanup` gains the two changed expectations
  (`value: i64 = 1` decl-init now eligible via LiteralI64;
  `init { items }` stays rejected); every other plan/coseal/store
  pin stays green.
- `apps/boxtorrent-mini --emit-mir-json` re-measured: record whether
  the first terminal stays `static-result-ingress/foreign-lineage`
  or moves.

## Non-claims

- No code issued under this card; no production switch, caller-zero,
  retirement, Gate-1 green, or whole-app success is claimed.
- `static-result-ingress/foreign-lineage` (`Main.main`, `using`
  lineage) is a different named family — its own row, not this one.
- Caller-side `new HakoAllocPage` admission (ordinary-new
  eligibility for object-fielded boxes) is the D18-successor's
  queued row 6; this slice only admits the provider's own store
  inside the construction plan.
- No release/fini semantics for field homes is minted; user-box
  providers stay outside the claim entirely.

## Exit

- [x] Owner seam confirmed: `issue_construction_plan` arms +
      `statement_surface` take/emit + canonical `Invoke FieldSet` —
      no second physical owner.
- [x] Bounded acceptance defined (builtin provider class, declared
      type = provider class or untyped-with-provider, same RHS arm
      rules for generated and handwritten stores).
- [x] Reclaim/fault claim bounded: builtin `new` homes carry no
      `HomeRelease` anywhere in this lane; user-box providers stay
      out; `ReclaimUnpublished` untouched.
- [x] Named next terminals recorded: artifact lane =
      this card's scope; document lane =
      `static-result-ingress/foreign-lineage` (separate family).
- [x] One bounded next slice emitted:
      `MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-S1`.
