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
`MIRBUILDER-APP-MIMALLOC-LITE-HEAP-CONSTRUCTION-SIZE-T0`.
Next semantic row:
`MIRBUILDER-APP-MIMALLOC-LITE-HEAP-OWNED-PROVIDER-S0`.

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

## Landed: HEAP-CONSTRUCTION-SIZE-T0 (commit: pending)

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
