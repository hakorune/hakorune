# MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-S1

Status: active
Date: 2026-09-27
Parent: MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-D1 (closed,
  decision accepted) — implements the accepted relation from
  `mirbuilder-gate1-field-array-i64-push-d1-2026-09-27.md`.
  Workstream row H / Gate-1 series row 3 (provider-store
  construction eligibility).
Mode: fast — one responsibility, bounded to the accepted relation.

## Responsibility

Admit `me.<field> = new <builtin>` provider stores into the single
construction-store owner so `HakoAllocPage`'s production field forms
pass `issue_construction_plan` -> `ConstructionState` -> canonical
`Invoke FieldSet` on the artifact lane — without forking a second
store lane and without changing ordinary-new eligibility.

## Boundary (from the accepted Decision)

- New `ConstructionStoreRhsV1` arm (coverage-only): records the
  provider's `value_site`; `construction_source` must be a bare `new`
  of a builtin class — zero args, zero field initializers, class has
  no user `Kind::Birth` row (the S0 `resolve_birth_provider`
  predicate class). The plan never allocates; value production is
  deferred to the statement-surface port.
- Field-type gate becomes two-phase: classify stores first, then
  verify per field — `i64` decl -> LiteralI64/Parameter only;
  `<Class>` decl -> provider RHS of the same named class; untyped
  decl -> provider RHS supplies the type. Mismatch ->
  `FieldContractUnsupported`.
- `has_stored_field_initializer` stops being an unconditional
  reject; generated stores satisfy the same arm rules (README
  contract paragraph updated in this same slice).
- `field_demands[ordinal]` = `Handle` for provider fields (existing
  `callable_home_demands` class mapping); `install_construction`
  accepts non-Trivial iff that field's store is the provider arm.
- Statement surface (`statement_surface.rs` take/emit pair): on the
  provider arm, lower the `new` through
  `lower_prepared_raw_new_expression_with_port_v1` /
  `RawOrdinaryNewClaimPortV1` — the same route that performs S0's
  `named_array_field_provider_recording` — then emit the canonical
  `Invoke FieldSet` with the produced `ValueId`.
- Shared fault landing stays bare `ReturnFault`; `ReclaimUnpublished`
  stays outer-only; `unowned-birth-call`/`emission-count` validators
  stay correct untouched (builtin `new` = `NewBox` instruction).
- `init {}` fields remain unadmitted (not the production shape).

## Non-claims

- No caller switch, no retirement, no Gate-1 green, no whole-app
  pass — this admits the provider's physical home only.
- No `new <UserBox>` provider RHS, no arbitrary init exprs, no
  release-suffix/fini work, no `init {}` field admission.
- No ordinary-new eligibility change: caller-side
  `new HakoAllocPage` admission stays the D18-successor row 6.
- No static-result-ingress work — `Main.main`'s `using` lineage
  terminal is a separate named family.

## Acceptance

1. `box Holder { free_stack: ArrayBox = new ArrayBox()
   capacity: i64 = 0 birth() {} seed() { local a = me.free_stack
   loop(i < 2) { a.push(i) i = i + 1 } } }` compiles on the ARTIFACT
   lane (`compile_normal_with_published` or the published artifact
   entry): canonical `Invoke FieldSet` stores for provider + scalar
   arms, `named_array_field_allocations` recorded per provider site,
   validated named-array rows, `validate_artifact_...` passes.
2. Negatives (typed rejects): `new <UserBox>` provider; declared
   `ArrayBox` + `new StringBox()`; untyped field + non-builtin
   provider; `field: i64 = f()`; explicit store duplicating a
   decl-init field; missing store; `i64` field + provider `new` RHS.
3. `construction_plan_keeps_unavailable_dependencies_out_of_empty_
   cleanup` updated: `value: i64 = 1` decl-init now eligible via
   LiteralI64; `init { items }` stays rejected. All other plan /
   coseal / store pins green.
4. `apps/boxtorrent-mini --emit-mir-json` re-measured; first
   terminal recorded (expected: still
   `static-result-ingress/foreign-lineage` — different family).
5. README contract paragraph (`normal_callable_semantic_package/
   README.md` "scalar Birth bodies" claim) updated to the widened
   bounded acceptance in this slice.
6. `current_state_pointer_guard` + `mirbuilder_qualified_route_scope_
   guard` green; every edited file < 800 lines.
