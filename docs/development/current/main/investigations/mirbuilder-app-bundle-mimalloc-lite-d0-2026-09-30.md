# MirBuilder app bundle — mimalloc-lite completion D0 (2026-09-30)

Parent card: `mirbuilder-gate1-callable-loop-string-indexof-s0-2026-09-27.md`
(the Gate-1 thinning ledger). This card carries the one-app completion
bundle for `mimalloc_lite_exe` from the fixed `real-apps-exe-boundary`
suite.

## Decision — MIRBUILDER-APP-BUNDLE-MIMALLOC-LITE-D0 (accepted)

Decision:
  `mimalloc_lite_exe` becomes one completion bundle: ordered internal
  slices, each with its own D0/S0 + positive/negative/Focused-gate
  verification; the app closes only when `mimalloc_lite_exe` passes
  end-to-end (MIR-JSON negative pin re-worded for the drifted terminal +
  pure-first EXE parity run).

Source authority + canonical issuer:
  Empirical probe census (fail-closed ladder terminals) +
  `ObjectDestructionDispositionV1` (`object_definition.rs`),
  `issue_construction_plan` (`instance_construction.rs`), the generated
  `CORE_METHOD_CONTRACT_ROWS_V2` manifest, and the sealed resolver facts
  already used by the prefix/home machinery. No MIR/runtime inference.

Non-authority:
  The 6-FAIL count is not a task count; VM-route behavior is not
  evidence for the EXE lifecycle lane; a passing focused unit test is
  not a production claim.

Fail-fast boundary:
  Every slice keeps `RetainedUnavailable`/`PrefixNotCovered` truthful —
  no permissive arms, no fallback emission. Unsupported shapes keep
  their designed terminal.

Probe census (post-`1eba1bcb11`, `--emit-exe --backend mir pure-first`,
minimal drivers — each isolates one family):

| probe | shape | first terminal |
| --- | --- | --- |
| `new Page{v:i64}` + `return 0` | baseline | compiles (nyrt gate) |
| `v: usize` field | usize scalar field | `artifact-source-unavailable` |
| `extra: usize = 0` default | usize + default | `artifact-source-unavailable` |
| `items: ArrayBox = new ArrayBox()` | container field | `artifact-source-unavailable` |
| `c: Child = new Child(1)` field-init | object field + user `new` init | `artifact-source-unavailable` |
| `new HakoAllocPage(0,8,4)` (real `using`) | usize+ArrayBox fields | `artifact-unowned-lifecycle-site` |
| `LayoutBox.class_size(0)` | static-box call | `artifact-root-completion-unavailable` |
| `c.get(): i64` no-arg | instance call | compiles (existing claim) |
| `c.add(5)` scalar arg | argumented call | `artifact-source-unavailable` |
| `new Outer(i)` object arg | object argument | `artifact-source-unavailable` |
| `local s = i.v` | `handle.<field>` read | `artifact-source-unavailable` |
| `return false/true` in unused method | Bool literals | compiles (needs reachable-fn recheck) |

Claim internals for the `usize` probe (pending-claims census):
`home_prefix=Ok`, `construction=Ok`, `argument_rows=Ok`,
`destruction=Unavailable(FieldType)` — the only failing component is
`object_definition.rs`'s closed `declared_type_name == "i64"` profile.
The claim's object must end its Home at every exit; a non-`i64` field
makes that unprovable today, so the row stays `RetainedUnavailable`.

Bundle slice inventory (each becomes its own bounded D0/S0; order is
the dependency order, not a strict serialization):

1. `DESTRUCTION-SCALAR-FIELD` — admit integer-scalar declared field
   types (`usize` etc., via `classify_numeric_type_name` authority) as
   trivially releasable alongside `i64`. Design Q: extend
   `PlainI64NoHook` or rename disposition; verify the physical release
   path treats `usize` identically. This is the universal first gate —
   every allocator box (`HakoAllocHandle.requested_size`,
   `HakoAllocPage`'s usize fields) trips `FieldType`.
2. `DESTRUCTION-ARRAYBOX-FIELD` — owned `ArrayBox` fields (4× on
   `HakoAllocPage`): child release plan at object end.
3. `DESTRUCTION-OBJECT-FIELD` — owned user-object fields
   (`HakoAllocHeap.small_page`/`medium_page`): nested release.
4. `FIELD-INIT-USER-NEW` — `small_page: HakoAllocPage = new
   HakoAllocPage(LayoutBox.class_size(0), …)`: the
   `ProviderConstruction` store arm admits builtin zero-arg `new` only;
   user-class field-init `new` with call arguments needs its own
   construction row. (Today → `FieldContractUnsupported`.)
5. `STATIC-CALL-CLAIM` — `LayoutBox.class_size/class_id/class_capacity`
   static-box calls: `artifact-root-completion-unavailable`.
6. `INSTANCE-CALL-SCALAR-ARG` — `heap.allocate(8)`, `heap.release(h)`:
   argumented instance calls (0-arg i64 already compiles).
7. `CALL-RESULT-ARG-POSITION` — `handles.push(heap.allocate(8))`: a
   live Invoke inside a core-method argument.
8. `HANDLE-FIELD-READ` — `handle.block_id`, `heap.small_page`,
   `small.alloc_count`: `handle.<field>` reads.
9. `OPAQUE-SCALAR-POSITION` — `requested_size` in `.set` args and
   `requested_bytes + requested_size` (allocate Body(8)+).
10. `OBJECT-FIELD-RECEIVER-CALL` — `me.small_page.seedBlocks()`,
    `me.small_page.allocate(size)`: `me.<ObjectField>.m(..)` (the
    ArrayBox lane of the nested-receiver S0 does not cover object
    fields — needs callee-claim authority, not the core manifest).
11. `RESULT-NEW-OBJECT-ARGS` — `return new HakoAllocHandle(me.page_id,
    block_id, requested_size)`: `me.<i64 field>` arg partially covered;
    BoundValue/OpaqueHandle arg positions remain.
12. `FORWARDED-RESULT` — `return me.small_page.allocate(size)`,
    `return same`, nullable `Handle`/`NullableHandle` results.
13. `SEEDBLOCKS-LOOP-ALIAS` — `local free_stack = me.free_stack` +
    `loop(i < capacity) { free_stack.push(i) }`: container-field alias
    locals + calls inside loop bodies (callable-loop domain).
14. `MIR-JSON-PIN-REWORD` — the negative leg now freezes
    `emission-binding-drift` (drifted inside the fail-closed ladder);
    re-pin when the app approaches.

Normal/Fault verification per slice: positive + negative pin tests in
`brand_catalog_*_tests.rs`, the `page_heap_fixture_result_claim_census`
frontier pin, official quick-profile serial, and the 3× emit-exe smoke.
Production switch + legacy retirement are assessed when the app passes.

Non-claims:
  No promise on ordering beyond "destruction admission is empirically
  first"; slices may merge/split at their own D0s. No claim that
  unused-by-app methods (`realloc*`, `allocateResult`, `isLiveHandle`,
  `resizeInPlace`, `HakoAllocHandleResult` births) are required — the
  reachable-set boundary is a per-slice question. Bool literals are
  provisionally unblocked (probe_s) pending a reachable-function
  recheck.

Next: `MIRBUILDER-APP-MIMALLOC-LITE-DESTRUCTION-SCALAR-FIELD-D0` —
bundle slice 1: admit integer-scalar declared field types to the
destruction disposition (the `object_definition.rs` `FieldType` gate);
empirically the first-order blocker for every allocator box claim.

## Decision — MIRBUILDER-APP-MIMALLOC-LITE-DESTRUCTION-SCALAR-FIELD-D0 (accepted)

Decision:
  Admit integer-scalar declared field types (`i8..i64`, `isize`,
  `u8..u64`, `usize`) to the destruction disposition. These are
  teardown-free scalar slots; `DestructionUnavailable::FieldType` keeps
  its closed meaning for `None`/`weak`/non-numeric/handle fields.

Source authority + canonical issuer:
  `object_definition::issue` is the SOLE issuer of
  `ObjectDestructionDispositionV1` — production definitions flow only
  through `take_object_definitions` →
  `install_object_definitions_from_package`. The scalar-class authority
  is `exact_numeric_storage_for_declared_type` (declared_type_storage),
  which already seals the integer-scalar set including `usize` —
  `TypedObjectFieldStorage::USize` exists in the layout plan.

Non-authority:
  No field-type re-inspection downstream: `verification/invoke.rs`,
  `physical_abi.rs`, `end_available`, and the prior-home predicate only
  compare the disposition enum. `PlainI64NoHook` physically means
  "release = one plain `HomeRelease`, no per-field teardown" — the
  emitted op is `InvokeOperation::HomeRelease{object, value}` and is
  width-agnostic.

Fail-fast boundary:
  `None`-typed (legacy `init_fields`), `weak`, `ArrayBox`/`MapBox`/
  `StringBox`, user-box, `bool`/`f64`, and any other non-integer-scalar
  field keeps `Unavailable(FieldType)`. Declaration-shape, member-role,
  and weak-field gates unchanged and still evaluated before the field
  type check.

Smallest next slice:
  Replace `field.declared_type_name.as_deref() != Some("i64")` with the
  sealed `exact_numeric_storage_for_declared_type(name).is_some()`
  predicate in `object_definition.rs`; update the FieldType unit pins;
  add positive `usize`/`u64` fixture + negative `ArrayBox`/object/
  `None`-type pins; re-run probe_f (`v:i64 + extra:usize`) → expect
  progress past `artifact-source-unavailable`; re-pin the
  `page_heap` census frontier.

Non-claims:
  The variant name `PlainI64NoHook` becomes semantically wider than its
  name — renaming to a scalar-honest name is a BoxShape follow-up, NOT
  this slice (BoxCount/BoxShape never mix). This slice admits no
  handle/container/object field — nested release stays a separate
  design. No VM behavior change; the disposition is consumed only by
  the lifecycle lane.

### S0 landed — DESTRUCTION-SCALAR-FIELD (2026-09-30)

`object_definition.rs` now admits any declared type resolved by the
sealed `exact_numeric_storage_for_declared_type` authority
(i8..i64/isize/u8..u64/usize) as teardown-free; every other field keeps
`Unavailable(FieldType)`. The predicate matches the layout-side
authority at `canonical_layout.rs` — a field is teardown-free exactly
when it has scalar numeric storage.

Evidence:
- Unit pins: `usize`/`u32`/`i8`/`isize` fields → `PlainI64NoHook`;
  untyped, `StringBox`, and legacy `init`-fields stay `FieldType`;
  weak stays `WeakField` (checked first). 20/20
  instance-constructor-semantic tests green.
- Probe progression: `v: usize` now compiles to the nyrt gate (was
  `artifact-source-unavailable`); `ArrayBox` and user-object fields
  stay `artifact-source-unavailable` — the closed boundary is intact.
- `new HakoAllocPage(0,8,4)` / `new HakoAllocHeap()` still stop at
  `artifact-unowned-lifecycle-site` (container/object fields remain
  unavailable, by design — slices 2/3).
- Official quick-profile serial: **8104 passed / 127 failed / 56
  ignored** — sole delta vs the wired 126-manifest is the
  baseline-flaky `nullable_receiver_call_serializes` (nondeterministic
  standalone: 2 fail / 4 pass; reproduced on the parent commit
  earlier).
- mimalloc-lite emit-exe smoke: 3/3 deterministic at
  `artifact-unowned-lifecycle-site`.
- `page_heap_fixture_result_claim_census` green — `allocate` prefix
  stays pinned at Body(8).

Next: `MIRBUILDER-APP-MIMALLOC-LITE-DESTRUCTION-ARRAYBOX-FIELD-S0` —
implement the bounded slice: disposition variant + residence-proof
gate + new `InvokeOperation` child-release variant + origin expansion
in reverse declaration order + positive/negative pins.

## Decision — MIRBUILDER-APP-MIMALLOC-LITE-DESTRUCTION-ARRAYBOX-FIELD-D0 (accepted)

Decision:
  Admit owned `ArrayBox` fields to destruction via a **reverse
  declaration-order child release plan**: parent teardown emits one
  child-release step per proven `me.<field>` residence (reverse
  declaration order), then the parent's own `HomeRelease`. The child
  step is a NEW `InvokeOperation` variant — `ArrayResidenceRelease`
  cannot serve: it is a bare instruction with no fault edge and no
  seat in the Invoke cleanup graph.

Source authority + canonical issuer:
  - `box-lifecycle-cprime-terminal-home-finalization-ssot.md` pins:
    "parent teardown → release fields in reverse declaration order;
    child hook only if the child becomes terminal." A field-init
    `new` child has its sole Home in the parent field → terminal.
  - `named_array_residence.rs` already seals `FieldResidence` claims:
    `me.<field> = new ArrayBox()` birth-side provider store +
    `CanonicalFieldRefV1` + provider caller key — the exact shape
    `HakoAllocPage`'s `free_stack`/`block_used`/`use_counts`/
    `requested_sizes` inits normalize into.
  - `object_definition::issue` remains the sole disposition issuer; a
    declared non-weak `ArrayBox` field moves the disposition from
    `Unavailable(FieldType)` to a plan-carrying variant, gated at
    emission by the residence proof.

Physical owner + wiring (traced, no gaps):
  - Cleanup graph is an Invoke chain (`cleanup_step` → `Invoke` with
    normal/fault landings). Each op runs exactly once on exactly one
    path — per-child ops preserve no-double-release for free.
  - New op lowers with EXISTING runtime symbols: read the field's
    Handle slot via `nyash.object.checked_field_get_i64_v1`, release
    the residence via `nyrt_handle_release_h` (the same symbol the
    bare `array_residence_release` emit arm already calls). No new
    kernel export.
  - `physical_abi.rs` today requires `field.storage == I64` for every
    referenced object (`layout-field-drift`) and `PlainI64NoHook`
    (`object-destruction`) — the ABI admission must widen to `Handle`
    storage for plan-carrying objects. Handle is an i64 wire slot.
  - Emit plumbing points: `InvokeOperation` variant + JSON kind +
    `physical_program`/`vocabulary`/`verification` seats + emit
    `.inc` arm + native-admission `.inc` arm.

Non-authority:
  The kernel's `reclaim_typed_object_storage` explicitly does NOT
  recurse into handle fields ("cannot discharge child Homes") — the
  MIR-level plan must emit each child release; no hidden runtime
  cascade is assumed or allowed.

Fail-fast boundary:
  Only fields proven `FieldResidence` (birth-side `new ArrayBox()`
  provider store, no weak, no reassign) join the plan. Reassigned
  residence, weak field, foreign provider, unproven residence, or an
  uninitialized `ArrayBox` field (no provider store) →
  `Unavailable(FieldType)` stays.

Open sub-questions consumed by the S0, not semantics:
  - `FieldResidence` claims today serve the NamedArray append lane;
    the teardown consumer must reuse that detection (or its sealed
    rows), not re-scan source.
  - `end_operation` returns ONE `InvokeOperation`; the plan expands
    a home row into N+1 origins (children reversed + parent
    `HomeRelease`) inside `prepare_root_home_exit`/`begin_root_home_exit`
    and the `emission_prepare` prior-homes path.

Smallest next slice:
  Land the slice-2 S0 as: (1) disposition variant for declared
  non-weak `ArrayBox` fields + residence-proof gate, (2) new
  `InvokeOperation` child-release variant wired through JSON/ABI/
  emit/admission, (3) origin expansion in reverse declaration order,
  (4) positive (`items: ArrayBox = new ArrayBox()` field compiles past
  `artifact-source-unavailable`) and negative (reassigned/weak/
  uninitialized field stays `FieldType`) tests.

Non-claims:
  The NamedArray `FieldResidence` lane is observation authority only —
  this slice consumes it, never duplicates its detection. No
  statement-position release is emitted speculatively; releases occur
  only at proven teardown points. User-object field children
  (`HakoAllocPage` inside `HakoAllocHeap`) remain slice 3 — the app's
  own teardown chain needs both slices before mimalloc's `new
  HakoAllocHeap()` passes.

## S0 implementation notes (surveyed)

Residence-proof substrate:
  `ordinary_new_field_write_claim.rs` already seals the exact rule the
  plan needs: a `(box, field)` claim stands only when every package
  `FieldWrite` is an attributed `me.` write storing `new` of ONE
  agreed class, with a global veto when the field name appears on any
  unattributed receiver or a function's body-shape inventory is
  missing (`opaque` empties the product — additive evidence, no
  fallback). Today `finish()` requires the agreed class to resolve in
  `ordinary_box_coverage`, so `free_stack <- new ArrayBox()` claims
  nothing (ArrayBox is a core box). The S0 extends the seal to also
  record core-`ArrayBox` writes — same authority, wider value class —
  rather than building a second scanner. `resolve_birth_provider` in
  `named_array_residence.rs` is the sibling proof for the NamedArray
  lane and stays untouched.

Disposition placement:
  `instance_constructor_semantic/object_definition.rs::issue` sees only
  the declaration — it issues `OwnedArrayFields`-style plan-PENDING
  disposition for non-weak `ArrayBox` declared fields (replacing
  `FieldType` for exactly that shape). The residence proof is the
  claim-side gate (`end_available`/origin expansion consult
  `field_write_claims`), so an unproven field keeps
  `RetainedUnavailable` — the disposition alone never emits a child
  release.

Emission expansion:
  `NewLocalCommitV1::end_operation` returns one `InvokeOperation`
  consumed by `prepare_root_home_exit` (per-binding origins) and
  `compute_emission_prepare` (prior-homes operands). The plan expands
  to child origins (reverse declaration order) + the parent
  `HomeRelease` origin; each child is its own Invoke so the
  normal/fault cleanup lanes keep the "emitted once, on exactly one
  path" property — no double release by construction.

Physical surface (all existing runtime symbols):
  - `InvokeOperation::OwnedFieldArrayRelease { field, base }` (working
    name) → JSON kind + `physical_program`/`vocabulary`/`verification`
    seats + `lifecycle_v4_emit.inc` arm
    (`checked_field_get_i64_v1` → `nyrt_handle_release_h`) +
    `lifecycle_v4_native_admission.inc` arm.
  - `physical_abi.rs`: admit `TypedObjectFieldStorage::Handle` slots for
    objects whose disposition carries the plan (today `I64`-only →
    `layout-field-drift`); disposition check admits the plan variant
    alongside `PlainI64NoHook`.

### S0 landed — DESTRUCTION-ARRAYBOX-FIELD (2026-10-01)

Semantics (source-sealed, one issuer each):
- `object_definition::issue` admits declared non-weak `ArrayBox` fields
  as `OwnedArrayFieldsNoHook` (plan-pending; `FieldType` retained for
  weak/untyped/other).
- `ordinary_new_field_write_claim` widened: the same seal now records
  core-`ArrayBox` writes into `owned_field_children` per definition —
  every `me.<field>` write must be a `new` of one agreed class, with the
  existing unattributed-receiver and opaque-body vetoes unchanged.
- `OrdinaryNewClaimCoreV1::array_children` carries `Some(proven)` /
  `Some(unproven)` / `None`; `end_plan` expands a home row into child
  `OwnedFieldResidenceRelease` origins (reverse declaration order) +
  parent `HomeRelease`. Unproven plans freeze `artifact-source-unavailable`
  at `end_available` before any emission.
- Birth-fault reclaim chain carries the same child plan:
  `frr(last..first) → reclaim_unpublished`; the normal chain runs
  `frr(last..first) → home_release`. Exactly-once holds because each op
  is one node of the Invoke cleanup graph (fault edges replay only the
  remaining suffix).

Provider emission (the actual blocker found in S0):
- `ProviderConstruction` of builtin `ArrayBox` was already admitted
  semantically, but the store consumer emitted bare `NewBox`, which has
  no published wire shape (`instruction-unsupported`).
- `emit_construction_store` now emits `Invoke{IntrinsicArrayNew}` +
  `InvokeNormalResult` on the shared fault frame under the
  assignment-value source context, then the `FieldSet`. `StoreProgress::
  Emitted.provider` records the origin; `validate_bindings` pins exact
  provider/normal-result pairing.
- The named-array provider ledgers stay exact:
  `record_named_array_allocation` + `metadata.
  named_array_field_allocations` (the generic-lane accounting was the
  `incomplete-consumption` regression); `named_array_obligation`'s
  `FieldResidence` arm accepts the `InvokeNormalResult`←
  `Invoke{IntrinsicArrayNew}` pair as the provider allocation producer.

Physical wire (all pre-existing runtime symbols):
- `OwnedFieldResidenceRelease { field, base }` → JSON
  `field_residence_release`; C emit reads the slot via
  `nyash.object.checked_field_get_i64_v1` and releases through
  `nyrt_handle_release_h` only when the zero-initialized slot is live
  (partial-birth rollback is the same op).
- `array_new` admitted on the non-native lane as the proven provider
  allocation: `hako_physical_validate_operation` arm,
  `hako_physical_result_operation`, indexed-flow lease arm
  (`birth`/`ordinary` only), and `invoke_normal_result` no longer
  birth-barred (still must be row 0 of the normal landing).
- `field_set` with a handle-typed value consumes the lease on BOTH
  edges: the emit discharges it through `nyrt_handle_release_h` on the
  fault landing because `checked_field_set_v1` faults before storing —
  no silent lease drop.
- `physical_abi` admits `Handle` field storage for plan-carrying
  objects; `canonical_layout` resolves `ArrayBox` → `Handle`; kernel
  wire keeps `I64` slot tags (all `TypedObjectFieldStorage` variants
  ride the integer lane).

Evidence:
- Positive pin `owned_array_fields_release_in_reverse_order_before_
  home_release` walks the published JSON *edges* (block order ≠
  execution order): normal chain `frr(last)→frr(first)→home_release`,
  birth-fault chain `frr(last)→frr(first)→reclaim_unpublished`, and the
  exit fault lane correctly replays only the remaining suffix.
- Negative pins: unproven (reassigned) ArrayBox field keeps
  `artifact-source-unavailable`; claim-layer pins cover ambiguous writes
  and declaration order.
- End-to-end probe `new Page{left:i64=0, items/children:ArrayBox=new
  ArrayBox()}` + `return 0` compiles through Lifecycle V4 + LLVM C API
  and the EXE runs `Result: 0` — the teardown chain actually executes.
- Official `mimalloc_lite_exe` smoke: negative pin re-pinned to the
  current first terminal `emission-binding-drift` (same fail-closed
  ladder, moved forward); EXE still stops at the designed frontier
  `artifact-unowned-lifecycle-site` (later slices).
- Touched-module tests green; sole red in runs remains the
  baseline-flaky `nullable_receiver_call_serializes`.

Frontier census update:
- `items: ArrayBox = new ArrayBox()` probe: compiles and runs.
- `c: Child = new Child(1)` and user-object field destruction stay
  closed (next slice). `handle.<field>` calls, scalar args, static-box
  calls and the rest of the inventory remain their own slices.

Next: `MIRBUILDER-APP-MIMALLOC-LITE-DESTRUCTION-OBJECT-FIELD-D0` —
bundle slice 3: nested release for owned user-object fields
(`HakoAllocHeap.small_page`/`medium_page`). The child-side claim must
corroborate the child's own teardown plan (not just terminal residence),
and `FIELD-INIT-USER-NEW` (`ProviderConstruction` admits builtin
zero-arg `new` only — user-class field init is `FieldContractUnsupported`
today) is the sibling gate a compiling probe needs.

## Decision — MIRBUILDER-APP-MIMALLOC-LITE-DESTRUCTION-OBJECT-FIELD-D0 (accepted)

Decision:
  Owned user-object fields join the parent teardown through ONE new
  Invoke op, `OwnedObjectFieldRelease{field, base, child}` — wire
  `object_field_release`. Inline expansion of the child's plan into the
  parent cleanup chain is impossible: cleanup-graph nodes are
  terminal-only (`root_cleanup_graph.rs` admits only Invoke/Return/
  Jump rows), `object_field_get` is rejected on faulted blocks (fault
  copies run `faulted=1`), and the intermediate child handle has no
  lease representation on the wire. The op's C emit reads the slot
  (`checked_field_get_i64_v1`) and, when live, calls a generated
  `@hako_lifecycle_teardown_<child>` helper that runs the child's own
  sealed plan then its `home_release_plain_i64_v1` — recursion for
  grandchildren lands in the child's helper, once per class.

Source authority + canonical issuer:
  - Same SSOT: parent teardown releases fields in reverse declaration
    order; a child finalizes only when its sole Home is terminal —
    the claim-sealed birth-side store establishes exactly that.
  - `field_write_claims` gains a typed child descriptor
    `OwnedFieldChildV1{field, child: Array | Object(CanonicalObjectIdV1)}`
    (replaces/widens `owned_field_children`'s `CanonicalFieldRefV1`
    rows); the parent's issue path resolves the child class through
    `instance_constructors` — accessible at claim issue time — and
    requires the child's OWN disposition to be a supported plan
    (recursive admission), its construction plan Ok, and no teardown
    cycle (`box A { a: A }` rejected at plan build).
  - The child's teardown plan is a NEW published product: the emit
    helper is generated per referenced object from claim-sealed child
    evidence — nothing on today's wire (`layouts` rows are placement
    only) carries teardown data, so a `teardowns` product or generated
    helper section is issued by the sole physical owner.

Non-authority:
  No statement-position reads, no per-field chain inside the parent
  cleanup graph, no recursion inside kernel exports. `map.rs` stays
  fail-closed (`map-candidate-end-unavailable`) for object-child
  objects — correct, untouched.

Fail-fast boundary:
  Unproven child disposition (child's own fields unproven, weak field,
  unsealed provider, teardown cycle, missing birth recipe) → parent
  keeps `Unavailable`/`artifact-source-unavailable`; no speculative
  release is emitted.

Sibling dependency (ordering decision):
  `FIELD-INIT-USER-NEW` is a hard prerequisite for any end-to-end
  pin: `ProviderConstruction` today rejects user classes
  (`FieldContractUnsupported`) — the arm must carry child object
  identity + verified birth recipe + argument rows, emit
  `Invoke{NewBox}+InvokeNormalResult+Invoke{Call{Birth,Unit}}+FieldSet`
  on the shared fault frame, and the C flow admits `new_box`/
  `birth_call` in the birth role (`(fi && !ordinary && !birth)`)
  plus an in-flight child discharge admitted in birth on the
  birth_call fault landing. Known gap recorded: the `field_set`
  fault-edge discharge emits bare `nyrt_handle_release_h` — for a
  user-object value it must instead run the child teardown (ArrayBox
  is exact today because a runtime residence has no children).

Smallest next slice:
  `MIRBUILDER-APP-MIMALLOC-LITE-FIELD-INIT-USER-NEW-D0` —
  bounded provider-arm admission for
  user-class field init inside birth units (emit shape above, validator
  formula `stores + 2*provider` update, C admission `|| birth`,
  fault-edge in-flight discharge). Then `DESTRUCTION-OBJECT-FIELD-S0`
  lands the release op on top of proven field-initialized children.

Non-claims:
  No claim that object-child release shares the ArrayBox op — the
  descriptor carries the child kind and each kind lowers to its own
  op/helper. No nullable/weak field teardown. No teardown for
  shared/multi-home children (claim proves sole residence).

