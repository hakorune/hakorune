# mimalloc-lite object-formal field store D0 (caller-provided object into declared field)

Status: S0 landed — census recorded, Decision accepted and
implemented. Owns the EXE lane's frontier after TASK4-ARTIFACT-S0
(ad1fc53776): the `artifact-unowned-lifecycle-site` stop on the
`Call(BirthConstructor HakoAllocHandleResult.birth/3)` sites inside
`HakoAllocHeap.allocateResult`/`reallocResult` — now reduced to the
two `me.realloc`-prefix-blocked sites (owner slot 38 measured).
Parent bundle slice: `RESULT-NEW-OBJECT-ARGS` (bundle card
`mirbuilder-app-bundle-mimalloc-lite-d0-2026-09-30.md` slice 11).
Scope: `MIRBUILDER-APP-MIMALLOC-LITE-OBJECT-FORMAL-FIELD-STORE-D0`
Related: `mirbuilder-app-mimalloc-lite-opaque-checked-compare-task4-artifact-d0-2026-10-04.md`

## Census — eight `HakoAllocHandleResult` result claims

`page_heap_fixture_result_claim_census` retains nine result claims: one
`HakoAllocHandle` (fully sealed post-S0) and eight
`HakoAllocHandleResult` sites — three in `allocateResult`
(`page_heap_box.hako:214-225`), five in `reallocResult` (`:300-319`).

| site | `home_prefix` | `construction` | `argument_rows` |
|---|---|---|---|
| alloc:216/221/224 | Ok | Err | Ok |
| realloc:302/306/310 | Ok | Err | Ok |
| realloc:315/318 | Err `PrefixNotCovered(Body(3))` | Err | Ok |

Decomposition (worker census + S0 measured correction):

- `construction()` Err measured at S0 is `ConstructionUnavailableV1::
  BodyCoverageUnsupported`, not the `FieldContractUnsupported` the
  worker census inferred from code reading. `issue_construction_plan`
  walks the birth body before the field-demand check: parser
  normalization (`apply_stored_field_initializer_constructor_prologues`)
  prepends `me.ok = 0` / `me.reason = 0` for the `= 0` declared defaults,
  so the handwritten `me.ok = ok` / `me.reason = reason` re-store the
  same ordinals and the first-store guard rejects before the
  `Some(name)` arm is ever evaluated. Two blockers sat behind one row:
  (a) the scalar default-overwrite re-store, and (b) `me.handle =
  handle` — a `Parameter` RHS into the `handle: HakoAllocHandle`
  declared-type field, where the `Some(name)` arm admitted only
  `ProviderConstruction`.
- `home_prefix` Err on the last two `reallocResult` sites is
  `PrefixNotCovered(Body(3))`: `local replacement =
  me.realloc(handle, requested_size)` (`page_heap_box.hako:313`) is an
  unclaimed forward — a callee-claim question (`realloc` takes the
  object-typed `handle` formal and forwards to `release`), separate
  from this card's store arm.
- `argument_rows()` Ok on all eight: `null` literal and `handle`/
  `replacement` BoundValue args all convert to
  `OrdinaryNewTrivialArgumentV1` rows. Ok means source observation
  sealed; physical consumer admission is verified by the lane, not the
  row.
- `main.hako` never calls `allocateResult`/`reallocResult`: the eight
  sites sit in methods unreachable from `main`. The compiled set is
  every same-module declaration — `source_backed.rs:280-303` seals the
  full parser inventory unconditionally, `program_root_work_plan.rs:551-603`
  classifies every non-static box `ImmediateAndRuntime`, and
  `install/lowering_port.rs:528-543` demands
  selected==consumed==emitted (`IncompleteSelectedCoverage`/
  `IncompleteOrdinaryNewCoverage`). The package README explicitly
  states the sealed undertaking — "never AppMain reachability" — is the
  coverage criterion.
- Caller exclusion alone cannot resolve this lane: `HakoAllocHandleResult
  .birth` is emitted through the declaration-derived constructor batch
  regardless of callers; narrowing would have to drop the whole
  `HakoAllocHandleResult` box and would need a new transitive-closure
  authority over birth edges, field-initializer provider `new`s,
  `me.<field>` receivers, and static calls — none exists.

## Decision — `MIRBUILDER-APP-MIMALLOC-LITE-OBJECT-FORMAL-FIELD-STORE-D0` (accepted)

The `Some(<user-class>)` field arm of `issue_construction_plan` gains
one RHS form: the existing `ConstructionStoreRhsV1::Parameter` (a birth
formal recorded with its exact site and binding), admitted when the
**declared field class** resolves uniquely to a `PlainI64NoHook`
user-class definition — birth formals are unannotated (`OpaqueHandle`
kind only), so the declared field type is the sole class authority;
the actual object's class remains the `new` call site's obligation.
Ownership transfers into the storing box: the field is an owned object
field, so `field_demands` marks the ordinal `Handle` and the
destruction/reclaim machinery treats the stored value exactly like a
`ProviderConstruction` child — same `PlainI64NoHook` child bound, same
if-live `OwnedObjectFieldRelease` discharge (zero-initialized slots
make the stored `null` case safe).

Residence proof extends symmetrically: `OwnedFieldResidencesV1` gains a
`Provided` arm recorded when the field's sole observed write is a
birth-side `me.<field> = <Parameter>` store. Children sealing resolves
`Provided` to the declared class and runs the same checks as
`Provider(class)` — user class, `PlainI64NoHook`, non-self. The
`OrdinaryNewFieldWriteClaimsV1` class-claim map stays untouched: no
`me.<field>.<method>` resolver needs a provided-store class, and the
map records stored-`new` classes only.

`HakoAllocHandle` (all-scalar fields) is `PlainI64NoHook`, so
`me.handle = handle` becomes the first admitted instance.

Rejected alternatives:

- Reachable-set narrowing: no reachability authority exists, the
  sealed-undertaking coverage criterion explicitly excludes AppMain
  reachability, emission-time skipping violates
  `IncompleteSelectedCoverage`, and caller-only exclusion leaves the
  declaration-derived constructor batch. A new transitive-closure
  selection authority is a far larger design than the frontier needs.
- Borrowed (non-owning) field: the caller returns immediately after
  the `new`; nothing retains ownership, so a borrowed view would
  dangle. Transfer is forced by the usage shape.
- Widening `ProviderConstruction`: the provider arm records a `new`
  site with child identity and sealed trivial args; a parameter has no
  construction site — conflating them forges authority the source does
  not carry.

Boundaries that stay closed in this slice:

- `BoundValue` (call-result) RHS: `allocateResult` binds `local handle
  = me.allocate(size)` and passes it to `new`, but the STORE in
  `birth` sees only the `Parameter`. Locals rebound or copied inside
  `birth` stay rejected — the store RHS must resolve to the formal.
- Non-`PlainI64NoHook` children, wrong-class formals, weak fields,
  re-stores: unchanged `FieldContractUnsupported`.
- The `me.realloc` unclaimed forward (`reallocResult` Body(3) prefix)
  and object-formal call-argument use kinds: separate rows — the 2
  prefix-blocked sites stay `RetainedUnavailable` even after this
  slice.
- No physical whitelist, emission, or selection change; the physical
  lane's object-arg admission is observed, not widened.

Smallest next slice:
`MIRBUILDER-APP-MIMALLOC-LITE-OBJECT-FORMAL-FIELD-STORE-S0` — the
`Some(name)` `Parameter` arm (declared-class `PlainI64NoHook` bound)
plus the `Provided` residence arm and `Handle` demand marking; unit
pins (admit a formal into a plain user-class field; reject
builtin/`ArrayBox` fields, non-plain declared classes, rebound/copy
locals, non-formal expressions); census flip — all eight
`construction()` Ok, `home_prefix` keeps the 6/2 split — and
real-lane observation of the next named stop (the two
`me.realloc`-blocked sites stay `RetainedUnavailable`).

## S0 landed — `OBJECT-FORMAL-FIELD-STORE-S0` (commit `57562186df`)

Landed scope covers the accepted arm plus one measured co-blocker the
D0 census had folded into the wrong reject kind:

- `issue_construction_plan`: `Some(<user-class>)` fields admit a birth
  `Parameter` RHS when the declared class resolves uniquely to a
  non-self `PlainI64NoHook` user class; every non-scalar declared field
  is marked `HomeDemandV1::Handle`. A second bounded admission —
  `has_stored_field_initializer() &&` scalar (`i64`/`usize`) declared
  type — lets a generated `me.<field> = <default>` initializer be
  overwritten by the field's handwritten store; the dead literal is a
  plain scalar never observed before constructor return, so release is
  trivial and both stores emit in source order. Object-field re-stores
  stay `BodyCoverageUnsupported` (real release semantics required).
- `ordinary_new_field_write_claim.rs`: `OwnedFieldResidenceV1::
  Provided` + `ObservedFieldStoreV1::Provided` — a sole birth-attributed
  `me.<field> = <Parameter>` write issues a provided residence without
  minting a stored-`new` class claim.
- `ordinary_new_coseal_issue_source.rs`: `Provided` resolves through
  the declared field type — user class, `PlainI64NoHook`, non-self —
  and seals `OwnedFieldChildKindV1::Object` children identical to the
  provider path.
- Focused pins (`ordinary_new_result_claim_tests.rs`):
  `provided_parameter_object_store_seals_and_prepares`,
  `provided_parameter_store_rejects_builtin_and_non_plain_fields`,
  `provided_parameter_store_rejects_copied_local`,
  `defaulted_scalar_field_accepts_birth_overwrite`,
  `defaulted_object_field_rejects_birth_overwrite`.
- Real census: all eight `HakoAllocHandleResult` `construction()` Ok;
  `home_prefix` keeps the 6/2 split — `reallocResult`'s two trailing
  sites stay `PrefixNotCovered(Body(3))` at the unclaimed `me.realloc`
  forward.
- Physical consumer edge (`normal_callable_construction_state.rs`):
  `install_construction`'s `source-or-cleanup-contract` check encoded a
  provider-only invariant — `Handle` demand was legal iff a
  `ProviderConstruction` store occupied the same ordinal. A sealed plan
  whose object store is `Parameter` (provided residence) violated it,
  surfacing as the new real-lane terminal
  `[freeze:contract][construction-store/source-or-cleanup-contract]`.
  Bounded repair: the check now derives provided-object ordinals from
  the plan's `Parameter` stores and accepts `Handle` demand on either
  provider or provided ordinals. `emit_construction_store` already
  lowers `Parameter` to plain `FieldSet` — the caller hands the object
  across the birth ABI, so no emission change is needed.
- Lane observation: `--emit-mir-json` regained the designed negative
  `unsupported terminator Invoke`; `--emit-exe` still stops at
  `[freeze:contract][ordinary-new/local-commit/artifact-unowned-lifecycle-site]`,
  attributed to owner slot 38's
  `Call(BirthConstructor HakoAllocHandleResult.birth/3)` — the two
  `RetainedUnavailable` `reallocResult` sites whose `home_prefix` Err
  keeps their claims untaken, leaving the birth calls to emit
  unowned. Residual boundary, ordered task 2 territory.
- Focused evidence: 13/13 `ordinary_new_result_claim_tests` ok (incl.
  relocated census); regression `mir::normal_callable_semantic_package`
  587 passed / 3 failed — same known baselines
  (`birth_receiver_non_escape_rejects_unproven_uses_before_row_publication`,
  `main_static_child_port_consumes_all_role_rows_once`,
  `qualified_call_map_argument_reaches_the_named_capability_boundary`).
  `mir::resolved_semantics` 396/396.
- Guard: S0 pins added to `mirbuilder_qualified_route_scope_guard.sh`
  (Parameter arm, `Provided` residence, demand handling, five focused
  tests, relocated census, `INSTANCE_CONSTRUCTION_SRC`/
  `RESULT_CLAIM_TESTS` watch). The guard still stops at the unchanged
  baseline `brand_catalog_tests.rs`=961; `construction_state.rs` grew
  960→971 (already-over boundary, unwatched);
  `brand_catalog_mixed_result_class_tests.rs` shrank 895→841 after the
  census moved out.

Non-claims stand: no app EXE PASS, no production switch, the two
`me.realloc`-blocked sites stay `RetainedUnavailable`.

## Ordered tasks

1. `OBJECT-FORMAL-FIELD-STORE-S0`: implement the arm above; focused
   positive/negative unit tests; flip the census pin (six
   `construction()` Ok, two still prefix-blocked Err); record the new
   EXE-lane terminal; scope-guard pin.
2. Residual census on `me.realloc` unclaimed forward — object-formal
   call-argument/callee-claim owner; assign to its own card (bundle
   slices 10/12 territory).

## Non-claims

- No `all`/`complete` claim on result claims: `reallocResult`'s two
  `PrefixNotCovered` sites stay unavailable under this slice.
- No reachable-set selection authority; no emission-time skip.
- No app EXE PASS and no production switch claim from this Decision —
  the smoke's `--emit-exe` PASS leg remains the finish line.
- No claim on `HakoAllocHandle` non-plain children: the arm is bound
  to `PlainI64NoHook`; richer child dispositions are their own
  Decisions.
