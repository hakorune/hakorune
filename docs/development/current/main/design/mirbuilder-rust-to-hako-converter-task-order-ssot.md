---
Status: SSOT
Date: 2026-06-23
Scope: MirBuilder-only Rust-to-Hako converter task order.
Related:
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
  - docs/development/current/main/design/rust-lifecycle-projection-ssot.md
  - docs/reference/architecture/rust-to-hako-lifecycle-projection.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/design/mirbuilder-ordering-capability-ssot.md
---

# MirBuilder Rust-to-Hako Converter Task Order

This file is the current task-order entry. It is not a landed-history ledger.
Detailed historical rows live in phase cards and git history.

## Current Target

```text
active blocker:
  LOWER-REGION-OBSERVER-SOURCE-ORDERED-READ-FOLD-001

current implementation task:
  Lower RegionObserver through verified source-ordered read-fold

selected source slice:
  variable_map().iter() observer fold

blocked lowering:
  Rust facts
    -> BorrowUseFacts
    -> StorageAccessFacts
    -> order=KeyAscending(RustStringOrdV1)
    -> ElideToReadFold
    -> owned SlotMetadata output

blocker evidence:
  RegionObserver probe with insertion order b, a, args reaches MIR/EXE after
  loop enum-constructor acceptance, but the `.hako` OrderedMapBox output does
  not prove Rust BTreeMap<String> ordering. SourceOrdered conversion must not
  silently downgrade to insertion order.

current decision:
  ORDERED-MAP-SOURCE-ORDERED-STRING-COMPARE-001 is closed as fail-closed.
  The total-order comparator is now proved across VM/EXE/AOT and consumed by
  OrderedMapBox.

  SOURCE-ORDERED-UNBLOCK-ROUTE-DESIGN-001 is closed as Option 1:
  implement backend-accepted StringBox lexical comparison as a generic
  ComparatorCapability, not as an OrderedMapBox / RegionObserver special case.

implemented route-selection guardrail:
  `tools/rust_lifecycle/mirbuilder_region_observer_variable_map_route.py`
  extracts the live source line, accepts the comparator proof, and now denies
  artifact generation at the next precise boundary:
    Deny(UnsupportedOutputTransport)
    detail=OutputTransportUndecided
    output=Vec<SlotMetadata>

current design stop:
  Do not invent SlotMetadata / RefSlotKind transport inside the RegionObserver
  route. The next design decision must fix the owned output representation
  before generated Hako can be claimed.

forbidden:
  raw aggregate map return
  read-view / lease framework
  new Hako pointer syntax
  source-name hardcode
  runtime fallback
```

Acceptance for the current task:

```text
SourceOrdered read-fold conversion is denied before artifact generation
standalone value_origin_callers() conversion -> Deny(ReturnedReadBorrow)
variable_map().iter() source has exact file:line evidence
known ordered observer consumer -> ElideToReadFold only for live source
order=SourceOrdered is preserved or the slice is denied
raw aggregate alias = 0
element reference escape = 0
unknown consumer -> Deny(ReturnedReadBorrow)
owner mutation during projected use -> Deny(ReturnedReadBorrow)
generated .hako MIR green
generated .hako EXE green
rust_mirbuilder_converter_matrix_guard green
```

Current mechanical status:

```text
region-observer variable_map read-fold route = Deny(UnsupportedOutputTransport)
comparator proof = VmExeAotAccepted
slot_metadata_output_transport_claim = 0
generated_hako = 0
next step = decide SlotMetadata / RefSlotKind owned output transport
ordering SSOT = docs/development/current/main/design/mirbuilder-ordering-capability-ssot.md
```

## Active Task Order

0. `Define total text ordering capability`

   Status: landed.

   Scope:

   ```text
   IterationOrder::KeyAscending(RustStringOrdV1)
   CompareTotal(comparator=RustStringOrdV1)
   Deny(UnsupportedOrderCapability)
   detail=ComparatorUnavailable
   ```

   This is converter IR / capability work under `tools/rust_lifecycle`. Do not
   put the intermediate order model in `crates/hakorune_mir_builder`.

0.5. `Implement backend-accepted total text ordering capability`

   Status: landed.

   Scope:

   ```text
   CompareTotal(RustStringOrdV1)
   VM / EXE / AOT acceptance
   equal / less / greater / prefix / non-ASCII oracle cases
   ```

   Forbidden:

   ```text
   OrderedMapBox-name backend branch
   RegionObserver-name backend branch
   runtime fallback
   locale-dependent compare
   ```

0.6. `Use total text ordering in OrderedMapBox`

   Status: landed.

   Scope:

   ```text
   OrderedMapBox.set uses TextOrder.compare_rust_string_v1
   b, a, args -> a, args, b
   update existing key
   remove
   clone_owned
   clear
   ```

   This consumes the generic comparator capability from `apps/lib/collections`
   without adding backend branches or RegionObserver-specific policy.

0.7. `Lower RegionObserver through verified source-ordered read-fold`

   Status: design stop.

   Scope:

   ```text
   order = KeyAscending(RustStringOrdV1)
   comparator proof = VM/EXE/AOT accepted
   output = owned SlotMetadata sequence
   raw aggregate borrow = 0
   insertion-order substitution = 0
   ```

   Current blocker:

   ```text
   Vec<SlotMetadata> output transport is undecided.
   RefSlotKind representation is undecided.
   classify_slot(builder, vid, name) lowering must not be special-cased by
   RegionObserver name.
   ```

1. `Generalize access capabilities through value-caller clone elimination`

   Status: landed.

   Scope:

   ```text
   value_origin_callers().get(&id).cloned()
     -> StorageAccessFacts
     -> ElideToLeafProjection
     -> MapGetOption
   ```

   This is the first slice that makes `StorageAccessFacts` live. Keep
   `BorrowUseFacts` as the Rust adapter input, then normalize into the
   language-neutral access facts before lowering.

2. `Inventory real live read-fold consumers`

   Status: landed.

   Scope:

   ```text
   Find actual Rust source shapes, not planned/docs examples, where an
   aggregate borrow is consumed by an owned read fold.
   ```

   Evidence note:

   ```text
   Current source has value_origin_callers().get(&dst).cloned(), which is
   already covered by ElideToLeafProjection. The previously listed
   value_origin_callers().iter() owned-copy shape is not present in the current
   src/mir/builder/emission/phi_lifecycle.rs source.

   The inventory found the actual read-fold shape in:
     src/mir/builder/module_lifecycle.rs
     src/mir/builder/calls/lowering.rs
   ```

   Do not add generated methods or emitter operations for a source shape that
   is not present.

3. `Resolve MapBox key-domain preserving read-fold acceptance`

   Status: landed for the MetadataContext.value_origin_callers slice.

   Scope:

   ```text
   Required only if the selected live read-fold source uses MapBox key
   iteration with ValueIdAsI64 transport.
   ```

   Current evidence:

   ```text
   MapKeyDomain::from_text("7") normalizes to CanonicalI64(7), so canonical
   numeric public text round-trips to the i64 key domain. Keep this as an
   explicit verifier condition if MapBox.keys() is used for ValueId-key folds.
   ```

4. `Implement selected live read-fold slice`

   Status: landed for MetadataContext.value_origin_callers.

   Scope:

   ```text
   source-specific borrow facts
     -> StorageAccessFacts
     -> ElideToReadFold
     -> typed Hako operation
     -> MIR/EXE behavior green
   ```

   Do not expose a map view or public snapshot API for this slice.

5. `Accept enum constructors in loop value lowering`

   Status: landed.

   Scope:

   ```text
   ASTNode::FromCall for known enum constructors
     -> CoreEffectPlan::VariantMake
     -> MirInstruction::VariantMake
   ```

   This is generic compiler acceptance. It does not add RegionObserver,
   MirType, or ArrayBox-specific lowering.

6. `Fix OrderedMapBox source-ordered String compare`

   Status: next.

   Scope:

   ```text
   variable_map().iter()
     -> ElideToReadFold
     -> order=SourceOrdered
     -> cross-context TypeContext reads verified
     -> owned SlotMetadata output
   ```

   This slice must distinguish BTreeMap source ordering from unordered map
   folds. Do not use a generic unordered iteration rule here.

7. `Implement ordered observer fold for VariableContext`

   Status: parked behind OrderedMapBox source-order proof.

   Scope:

   ```text
   variable_map().iter()
     -> ElideToReadFold
     -> order=SourceOrdered
     -> cross-context TypeContext reads verified
     -> owned SlotMetadata output
   ```

8. `Reassess returned mutable borrow`

   Status: parked behind the three read-borrow elimination slices.

   Standalone returned mutable aliases remain `Deny(ReturnedMutableBorrow)`.
   Only explicit mutation APIs or bounded with-map operations may reopen this.

9. `Reassess NonTrivialDrop / unsafe capability boundaries`

   Status: parked.

   Do not add cleanup, lease, or unsafe syntax while the selected MirBuilder
   slices are still solvable by value projection, read fold, or owned transfer.

## Direct-Lowering Policy

The converter is direct-first:

```text
Rust source
  -> lightweight body/signature facts
  -> directability check
  -> typed VerifiedHakoFamilyIR
  -> shared emitter
  -> runnable native-shaped .hako
```

The older lifecycle vocabulary may remain as provenance/guard input for
families that already use it, but it is not the standard path for simple
mechanical shapes.

Use direct shape lowering when all are true:

```text
source body has a bounded shape
all calls are in the allowed vocabulary
field ownership is local to the translated box
no returned mutable alias escapes
no Drop / unsafe / FFI is required
control-flow and PHI facts are explicit when present
generated operation IR is typed before emission
```

When directness cannot be proved, emit a stable `Deny(reason)`. Do not emit
fallback Hako, TODO bodies, null placeholder bodies, or try-Hako-then-Rust
runtime routes.

## Shape Rule Table

The active rule table uses shape names, not family names.

| Shape | Operation family | Status |
| --- | --- | --- |
| `single_ordered_map_context` | `NewOrderedMap`, `MapGetCopied`, `MapHas`, `MapLength`, `MapIsEmpty`, `MapSet`, `MapRemove`, `MapClear` | landed |
| `owned_ordered_map_snapshot` | `CloneOwnedMap`, `ReplaceOwnedMap` | landed |
| `multi_ordered_map_context` | `NewOrderedMap`, `AllMapsEmpty` | landed |
| `scalar_counter_context` | `InitFieldConst`, `TakeThenSaturatingIncrementU32`, `ReturnI64` | landed |
| `owned_map_carrier_projection` | `CarrierSnapshotFromOwnedMap`, `ExplicitCarrierSnapshotFromOwnedMap` | landed |
| `map.optional_copy_default` | `NewMap`, `MapGetOption`, `MapSet`, `ReturnDefaultIfMissing` | landed |
| `map.optional_owned_atom` | `MapGetOption`, `MapSet`, `MapClear` | landed |
| `aggregate.take_restore_with_defaults` | `MoveFieldAndResetSource`, `AssertNotConsumed`, `MarkConsumed` | landed |
| `control.structured_loop_without_carried_state` | `StructuredLoop`, `ArrayPush`, `Assign`, `ReturnI64` | landed |
| `control.single_scalar_loop_carrier` | `StructuredLoop`, `Assign`, `ReturnSource` | landed |
| `control.canonical_explicit_phi` | `ExplicitPhiI64`, `ReturnSource` | landed |
| `control.multi_carrier_exit_phi` | `ExplicitMultiExitPhiI64Array`, `ReturnSource` | landed |
| `borrow.leaf_projection` | `MapGetOption`, `SequenceLastOption` | active |
| `borrow.read_fold` | map/sequence fold into owned output | queued |

Do not create rules like `type_context.value_kind_map_context`; that is a
family-specific hardcode table under another name.

## Storage Access Facts

`BorrowUseFacts` is Rust-specific adapter input. It is not the universal model
for all source languages. Normalize source-specific references into
`StorageAccessFacts` before Hako lowering:

```text
source-specific facts
  Rust borrow / Go pointer-slice-map / C pointer
        ↓
StorageAccessFacts
        ↓
lowering decision
```

Use small orthogonal facts:

```text
carrier:
  Value | Place | SharedHandle | Span | RawAddress

access:
  Read | ReadWrite | Atomic

alias:
  Unique | Shared | Unknown

lifetime:
  Lexical | OwnerBound | Managed | Foreign | Untracked

escape:
  None | Return | Store | ForeignRetained

order:
  Unobserved | Unspecified | SourceOrdered

cleanup:
  Trivial | Managed | ExplicitRelease | CustomDrop
```

Lowering decisions:

```text
ElideToLeafProjection
ElideToReadFold
FreezeOwned
KeepSharedHandle
MaterializeSharedCell
MaterializeSpan
RequireUnsafeCapability
Deny
```

Current Rust borrow path:

```text
Rust lightweight facts
  -> BorrowUseFacts
  -> StorageAccessFacts
  -> BorrowLoweringDecision
```

Future language adapters can enter directly at `StorageAccessFacts`:

```text
Go map:
  SharedHandle(Map)

Go slice:
  Span or SliceDescriptor(backing=SharedHandle(Buffer), offset, len, cap)

Go address-taken scalar:
  SharedCell only when shared addressable mutation is required

C / unsafe Rust pointer:
  RawAddress, then RequireUnsafeCapability or Deny
```

## Hako Syntax Boundary

Do not add source-language pointer syntax for this lane:

```text
no general &
no general *
no arrow / ->
no general borrow lifetime syntax
no raw pointer syntax in safe Hako
```

If a future source needs shared mutable or span semantics frequently, add a
capability type first:

```text
SharedCell<T>
Span<T>
Slice<T>
ValidatedHandle<T>
RawPtr<T> only inside an unsafe capability boundary
```

Unsafe and foreign are separate axes:

```text
unsafe:
  memory-safety obligation is not compiler-proved

foreign:
  ABI / external symbol / layout boundary
```

Use the stable top-level deny reason and detail fields:

```text
Deny(UnsafeOrFFI)
  detail=RequireUnsafeCapabilityBoundary
  detail=RawAddressRequired
  detail=PointerArithmeticRequired
  detail=UntrackedAliasRequired
  detail=ForeignCallRequired
  detail=LayoutDependentCastRequired
  detail=ManualLifetimeRequired
```

Output from such a boundary may only be a safe value, owned aggregate, owned
buffer, validated opaque handle, or verified box.

## Stable Deny Reasons

Use medium-grained reasons:

```text
UnsupportedResolvedCallTarget
UnsupportedDirectShape
UnsupportedTypeTransport
UnsupportedKeyTransport
UnsupportedOrderCapability
NullableMapValue
DefaultSemanticMismatch
UnstructuredControlFlow
LoopCarriedStateRequired
PhiJoinRequired
ReturnedReadBorrow
ReturnedMutableBorrow
CarrierSensitiveAlias
NonTrivialDrop
UnsafeOrFFI
ConstructorLifecycleMismatch
```

Do not encode family names in Deny reasons.

## Parked Backlog

These are intentionally not part of the current task:

```text
full MirBuilder crate claim
crate-wide generated-to-native authority cutover
variable_map_mut raw alias
live read-view / lease framework
general Drop / RAII lowering
nightly rustc adapter for easy-tier families
runtime try-Hako-then-Rust fallback
new Hako pointer syntax
```

## Fast-Path Lowering Reminder

Fast-path lowering is important for the long-term speed goal, but it is a
backend/perf lane, not the active Rust-to-Hako converter task.

Tracker:

```text
docs/development/current/main/design/perf-owner-first-optimization-ssot.md
```

Current status:

```text
fastpath_analysis_ready=partial
fastpath_backend_consumption=parked
speed_goal_blocker=backend_lowering_consumer_missing
optimization_open=0
```

Parked task order when perf/backend work reopens:

```text
1. Inventory fast-path fact consumers.
2. Implement the first backend fact consumer.
   preferred first target: DirectArrayI64 / ArrayBox i64 lowering
3. Measure exact / meso / whole fronts.
4. Select the next owner only after measurement.
```

Do not block MirBuilder migration on this backlog. Do not add Hako syntax for
fast paths. Backend facts must be consumed by lowering before claiming speed.

## Completion Estimate

Short-term active read-borrow sequence:

```text
value-caller clone elimination:
  1 commit

metadata caller owned read-fold:
  1-2 commits

variable-map ordered observer fold:
  2-4 commits, because TypeContext cross-read facts must be verified
```

Remaining MirBuilder-wide selfhost work still includes mutable alias, Drop,
unsafe/FFI, and broader native adoption. Treat those as separate design stops,
not hidden work inside the current aggregate-borrow lane.
