---
Status: SSOT
Date: 2026-06-20
Scope: MirBuilder-only Rust-to-Hako converter task order.
Related:
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
  - docs/development/current/main/design/rust-lifecycle-projection-ssot.md
  - docs/reference/architecture/rust-to-hako-lifecycle-projection.md
---

# MirBuilder Rust-to-Hako Converter Task Order

## Goal

Build a practical MirBuilder-only converter with two explicit lanes:

```text
easy tier:
  Rust source
    -> lightweight body/signature facts
    -> DirectShapeLowerer
    -> VerifiedHakoFamilyIR
    -> shared emitter
    -> runnable native-shaped .hako

hard tier:
  Rust source
    -> lightweight facts
    -> Deny(reason)
    -> human design stop / later explicit plan
```

The easy tier is direct mechanical translation. Do not turn simple Rust shapes
such as ordered-map contexts or scalar counters into route-selection,
consultation, or lifecycle-design tasks.

The older lifecycle wording remains valid only as a guard/provenance layer for
families that already need it:

```text
Rust source
  -> lightweight signature facts
  -> HakoLifecyclePlan
  -> verifier
  -> VerifiedHakoFamilyIR
  -> runnable native-shaped .hako
```

This is not a full Rust translator. Rust remains bootstrap, oracle, and source
reference. Generated `.hako` is an executable derived artifact. Mature family
source moves to native `.hako` as the editing and semantic authority.

For the current easy tier, `facts` means the existing lightweight
signature-based extraction path that is already green for BindingContext-style
families. Do not open the nightly `rustc_private` adapter path for
BindingContext or VariableContext simple-map. Nightly rustc MIR/borrowck facts
are reserved for a later gated hard tier, such as PHI, loop lowering, or
non-trivial borrow/drop ownership.

## Easy-Tier Direct Shape Rule

Use direct shape lowering when all of these are true:

```text
source body has a bounded shape
all calls are in the allowed vocabulary
field ownership is local to the translated box
no returned borrow or mutable alias escapes
no Drop / unsafe / FFI / PHI / loop-carried state is required
the generated operation IR is typed before emission
```

The direct lowerer is allowed to map these shapes without a new design card:

| Shape | Rust surface | Hako operation family | Current users |
| --- | --- | --- | --- |
| `single_ordered_map_context` | one `BTreeMap` field with get/contains/len/is_empty/insert/remove/clear | `NewOrderedMap`, `MapGetCopied`, `MapHas`, `MapLength`, `MapIsEmpty`, `MapSet`, `MapRemove`, `MapClear` | BindingContext, VariableContext simple-map |
| `owned_ordered_map_snapshot` | `BTreeMap::clone`, owned restore/replace | `CloneOwnedMap`, `ReplaceOwnedMap` | VariableContext snapshot/restore |
| `multi_ordered_map_context` | multiple map fields, default construction, all-fields empty check | `NewOrderedMap`, `AllMapsEmpty` | BoxCompilationContext |
| `scalar_counter_context` | integer fields with next/peek counter methods | `InitFieldConst`, `TakeThenSaturatingIncrementU32`, `ReturnI64` | CoreContext scalar counters |
| `owned_map_carrier_projection` | known non-escaping bulk consumer over an owned snapshot | `CarrierSnapshotFromOwnedMap`, `ExplicitCarrierSnapshotFromOwnedMap` | CarrierInfo snapshot slices |

These are design stops, not direct-lowering work:

```text
ReturnedReadBorrow
ReturnedMutableBorrow
CarrierSensitiveAlias outside the owned-snapshot projection
PhiJoinRequired
LoopCarriedState
NonTrivialDrop
UnsafeOrFFI
NullableMapValue
NonAsciiOrderedKey
CoreContext generator-object transport
```

If a shape lands in the design-stop list, do not patch around it with
by-name branches, raw Hako strings, or fallback `RuntimeDataBox` routes.

## Work Unit Rule

Prefer implementation commits over process commits.

One normal task should close:

```text
usable source
+ behavior test
+ MIR/EXE acceptance
+ converter capability, when the task is a converter task
```

Do not create a numbered row, route-selection card, or manifest-only commit for
ordinary progress. Create a durable card only when a deny reason below requires
human design judgment.

## Current Direct-Lowering Task Order

The next cleanup/implementation series is:

1. `Document direct-shape lowerer boundary`

   Status: this SSOT update.

   Purpose:

   ```text
   easy tier = direct Rust-shape lowering
   hard tier = explicit Deny(reason) / design stop
   ```

2. `Implement direct-shape rule table`

   Move the easy-tier operation selection into a small rule table keyed by
   source shape, not by family name.

   Acceptance:

   ```text
   BindingContext and VariableContext simple-map compile through the same
   single_ordered_map_context rule.

   emitter has no BindingContext/VariableContext by-name branch.

   generated artifacts remain deterministic.
   ```

3. `Move BoxCompilationContext to multi-map direct shape`

   Treat `BoxCompilationContext::new` and `is_empty` as
   `multi_ordered_map_context`, not a bespoke lifecycle consultation.

   Acceptance:

   ```text
   DefaultConstruct + AllMapsEmpty are produced from live lightweight facts.
   no ValueId-key set/get support is claimed.
   ```

4. `Move CoreContext scalar counters to scalar-counter direct shape`

   Keep only scalar counter methods in the easy tier. Generator-object methods
   stay parked.

   Acceptance:

   ```text
   next/peek counter methods are emitted from scalar_counter_context.
   next_value / next_block / peek_* generator-object methods deny explicitly.
   ```

5. `Downgrade process-only inventories to legacy traceability`

   Keep old readiness/selection/probe documents as historical evidence, but do
   not make them the active workflow.

   Active workflow:

   ```bash
   python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --all --check
   bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
   ```

6. `Re-evaluate partial crate bundle after direct lowerer cleanup`

   Do not add another family or crate linker slice until the easy-tier direct
   rule table owns the existing BindingContext, VariableContext,
   BoxCompilationContext, and CoreContext scalar-counter paths.

## Current Completed Native Targets

```text
BindingContextNative:
  source: apps/lib/hakorune_mir_builder/binding_context.hako
  test:   apps/tests/phase296x_binding_context_native_min.hako

VariableContextNative simple-map:
  source: apps/lib/hakorune_mir_builder/variable_context.hako
  test:   apps/tests/phase296x_variable_context_native_simple_map_min.hako
```

The native targets intentionally use `*Native` names to avoid collisions with
checked-in generated artifacts.

## Task Order

1. `Write shared MirBuilder behavioral emitter`

   Input is `VerifiedHakoFamilyIR`, not raw Rust syntax and not family names.
   The emitter only renders verified operations to `.hako` source.

   Initial operation vocabulary:

   ```text
   NewOrderedMap
   MapGet
   MapHas
   MapLength
   MapIsEmpty
   MapSet
   MapRemove
   MapClear
   Return
   ReturnBoolI64
   ReturnNull
   ```

   Acceptance:

   ```text
   BindingContextNative emitted source parses, MIR-emits, and EXE-runs.
   VariableContextNative simple-map emitted source parses, MIR-emits, and EXE-runs.
   Emitter contains no BindingContext/VariableContext by-name branch.
   Emitter emits no TODO/null placeholder bodies.
   ```

2. `Connect lightweight facts to simple-map converter`

   Add the first one-command path for BindingContext and VariableContext
   simple-map only.

   ```text
   Rust source
     -> lightweight signature facts
     -> HakoLifecyclePlan
     -> verifier
     -> VerifiedHakoFamilyIR
     -> shared emitter
   ```

   Allowed resolved call targets:

   ```text
   BTreeMap::new
   BTreeMap::get
   BTreeMap::contains_key
   BTreeMap::len
   BTreeMap::is_empty
   BTreeMap::insert
   BTreeMap::remove
   BTreeMap::clear
   Option::copied
   ```

   Unknown resolved call targets must fail with
   `Deny(UnsupportedResolvedCallTarget)`.

3. `Fix VariableContext snapshot ownership`

   Do not native-adopt the current snapshot/restore artifact as-is. The current
   derived artifact aliases the map object; Rust `snapshot()` clones the
   `BTreeMap`.

   Current backend result:

   ```text
   restore(ctx, snapshot) uses snapshot.clone_owned().
   The native EXE smoke proves post-restore alias isolation.
   ```

   Required native operations:

   ```text
   OrderedMapBox.clone_owned()
   VariableContextNativeApi.snapshot(ctx)
   VariableContextNativeApi.restore(ctx, snapshot)
   ```

   Required behavior:

   ```text
   ctx x=1
   snapshot
   ctx x=2
   restore(snapshot)
   ctx x==1

   mutating snapshot after restore does not mutate ctx
   clearing ctx after snapshot does not mutate snapshot
   ```

4. `Replace hard-coded family generators`

   Start only after the shared converter emits BindingContextNative,
   VariableContextNative simple-map, and snapshot/restore behavior. Keep old
   family generators until the converter matrix is green.

5. `Inventory MirBuilder next-family lifecycle readiness`

   Re-evaluate `context`, `core_context`, `type_context`, and
   `metadata_context` as behavioral candidates. Skeleton transport alone is
   insufficient, so this row records readiness only and does not select a new
   family route.

6. `Select MirBuilder next-family facts pilot`

   Decision: keep VariableContext as the only active behavioral family.

   The `context`, `core_context`, `type_context`, and `metadata_context`
   candidates are still skeleton transport only. Do not select a new
   non-VariableContext family until one has lifecycle facts, plan, recipe,
   oracle, artifact readiness, and route readiness.

7. `Implement operation-backed VariableContext simple-map conversion`

   Status: current implementation landed.

   VariableContext simple-map generation now compiles live lightweight body
   facts and the selected plan into typed ordered-map operations before the
   shared emitter renders `.hako`. This removes raw Hako API body strings from
   the simple-map spec.

8. `Generalize ordered-map converter to BindingContext`

   Status: current implementation landed.

   BindingContext generation now uses the same ordered-map operation compiler,
   including `MapClear`. The BindingContext spec no longer stores raw Hako API
   body strings.

9. `Implement owned VariableContext snapshot/restore conversion`

   Status: current implementation landed.

   Snapshot/restore generation now uses live lightweight body facts and
   `CloneOwnedMap` / `ReplaceOwnedMap` operation IR. The emitter keeps
   `restore(ctx, snapshot)` alias-proof by cloning the owned snapshot when
   writing the context field.

10. `ReturnedReadBorrow design stop`

   Status: closed by design decision.

   Easy-tier contract:

   ```text
   NoReturnedAlias
   + OwnedReadSnapshotProjection
   ```

   `VariableContext::variable_map()` standalone conversion is
   `Deny(ReturnedReadBorrow)`. Known non-escaping bulk consumers use
   `VariableContextApi.snapshot(ctx)` and consume an owned snapshot. Do not
   introduce `OrderedMapReadViewBox` until a later Drop/lifetime hard tier.

11. `Accept OrderedMapBox parameter receiver route`

   Status: landed.

   The owned-snapshot `CarrierInfo` converter slice now has the intended
   meaning path:

   ```text
   VariableContextApi.snapshot(ctx)
     -> CarrierInfoApi.from_snapshot(carrier_data, loop_var_name, snapshot)
   ```

   The generated artifact reaches MIR and EXE. The route was unblocked by
   making user-box method target arity strip duplicate receiver arguments by
   origin, not only by exact value id.

   Previous failing backend receiver shape:

   ```text
   CarrierInfoApi.from_snapshot(
       carrier_data: OrderedMapBox,
       loop_var_name,
       snapshot: OrderedMapBox
   )

   carrier_data.set(...)
   ```

   Current failure:

   ```text
   reason=mir_call_no_route
   callee_symbol=set
   next_check_hint=check_callee_route_or_receiver_origin
   ```

   This is a BoxCount task: add the smallest backend/MIR acceptance shape for
   method calls on typed `OrderedMapBox` parameters. Do not weaken the owned
   snapshot contract and do not reintroduce raw `return ctx.variable_map`.

12. `Close owned-snapshot CarrierInfo conversion`

   Status: landed.

   Acceptance:

   ```text
   bash tools/checks/rust_lifecycle_variable_context_carrier_snapshot_derived_artifact_guard.sh

   expected:
     generated_hako_mir_emit=green
     generated_hako_exe=green
     owned_snapshot_alias_isolation=green
     publishes_variable_map=0
     returned_read_borrow_deny=green
   ```

13. `Implement explicit CarrierInfo conversion from owned snapshot`

   Status: landed.

   This must not use raw `VariableContextApi.variable_map()` and must preserve
   missing requested carrier fail-fast behavior.

14. `Adopt native CarrierInfo snapshot APIs`

   Status: landed.

   Native `.hako` authority is added after generated owned-snapshot carrier
   paths are green. The native API uses `CarrierInfoNative` fields plus
   static `CarrierInfoNativeApi` methods and has EXE smokes for `from_snapshot`
   and explicit snapshot input.

15. `Accept imported instance method routes for native Hako APIs`

   Status: landed.

   Accept direct imported user-box instance calls such as
   `info.from_snapshot("i", snapshot)` without by-name fallback or
   RuntimeDataBox dispatch widening.

16. `Accept typed output-argument mutation for generated bridge APIs`

   Status: landed.

   Let generated `CarrierInfoApi.from_snapshot(carrier_data, ...)` mutate a
   typed `OrderedMapBox` output parameter that remains visible to the caller,
   so generated Main no longer has to inline the operation body.

17. `Preserve OrderedMapBox.get result type origin`

   Status: landed.

   Preserve known object origins through `OrderedMapBox.set/get` pairs for
   focused carrier-data objects, so `info.get("carrier_names").get(0)` can
   route as `ArrayBox.get` rather than `RuntimeDataBox.get`.

18. `Accept explicit carrier ArrayBox.get`

   Status: landed.

   The explicit carrier snapshot artifact now accepts the remaining route
   shape at the backend/MIR acceptance boundary for:

   ```text
   local requested_name_copy = info.get("requested_names")
   requested_name_copy.get(0)
   ```

   This was solved as a general accepted `ArrayBox.get` route / receiver-
   origin publication shape. No key-name-specific type escape was added in
   `ordered_map_origin_plan.rs`.

   Guard:

   ```text
   bash tools/checks/rust_lifecycle_no_carrier_key_type_special_case_guard.sh
   ```

19. `Retire raw Hako method body strings`

   Status: landed.

   Goal:

   ```text
   generated family API method bodies
     = typed operations only

   acceptance Main harnesses
     = allowed to remain as main_lines for now
   ```

   Current inventory:

   ```text
   Active MirBuilder converter specs:
     operations-backed only

   Not debt for this task:
     FamilyArtifactSpec.main_lines
     static Main acceptance harness text
     generated behavior smoke scripts
   ```

   Implementation order:

   ```text
   1. Replace immutable-borrow raw alias artifact with
      Deny(ReturnedReadBorrow) verification.

   2. Remove ApiMethodSpec.body_lines from active generator specs.

   3. Remove the emitter/builders body_lines compatibility path.

   4. Add or update a guard so generated API method specs cannot use
      body_lines again. Keep main_lines allowed until a separate harness
      IR task exists.
   ```

   Acceptance:

   ```text
   rg -n "body_lines" tools/rust_lifecycle
     shows only transitional docs or no matches in executable generator code.

   bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
     remains green.

   bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
     remains green.

   ReturnedReadBorrow stays fail-fast. Do not reintroduce raw
   `return ctx.variable_map`.
   ```

20. `Close CarrierInfo easy-tier live facts conversion`

   Status: landed.

   Close the remaining easy-tier gap by making CarrierInfo converter methods
   compile typed operation IR from live lightweight facts plus plan inputs
   instead of hard-coded spec operations. Keep the hard-tier stops parked and
   do not open the nightly rustc adapter path for this slice.

   Acceptance:

   ```text
   live lightweight facts + plan compile the CarrierInfo snapshot methods
   into ApiMethodSpec.operations

   spec contains no hard-coded CarrierSnapshotFromOwnedMap or
   ExplicitCarrierSnapshotFromOwnedMap operation bodies

   unknown resolved call targets still fail closed

   generated .hako stays byte-identical

   carrier snapshot and explicit carrier EXE remain green

   bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh remains green

   no new route/card/guard files are added
   ```

   Evidence:

   ```text
   454482b5c7 Implement CarrierInfo live facts conversion
   bash tools/checks/rust_lifecycle_variable_context_carrier_snapshot_derived_artifact_guard.sh
   bash tools/checks/rust_lifecycle_variable_context_explicit_carrier_snapshot_derived_artifact_guard.sh
   bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
   ```

## Remaining Work Backlog

This backlog records the post-CarrierInfo inventory. It is a task-order guide,
not a new route/card lane. Prefer implementation commits that close a real
converter capability or a real guard coverage gap.

### Phase A: Converter Coverage Hygiene

Goal: make the existing easy-tier converter surface truthful and one-command
checkable before selecting a larger new family.

21. `Repair MirBuilder converter matrix coverage`

   Status: closed.

   Current issue:

   ```text
   rust_mirbuilder_converter_matrix_guard.sh reports a broad converter matrix,
   but it does not run every current green artifact/native/hardcode guard it
   should report.
   ```

   Required scope:

   ```text
   include carrier snapshot artifact guard
   include explicit carrier snapshot artifact guard
   include no-carrier-key-type-special-case guard
   include no-silent-hardcode guard where meaningful for staged diffs
   update docs/tools/check-scripts-index.md matrix wording
   ```

   Acceptance:

   ```text
   bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
     covers every family/status it reports

   docs/tools/check-scripts-index.md
     describes the same coverage

   no generated artifact output changes
   no route/card file added
   ```

22. `Expand lightweight converter entrypoint`

   Status: closed.

   Current issue:

   ```text
   tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py
   only covers BindingContext and VariableContext simple-map.
   ```

   Required scope:

   ```text
   add --all
   include variable-context-snapshot-restore
   include variable-context-carrier-snapshot
   include variable-context-explicit-carrier-snapshot
   keep immutable borrow as Deny(ReturnedReadBorrow), not generated alias
   update tools/checks/rust_mirbuilder_lightweight_facts_converter_guard.sh
   ```

   Acceptance:

   ```text
   python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --all --check
     green

   generated .hako artifacts stay byte-identical
   unknown shapes still fail closed
   no nightly rustc adapter path opened
   ```

23. `Add durable negative converter fixtures`

   Status: closed.

   Current issue:

   ```text
   Some fail-closed checks are inline Python mutations inside guards rather
   than a small reusable negative fixture corpus.
   ```

   Required negative cases:

   ```text
   UnsupportedResolvedCallTarget
   ReturnedReadBorrow
   ReturnedMutableBorrow
   CarrierSensitiveAlias
   missing requested carrier fail-fast
   hardcoded representation token in decision path
   TODO/null placeholder emission
   ```

   Result:

   ```text
   The negative converter matrix guard now loads case order and parked status
   from `mirbuilder-negative-converter-fixtures-v0.json`.
   ```

   Acceptance:

   ```text
   one negative matrix guard runs the fixture corpus
   each case reports the intended Deny reason
   no happy-path generated artifact changes
   ```

24. `Unify carrier artifact generator runner`

   Status: closed.

   Current issue:

   ```text
   Binding/simple-map/snapshot use the shared generator runner, while carrier
   artifacts use a separate write_outputs flow.
   ```

   Acceptance:

   ```text
   carrier snapshot and explicit carrier snapshot use the shared validated
   generator path or a documented equivalent helper

   generated .hako and artifact manifests stay byte-identical
   carrier snapshot and explicit carrier EXE guards stay green
   ```

   Result:

   ```text
   Both carrier artifact generators now call the shared validated generator
   path instead of a bespoke write_outputs branch.
   ```

25. `Track generated-to-native adoption matrix`

   Status: closed.

   Current issue:

   ```text
   Native Hako sources and guards exist, but generated route selection and
   native semantic-authority adoption are not summarized by one matrix.
   ```

   Required scope:

   ```text
   BindingContextNative
   VariableContextNative simple-map
   VariableContextNative snapshot/restore
   CarrierInfoNative snapshot APIs
   ```

   Acceptance:

   ```text
   one adoption matrix guard or report distinguishes:
     generated derived_hako route
     native_hako source existence
     native behavior EXE guard
     source_selfhost_claim=0 unless explicitly promoted
   ```

   Result:

   ```text
   The adoption matrix report now distinguishes generated route, native source,
   native EXE smoke, and source_selfhost_claim for BindingContextNative,
   VariableContextNative simple-map, VariableContextNative snapshot/restore,
   and CarrierInfoNative snapshot APIs.
   ```

### Phase B: Next Easy-Tier Family Pilot

Goal: choose exactly one bounded MirBuilder family slice with real behavior
facts. Skeleton transport alone is not enough.

26. `Select bounded BoxCompilationContext facts pilot`

   Status: landed.

   Rationale:

   ```text
   context / BoxCompilationContext is the smallest plausible next easy-tier
   candidate. Start with constructor + is_empty only.
   ```

   First landed slice:

   ```text
   lightweight facts extraction guard for constructor + is_empty only is green
   typed operation IR + generated artifact + route selection are green
   ```

   Required scope:

   ```text
   live lightweight facts
   HakoLifecyclePlan
   oracle vectors
   typed operation IR
   generated artifact
   MIR/EXE acceptance
   no size_info claim unless separately proven
   ```

   Rough size:

   ```text
   5-7 tasks for constructor + is_empty
   ```

27. `Evaluate CoreContext scalar counter vocabulary`

   Status: landed.

   Required new vocabulary candidates:

   ```text
   scalar counter field initialization
   increment / saturating_add
   ID constructor calls
   struct-return construction
   ```

   Result:

   ```text
   The CoreContext scalar-counter vocabulary is fixed in a separate
   machine-readable consultation fixture, while route selection remains
   unopened.
   ```

   Rough size:

   ```text
   7-10 tasks
   ```

28. `Evaluate TypeContext bounded map slice`

   Status: landed.

   Known complications:

   ```text
   non-String keys
   HashMap rather than BTreeMap
   Option/default behavior
   closure-shaped source paths
   snapshot struct behavior
   ```

   Rough size:

   ```text
   8-12 tasks for a narrow first slice
   ```

   Result:

   ```text
   The bounded TypeContext map slice is fixed in a separate
   machine-readable consultation fixture, while route selection remains
   unopened and the nightly rustc adapter path stays parked.
   ```

29. `Keep MetadataContext deferred`

   Status: landed.

   Reason:

   ```text
   generics, Option, Vec, HashMap, closures/macros, source-file cloning, and
   region stack push/pop make this too broad for the next easy-tier pilot.
   ```

   Result:

   ```text
   MetadataContext is explicitly parked in a consultation-only inventory.
   The deferred decision is durable, while route selection and the nightly
   rustc adapter path remain unopened.
   ```

   Rough size:

   ```text
   10-15 tasks if later narrowed
   ```

29.5 `Record CoreContext readiness inventory`

   Status: landed.

   This row is consultation-only and does not select a route. It records that
   `hakorune_mir_builder::core_context::CoreContext` is the next plausible
   easy-tier smoke candidate after BoxCompilationContext, but lifecycle facts,
   Hako lifecycle plan, behavior recipe, oracle vectors, derived artifact
   manifest, and route entry are still absent. The inventory is fixture-backed
   so the consultation row stays durable without opening route selection.

   Design stop:

   ```text
   scalar counter field initialization
   increment / saturating_add
   ID constructor calls
   struct-return construction
   ```

### Phase C: Hard-Tier Design Stops

Goal: enter these only with an explicit design decision. Do not drive-by extend
the easy-tier converter into these areas.

30. `ReturnedMutableBorrow replacement decision`

   Status: landed.

   Choices to decide:

   ```text
   explicit mutation APIs
   bounded with-map operation
   ReplaceOwned-style ownership transfer
   ```

   Rough size:

   ```text
   2-4 tasks after decision
   ```

   Result:

   ```text
   The returned mutable borrow decision space is fixed in a
   machine-readable consultation fixture while the Deny(ReturnedMutableBorrow)
   boundary remains intact and route selection stays unopened.
   ```

31. `ReturnedReadBorrow / read-view decision`

   Status: landed.

   Current contract remains:

   ```text
   NoReturnedAlias + OwnedReadSnapshotProjection
   ```

   Result:

   ```text
   The current contract keeps bulk read consumers on owned snapshots, and
   true read views are deferred to a later hard tier.
   ```

   Rough size:

   ```text
   3-5 tasks if true read views are selected
   ```

32. `CarrierSensitiveAlias proof`

   Status: landed.

   Result:

   ```text
   Carrier-sensitive consumers remain inventory-only, the read contract stays
   NoReturnedAlias + OwnedReadSnapshotProjection, and route selection remains
   unopened.
   ```

   Rough size:

   ```text
   3-5 tasks
   ```

33. `PHI and join_id lifecycle`

   Status: landed.

   Result:

   ```text
   CarrierVar.join_id remains parked as test vocabulary, trim_helper and
   promoted_body_locals stay separate inventory boundaries, and route
   selection remains unopened.
   ```

   Rough size:

   ```text
   6-9 tasks
   ```

34. `Loop / trim route lowering`

   Status: landed.

   Result:

   ```text
   Trim route lowering remains parked, the readiness gate and route-boundary
   probes stay separate, and executable lowering remains unopened.
   ```

   Rough size:

   ```text
   5-8 tasks
   ```

35. `NonTrivialDrop`

   Status: landed.

   Result:

   ```text
   Nontrivial Drop remains parked, TrivialMemory cleanup remains the only
   erase path, and route selection remains unopened.
   ```

   Rough size:

   ```text
   6-10 tasks
   ```

36. `UnsafeOrFFI`

   Status: landed.

   Result:

   ```text
   UnsafeOrFFI remains parked as a consultation-only inventory slice.
   Broad unsafe surface and FFI stay separate from the easy-tier converter,
   and route selection remains unopened.
   ```

   Rough size:

   ```text
   6-10 tasks
   ```

37. `NullableMapValue`

   Status: landed.

   Result:

   ```text
   NullableMapValue remains parked as a consultation-only inventory slice.
   Null-free Option stays separate from nullable map payload handling, and
   route selection remains unopened.
   ```

   Rough size:

   ```text
   2-4 tasks
   ```

38. `NonAsciiOrderedKey`

   Status: landed.

   Result:

   ```text
   NonAsciiOrderedKey remains parked as a consultation-only inventory slice.
   String-only OrderedMapBox remains separate from non-ASCII collation
   policy, and route selection remains unopened.
   ```

   Rough size:

   ```text
   2-3 tasks
   ```

39. `ConstructorLifecycleMismatch`

   Status: landed.

   Result:

   ```text
   ConstructorLifecycleMismatch remains parked as a consultation-only
   inventory slice. Declaration-site field initializers stay separate from
   birth-time constructor logic, and route selection remains unopened.
   ```

   Rough size:

   ```text
   2-4 tasks
   ```

40. `CoreContext`

   Status: landed.

   Result:

   ```text
   CoreContext remains the next easy-tier family pilot selection. The pilot
   stays bounded to the scalar-counter slice, and route selection remains
   unopened.
   ```

   Rough size:

   ```text
   5-8 tasks
   ```

41. `CoreContext scalar-counter facts pilot`

   Status: landed.

   Result:

   ```text
   CoreContext scalar-counter facts pilot remains parked as a
   consultation-only inventory slice. The extracted facts fixture, readiness
   inventory, and scalar-counter vocabulary stay bounded, and route selection
   remains unopened.
   ```

   Rough size:

   ```text
   1-3 tasks
   ```

42. `CoreContext scalar-counter plan/oracle`

   Status: landed.

   Result:

   ```text
   CoreContext scalar-counter plan/oracle remains parked as a
   consultation-only inventory slice. The missing plan and oracle fixtures are
   recorded explicitly, and route selection remains unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

43. `TypeContext bounded map slice pilot selection`

   Status: landed.

   Result:

   ```text
   TypeContext bounded map slice pilot selection remains parked as a
   consultation-only selection slice. The bounded map-slice readiness stays
   explicit, and route selection remains unopened.
   ```

   Rough size:

   ```text
   1-3 tasks
   ```

44. `TypeContext bounded map slice facts pilot`

   Status: landed.

   Result:

   ```text
   TypeContext bounded map slice facts pilot remains parked as a
   consultation-only inventory slice. The bounded facts extractor stays
   absent, and route selection remains unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

45. `Inventory remaining easy-tier smoke readiness before crate smoke`

   Status: landed.

   Result:

   ```text
   BoxCompilationContext is the current crate-smoke readiness boundary.
   CoreContext, TypeContext, and MetadataContext remain consultation-only,
   and the crate-level probe remains unopened.
   ```

   Rough size:

   ```text
   1-3 tasks
   ```

46. `Select BoxCompilationContext crate smoke probe candidate`

   Status: landed.

   Result:

   ```text
   BoxCompilationContext remains the first crate-level probe candidate. The
   landed slice stays bounded to constructor plus is_empty, and crate-level
   probe selection remains consultation-only.
   ```

   Rough size:

   ```text
   1-3 tasks
   ```

47. `Select minimal BoxCompilationContext crate smoke harness owner`

   Status: landed.

   Result:

   ```text
   The minimal crate smoke harness design is the next owner. The selected
   probe candidate remains BoxCompilationContext, and crate-level probe
   selection remains consultation-only.
   ```

   Rough size:

   ```text
   1-3 tasks
   ```

48. `Define minimal BoxCompilationContext crate smoke harness`

   Status: landed.

   Result:

   ```text
   The minimal crate smoke harness is a thin wrapper over the landed
   readiness, selection, and owner-selection rows. It stays consultation-only
   and keeps the crate-level probe unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

49. `Define minimal BoxCompilationContext crate smoke harness command contract`

   Status: landed.

   Result:

   ```text
   The minimal crate smoke harness command contract is pinned as a thin
   machine-readable wrapper over the landed readiness, selection,
   owner-selection, and harness-design rows. It keeps the crate-level probe
   unopened and does not widen route selection or the nightly rustc adapter.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

50. `Inventory representative BoxCompilationContext crate smoke probe surface`

   Status: landed.

   Result:

   ```text
   The representative crate-smoke probe surface is pinned as a thin
   machine-readable inventory over the landed readiness, selection,
   owner-selection, harness-design, and command-contract rows. It keeps the
   crate-level probe unopened and does not widen route selection or the
   nightly rustc adapter.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

51. `Define representative BoxCompilationContext crate smoke probe output contract`

   Status: landed.

   Result:

   ```text
   The representative crate-smoke probe output contract is pinned as a thin
   machine-readable contract over the landed readiness, selection,
   owner-selection, harness-design, command-contract, and probe-inventory
   rows. It keeps the crate-level probe unopened and does not widen route
   selection or the nightly rustc adapter.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

52. `Define representative BoxCompilationContext crate smoke probe result contract`

   Status: landed.

   Result:

   ```text
   The representative crate-smoke probe result contract is pinned as a thin
   machine-readable contract over the landed readiness, selection,
   owner-selection, harness-design, command-contract, probe-inventory, and
   probe-output-contract rows. It keeps the crate-level probe unopened and
   does not widen route selection or the nightly rustc adapter.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

53. `Define representative BoxCompilationContext crate smoke probe closeout contract`

   Status: landed.

   Result:

   ```text
   The representative crate-smoke probe closeout contract is pinned as a thin
   machine-readable contract over the landed readiness, selection,
   owner-selection, harness-design, command-contract, probe-inventory,
   probe-output-contract, and probe-result-contract rows. It keeps the
   crate-level probe unopened and does not widen route selection or the
   nightly rustc adapter.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

54. `Verify representative BoxCompilationContext crate smoke consultation bundle`

   Status: landed.

   Result:

   ```text
   The representative crate-smoke consultation bundle is pinned as a thin
   machine-readable verification bundle over the landed readiness, selection,
   owner-selection, harness-design, command-contract, probe-inventory,
   probe-output-contract, probe-result-contract, and probe-closeout-contract
   rows. It keeps the crate-level probe unopened and does not widen route
   selection or the nightly rustc adapter.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

55. `Run representative BoxCompilationContext crate smoke consultation bundle`

   Status: landed.

   Result:

   ```text
   The representative crate-smoke consultation bundle is now executable as a
   single guarded run over the landed readiness, selection, owner-selection,
   harness-design, command-contract, probe-inventory, probe-output-contract,
   probe-result-contract, probe-closeout-contract, and verification-bundle
   rows. It keeps the crate-level probe unopened and does not widen route
   selection or the nightly rustc adapter.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

56. `Record representative BoxCompilationContext crate smoke consultation bundle execution summary`

   Status: landed.

   Result:

   ```text
   The representative crate-smoke consultation bundle execution is now
   recorded as a durable summary over the landed readiness, selection,
   owner-selection, harness-design, command-contract, probe-inventory,
   probe-output-contract, probe-result-contract, probe-closeout-contract,
   verification-bundle, and run rows. It keeps the crate-level probe
   unopened and does not widen route selection or the nightly rustc adapter.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

57. `Inventory remaining converter coverage hygiene slices`

   Status: landed.

   Result:

   ```text
   The remaining MirBuilder converter coverage hygiene debt is now
   machine-readable: five raw-harness family-spec slices remain across the
   main_lines carriers, plus one raw ReturnSource contract slice. The typed
   converter core, shared renderer, shared generator layer, and
   ordered-map converter stay intact.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

58. `Choose BindingContext and VariableContext simple-map harness family as first typed harness rewrite slice`

   Status: landed.

   Result:

   ```text
   The BindingContext and VariableContext simple-map harness family is the
   first typed harness rewrite slice because it is the smallest shared
   ordered-map harness cluster, it is already covered by the lightweight
   converter entrypoint, and it can be isolated before BoxCompilationContext,
   snapshot/restore, and carrier snapshot work.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

59. `Define typed harness rewrite contract for BindingContext and VariableContext simple-map`

   Status: landed.

   Result:

   ```text
   The first typed harness rewrite contract is now explicit: the shared
   ordered-map family is the rewrite surface, the current raw main_lines are
   the input carrier, and route selection / nightly rustc adapter / runtime
   fallback stay unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

60. `Define typed harness rewrite emission contract for BindingContext and VariableContext simple-map`

   Status: landed.

   Result:

   ```text
   The typed harness rewrite emission contract is now explicit: the shared
   emitter is the only emission path for BindingContext and VariableContext
   simple-map, and it must not widen route selection or reopen the nightly
   rustc adapter.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

61. `Define typed harness rewrite implementation boundary for BindingContext and VariableContext simple-map`

   Status: landed.

   Result:

   ```text
   The implementation boundary is now explicit: BindingContext and
   VariableContext simple-map remain the first rewrite family, the shared
   emitter remains the only emission path, and route selection / nightly
   rustc adapter / runtime fallback stay unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

62. `Define typed harness rewrite implementation entry contract for BindingContext and VariableContext simple-map`

   Status: landed.

   Result:

   ```text
   The implementation entry contract is now explicit: if implementation starts,
   the first touch set is the shared ordered-map family only, and it must keep
   BindingContext and VariableContext simple-map as the only members.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

63. `Define typed harness rewrite initial patch sequence for BindingContext and VariableContext simple-map`

   Status: landed.

   Result:

   ```text
   The initial patch sequence is now explicit: the shared spec carrier is
   updated first, then the shared builder, then the shared emitter, and only
   after that does the selected family spec host switch to the typed harness
   payload.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

64. `Confirm BoxCompilationContext as the next crate-level probe candidate after the remaining converter coverage hygiene inventory`

   Status: landed.

   Result:

   ```text
   BoxCompilationContext remains the next crate-level probe candidate after
   the remaining converter coverage hygiene inventory, and the probe itself
   stays unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

65. `Select BoxCompilationContext harness as the next raw-string coverage slice after the remaining converter coverage hygiene inventory`

   Status: landed.

   Result:

   ```text
   BoxCompilationContext harness is now the selected next raw-string
   coverage slice after the remaining converter coverage hygiene inventory,
   while other raw-string slices stay deferred.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

66. `Define the typed execution harness IR contract for the selected BoxCompilationContext harness slice`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext harness contract is now explicit as typed
   execution intent, while the raw harness text, route selection, nightly
   rustc adapter, and runtime fallback all remain unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

67. `Define the typed execution harness IR shape contract for the selected BoxCompilationContext harness slice`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext typed harness IR shape is now explicit as a
   minimal consultation-only data shape, while builder behavior, emitter
   behavior, route selection, nightly rustc adapter, and runtime fallback all
   remain unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

68. `Define the acceptance owner and summary contract for the selected BoxCompilationContext typed execution harness IR`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext acceptance owner and summary are now explicit
   as a consultation-only contract, while execution wiring, route selection,
   nightly rustc adapter, and runtime fallback all remain unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

69. `Define the validation boundary contract for the selected BoxCompilationContext typed execution harness IR`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext validation boundary is now explicit as a
   consultation-only contract, while execution wiring, route selection,
   nightly rustc adapter, and runtime fallback all remain unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

70. `Define the validation execution bundle contract for the selected BoxCompilationContext typed execution harness IR`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext validation execution bundle is now explicit as
   a consultation-only descriptor, while execution wiring, route selection,
   nightly rustc adapter, and runtime fallback all remain unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

71. `Define the validation summary artifact contract for the selected BoxCompilationContext typed execution harness IR`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext validation summary artifact is now explicit as
   a consultation-only descriptor, while execution wiring, route selection,
   nightly rustc adapter, and runtime fallback all remain unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

72. `Define the later implementation boundary for the selected BoxCompilationContext typed execution harness IR`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext later implementation boundary is now explicit
   as a consultation-only separator between the typed harness consultation
   slices and any later implementation work.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

73. `Define the validation run summary contract for the selected BoxCompilationContext typed execution harness IR`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext validation run summary is now explicit as a
   consultation-only descriptor, while execution wiring, route selection,
   nightly rustc adapter, and runtime fallback all remain unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

74. `Define the implementation-start boundary contract for the selected BoxCompilationContext typed execution harness IR`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext implementation-start boundary is now explicit
   as the consultation-only point where implementation work would begin,
   while route selection, nightly rustc adapter, and runtime fallback all
   remain unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

75. `Define the implementation entry contract for the selected BoxCompilationContext typed execution harness IR`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext implementation entry is now explicit as the
   consultation-only point where implementation work would actually begin,
   while route selection, nightly rustc adapter, and runtime fallback all
   remain unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

76. `Define the implementation touch set for the selected BoxCompilationContext typed execution harness IR`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext implementation touch set is now explicit as
   the consultation-only first concrete slice that implementation would
   touch, while route selection, nightly rustc adapter, and runtime fallback
   all remain unopened.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

77. `Define the implementation wiring contract for the selected BoxCompilationContext typed execution harness IR`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext implementation wiring is now explicit as the
   consultation-only connection between the first implementation touch set
   and the later implementation boundary.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

78. `Define the initial patch sequence for the selected BoxCompilationContext typed execution harness IR`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext initial patch sequence is now explicit as the
   consultation-only first patch order that follows the implementation
   wiring contract.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

79. `Define the implementation touch set for the selected VariableContext snapshot/restore typed execution harness IR`

   Status: landed.

   Result:

   ```text
   The VariableContext snapshot/restore implementation touch set is now
   explicit as the consultation-only first concrete slice that would follow
   the remaining raw-string debt inventory.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

80. `Define the implementation touch set for the selected CarrierInfo snapshot harness typed execution path`

   Status: landed.

   Result:

   ```text
   The CarrierInfo snapshot harness implementation touch set is now
   explicit as the consultation-only first concrete slice that would follow
   the remaining raw-string debt inventory.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

81. `Replace the VariableContext immutable-borrow ReturnSource contract with an owned snapshot contract decision`

   Status: landed.

   Result:

   ```text
   The VariableContext immutable-borrow surface now has an explicit
   consultation-only decision to replace the raw ReturnSource contract with
   an owned snapshot contract.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

82. `Define the BoxCompilationContext typed execution harness rewrite contract`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext harness rewrite boundary is now explicit as
   the consultation-only contract that follows the remaining raw-string
   debt inventory.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

83. `Define the first representative easy-tier crate-level probe contract for the selected BoxCompilationContext harness path`

   Status: landed.

   Result:

   ```text
   The first representative easy-tier crate-level probe for
   BoxCompilationContext is now explicit as the consultation-only contract
   that follows the landed harness wrapper rows.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

84. `Define the typed harness payload schema for the selected BoxCompilationContext harness path`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext typed harness payload schema is now explicit
   as the consultation-only payload contract that follows the probe
   contract.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

85. `Define the builder rendering contract for the selected BoxCompilationContext typed harness payload schema`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext builder rendering contract is now explicit as
   the consultation-only rendering boundary that follows the payload schema
   contract.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

86. `Define the emitter consumption contract for the selected BoxCompilationContext typed harness payload schema`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext emitter consumption contract is now explicit
   as the consultation-only shared-emitter boundary that follows the
   builder rendering contract.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

87. `Define the family artifact host contract for the selected BoxCompilationContext typed harness payload path`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext family artifact host contract is now explicit
   as the consultation-only host boundary that follows the emitter
   consumption contract.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

88. `Define the artifact manifest contract for the selected BoxCompilationContext typed harness payload path`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext artifact manifest contract is now explicit as
   the consultation-only manifest boundary that follows the family artifact
   host contract.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

89. `Define the typed harness contract for the selected VariableContext snapshot/restore surface`

   Status: landed.

   Result:

   ```text
   The VariableContext snapshot/restore typed harness contract is now
   explicit as the consultation-only surface that follows the
   implementation touch set contract.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

90. `Define the BoxCompilationContext typed harness consultation closeout contract`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext consultation chain is now explicitly closed
   as a consultation-only boundary that follows the artifact manifest
   contract.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

91. `Define the VariableContext snapshot/restore consultation closeout contract`

   Status: landed.

   Result:

   ```text
   The VariableContext consultation chain is now explicitly closed as a
   consultation-only boundary that follows the typed harness contract.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

92. `Define the shared ordered-map family consultation closeout contract`

   Status: landed.

   Result:

   ```text
   The shared ordered-map family consultation chain is now explicitly
   closed as a consultation-only boundary that follows the initial patch
   sequence contract.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

93. `Define the VariableContext immutable-borrow consultation closeout contract`

   Status: landed.

   Result:

   ```text
   The VariableContext immutable-borrow consultation chain is now
   explicitly closed as a consultation-only boundary that follows the
   ReturnSource decision.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

94. `Define the CarrierInfo snapshot harness consultation closeout contract`

   Status: landed.

   Result:

   ```text
   The CarrierInfo consultation chain is now explicitly closed as a
   consultation-only boundary that follows the snapshot harness touch set
   contract.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

95. `Select typed BoxCompilationContext execution harness as the next implementation slice`

   Status: landed.

   Result:

   ```text
   The review consensus now selects the typed BoxCompilationContext
   execution harness as the next implementation slice, while partial crate
   bundle and crate linker work remain parked.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

96. `Define the BoxCompilationContext typed Main operations design`

   Status: landed.

   Result:

   ```text
   The remaining BoxCompilationContext gap is now narrowed to a typed Main
   payload design with a minimal shared vocabulary, while main_lines remains
   parked as the current raw surface until implementation.
   ```

   Rough size:

   ```text
   1 task
   ```

97. `Record the BoxCompilationContext typed Main execution harness implementation closeout`

   Status: landed.

   Result:

   ```text
   The BoxCompilationContext typed Main execution harness is now implemented
   and the remaining raw-string debt no longer includes that family.
   ```

   Rough size:

   ```text
   1 task
   ```

98. `Implement the first representative easy-tier crate-level bundle for BindingContext and VariableContext simple-map`

   Status: landed.

   Result:

   ```text
   The ordered-map crate-level bundle is now implemented as the first
   representative easy-tier executable bridge for BindingContext and
   VariableContext simple-map.
   ```

   Rough size:

   ```text
   1-2 tasks
   ```

99. `Record the VariableContext snapshot/restore typed Main execution harness implementation closeout`

   Status: landed.

   Result:

   ```text
   The VariableContext snapshot/restore typed Main execution harness is now
   implemented and the remaining raw-string debt no longer includes that
   family.
   ```

   Rough size:

   ```text
   1 task
   ```

## Rough Remaining Size

Current estimate after task 99:

```text
converter coverage hygiene:
  2 tasks

first representative easy-tier crate-level probe:
  0 tasks

all inventoried easy-tier candidates:
  17-32 tasks

hard-tier design-stop work:
  33-56 tasks

MirBuilder-wide selfhost remaining:
  roughly 53-88 tasks if all parked hard-tier areas are included
```

## Parked Work

Do not extend the simple-map path into these areas:

```text
full VariableContext claim
variable_map_mut
returned immutable borrow over raw OrderedMapBox
carrier-sensitive behavior
PHI / loop lowering
Drop / unsafe / FFI
crate-wide MirBuilder conversion
runtime try-Hako-then-Rust fallback
```

## Human Design Stops

The converter must fail-fast and ask for design judgment on:

```text
ReturnedReadBorrow
ReturnedMutableBorrow
CarrierSensitiveAlias
PhiJoinRequired
NonTrivialDrop
UnsafeOrFFI
UnresolvedCallTarget
NullableMapValue
NonAsciiOrderedKey
ConstructorLifecycleMismatch
```

No source-name fallback, TODO body, null placeholder body, or runtime
try-Hako-then-Rust fallback is allowed for these stops.
