---
Status: SSOT
Date: 2026-06-27
Scope: MirBuilder-only Rust-to-Hako converter task order.
Related:
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
  - docs/development/current/main/design/rust-lifecycle-projection-ssot.md
  - docs/reference/architecture/rust-to-hako-lifecycle-projection.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/design/mirbuilder-ordering-capability-ssot.md
  - docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
  - docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md
---

# MirBuilder Rust-to-Hako Converter Task Order

This file is the current task-order entry. It is not a landed-history ledger.
Detailed historical rows live in phase cards and git history.

## Current Target

```text
active blocker:
  MIRBUILDER-COMPOSED-PREFIX-GUARD-DRIFT-REPAIR-001

current implementation task:
  MIRBUILDER-COMPOSED-PREFIX-GUARD-DRIFT-REPAIR-001.
  The composed-prefix / continuation / frontier / readiness / adoption-recheck
  guards still pin historical current-state values. Repair the stale
  current-state exact pinning first, then rerun the resolver family before
  touching any new semantic owner.

selected decision slice:
  composed_prefix.current_state
    -> latest_card/latest_card_path presence only
    -> no exact current_blocker_token pin
    -> MIRBUILDER-COMPOSED-PREFIX-GUARD-DRIFT-REPAIR-001

selected evidence:
  guard repair evidence
    -> tools/rust_lifecycle/mirbuilder_minimal_path_composed_execution_continuation.py
    -> tools/rust_lifecycle/mirbuilder_minimal_path_composed_prefix_advance.py
    -> tools/rust_lifecycle/mirbuilder_minimal_execution_path_frontier_resolution.py
    -> tools/rust_lifecycle/mirbuilder_minimal_path_mainline_readiness_resolver.py
    -> tools/checks/rust_lifecycle_mirbuilder_allocation_policy_hako_adoption_decision_recheck_guard.sh
    -> tools/checks/rust_lifecycle_mirbuilder_allocation_policy_hako_adoption_decision_recheck_002_guard.sh
    -> docs/development/current/main/phases/phase-296x/1772-MIRBUILDER-COMPOSED-PREFIX-GUARD-DRIFT-REPAIR-001.md
    -> docs/development/current/main/phases/phase-296x/1771-MIRBUILDER-CONTEXT-HAKO-ADOPTION-DECISION-001.md
    -> docs/development/current/main/phases/phase-296x/1770-MIRBUILDER-CONTEXT-HAKO-NATIVE-SOURCE-OWNER-001.md
    -> docs/development/current/main/phases/phase-296x/296x-1764-MIRBUILDER-MINIMAL-PATH-MAINLINE-PILOT-001.md
    -> docs/development/current/main/CURRENT_STATE.toml
    -> docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
    -> tools/checks/current_state_pointer_guard.sh
    -> design consultation inventory

landed evidence pointer:
  Detailed landed rows live in the guard repair card, readiness resolver,
  and git history. This task-order only keeps the active blocker, fail-fast
  boundary, and Active Next 3.

selected next owner:
  composed-prefix guard drift repair candidate

current fail-fast boundary:
  The minimal-path composed-prefix / continuation / frontier / readiness /
  adoption-recheck family still carries stale current-state exact pins.
  De-drift those guards before resuming the already-ready mainline pilot.

latest design decision:
  Guard drift, not a new semantic owner, is the current blocker. The next
  concrete route after the repair is the already-ready mainline pilot.

## Composed Prefix Evidence

```text
same-state composed prefix evidence
next_unconsumed_edge = Closed
generated Hako executable closure = Closed
ReadyForMinimalPathMainlinePilot
MIRBUILDER-MINIMAL-PATH-MAINLINE-PILOT-001
```

forbidden:
  callee-name branches; C-side ArrayBox inference; scalar fail-code
  reinterpretation; mixed-runtime promotion; extern fallback; new route kind;
  new canonical MIR instruction; runtime fallback
```

## Evidence Pointers

```text
semantic_closure_report =
  docs/development/current/main/design/fixtures/rust-lifecycle/
  minimal-mirbuilder-execution-path-semantic-closure-report-v0.json

latest_frontier_card =
  docs/development/current/main/phases/phase-296x/
  296x-1745-MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001.md

latest_integration_card =
  docs/development/current/main/phases/phase-296x/
  296x-1763-MIRBUILDER-MINIMAL-PATH-MAINLINE-READINESS-RESOLVER-001.md

latest_guard_repair_card =
  docs/development/current/main/phases/phase-296x/
  1772-MIRBUILDER-COMPOSED-PREFIX-GUARD-DRIFT-REPAIR-001.md

latest_adoption_card =
  docs/development/current/main/phases/phase-296x/
  1771-MIRBUILDER-CONTEXT-HAKO-ADOPTION-DECISION-001.md

latest_native_owner_card =
  docs/development/current/main/phases/phase-296x/
  1770-MIRBUILDER-CONTEXT-HAKO-NATIVE-SOURCE-OWNER-001.md

frontier_resolution_card =
  docs/development/current/main/phases/phase-296x/
  296x-1757-MIRBUILDER-MINIMAL-EXECUTION-PATH-FRONTIER-RESOLUTION-001.md
```

## Active Next 3

Keep this section short. Detailed landed rows belong in phase cards and git
history, not in this task-order SSOT.

```text
1. Composed-prefix guard drift repair
   status=selected
   boundary=stale current-state exact pins in the composed-prefix family
   semantic_authority=current-state pointer existence, semantic closure report,
   and route evidence
   non_authority=historical current_blocker_token exact matches

2. Resolver family rerun after the guard repair
   status=parked until the repair lands
   boundary=continue, frontier, and readiness checks rerun on the current state
   semantic_authority=the existing resolver fixtures
   non_authority=manual next-owner selection

3. Minimal-path mainline pilot
   status=parked until readiness stays green after the repair
   boundary=the already-ready minimal-path route remains the next concrete lane
   semantic_authority=readiness resolver plus composed execution closure
   non_authority=Source Selfhost, Rust deletion, new semantic owner selection
```

## Landed Converter Capability Summary

```text
ordered-map contexts, snapshots, carrier projection, scalar counters
TypeContext value-kind / origin-map / value-type / snapshot-restore
MetadataContext scalar/source-file, value-caller
MetadataContext region-parent EXE/AOT
structured loop, scalar loop carrier, explicit PHI, multi-exit PHI
RegionObserver source-ordered read-fold and SlotMetadata output
boxed native enum ABI and boxed enum container round trip
mixed runtime value carrier for MapBox and ArrayBox
generic method route descriptor SSOT and mismatch diagnostics
generic read-fold operation decomposition
TypeContext string literal leaf projection
boxed-sum const payload definition index
boxed-sum lowering facade
C shim variant binding fact owner drain
same-module typed-field RMW fusion plan
same-module result-capsule reset batch fusion plan
same-module boxed-sum handle fact owner selected
same-module explicit boxed-sum value fact
generic-method explicit boxed-sum result fact
```

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
| `map.optional_immutable_atom` | `MapGetOption`, `MapSet`, `MapClear` | landed |
| `aggregate.take_restore_with_defaults` | `MoveFieldAndResetSource`, `AssertNotConsumed`, `MarkConsumed` | landed |
| `control.structured_loop_without_carried_state` | `StructuredLoop`, `ArrayPush`, `Assign`, `ReturnI64` | landed |
| `control.single_scalar_loop_carrier` | `StructuredLoop`, `Assign`, `ReturnSource` | landed |
| `control.canonical_explicit_phi` | `ExplicitPhiI64`, `ReturnSource` | landed |
| `control.multi_carrier_exit_phi` | `ExplicitMultiExitPhiI64Array`, `ReturnSource` | landed |
| `map.immutable_leaf_projection` | `MapGetOption` | active |
| `borrow_use.sequence_last_copy` | `SequenceLastOption` | landed |
| `borrow.read_fold` | map/sequence fold into owned output | landed |

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
general Option payload support
InlineRecord / packed / SoA SlotMetadata transport
nightly rustc adapter for easy-tier families
runtime try-Hako-then-Rust fallback
new Hako pointer syntax
```

## Task Hygiene Backlog

Keep this lane separate from semantic converter slices:

```text
landed: guard表示の false-green 修正
landed: current docs を thin pointer 化
landed: task-order SSOT を active next 3 + parked index へ圧縮
  boundary=keep task-order as pointer; detailed artifact/evidence rows belong
  to semantic closure reports, phase cards, and git history
landed: compiler projector helper support box v0
  boundary=small lang/src/compiler/lib helper only (_tag/fail/require/copy);
  first users are ReturnEmission, FunctionRegionStackPop, and
  SlotRegistryRelease projectors; no projector framework or semantic DSL
landed: Python semantic projector freeze reverse coverage hardening
  boundary=reverse-enumerate tools/rust_lifecycle/*.py roles and require
  exception tokens for new SemanticProjector files
landed: mirbuilder_family_artifacts.py 分割
  boundary=behavior_preserving_split_only
landed: leaf projection validator 二重化を整理
  boundary=one validator owns map.immutable_leaf_projection acceptance
```

## C ABI Shim Responsibility Cleanup Backlog

See [c-abi-shim-responsibility-cleanup-backlog-ssot.md](./c-abi-shim-responsibility-cleanup-backlog-ssot.md) for the full P0/P1/P2 cleanup inventory.

## MIR Instruction SSOT Cleanup Backlog

This is a cleanup lane, not the active boxed enum ABI blocker.

Accepted finding:

```text
instruction enum / backend ledger / INSTRUCTION_SET.md counts are partially
sync-tested, but docs/reference/mir/json_v0.schema.json is not part of that
sync contract.

src/mir/contracts/backend_core_ops.rs also mixes:
  instruction tag/cohort classification
  per-backend support policy
  ledger constants
  sync tests

docs/reference/mir/INSTRUCTION_SET.md and docs/reference/mir/json_v0.schema.json
are independently maintained outputs today. They are not generated from
src/mir/instruction.rs.
```

Task order:

```text
P1. Add JSON schema to MIR instruction SSOT sync coverage
    - extend the existing backend_core_ops doc-sync tests or add a small
      adjacent test module
    - assert doc <-> ledger <-> json_v0.schema.json agree on kept JSON ops
    - ensure VariantMake/VariantTag/VariantProject and MemOp stay schema-visible
    - no generator
    - no backend behavior change

P2. Generate doc machine-readable rows and JSON schema from enum metadata
    - only if instruction vocabulary starts changing frequently
    - doc / JSON become derived outputs, not independent sources

P3. Split backend_core_ops.rs owners
    - structural instruction classification near the enum/introspection layer
    - per-backend support policy in a policy module
    - tests outside the mixed owner file where practical
```

Immediate recommendation:

```text
implement P1 when the current boxed enum ABI slice has a clean stopping point.
park P2/P3 until churn justifies the extra generator/refactor machinery.
do not block boxed enum ABI work on this cleanup backlog.
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

## Completion Boundary

MirBuilder-wide selfhost still has mutable alias, Drop, unsafe/FFI, boxed scalar
payloads, and broader native adoption parked as explicit design stops. Do not
hide them inside the current leaf-projection lane.
