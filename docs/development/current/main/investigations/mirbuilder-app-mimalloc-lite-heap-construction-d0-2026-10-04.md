# mimalloc-lite heap construction D0 (`new HakoAllocHeap` contract)

Status: Decisions accepted (2026-10-05); SIZE-T0 selected, then A -> B -> C.
Owns the EXE lane's frontier after FORMAL-FORWARD-RESULT-S0
(`a92f003d8e`): the `artifact-source-unavailable` stop on
`MiWorkload.run/0` Body(0), where `local heap = new HakoAllocHeap()`
stays `RetainedUnavailable { progress: Installed }` because the
`HakoAllocHeap` construction plan is
`Err(ConstructionUnavailableV1::FieldContractUnsupported)`.
Parent bundle slice family: RESULT-NEW-OBJECT-ARGS (11) /
OBJECT-FIELD-RECEIVER-CALL (10) of
`mirbuilder-app-bundle-mimalloc-lite-d0-2026-09-30.md`; one layer up
the ownership tree from OBJECT-FORMAL-FIELD-STORE.
Scope: `MIRBUILDER-APP-MIMALLOC-LITE-HEAP-CONSTRUCTION-D0`
Related: `mirbuilder-app-mimalloc-lite-formal-forward-result-d0-2026-10-04.md`,
`mirbuilder-app-mimalloc-lite-object-formal-field-store-d0-2026-10-04.md`

## Census — the `FieldContractUnsupported` chain

`MiWorkload.run` (`apps/mimalloc-lite/main.hako:10-51`) opens with

```hako
local heap = new HakoAllocHeap()   // Body(0) — retained
local handles = new ArrayBox()     // Body(1) — Map lane, outside this card
```

`HakoAllocHeap.birth()` (`page_heap_box.hako:196-199`) declares no
handwritten stores — both fields initialize from declaration defaults:

```hako
small_page: HakoAllocPage = new HakoAllocPage(
    0, LayoutBox.class_size(0), LayoutBox.class_capacity(0))
medium_page: HakoAllocPage = new HakoAllocPage(
    1, LayoutBox.class_size(1), LayoutBox.class_capacity(1))

birth() {
    me.small_page.seedBlocks()
    me.medium_page.seedBlocks()
}
```

Parser normalization prepends `me.<field> = <default>` stores to every
birth, so the plan's effective body is two generated `New` stores plus
two method-call statements. `issue_construction_plan`
(`instance_construction.rs`) then hits **three independent edges**, all
of which must be solved — this is not an either/or frontier:

1. **Provider child is not `PlainI64NoHook`** (first encountered,
   `instance_construction.rs:360-369`). The provider-`new` arm admits a
   user-class child only when its `destruction_disposition` is
   `PlainI64NoHook`. `HakoAllocPage` owns four `ArrayBox` fields
   (`free_stack`, `block_used`, `use_counts`, `requested_sizes`) —
   `OwnedArrayFieldsNoHook` — so the provider arm rejects before
   argument inspection. Nested reclamation of an owned-field child is
   an ownership-semantics question: the existing
   `OwnedFieldChildV1`/`children` sealing proves direct owned field
   `new`s; a provider child brings its own children tree whose
   construction-fault reclaim ordering and discharge arm are
   undesigned.
2. **Provider `new` arguments are calls** (`instance_construction.rs:
   384-399`). `LayoutBox.class_size(0)` / `class_capacity(0)` are
   static-box method calls; the sealed trivial-argument grammar admits
   only `Integer`/`Bool` literals. A call-typed provider argument
   needs an authority — the static call's result class is
   `i64`-shaped but the claim vocabulary for "this call result is a
   sealed trivial actual" does not exist in the provider arm.
3. **Birth body is not store-only** (`instance_construction.rs:
   245-247`). `me.small_page.seedBlocks()` /
   `me.medium_page.seedBlocks()` are non-`Assignment` statements —
   `BodyCoverageUnsupported`. Birth-time method calls run against a
   partially-initialized `me`: `seedBlocks` reads `me.free_stack`
   (already stored by the generated store) and `me.capacity` (stored
   by `birth`'s own `me.free_top = capacity`... — actually
   `HakoAllocPage.birth` stores `me.capacity`; the *page's own* birth
   ran at the provider `new`, not here). Admitting call statements in
   birth needs a statement-order/partial-init contract: which `me`
   fields are proven stored before the call, and what receiver the
   call legally observes.

Supporting facts already established:

- `run`'s `home_prefix` for Body(0) is `Ok` — the block is claimable
  the moment the construction plan exists; no prefix work is owed.
- `new ArrayBox()` (Body(1)) is not an ordinary row — the Map lane
  owns it; it does not appear in the retained-ordinary check.
- `HakoAllocHeap` is unreachable from `main` only in the weak sense —
  `main` → `workload.run()` → `new HakoAllocHeap()` — this site *is*
  reachable and its raw-lane emission is exactly what
  `artifact-source-unavailable` rejects. There is no skip path.
- The MIR JSON lane keeps its designed negative `unsupported
  terminator Invoke` — this frontier is EXE/artifact-lane only.

## Why the residual still freezes the EXE lane

`RetainedUnavailable` rows keep their raw-lane emission: the object is
physically created, but no completed local commit owns the lifecycle
site. `validate_finalized_child_functions` in artifact mode rejects any
owner carrying a retained ordinary row
(`root_validation.rs:82-98`, now with owner/symbol/site/retained-row
diagnostics). `MiWorkload.run` is a selected owner with such a row —
the freeze is the designed boundary, not a bug.

## Decision space

All three edges gate the same site; the ordering below is by
dependency, not preference — every gap must land before
`new HakoAllocHeap()` emits.

- **(A) Nested owned-field provider child.** Admit a provider `new`
  whose child disposition is `OwnedArrayFieldsNoHook` (or, more
  generally, any owned-field disposition whose own construction plan
  is Ok). Requires: recursive child-plan resolution inside
  `issue_construction_plan`, a discharge arm for the child's owned
  children inside the parent's construction-fault reclaim
  (`ReclaimUnpublishedOriginV1.children` already carries
  `OwnedFieldChildV1` — the question is whether a *nested* tree
  composes or needs a new arm), and the destruction contract that the
  child's ArrayBox fields are released correctly on unwind.
- **(B) Call-typed provider `new` argument.** `LayoutBox.class_size(0)`
  is a static `LayoutBox` method returning `usize`. Candidate
  authorities: (i) a sealed call-argument row naming "this exact
  static call result is the actual" — needs a result contract for the
  static callee (annotating `class_size`/`class_capacity` with
  `: i64`/`: usize` would also exercise the declared-result-contract
  lane); (ii) constant folding — declined: evaluation is not
  observation. (i) is the honest direction but adds a second new
  contract surface inside the provider arm.
- **(C) Non-store birth statements.** `seedBlocks` populates the
  freshly-built `ArrayBox` fields — it is a mutating call on an
  already-stored field. Candidate contracts: (i) post-store method
  calls on `me.<field>` receivers, gated on the field's earlier store
  in the same birth (partial-init ordering proof); (ii) restricting to
  calls whose receiver field is a provider/`new`-stored object —
  `seedBlocks` fits. Either arm must pin "the call observes only
  fields proven stored" — a new intra-birth ordering relation.

Declined/closed:

- `.hako` workaround (literal sizes, moving `seedBlocks` calls out of
  birth) — forbidden by policy; the app source is the contract.
- Skipping `run` or relaxing the artifact-mode retained check — the
  check is the designed boundary.
- Treating `ArrayBox` field ownership as trivial — ArrayBox fields
  carry real storage; the owned-field discharge arm exists precisely
  because they are not trivial.

## Historical smallest-slice proposal (superseded by Decision below)

`MIRBUILDER-APP-MIMALLOC-LITE-HEAP-CONSTRUCTION-S0` — one bounded arm,
suggested order by dependency: (C) cannot help before (A)+(B) since
the generated stores still fail; (A) alone does not help since (B)
rejects next; (B) alone fails at (A). Honest decomposition:

- First candidate: **(A) + (B) composite** — the two provider-arm
  edges share one acceptance boundary (the provider `new` inside a
  field store); without both, no provider shape validates.
- Then **(C)** — birth-statement grammar, separate acceptance class.

Or a different split if the owner prefers: (C) first as an
independent grammar slice still leaves `new HakoAllocHeap` retained
(stores fail first), so (C) cannot be verified end-to-end alone —
arguing for (A)+(B) first.

Non-claims:

- No EXE PASS, no production switch, no new receipt family —
  every extension lands inside `issue_construction_plan`'s existing
  arms or their existing sealed products.
- `new ArrayBox()`, `seedBlocks` internals, and `MiWorkload`'s other
  statements are outside this frontier; their lanes are separate.
- No claim that all three gaps share one slice — the split above is
  a proposal awaiting the owner Decision.

## Accepted Decision (2026-10-05)

Decision: keep A, B and C as separate semantic slices, in that order.
Sharing a provider-store boundary does not make reclamation and argument
evaluation one responsibility. A has an executable witness with literal
arguments and an owned-array child; B has one with a plain child and exact
static-call arguments. C has one with already-supported stores followed by
an exact field-receiver call. These are focused compiler fixtures, not
rewrites or substitutions for the production mimalloc-lite app.

All three remain required for the original heap construction. A focused
success is not whole-app acceptance. The MIR JSON negative stays at its
designed `unsupported terminator Invoke` boundary.

### Ordered tasks

All row names have prefix `MIRBUILDER-APP-MIMALLOC-LITE-`.

Selected execution row:
`MIRBUILDER-APP-MIMALLOC-LITE-HEAP-BIRTH-FIELD-CALL-S0`.
Next semantic row:
none — C is the final construction row on this card; the unchanged
heap/app frontier probe follows it.

1. **HEAP-CONSTRUCTION-SIZE-T0** (selected, BoxShape only). Split the
   provider emission/validation responsibilities out of
   `src/mir/builder/normal_callable_construction_state.rs` (971 lines at
   decision time) into private responsibility children. Preserve source
   predicates, evaluation order, fault landings, verification and visibility
   boundary. Bring touched parents/children below 800 without compression.
   `ordinary_new_coseal_issue.rs` is 797 lines: create headroom by a separate
   behavior-preserving extraction if the next issuer change needs it. Keep
   structural commits separate from A/B/C. Run existing constructor/provider
   positive/negative tests, touched-file formatting, scope/pointer guards
   and diff check; record pre-existing unrelated guard debt truthfully.
2. **HEAP-OWNED-PROVIDER-S0** (A). Extend the existing provider-owned
   residence to a child whose destruction is `OwnedArrayFieldsNoHook`,
   construction is covered and every owned ArrayBox residence is sealed.
   Bound this first slice to parent -> user-object child -> ArrayBox leaves;
   retain plain children. Further user-object nesting, hooks, cycles,
   unsupported providers and unsealed children remain unavailable.
3. **HEAP-PROVIDER-CALL-ARG-S0** (B). Admit exact qualified static-call
   scalar results as provider arguments through the existing source-target
   and result authority. Do not fold, add source annotations, or treat a
   scalar result as a non-faulting/non-effectful call.
4. **HEAP-BIRTH-FIELD-CALL-S0** (C). Admit exact sequential, discarded-result
   calls on an already-stored, fully constructed owned user-object field.
   Require canonical target, borrowed receiver lifetime and covered callee
   completion/effects. Then run the unchanged original heap/app frontier
   probe; report the next stop rather than predicting EXE PASS.

### Source authority + canonical issuer

**A:** constructor source identity and declaration-order field stores from
the verified constructor batch; canonical object definitions and destruction
dispositions from `instance_constructor_semantic/object_definition.rs`;
exact birth-side residences and child coverage from the existing co-seal.
`issue_construction_plan` remains the construction issuer;
`owned_field_children_of` and the existing ledger remain the child-obligation
owner. Extend their existing products, not a parallel recursive analyzer or
new receipt family. Unresolved/cyclic dependencies issue no plan.

Cleanup must cover Normal end as well as construction Fault. A child-birth
Fault releases only its Normal-committed child fields, then reclaims its
unpublished outer storage. A completed child not yet stored in the parent
is an in-flight owner; a parent-store Fault releases that child and its
ArrayBox fields exactly once. After a Normal store the parent owns it;
a later store/birth-call Fault releases the parent's committed residences,
including those nested fields, before parent storage reclamation. Preserve
the existing declaration-order cleanup convention and fault-frame policy.
Unstored slots need an explicit empty/initialization-state guarantee; neither
the final field inventory nor a zero-looking payload proves initialization.

The selected construction physical owner is
`normal_callable_construction_state` (after T0 extraction), with existing
local-commit cleanup and publication consumers. Current V4
`object_field_release` in `hako_llvmc_ffi_lifecycle_v4_emit.inc` calls
`home_release_plain_i64_v1`: widening only the issuer would leak nested
fields. A must carry the source-sealed nested cleanup obligation through
publication and this physical consumer; any wire change updates its owning
reference before code. Layout/type IDs alone do not issue child ownership.

**B:** borrow `VerifiedStaticImportAliasViewV1`, the whole-source static
target inventory and `VerifiedSameModuleCallableResultCatalogV1` through
the existing qualified-static-call membership owner. Seal exact caller,
new site, argument ordinal, call expression site, target and required scalar
argument obligations inside the existing construction argument product.
The existing call physical owner evaluates each source argument once, in
source order; only its Normal result is the Birth actual. Keep the language's
existing allocation/argument order, and discharge any already-acquired
unpublished storage on argument Fault. MIR value types, method names and
result annotation alone are not authority. Missing result/argument proof,
wrong result class, nested unsupported argument shapes and site drift reject.

**C:** the constructor plan owns statement order and the initialized-field
set. Only a prior Normal-committed store with completed child birth lends
the exact receiver; the child callee's existing verified callable contract
owns its body, including `seedBlocks` internals. The call borrows the child,
does not transfer it or lend the partially initialized parent `me`, and its
Fault feeds the same source-backed construction cleanup. A later store cannot
justify an earlier call. Receiver-before-store, unavailable/escaping callee,
unknown target, transfer/re-store and unsupported control flow stay closed.

### Acceptance and non-claims

A: execute a literal-argument nested-array fixture through publication/C;
verify Normal end and injected child allocation/store/birth Fault,
parent-store Fault and later-parent Fault. Observe each acquired lease
released once, no unacquired-slot release and no outer-only reclaim over
live children. Reject missing/forged child descriptors, wrong class/field,
duplicate cleanup and unsupported deeper/cyclic ownership.

B: execute a plain-child fixture with qualified static-call arguments;
verify result-to-actual identity, argument order/single evaluation and Fault
cleanup. Reject forged result/target/ordinal/site rows and wrong scalar kind.

C: execute stored-child mutation calls in order, then the unchanged
`new HakoAllocHeap()` probe. Verify second-call Fault discharges both stored
children, and reject receiver-before-store or unproved completion/lifetime.
If a callee's existing lane is uncovered, record that separate frontier;
do not infer coverage from this construction contract.

Each slice owns its focused positive/negative checks, publication/physical
checks and baseline comparison. No fallback, mimalloc-specific name branch,
source workaround, provider activation, whole-app completion or arbitrary
recursive destruction claim is authorized by this Decision.

## Landed: HEAP-CONSTRUCTION-SIZE-T0 (`e366f4c521`)

`src/mir/builder/normal_callable_construction_state.rs` (971 lines) split
into private responsibility children — pure code motion, no predicate,
evaluation-order, fault-landing, verification or visibility change:

- `normal_callable_construction_state/emission.rs` (379 lines):
  `emit_construction_store` plus `jump_landing` — literal/parameter value
  emission, the provider `new` chain (intrinsic `ArrayBox` and user-class
  `NewBox`/`birth_call`/`ReclaimUnpublished`/`ObjectFieldSet`+`HomeRelease`
  discharge) and the checked field-store invoke.
- `normal_callable_construction_state/validation.rs` (297 lines):
  `RetainedConstructionValidation` artifact checks, `validate_bindings`
  (emitted-shape census: invoke count, fault-return shape, per-store
  terminator/landing match) plus `lands_on`.
- Parent (321 lines): state/progress/transport types, `install_construction`,
  `take_construction_store`, completion/finalize/transfer entry points and
  the shared `construction-store/*` fault tag. The test file now imports
  its own MIR types instead of reusing parent imports.

Diff check: moved code is byte-verbatim against HEAD except the
`impl`/`pub(super)` wrappers needed by the new module boundary; file
doc comments record each child's responsibility.

Evidence:
- `cargo test --lib 'construction::'` — 2/2 (drift/residual and artifact
  transport pins unchanged).
- `cargo test --lib normal_callable_semantic_package` — 591 passed,
  3 failed — identical to the recorded baseline
  (`birth_receiver_non_escape_rejects_unproven_uses_before_row_publication`,
  `main_static_child_port_consumes_all_role_rows_once`,
  `qualified_call_map_argument_reaches_the_named_capability_boundary`).
- Production lanes (fresh debug bin): MIR JSON holds the designed
  `unsupported terminator Invoke` negative; EXE holds the same
  `artifact-source-unavailable` stop at `MiWorkload.run/0` Body(0) —
  `construction=Err(FieldContractUnsupported)` unchanged.
- `rustfmt --check` on the touched files shows only drift already present
  in HEAD's file carried verbatim by the move; no new deviation.
- Scope guard: SIZE-T0 pins added (emission/validation/parent symbols and
  all three files in the <800 watch); guard stops on the recorded baseline
  `brand_catalog_tests.rs=961`.
- `ordinary_new_coseal_issue.rs` remains 797 lines; the conditional
  headroom extraction was not needed by this structural slice and stays
  owned by the first semantic slice that needs it.

Non-claims: no admitted construction shape changed; the app frontier is
unchanged at `new HakoAllocHeap()`; no progress toward A/B/C semantics;
structural commit kept separate.

## Landed: HEAP-OWNED-PROVIDER-S0 (A)

Provider `new` admits a canonical user-object child whose destruction is
`OwnedArrayFieldsNoHook`, bounded at parent -> user-object child ->
`ArrayBox` leaves. The existing construction plan plus owned-field
ledger remain the proof owners; nothing mints a parallel analyzer.

- Plan (`instance_construction.rs`): the child bound widens to
  `{PlainI64NoHook, OwnedArrayFieldsNoHook}`; the store records the
  child's declaration-order `ArrayBox` ordinals as `owned_fields`.
- Seal (`ordinary_new_coseal_issue_source_owned_children.rs`, new 272-line
  child of `source_claims`): `seal_provider_owned_children_v1` runs inside
  `prepare_source_claims` against the same residence ledger a `new`
  claim would use, because a provider site mints no `local`-bound claim.
  `owned_field_children_of` moves here unchanged in role; its
  `OwnedArrayFieldsNoHook` arm admits exactly one nested level — the
  child's own `ArrayBox` residences must all seal.
- Emission (`emission.rs`): the provider store rejects a missing,
  unproven or drifted ledger row (`provider-children-*`), and the
  reclaim/discharge cleanup chains emit one
  `OwnedFieldResidenceRelease` per sealed residence newest-first before
  `ReclaimUnpublished`/`HomeRelease`.
- Validation (`validation.rs`): `residence_chain` walks each cleanup
  chain link-by-link (exact field order, base, frame, distinct
  landings); the invoke census adds `2 * nested_releases`.
- Wire (`physical_abi.rs`, `physical_program_json.rs`): each layout row
  publishes `owned_residences`, declaration-ordinals derived from the
  same field authority as the disposition; the physical validator
  requires the key and admits only strictly ascending in-range ordinals.
- Physical emit (`hako_llvmc_ffi_lifecycle_v4_emit.inc`):
  `object_field_release` reads the child layout's marks, walks each
  marked slot newest-first, releases the live residence, then releases
  the child home. `object_field_set` no longer releases the child
  inline on fault — the MIR discharge chain is the sole owner of
  in-flight cleanup.
- File split: the seal move keeps `ordinary_new_coseal_issue_source.rs`
  at 664 and `ordinary_new_coseal_issue.rs` at 797 under the 800
  boundary.

Evidence:
- `cargo test --lib construction` 29/29; focused family
  (owned/provider/physical_json/named_array filters) green;
  `provider` filter reproduces the recorded baseline flake
  `source_stringbox_literal_uses_source_anchor_admission` on HEAD too.
- Positive: `ordinary_new_owned_nested_array_child_seals` (seal),
  `provider_owned_array_child_reaches_artifact_lane` (chain census on
  the artifact lane, optimize on/off),
  `installed_owned_array_child_publishes_residence_marks` (wire marks +
  release naming).
- Negative: deeper user-object nesting, self-reference, and unproven
  nested residences stay `unproven`; foreign ledger rows reject as
  `provider-children-drift`; missing rows as
  `provider-children-missing`.
- C: `published_lifecycle_physical_parser_preartifact_test`,
  `published_lifecycle_v4_nested_call_test`,
  `published_lifecycle_v4_receiver_identity_test` PASS, including a new
  `object_field_release` + `owned_residences:[0]` emit fixture and
  five strict-layout rejections.
- Scope guard: A pins added; guard stops on the recorded baseline
  `brand_catalog_tests.rs=1010` (was 961 at HEAD, already over).

Non-claims: the unchanged `new HakoAllocHeap()` still stops at
`FieldContractUnsupported` — the provider arm now reaches its
call-argument check (B's edge) and `seedBlocks()` (C's edge); VM lane
terminal `birth-global-legacy-stopped` is unchanged. No whole-app
success is claimed.

## Landed: HEAP-PROVIDER-CALL-ARG-S0 (B)

A provider `new` argument admits a proven `Alias.m(..)` qualified static
call — `LayoutBox.class_size(0)` — with the result routed through the
existing `InvokeNormalResult` lane into the provider `birth_call` actuals.
One claim, one consumption boundary, one emitted invoke per sealed row.

- Claim (`qualified_static_call_claim.rs`): each `(caller, site)` index
  row now pairs the `ExactI64` claim with the sealed `StaticBoxMethod`
  target from one seal — `claim_target` is the construction issuer's
  membership lookup; claim and target are never two lookups.
- Seal (`instance_construction.rs`, `ordinary_new_arguments.rs`): the
  provider arm admits a `MethodCall` actual only under
  `provider_static_claims` — `QualifiedUnbound` receiver, exact
  `(caller, site)` claim row, arity match, Integer/Bool literal actuals,
  i64 evidence at every required-i64 ordinal — and seals
  `OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall { target,
  arguments }`. `ConstructionStoreRhsV1::ProviderConstruction` carries
  the birth caller key. The issuer mints the claim index before the
  constructor batch so the plan reads it as an AST-free fact.
- Transport (`child_lowering_impl.rs`,
  `source_call_publication.rs`): the port takes each sealed
  `(caller, site)` publication handoff through the sole module boundary
  and installs it on the callable ledger; `take_provider_static_
  result_publication` consumes exactly one row per argument site —
  missing, target-drifted, duplicated or residual rows all fail closed.
- Emission (`emission.rs`, `normal_callable_construction_state.rs`):
  the reclaim chain is built before argument evaluation so each
  `Call{Global, I64}` invoke faults onto the same cleanup the birth
  call owns — no new ownership mechanism. One invoke per sealed row in
  source order; each normal landing projects through
  `InvokeNormalResult`, and `ProviderCallArgEmission` records the exact
  invoke/landing/value triple for validation.
- Validation (`validation.rs`, `emission_validation.rs`): the census
  adds one invoke per `call_args` row; the chain walk requires `entry`
  -> each invoke -> its landing -> `birth_call` with the shared reclaim
  fault landing; `InvokeNormalResult` pairs are checked against the
  recorded triples. The local-commit lane re-verifies target, literal
  actuals and projection against the sealed row.
- Selected lane (`selected/arguments.rs`): a `QualifiedStaticCall` row
  reaching `local x = new` materialization is issuer drift —
  `argument-kind-provider-only`, never an admitted actual.
- Physical (`physical_program_projection.rs`,
  `compiled_entry_contract.rs`, `physical_abi.rs`): birth units seed
  the ordinary-call census — a provider argument is a sealed `ExactI64`
  call row, so its callee joins membership through the same proof and
  each birth function carries its own call set; a birth caller with a
  non-i64 call result is `compiled-entry-birth-call-result`.
  `scalar_actual_kind` tags the projected i64 actual 1.
- V4 (`hako_llvmc_ffi_lifecycle_v4_indexed_flow.inc`,
  `hako_llvmc_ffi_lifecycle_v4_emit.inc`): `ordinary_call` admits a
  birth caller only for `result == "i64"` (`birth_i64`) — map and
  handle results stay rejected. `field_set`,
  `field_residence_release` and `object_field_set` admit the birth
  caller under the same lifecycle proofs (stamped receiver origin -2,
  in-flight lease origin >= 0). Nested i64 calls get a caller-local
  `%call_out<block>` slot; `invoke_normal_result` loads from it.

Evidence:
- `provider_static_call_argument_seals_target_and_literal_actuals`,
  `provider_static_call_arguments_seal_in_source_order`,
  `provider_static_call_argument_stays_fail_closed` (new
  `provider_static_call_argument_tests.rs`): seal proves target, caller
  key and source order; unresolved method, non-static receiver,
  non-i64 result, compound inner arg, Bool at a required-i64 ordinal
  and an unqualified call all stay `FieldContractUnsupported`.
- `provider_static_call_argument_serializes_inside_birth_unit`
  (physical JSON): exactly one `ordinary_call` inside `Parent.birth/0`
  targeting `LayoutBox.class_size/1` with `result == "i64"`, the
  `invoke_normal_result` projection in the normal landing, and the
  i64-tagged actual on the `birth_call`.
- `cargo test --lib construction` 29/29, `provider` 207/207,
  `physical_program_json` 26/26, `qualified_static_call_claim` 6/6,
  `ordinary_new_emission_validation` 2/2 — no new failures; the
  recorded baseline set is unchanged.
- C: `published_lifecycle_v4_nested_call_test`,
  `published_rows_preartifact_test`,
  `published_lifecycle_physical_parser_preartifact_test` PASS —
  including a positive birth-caller `ordinary_call` fixture and a
  map-result birth-caller variant rejected at the physical parser
  boundary. A real emitted wire
  (`hako-provider-static-arg-wire.json`) compiles through V4 with
  rc=0.
- Scope guard: B pins added; guard stops on the recorded baseline
  `brand_catalog_tests.rs=1010` (was 961 at HEAD, already over).

Non-claims: the unchanged `apps/mimalloc-lite` `--emit-exe` probe now
stops at `construction=Err(BodyCoverageUnsupported)` at
`MiWorkload.run/0` — the provider `new` arguments seal and the plan
reaches the `seedBlocks()` statements (C's edge); the VM lane still
stops at `birth-global-legacy-stopped`. No fault-suppression,
annotation, fold or whole-app claim is made; a Birth-unit call outside
the qualified-static provider-argument lane issues no claim and stays
closed.

## Test-owner motion: HEAP-TEST-OWNER-SIZE-T1

Receiver-field/owned-child brand tests and composite-result/owned-residence
physical JSON tests moved verbatim into private included fragments. Parent
helpers, test-module identity, names and predicates are unchanged. Against
the preserved C WIP: brand 49 passed / 1 recorded ReceiverNonEscape baseline;
physical JSON 25/25. Before/after complete test inventory matched (8582).
All six selected test files are below 800 lines. Moved-owner guard pointers
match the new paths; later C scope pins reach the unchanged HEAD size debt
`normal_default_root_catalog_lifecycle_tests.rs=1351`. Full guard is not PASS.
Evidence: `/tmp/hako-goal-T1-{brand,physical}-tests.log` and byte-motion proof
`/tmp/hako-goal-test-owner-motion-proof.json`. C runtime acceptance stays open.

### Guard continuation (T1 structural series)

The existing dominated-add-through-size-check tail moved verbatim into
a private sourced lifecycle fragment. The public guard is the sole entry;
shared variables and evaluation order remain unchanged. Shell syntax passed.
Both shell files are below 800 lines. Current C semantic pins are separate;
full guard still reports unchanged HEAD root-catalog test size debt (1351).
No source/wire/runtime acceptance changed in this structural series.

## Selected: MIRBUILDER-APP-MIMALLOC-LITE-HEAP-LOCAL-ARRAY-ISSUER-SIZE-T0

Required structural series before ordinary helper local Array lifetime wiring:
co-seal issuer starts at 797 lines. Part one moves the exact App Main identity
query into its existing private source-claims child; part two moves the sealed
Nullable receiver query there. Predicates, errors, ordering, borrowed inputs and
authority stay unchanged. No Array admission is issued. Code/test evidence uses
the combined structural tree and explicit query-motion comparison; existing
C/S1/audit WIP is preserved outside these commits. Full provider Fault matrix,
unchanged app probes and C/S1 closeout remain open. No app/goal PASS is claimed.

Structural evidence: package 600 passed / the same 3 baseline failures
(`/tmp/hako-goal-array-size-package-tests.log`); test inventory 8588 unchanged.
Both query expressions equal their prior bodies after whitespace normalization
(`/tmp/hako-local-array-size-query-motion-proof.json`). Scope semantic pins pass;
full guard stops at unchanged HEAD root-catalog size debt (1351). Pointer and
selected diff checks pass. These results do not close C/S1 or its runtime matrix.

## Decision: MIRBUILDER-ORDINARY-I64-LOCAL-ARRAY-LIFETIME-D0

Issuer BoxShape closed in two commits: App Main query `ba4c248d29`, sealed
Nullable query in this commit. Parent/private child are 778/706, below 800.
Package 600 pass/3 unchanged baseline; child-call 4/4; inventory 8588 unchanged.
Query-motion proof, pointer and selected diff checks pass; full scope guard
reaches unchanged root-catalog size debt 1351. No semantic admission changed.

Source prerequisite for S1's unchanged provider matrix: qualified StaticBoxMethod
helper with sealed I64 result, straight-line zero-arg builtin ArrayBox local and
explicit literal Return. Existing callable ledger seals initializer owner/site/
binding, Core membership, collision absence, no rebind/escape and exact exit.
Source authority/issuer: existing co-seal `batch.with_lowering_input` and
`home_new_prefix_scan` local observation; add Array acquisition to the existing
source flow and `local_commits` enum. Reuse completed-local installation and
existing RootHome exit; do not borrow Script proof or invent a second program
plan/receipt. Resolve the closed cleanup-step mapping for nonfaulting Array
release (current origin/end plan only store InvokeOperation) before effects.
Physical new consumes the sealed exact site before raw New fallback and emits
checked acquisition/projection; Normal exit releases its live local, acquisition
Fault releases no unacquired local and forwards borrowed caller frame. Verify
local copy, acquisition, release and Fault terminal in the existing finalizer.
Non-authority: raw NewBox/MIR types, runtime handle, method/name matching,
Script/native single-function proof or an opcode whitelist. Negative fences:
collision/arguments/rebind/escape; missing/duplicate/foreign/pre-acquisition
release, wrong copy, bypassed release and raw New reentry. Retain unselected
Array consumers; no retry. After source acceptance, join existing indexed ABI
lease consumer and preserve Script/native/loop-writer boundaries. Both optimize
variants/full provider allocation/argument/Birth/store Fault matrix remain owed.
D0 resolves exact source-arm/cleanup/finalizer mapping; S1/C remain uncommitted
and open. No app PASS, production promotion or goal completion is claimed.
