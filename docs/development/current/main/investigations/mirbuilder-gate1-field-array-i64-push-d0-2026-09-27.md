# MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-D0

Status: closed__2026-09-27__decision-accepted
Date: 2026-09-27
Parent: MIRBUILDER-GATE1-DEPENDENCY-PLAN-D22 (selected owner series
  row 1; this card materializes that row). Owning decision remains
  D5 fork (a).
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-exe-acceptance-suite-red-disposition-d21-2026-09-27.md
  §MIRBUILDER-GATE1-DEPENDENCY-PLAN-D22 (selected next design + finite
  source tuple + acceptance specification)

## Slice

Design only. Settle field residence/lifetime, provider selection, I64
input, NoValue result, failure class, and append commit timing for the
four `HakoAllocPage.seedBlocks/0` field-resident `ArrayBox` `push/1`
call sites. Name the exact retained transport and consumer and the
selected old-edge map. Emit one accepted Decision + one bounded S-card.

This was design work: one read-only worker (`c7ee89b0`) produced the
end-to-end Text-family pipeline census; the primary verified the
load-bearing contracts directly and ran one focused source probe.
No code, fixtures, fallback, production switch, or new semantic
receipts were issued.

## Finite source tuple

`lang/src/hako_alloc/memory/page_heap_box.hako`:

```hako
box HakoAllocPage {
    free_stack: ArrayBox = new ArrayBox()        // lines 39-42:
    block_used: ArrayBox = new ArrayBox()        //   + use_counts
    ...                                          //   + requested_sizes
    seedBlocks() {                               // lines 58-70
        local free_stack = me.free_stack         // field-to-local aliases
        ...                                      //   (all four fields)
        local i = 0
        loop(i < capacity) {
            free_stack.push(i)                   // i64 variable
            block_used.push(0)                   // i64 literals
            use_counts.push(0)
            requested_sizes.push(0)
            i = i + 1
        }
    }
}
```

Recorded callers: `apps/boxtorrent-mini/main.hako`,
`apps/allocator-stress/main.hako`, `apps/mimalloc-lite/main.hako`,
`apps/mimalloc-object-return-api-proof/main.hako`,
`apps/mimalloc-result-contract-proof/main.hako` (via
`HakoAllocPageHeap`/`HakoAllocHeap` `using` edges).

## Observed first terminal (probe evidence)

`./target/quick/hakorune --emit-mir-json` on `apps/boxtorrent-mini`
reproduces the current rejection exactly:

```text
[freeze:contract][callable-loop/route-not-front-selected]
LoopCondRouteRejected(SourceCallOutsideSelectedFamily { call_sites:
  [Body(6)/LoopBody(0..3)] })
function=HakoAllocPage.seedBlocks/0
```

The four `push` calls are cataloged as loop source items
(`source_loop_bridge.rs`) but receive no
`SelectedSourceCoreMethodCallV1` rows — `NamedArrayConstructionRequirementV1::issue`
fails `ConstructionMissing` (the alias initializer site is a
`FieldAccess`, not a `new` expression) and the omission policy
(`source_call_target/named_array_method.rs:64-70`) silently continues.
The armed loop route then rejects the four uncovered items. Fail-fast
already works; what is missing is the source relation.

## Verified census (worker + primary)

- Requirement (`resolved_semantics/named_array_requirement.rs:58-148`):
  receiver `Lexical(Local)` + one initializer relation +
  `construction_source` = bare `new ArrayBox()` in the SAME owner +
  not rebound + statement position + arg `String` literal or same-owner
  `TextToCaller` contract. Cross-owner construction is inexpressible.
- Target issuer (`core_method_instance_target.rs:192-202`): `(ArrayPush,1)`
  hardwires `[TextRetainedByReceiver]` + `NoValue` + `NamedArrayTextV1` +
  `MutatesShape`. `I64Parameter` exists (substring/2) but unused for push.
- Contract issuer (`resolver_core_method_callable_contract.rs`):
  requirement/schema match is `(Some(req), ArrayTextAppend)` only;
  `verify_target` (`:419`) re-hardcodes `ArrayPush => [TextRetainedByReceiver]`.
- Field identity: `CanonicalObjectDefinitionV1.fields()`
  (`UserBoxFieldDecl{name, declared_type_name, is_weak}`) +
  `CanonicalFieldRefV1::from_declaration_ordinal` exist;
  `instance_constructors.with_source_object_definition` is issued
  (`issuer.rs:425`) BEFORE `source_core_method_calls` (`:466`), so the
  declaration authority is reachable at issue time. The only existing
  field-identity lane (`ordinary_new_terminal_home.rs::initialized_integer_field`)
  is `i64`-only by contract.
- Provider provenance: `GeneratedBirthTriggerSourceV1(StoredFieldInitializer)`
  (`parser/source_authority/constructor_source.rs`) records each
  field-initializer member site onto the birth constructor relation;
  inside `birth` the generated `me.f = new ArrayBox()` is a real
  `FieldWrite` assignment whose value site resolves through
  `construction_source` in the birth ledger. The `new` is a raw `NewBox`
  in `birth` — correctly NOT an ordinary-new claim
  (`is_direct_local_initializer` requires `[Body, Initializer]`).
- Physical correspondence today:
  `record_named_array_allocation` (same-owner `new` site -> `ValueId`
  slot on `SelectedSourceCoreMethodCallV1`) -> `into_named_array_emission`
  -> `NamedArrayWriteEmissionPortV1` -> `record_write` ->
  `NamedArrayWriteMarkerV1{owner, allocation: ValueId, ..}` ->
  `validate_physical_marker` requires the `NewBox` in the SAME function
  (`named_array_obligation.rs:34-92`) and `validate_named_array_coverage`
  cross-checks bidirectionally (`emission.rs:107-170`).
- Consumption: `take_source_array_push` (`named_array.rs:88-131`) is
  site-exact and receiver-agnostic — `read_variable` resolves any local
  binding (a FieldGet-initialized alias already carries a `ValueId`);
  `CoreEffectPlan::NamedArrayPush` -> `ArrayElementWrite{Push,MethodCall}`
  (`effect_emission.rs:119-146`) + emission port are reusable unchanged.
- Runtime transport already exists: `nyash.array.push_hh`
  (`handle,val_any` -> `array_slot_append_any`) and
  `nyash.array.push_hi` (`handle,i64` -> `append_integer_raw`) in
  `crates/nyash_kernel/src/plugin/array_runtime_aliases.rs:44-50`.
- Escape hatch noted: the generic `MethodCall` lane
  (`effect_emission.rs:181-205` + `receiver_is_array_like`) can emit the
  identical instruction WITHOUT a marker — the named contract is what
  makes the write audited.

## Decision (accepted 2026-09-27)

```text
Decision: add a sibling residence contract — do not stretch
  NamedArrayConstructionRequirementV1. Claim `a.push(arg)` when the
  receiver alias's single initializer is a `me`-field read whose
  declaration is `ArrayBox`-typed, non-weak, and non-shadowed, then
  issue `ArrayIntegerAppend` (NamedArrayReceiver + I64Parameter +
  NoValue + MutatesShape + NamedArrayIntegerV1) bound to a
  NamedArrayFieldResidenceRequirementV1 carrying
  CanonicalFieldRefV1 + provider (birth owner + construction site).
  Provider = the field's generated-birth `new ArrayBox()` proven by the
  birth ledger's construction_source at the FieldWrite value site,
  gated by the StoredFieldInitializer trigger membership.
Source authority + canonical issuer: parser field declaration
  (UserBoxFieldDecl.declared_type_name == "ArrayBox", declaration
  ordinal -> CanonicalFieldRefV1 via instance_constructors) +
  GeneratedBirthTriggerSourceV1(StoredFieldInitializer) membership +
  the birth ledger's FieldWrite/construction_source provider proof.
  Issued inside issue_source_bound_core_method_calls_with_named_arrays_v1
  (signature extended with &instance_constructors and the batch for the
  cross-owner provider join) -> CoreMethodInstanceTargetIssuerV1::
  array_integer_append -> ResolverCoreMethodCallableContractIssuerV1::
  issue_named_array residence arm -> SelectedSourceCoreMethodCallV1.
Non-authority: I64ExpressionFactV1 (AST-walking result-representation
  family), ExactNumericValueFact (physical layer), variable_map/name
  reconstruction, the i64-only initialized_integer_field lane, MIR
  value types, and the deprecated VM route are not source authority.
  No physical-value inference decides the integer family.
Fail-fast boundary: once the alias initializer resolves to a `me`-field
  read of a declared ArrayBox field the claim is entered and every
  failure is a typed NamedArrayFieldResidenceIssueV1 propagated through
  SourceBoundCoreMethodTargetIssueV1 — foreign field owner, weak field,
  missing/foreign/mis-shaped provider, reassigned alias, value-position
  use, non-integer argument all reject before any physical write.
  Initializers that are not `me`-field reads keep today's silent
  omission so the generic lanes keep owning their own errors.
Smallest next slice: MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-S0 —
  residence requirement + bounded integer-source predicate +
  ArrayIntegerAppend schema/contract arm + FieldResidence allocation
  ref on the marker + birth-side provider emission table +
  map_named_allocations consumer binding + focused positive/negative
  tests below.
Non-claims: no production caller switch, no retirement, no Gate-1
  green, no VM parity, no direct `me.f.push` receiver (alias form
  only), no Text-on-residence arm, no mixed-element family on one
  residence, no ordinary-new eligibility change, B3 StringHelpers
  stays its own scope.
```

## Settled answers

1. **Residence/lifetime**: the array is field-resident. Source meaning:
   `local a = me.f` is a handle snapshot; the object lives as long as
   the `me` instance. `me.f = ...` stores elsewhere do not invalidate
   the alias (snapshot semantics) — the contract binds the alias
   binding and the field ref, not the field's current contents.
   `is_weak` fields reject (`WeakFieldResidence`): a weak field gives
   no residence guarantee.
2. **Provider selection**: sole provider = the declared field's
   generated-birth initializer `new ArrayBox()` — exactly one per
   `ArrayBox`-declared field by constructor-inventory coverage.
   Absent initializer -> `ProviderMissing`; initializer that is not a
   bare zero-arg `new ArrayBox()` -> `ProviderShape`; the `new` must
   resolve inside the owner box's own `birth` ledger
   (`ProviderForeign` otherwise).
3. **I64 input**: `I64Parameter` relation under a new
   `ArrayIntegerAppend` schema. Argument integer meaning is a bounded
   source predicate over the existing ledger inventory — `Integer`/
   `TypedInteger` literal; `Lexical(Local)` binding whose initializer
   and every `BindingRebind` RHS are integer-producing (inductive; the
   binding itself counts while being proven, so a loop-carried
   `i = i + 1` closes); a same-owner `I64ToCaller` contract call; or a
   `me`-field read of an integer-declared field. Anything else ->
   `IntegerSourceMissing`/`ForeignIntegerSource` typed rejection.
   A field-resident push with a Text argument rejects — the
   Text+residence arm is not claimed in this slice.
4. **NoValue result**: `push/1` stays `NoValue`, statement position
   only; the existing `source_site_inventory().contains_statement`
   (`ValueDemand`) and `no-value-source-call-used-as-value` consumers
   are reused unchanged.
5. **Failure class**: new `NamedArrayFieldResidenceIssueV1` enum
   (typed: `ForeignCall`, `UnsupportedReceiver`, `InitializerMissing`,
   `AmbiguousInitializer`, `ResidenceMissing`, `ForeignFieldOwner`,
   `ResidenceNotNamedArray`, `DeclarationCollision`, `WeakFieldResidence`,
   `ProviderMissing`, `ProviderShape`, `ProviderForeign`,
   `ReassignedReceiver`, `ValueDemand`, `ArgumentShape`,
   `IntegerSourceMissing`, `ForeignIntegerSource`) propagated via
   `SourceBoundCoreMethodTargetIssueV1`. Issuance failure is terminal —
   no generic retry, no silent omission inside the claim.
6. **Commit timing**: `ArrayElementWrite{Push,MethodCall}` emits at the
   call site in program order inside the loop body; each emission is a
   committed in-place append on the field-resident array through the
   aliased handle — no write-back, no field store, no deferred commit.
   Later reads (`me.free_stack.get(...)` in `allocate`) observe
   committed state. The emission port's `write-not-emitted` finish
   contract is unchanged.
7. **Retained transport + consumer**: instruction
   `MirInstruction::ArrayElementWrite{Push}` + marker ->
   `NamedAllocationConsumer::Array` C-frame binding on the BIRTH-side
   `NewBox` (provider site) -> kernel `nyash.array.push_hi` for the
   integer family (`push_hh` remains the any-payload transport already
   registered). Marker `allocation` becomes
   `NamedArrayAllocationRefV1::{LocalValue(ValueId) |
   FieldResidence{provider_owner, provider_site}}`; the provider's
   recorded `NewBox` dst is resolved module-level at validation, not
   re-inferred.
8. **Old-edge map (map only — no deletion permission)**:
   `named_array_method.rs` silent `continue` narrows: field-initializer
   aliases now produce contract rows; the silent set stays for
   non-field shapes. `loop_source_route_items.rs` boundary unchanged —
   rows exist so the four sites become covered items.
   `loop_body_lowering_associated_input.rs` generic array-like escape
   stops seeing these four sites (contract consulted first via
   `exact_source_statement_call`); the arm is retained for genuinely
   uncontracted receivers. `reject_unretained_module`,
   `checked-write-contract-unavailable`, `incomplete-consumption`,
   `named-array-call-shape` remain the outer guards.
9. **Ordinary-new prerequisite**: none for this membership — the
   field-initializer `new` stays a raw `NewBox` inside `birth` bound by
   the residence provider row; `is_direct_local_initializer` correctly
   stays exclusive. The outer `new HakoAllocPage(...)` admission is the
   D18-successor's scope (queued row 6), not this slice.

## Focused acceptance specification (for S0)

- Positive: `HakoAllocPage.seedBlocks/0` four sites compile to contract
  rows + markers + provider-bound NewBox + C-frame consumer; a minimal
  field-resident fixture (alias push `i`/`0`) reaches the retained
  typed write; alpha-renamed aliases; zero/one/multiple iterations;
  exact I64 values including an integer numerically equal to a live
  handle; integer argument through an `I64ToCaller` nested call.
- Negative: shadow `ArrayBox` declaration; foreign field owner
  (`other.f`); `i64`/untyped/missing field name residence; weak field;
  `field: ArrayBox` without `= new ArrayBox()` provider; non-`new`
  provider shape; reassigned alias; Text argument on the I64 family;
  push result used as a value; handle-typed argument; residual source
  row (`incomplete-consumption`); duplicate site consumption.
- Keep every existing Text-family rejection pin green; add
  family-selection proof: local `new` + Text stays `ArrayTextAppend`,
  field + I64 lands `ArrayIntegerAppend`.

## Non-claims

- No code issued under this card; no production switch, caller-zero,
  retirement, Gate-1 green, or whole-app success is claimed.
- Direct `me.f.push(x)` (unaliased) is not claimed — its receiver is
  `Other` and stays on the generic lane.
- `map_named_allocations`/metadata additions are bounded to the
  residence provider correspondence; no generic dynamic dispatch.

## Exit

- [x] Field residence/lifetime, provider, I64 input, NoValue result,
      failure class, and commit meaning settled through the named
      issuer family.
- [x] Retained source-to-physical correspondence named
      (provider site -> birth `NewBox` -> marker FieldResidence ref ->
      `NamedAllocationConsumer::Array`).
- [x] Selected old-edge map recorded; no deletion authorized.
- [x] Ordinary-new prerequisite classified: none for this membership;
      outer `new` admission stays the D18-successor scope.
- [x] One bounded next slice emitted:
      `MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-S0`.
