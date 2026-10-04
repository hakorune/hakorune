# mimalloc-lite heap construction D0 (`new HakoAllocHeap` contract)

Status: design stop — residual census recorded, decision space below.
Owns the EXE lane's frontier after FORMAL-FORWARD-RESULT-S0
(`<commit>`): the `artifact-source-unavailable` stop on
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

## Smallest next slice (proposal, needs owner Decision)

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
