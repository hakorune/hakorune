# mimalloc-lite formal-forward result D0 (`me.realloc` unclaimed forward)

Status: historical S0 landed; its class-annotation ownership inference is
superseded by DECLARED-FORMAL-RESULT-IDENTITY-S0 (2026-10-05).

Current correction: ordinary declared object formals are borrowed handles under
language ownership.md. The parameter issuer supplies class membership/Handle,
not a moved-in Home; no exact ordinary transfer destination was issued. The
owned pass-through and declared actual -> NullableObject statements below are
historical implementation assumptions, not current authority. Returning a formal
preserves NullableForwarded identity. Mixed fresh/borrowed realloc results need
an exact result relation before receiver observation/received_nullable publication.
Page.release updates block policy and does not dispose the handle descriptor.

The earlier (8,0) census remains evidence of that implementation, not proof of
correct ownership. Current implementation/verification belongs to
mirbuilder-declared-formal-result-identity-s0-2026-10-05.md; design reconciliation
is in mirbuilder-declared-object-formal-contract-d1-2026-10-05.md.
Owns the EXE lane's frontier after OBJECT-FORMAL-FIELD-STORE-S0
(`57562186df`): the last `artifact-unowned-lifecycle-site` stop, on the
two `RetainedUnavailable` `reallocResult` sites whose
`Call(BirthConstructor HakoAllocHandleResult.birth/3)` instructions
emit raw-lane with no owning local commit (owner slot 38 measured).
Parent bundle slices: `OBJECT-FIELD-RECEIVER-CALL` (10),
`RESULT-NEW-OBJECT-ARGS` (11), `FORWARDED-RESULT` (12) of
`mirbuilder-app-bundle-mimalloc-lite-d0-2026-09-30.md`.
Scope: `MIRBUILDER-APP-MIMALLOC-LITE-FORMAL-FORWARD-RESULT-D0`
Related: `mirbuilder-app-mimalloc-lite-object-formal-field-store-d0-2026-10-04.md`

## Census — the `PrefixNotCovered(Body(3))` chain

`reallocResult` (`page_heap_box.hako:300-319`) holds two result claims
whose `home_prefix` stops at Body(3):

```hako
local replacement = me.realloc(handle, requested_size)   // Body(3)
if replacement == null {                                 // Body(4), site :315
    return new HakoAllocHandleResult(0, 4, null)
}
return new HakoAllocHandleResult(1, 0, replacement)      // site :318
```

Code-verified decomposition, top to bottom:

1. The `Local` statement falls through every claim lane in
   `scan_statement_flow` — I64 direct, qualified static, lexical
   nullable/i64, Map, Handle, receiver nullable — and lands on the
   inventory fallback: `install_inventoried_call_result` +
   `PrefixNotCovered(Body(3))`
   (`home_new_prefix_scan.rs:694-698`).
2. `me.realloc(..)` is a `me.<method>` receiver call returning a
   nullable object; its lane is `issue_receiver_local_call`, whose
   `local_nullable_call` predicate requires a sealed
   `receiver_call_observations` row carrying
   `OrdinaryNewResultClassV1::NullableObject`
   (`ordinary_new_coseal_issue.rs:583-594`,
   `home_local_call_flow.rs:693-737` — the row itself seals no
   arguments).
3. `observe_receiver_call_sites` mints that row only when
   `claims.get(&callee)` exists — `HakoAllocHeap.realloc/2` carries no
   `OrdinaryNewResultClassV1` claim
   (`ordinary_new_receiver_call_observation.rs:160-161`).
4. `realloc`'s pass-B exits are `Null` ×4 plus three forwards:
   `return same` ×2 → `me.small_page.resizeInPlace` /
   `me.medium_page.resizeInPlace` (resolved through the `small_page`/
   `medium_page` field-write claims to `HakoAllocPage.resizeInPlace/2`)
   and `return replacement` → `me.allocate` (claimed
   `NullableObject(HakoAllocHandle)` — the same claim that already
   covers `allocateResult`'s prefix). `evaluate_row` marks the row Dead
   because `HakoAllocPage.resizeInPlace` is neither claimed nor pending
   (`ordinary_new_result_class_claim.rs:354-364`).
5. `HakoAllocPage.resizeInPlace` (`page_heap_box.hako:145-178`) drops
   its entire draft row in pass A: `return handle` returns a
   `Parameter` binding — `observe_function`'s exit grammar admits only
   `Local`-kind `ForwardLocal`s, so the `_ => return` arm discards the
   row (`ordinary_new_result_class_claim.rs:424-432`). `BindingKindV1`
   keeps `Parameter` and `Local` disjoint (`records.rs:9-19`).

## The two semantic gaps behind the dead claim

The pass-A grammar hole is the surface; the deeper questions are the
same family the parent card predicted:

- **Pass-through formal result class.** `resizeInPlace` returns
  `handle` — a caller-provided object, not a construction.
  `Object(C)`/`NullableObject(C)` assert "every exit is `new C` or
  `null`"; a formal forward needs a new arm, and that arm needs a
  class authority the source does not carry: `handle` is an
  unannotated formal. Candidate authorities, none landed:
  (a) intra-body field-use constraint — `handle.page_id`,
  `handle.block_id`, `handle.requested_size =` uniquely name
  `HakoAllocHandle`'s field set (a new, duck-typing-shaped authority);
  (b) caller-side argument-class composition — `realloc`'s `handle`
  actual is `reallocResult`'s own opaque formal, and `reallocResult`
  has no callers in this program, so the chain has no ground truth;
  (c) a class-free `Nullable`-family claim — "returns null or some
  object" — honest from source alone but weaker than every existing
  arm and must still compose with `Fwd(allocate)`'s
  `NullableObject(HakoAllocHandle)` in `realloc`'s row.
- **Object-formal call-argument use kind.** `handle` at the
  `me.realloc` actual is not a dominated value leaf — inside `realloc`
  it is *consumed* (`me.release(handle)`) and inside `resizeInPlace`
  it is *mutated* (`handle.requested_size = requested_size`). In
  `reallocResult` `handle` is dead after Body(3), so a transfer shape
  fits, but the claim vocabulary has no consuming/mutating formal-arg
  arm — the borrowed-actual machinery records evidence without
  ownership classification.
- **Boundary to name, not a bug:** condition-position calls are not
  claim-gated at all — `if me.isLiveHandle(handle) == 0` (Body(2)) is
  covered because `observe_if_statement` only checks field-request
  scalar conditions; its `handle` actual needs no row. Arg-position
  object formals in condition calls are unclaimed evidence today;
  whether they need gating is this family's question, not this card's
  fix.

## Why the residual still freezes the EXE lane

The two sites stay `RetainedUnavailable`: their result-`new` claims
are retained (membership is sealed) but `home_prefix` is Err, so the
local commits never take them — while the raw lane still emits
`Call(BirthConstructor HakoAllocHandleResult.birth/3)` instructions
with `requires_lifecycle_validation()`. Root validation then finds a
lifecycle instruction with no owning completed commit
(`root_validation.rs:296-315`) → `artifact-unowned-lifecycle-site`.
The freeze is correct: an unclaimed result `new` emitting a lifecycle
site is exactly the gap it guards. Resolution must either complete the
claim (the `me.realloc` forward chain above) or change what selected
unclaimed sites emit — emission-time skipping stays rejected
(`IncompleteSelectedCoverage`; see the parent card's declined
reachable-set alternative).

## Sharpened finding — no in-program class authority exists

The census narrows to exactly one missing authority: the class of
`resizeInPlace`'s `handle` formal. Every existing authority bottoms
out empty:

- `BorrowedFormalObjectViewV1` composes a formal's class from incoming
  actuals (`ordinary_new_borrowed_formal_source.rs:19-34,56-69`).
  `reallocResult` has **no callers anywhere in the program** — `main`
  calls `heap.allocate` only — so `reallocResult.handle` mints no view,
  and the `Forward` chain through `realloc`/`resizeInPlace` stays
  pending forever.
- `field_write_claims` records stored-`new` classes for `me.<field>`
  writes only; `handle.<field>` accesses mint nothing.
- `DeclaredHandle` is admitted only for `ArrayBox`
  (`callable_parameter_contract/issuer.rs:49-51,93-101`) — a user-class
  declared formal (`handle: HakoAllocHandle`) is rejected at
  `UnsupportedDeclaredType`, and `OpaqueHandle` (unannotated) proves
  neither class nor object-ness.
- Field-access-signature inference (`{page_id, block_id,
  requested_size}` uniquely names `HakoAllocHandle` here) contradicts
  the documented authority boundary — `FieldReadOperand`: "class and
  field declaration belong to the issuer's sealed object view, never
  to this draft" (`ordinary_new_borrowed_formal_uses.rs:94-99`).
- A class-free `NullableOpaque`-style arm is unsound for this lane:
  `replacement` would install `received_nullable`, and releasing a
  possibly-i64 value is exactly what the Home ledger must not guess.

## Decision — `MIRBUILDER-APP-MIMALLOC-LITE-FORMAL-FORWARD-RESULT-D0` (accepted)

Owner contract (user-fixed):

1. **Class authority lives in the callable parameter contract issuer**:
   an explicit `formal: <name>` declaration resolves to an existing
   resolved class identity — the same `ordinary_box_coverage` inventory
   `new` sites use. No field-set inference (that boundary stays:
   `FieldReadOperand` keeps class under the sealed object view).
   `CallableParameterContractKindV1::DeclaredObject(class)` is a new
   kind **arm in the existing contract catalog**, not a new receipt —
   an ordinary-box declared formal, `home_demand() == Handle`, same
   parameter carrier as `OpaqueHandle`.
2. **The `.hako` annotation is a legitimate API contract change**, not
   a workaround: `reallocResult`/`realloc`/`resizeInPlace` already use
   `handle` exclusively as a `HakoAllocHandle` (field accesses and
   forwards); the declaration makes the implicit contract source-level
   authority. Existing inputs are untouched — unannotated formals stay
   `OpaqueHandle`, `i64`/`usize` keep `ExactTrivial`, `ArrayBox` keeps
   `DeclaredHandle`, unresolvable declared names keep
   `UnsupportedDeclaredType`. **Null is admissible**: every callee
   null-guards `handle` first — `handle == null → return null` is the
   existing check site, so the declared contract means "nullable
   `HakoAllocHandle`"; a `null` actual is the Void literal, legal for
   any object-typed formal.
3. **Class guarantee and ownership are separate, and borrowed values
   never become Homes**:
   - `realloc`/`reallocResult` `handle: HakoAllocHandle` →
     `DeclaredObject` = **owned** formal (moved-in). `me.realloc(handle,
     ..)` consumes `reallocResult.handle` at the call edge — last use.
   - `resizeInPlace` `handle` stays `OpaqueHandle` = **borrowed** —
     `realloc` keeps using `handle` after the call (`handle.page_id`
     reads, `me.release(handle)`), so the arg cannot be consuming. Its
     `return handle` is a **borrowed return**: the result is the
     caller's arg-0 object or `null`. New exit-draft arm
     `ForwardFormal{ordinal}` → pending `NullableForwarded{ordinal}` —
     an identity claim ("result ≡ caller's arg-0, or null"), class-free
     and sound for any actual kind.
   - Composition substitutes at the caller: `realloc`'s `return same`
     (`same` bound from `me.small_page.resizeInPlace(handle, ..)`)
     resolves `Fwd` to `NullableForwarded{0}` → substitutes arg-0's
     actual = `realloc.handle` → `DeclaredObject(HakoAllocHandle)` →
     the exit composes as `New(HakoAllocHandle)`. Unprovable actual
     classes stay Dead — `NullableForwarded` never leaks a borrowed
     value into a Home; only a resolved class composes `NullableObject`.
   - `realloc` then claims `NullableObject(HakoAllocHandle)` by
     fixpoint, `me.realloc` mints its observation, `replacement`
     installs `received_nullable` — the **caller re-owns** the returned
     object (consumed-in, owned-out); `new(1,0,replacement)` feeds the
     landed `Parameter` store arm end-to-end.
   - `realloc`/`reallocResult` are the only methods annotated:
     `resizeInPlace`'s `handle` must stay borrowed (caller reuse), and
     `release`/`isLiveHandle` are outside the claim chain — their
     `OpaqueHandle` formals accept owned bindings as borrowed actuals.
4. **Existing ABI owners close the carrier**: `DeclaredObject` →
   `home_demand() == Handle` → identical param carrier to
   `OpaqueHandle`; results cross the same Return edge —
   `unsupported terminator Invoke` stays the designed backend
   boundary. Caller-side lifetime/cleanup is the existing
   `received_nullable` machinery (same shape as `allocateResult`'s
   `handle`). No new Receipt types: `DeclaredObject` is a contract-kind
   arm, `NullableForwarded` is a result-class claim arm, the
   observation/prefix machinery is untouched.

Rejected alternatives:

- Field-access-signature class proof — mints a duck-typing authority
  against the documented object-view boundary (see above).
- `NullableOpaque` class-free owned claim — unsound: `received_nullable`
  would owe release on possibly-i64 values.
- Reachable-set narrowing — declined on the parent card (no selection
  authority; the sealed-undertaking criterion excludes AppMain
  reachability).
- `NoSafeSlice` — a real authority exists via explicit declaration;
  stopping would strand a solvable frontier.

Residual notes (recorded, not blocking):

- `realloc`/`resizeInPlace` bodies are never homes-verified (no `new`
  sites): under owned-formal semantics, `realloc`'s early `return null`
  paths keep `handle` owned — the block is freed by `me.release` or
  never freed; descriptor accounting is outside the verified set.
  `release`'s consuming contract is the honest next ownership row —
  naming it, not implementing it.
- Consuming-actual enforcement (`reallocResult.handle` dead after
  `me.realloc`) is untracked, same as all arg actuals today.
- `isLiveHandle`/`release`/`HakoAllocPage.release` formals stay
  `OpaqueHandle`; annotating them is an API-consistency choice outside
  this slice.

Smallest next slice:
`MIRBUILDER-APP-MIMALLOC-LITE-FORMAL-FORWARD-RESULT-S0` — contract
`DeclaredObject` arm (issuer resolves declared name → unique ordinary
box), `.hako` annotations on `reallocResult`/`realloc` only,
`ForwardFormal`/`NullableForwarded` exit arms with caller-side arg-0
substitution, `install_parameters` arm, census flip (the two sites'
`home_prefix` → Ok, all eight claims complete), focused pins
(declared-formal claim mints `NullableObject`; opaque-formal return
mints `NullableForwarded`; unresolvable declared name stays
`UnsupportedDeclaredType`; forwarded-actual without class stays dead),
scope-guard pin, real-lane observation.

Non-claims:

- No claim that `reallocResult`/`realloc`/`resizeInPlace` must be
  covered — they are unreachable from `main`; the selection authority
  question stays declined as before.
- No reachable-set narrowing, no emission-time skip.
- No EXE PASS, no production switch.
- The `home_prefix` 6/2 split is the status quo, not a regression — the
  six covered sites keep their claims under any fix.

## S0 landed record — `MIRBUILDER-APP-MIMALLOC-LITE-FORMAL-FORWARD-RESULT-S0`

Landed contract (as decided above):

- `CallableParameterContractKindV1::DeclaredObject(Box<str>)` — the
  issuer resolves an explicit `formal: <name>` against
  `ordinary_box_coverage` after the existing `Map`/`DeclaredHandle`
  arms; unresolvable names keep `UnsupportedDeclaredType`
  (`callable_parameter_contract/{model,issuer}.rs`). `home_demand` stays
  `Handle`; `install_parameters`, `physical_signature` and
  `dynamic_admission` route it identically to `OpaqueHandle`.
- `.hako` annotations: `realloc(handle: HakoAllocHandle, ..)` and
  `reallocResult(handle: HakoAllocHandle, ..)` only — `resizeInPlace`
  stays `OpaqueHandle` (borrowed) per the Decision.
- `ResultClassExitDraftV1::ForwardFormal{binding, ordinal}` for
  `return <Parameter>`; `OrdinaryNewResultClassV1::NullableForwarded{
  ordinal}` is the class-free identity arm (`class()` → `None`,
  `is_nullable` → true). `PendingExitV1::Fwd` now carries call-site
  `Local` actuals; `evaluate_row` substitutes `NullableForwarded`
  callees through `parameter_kind` — only a `DeclaredObject` actual
  resolves a class, anything else stays Dead.
- `return <DeclaredObject formal>` mints `NullableObject` (owned
  pass-through, nullable because object formals admit `null`).
- `observe_receiver_call_sites` takes the batch-slot contract rows:
  a `Local` argument whose binding is a `DeclaredObject` formal
  classifies as `SelectedNewArgumentKindV1::Handle`, and class-free
  claims (`class().is_none()`) mint no observation.
- The nullable emission lane admits the `Handle` argument kind; the
  declared `Box(_)` carrier publication is consistent under the
  issuer's proof (`terminal_call.rs` `emit_receiver_nullable`).
- `physical_boundary.rs` derives expected incoming edges from
  `all_edges` under the destination projection — a contractible
  unrecorded bridge block no longer drops a projected edge
  (`reallocResult`'s `Branch → bridge(Jump) → Invoke` fold).
- Freeze diagnostics carry owner/symbol/site/retained-row detail
  (`root_validation.rs`, `raw_ordinary_new_claim/terminal_call.rs`,
  `terminal_call.rs`) — same fail-closed boundary, named evidence.

Evidence:

- `cargo test --lib callable_parameter_contract::` — 13/13
  (`DeclaredObject` projection, `UnsupportedDeclaredType` edges, the
  stale `MapBox → Map` baseline expectation corrected).
- `cargo test --lib normal_callable_semantic_package` — 591 passed, 3
  failed: only the known baseline
  (`birth_receiver_non_escape_rejects_unproven_uses_before_row_publication`,
  `main_static_child_port_consumes_all_role_rows_once`,
  `qualified_call_map_argument_reaches_the_named_capability_boundary`).
- `cargo test --lib resolved_semantics` — 349/349.
- Fixture pins: `page_heap_fixture_forwarded_formal_claims`,
  `opaque_formal_return_mints_nullable_forwarded_claim`,
  `declared_formal_return_composes_nullable_object_claim`,
  `forwarded_formal_substitutes_declared_actual_and_rejects_opaque`,
  `page_heap_fixture_composes_allocate_and_realloc`,
  `page_heap_fixture_observes_me_allocate_and_realloc`,
  `page_heap_fixture_result_claim_census` — the census flipped to
  `(8, 0)`: all eight `HakoAllocHandleResult` constructions claim
  `NullableObject(HakoAllocHandle)` with covered `home_prefix`.
- Scope guard: new FORMAL-FORWARD-RESULT-S0 pin block; the watch list
  stops only at the known `brand_catalog_tests.rs=961` baseline.
  `ordinary_new_coseal_issue.rs` re-hoisted to 797; the mixed fixture
  tests moved beside the census in `ordinary_new_result_claim_tests.rs`
  (mixed file back to 790, below its 841 baseline).
- Production lane (`mimalloc_lite_exe.sh`, debug bin):
  - MIR JSON: `unsupported terminator Invoke` — the designed negative
    the smoke pins, reached again after the fixes above.
  - EXE: `artifact-source-unavailable` at `MiWorkload.run/0` Body(0) —
    `local heap = new HakoAllocHeap()` is `RetainedUnavailable`
    because `HakoAllocHeap`'s construction plan is
    `Err(FieldContractUnsupported)`. `reallocResult` (the slice's
    target) passes local-commit validation completely.

## Next residual census — `new HakoAllocHeap` construction (EXE lane)

`HakoAllocHeap.birth()` (`page_heap_box.hako:196-199`) declares no
stores; both fields initialize from defaults
`small_page: HakoAllocPage = new HakoAllocPage(0,
LayoutBox.class_size(0), LayoutBox.class_capacity(0))` (and
`medium_page`), which the parser prepends to `birth` as generated
stores. `issue_construction_plan` then fails `FieldContractUnsupported`
on three independent edges, in encounter order:

1. The provider child `HakoAllocPage` is **not** `PlainI64NoHook` —
   it owns four `ArrayBox` fields (`OwnedArrayFieldsNoHook`), and the
   provider arm only admits `PlainI64NoHook` user classes
   (`instance_construction.rs:360-369`). Nested reclamation of an
   owned-field child is an ownership-semantics extension, not a shape
   extension.
2. Provider `new` arguments `LayoutBox.class_size(0)` /
   `class_capacity(0)` are static calls — only integer/bool literals
   seal today (`instance_construction.rs:384-399`). Call-typed provider
   args need a trivial-argument authority.
3. `birth()`'s own body is `me.small_page.seedBlocks()` /
   `me.medium_page.seedBlocks()` — non-store statements hit
   `BodyCoverageUnsupported` at the statement grammar
   (`instance_construction.rs:245-247`). Birth-body call statements are
   a separate coverage question.

This residual predates the slice (the EXE lane previously stopped
earlier at `reallocResult`) and is a bounded design problem of its
own — same family as OBJECT-FORMAL-FIELD-STORE, one layer up the
ownership tree.
