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

Next: `MIRBUILDER-APP-MIMALLOC-LITE-DESTRUCTION-ARRAYBOX-FIELD-D0` —
bundle slice 2: owned `ArrayBox` fields need a child-release plan
before their destruction can be provable.

## Decision — MIRBUILDER-APP-MIMALLOC-LITE-DESTRUCTION-ARRAYBOX-FIELD-D0 (drafted)

Decision:
  Admit owned `ArrayBox` fields to destruction via a **reverse
  declaration-order child release plan**: parent teardown emits one
  `ArrayResidenceRelease` per proven `me.<field>` residence (read the
  field handle, release the native array), then the parent
  `HomeRelease`. The SSOT pins the semantics; no new language rule is
  invented.

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
  - `object_definition::issue` remains the sole disposition issuer; the
  disposition gains a child-release plan, not a boolean flip.
  - Physical op `ArrayResidenceRelease{value}` already exists with
    emitters and a lifecycle-validation seat.

Non-authority:
  The kernel's `reclaim_typed_object_storage` explicitly does NOT
  recurse into handle fields ("cannot discharge child Homes") — the
  MIR-level plan must emit each child release; no hidden runtime
  cascade is assumed or allowed.

Fail-fast boundary:
  Only fields proven `FieldResidence` (birth-side `new ArrayBox()`
  provider store, no weak, no reassign) join the plan. Reassigned
  residence, weak field, foreign provider, or unproven residence →
  `Unavailable(FieldType)` stays.

Open mechanical questions for the S0 (implementation shape, not
  semantics):
  - `end_operation` returns ONE `InvokeOperation` today; the parent
    release becomes a sequence (child `ObjectFieldGet` +
    `ArrayResidenceRelease` × N reverse order + parent `HomeRelease`).
  - The disposition needs a plan-carrying variant (e.g.
    `ReverseFieldsNoHook` listing proven children) — or the claim
    computes the plan at emission from the object definition.
  - `HakoAllocHeap`'s fields are `HakoAllocPage` (user object) — slice
    3; this slice is `ArrayBox` fields only.

Smallest next slice:
  Land the slice-2 S0 as: (1) prove `FieldResidence` for the four
  `HakoAllocPage` container fields, (2) extend the disposition/emission
  to emit reverse-order child releases, (3) pin positive (`items:
  ArrayBox` field compiles past `artifact-source-unavailable`) and
  negative (reassigned/weak field stays `FieldType`) tests.

Non-claims:
  The NamedArray `FieldResidence` lane is observation authority only —
  this slice must consume it, not duplicate its detection. No
  statement-position `ArrayResidenceRelease` is emitted speculatively;
  releases occur only at proven teardown points.

