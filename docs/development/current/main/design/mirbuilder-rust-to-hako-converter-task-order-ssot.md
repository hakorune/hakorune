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

   Next implementation task. Move snapshot/restore from raw Hako body strings
   onto `CloneOwnedMap` and `ReplaceOwnedMap` operation IR.

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
