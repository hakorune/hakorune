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

Build a practical MirBuilder-only converter:

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

   Status: next implementation task.

   Goal:

   ```text
   generated family API method bodies
     = typed operations only

   acceptance Main harnesses
     = allowed to remain as main_lines for now
   ```

   Current inventory:

   ```text
   Real converter debt:
     tools/rust_lifecycle/mirbuilder_family_artifacts.py
       variable_context_immutable_borrow_spec()
         VariableContextApi.variable_map(ctx)
         VariableMapViewApi.is_empty/len/contains/lookup

     tools/rust_lifecycle/shared_mirbuilder_emitter.py
       _render_method_body() body_lines compatibility path

     tools/rust_lifecycle/family_artifact_spec.py
       ApiMethodSpec.body_lines

     tools/rust_lifecycle/family_artifact_builders.py
       _build_api_method_ir() body_lines fallback

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
