---
Status: SSOT
Date: 2026-06-25
Scope: MirBuilder-only Rust-to-Hako converter task order.
Related:
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
  - docs/development/current/main/design/rust-lifecycle-projection-ssot.md
  - docs/reference/architecture/rust-to-hako-lifecycle-projection.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/design/mirbuilder-ordering-capability-ssot.md
  - docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
---

# MirBuilder Rust-to-Hako Converter Task Order

This file is the current task-order entry. It is not a landed-history ledger.
Detailed historical rows live in phase cards and git history.

## Current Target

```text
active blocker:
  MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-001

current implementation task:
  Address the record/packed layout refresh edge derived after ModuleMetadataPublication became available.

selected source slice:
  prepared-state build_module(AST Literal Integer(0)) execution surface

selected lowering:
  explicit artifact contracts -> frontier analyzer -> next unsupported edge

landed evidence:
  Same-module scalar-counter helper execution is green for CoreContextApi
  next_binding / next_temp_slot / next_debug_join through GlobalCallRoute plus
  SameModuleDefinitionPlan.
  Borrow read-fold owned-map merge is green and uses ValueIdOrderedMapBox for
  ValueId/i64 keys. OrderedMapBox remains String-key only.
  Same-module ArrayBox return is green for MultiCarrierExitPhi via
  source-derived default_exit facts, body-wide ObjectHandle return contract,
  SameModuleDefinitionPlan, and backend-ready semantic refresh before ny-llvmc.
  CoreContext ID generators are green through GeneratorStateFacts,
  NominalIdTransportPlan, value_next_id/block_next_id scalar fields, and
  ValueIdAsI64 / BasicBlockIdAsI64 verifier metadata.
  CoreContext artifact contract projection is green: VerifiedHakoFamilyIR plus
  stable Deny results plus artifact identity project the manifest, verifier
  expectation, and guard consumer through VerifiedFamilyArtifactContractV1.
  MirBuilder derived context bundle v1 is green as a membership-only bundle:
  ordered_map_crate_bundle now references CoreContext's
  VerifiedFamilyArtifactContractV1, exercises scalar counters and ID
  generators, and avoids copying family selected methods, semantic transports,
  or denials.
  MirBuilder allocation policy facts are green: live source now projects to
  MirBuilderAllocationPolicyFactsV1, ResolvedValueAllocationPolicyV1, and an
  explicit DirectabilityDecision=Deny until current_function / reserved-set /
  parameter fallback / sentinel / overflow boundaries are selected.
  Function-local ValueId allocator is green: FunctionAllocatorFacts now project
  to FunctionLocalValueIdAllocatorPlanV1 with param_count seeded state,
  ValueIdAsI64 result transport, and oracle vectors for param_count 0/1/3.
  Reserved ValueId exclusion policy is green: ReservedValueExclusionSetFacts
  now project to membership-only rejection with PHI destinations plus JoinIR
  function parameters, consumed rejected candidates, and GenerateNextCandidate
  retry. Concrete representation remains unselected.
  MirBuilder next_value_id composition is green: ResolvedValueAllocationPolicyV1
  now composes FunctionLocalValueIdAllocatorPlanV1 and
  ReservedValueExclusionPolicyPlanV1 into current_function selection plus
  reserved retry oracle vectors.
  Prepared-state next_value_id Hako kernel is adopted into the ordered-map
  crate bundle as membership-only evidence and bundle-level EXE/AOT smoke.
  Prepared-state reserved membership transport is aligned with its projection:
  actual generated storage now uses ValueIdOrderedMapBox and ValueIdOrderedMap.
  Minimal MirBuilder execution path selection is green: live Rust source order
  plus explicit artifact contracts derive the first unsupported edge at
  prepare_module -> MirModule::new without generated Hako, backend, ABI,
  runtime fallback, or mainline-selection changes.
  MirModule shell, MirFunction constructor, literal integer lowering, bounded finalize composition, minimal execution smoke, allocation-policy mainline pilot, ReturnEmission, ReturnTypePublication, CurrentModuleTake, TypedValueDefinitionVerification, CurrentFunctionTake, TypePropagationPipelineExecution, TypeHintProvision, MetadataValueTypePublication, MetadataOriginCallerMerge, PhiReturnTypeInference, PhiInputMaterialization, DevBirthVerification, ModuleFunctionInsertion, ConditionFnInjection, FunctionRegionStackPop, SlotRegistryRelease, and ModuleMetadataPublication are green; the frontier now derives record/packed layout refresh as the next unsupported edge.

selected next owner:
  MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-001

current fail-fast boundary:
  The next slice may only derive the next unsupported live edge. It must not add routes, widen source authority, or add runtime fallback.

latest design decision:
  ModuleMetadataPublication is green as a PlanOnly capability. Proceed to record/packed layout refresh, not typed object refresh, direct state refresh, all-functions PHI materialization, or full finalize.

forbidden:
  callee-name branches; C-side ArrayBox inference; scalar fail-code
  reinterpretation; mixed-runtime promotion; extern fallback; new route kind;
  new canonical MIR instruction; runtime fallback
```

Recent acceptance evidence:

```text
same_module_scalar_counter_routes=green
same_module_scalar_counter_definitions=green
CoreContext scalar-counter EXE/AOT green
multi_carrier_exit_phi ArrayBox return selected to close matrix red edge
multi_carrier_exit_phi ArrayBox return green
core_context_artifact_contract_projection green
mirbuilder_derived_context_bundle_v1 green
mirbuilder_allocation_policy_facts green
function_local_value_id_allocator green
reserved_value_exclusion_policy green
mirbuilder_next_value_id_composition green
allocation_policy_execution_surface_selected=prepared_state_hako_kernel
mirbuilder_next_value_id_prepared_state_kernel green
mirbuilder_allocation_policy_bundle_adoption green
mirbuilder_prepared_state_reserved_membership_transport_alignment green
mirbuilder_minimal_execution_path_selection green
mir_module_minimal_shell_transport green
mir_function_constructor_composition green
mirbuilder_literal_integer_lowering green
mirbuilder_bounded_finalize_composition green
mirbuilder_minimal_execution_path_smoke green
mirbuilder_allocation_policy_mainline_pilot green
mirbuilder_function_region_stack_pop green
mirbuilder_slot_registry_release green
mirbuilder_module_metadata_publication green
derived_first_red_edge=RecordAndPackedLayoutRefreshRequired
full converter matrix green
task-order remains under 800 lines
```

Current mechanical status:

```text
borrow_read_fold_owned_map_merge = landed
boxed_sum_variant_make_site_fact_normalization = landed
metadata_context_region_parent_backend = green
same_module_uniform_mir_scalar_counter_emitter = landed
same_module_arraybox_return_contract = landed
newtype_id_generator_scalarization = landed
core_context_artifact_contract_projection = landed
mirbuilder_derived_context_bundle_v1 = landed
mirbuilder_allocation_policy_facts = landed
function_local_value_id_allocator = landed
reserved_value_exclusion_policy = landed
mirbuilder_next_value_id_composition = landed
allocation_policy_execution_surface_consultation = landed
mirbuilder_next_value_id_prepared_state_kernel = landed
mirbuilder_allocation_policy_bundle_adoption = landed
mirbuilder_prepared_state_reserved_membership_transport_alignment = landed
mirbuilder_minimal_execution_path_selection = landed
mir_module_minimal_shell_transport = landed
mir_function_constructor_composition = landed
mirbuilder_literal_integer_lowering = landed
mirbuilder_bounded_finalize_composition = landed
mirbuilder_minimal_execution_path_smoke = landed
mirbuilder_allocation_policy_mainline_pilot = landed
mirbuilder_return_emission = landed
mirbuilder_return_type_publication = landed
mirbuilder_current_module_take = landed
mirbuilder_typed_value_verification = landed
mirbuilder_current_function_take = landed
mirbuilder_type_propagation_pipeline = landed
mirbuilder_type_hint_provision = landed
mirbuilder_metadata_value_type_publication = landed
mirbuilder_metadata_origin_caller_merge = landed
mirbuilder_phi_return_type_inference = landed
mirbuilder_phi_input_materialization = landed
mirbuilder_dev_birth_verification = landed
mirbuilder_module_function_insertion = landed
mirbuilder_condition_fn_injection = landed
mirbuilder_function_region_stack_pop = landed
mirbuilder_slot_registry_release = landed
mirbuilder_module_metadata_publication = landed
selfhost_checkpoint_lane = artifact_selfhost
```

## Active Next 3

Keep this section short. Detailed landed rows belong in phase cards and git
history, not in this task-order SSOT.

```text
1. Record/packed layout refresh edge
   status=selected
   boundary=finalize_module refresh_module_record_and_packed_layout_plans
   semantic_authority=frontier analyzer plus ModuleMetadataPublication non_claim record_and_packed_layout_refresh=0
   non_authority=manual next-edge selection

2. Next semantic owner after record/packed layout refresh
   status=parked until record/packed layout refresh edge is green
   boundary=first unsupported edge only
   semantic_authority=live source order plus contracts
   non_authority=coverage percentage

3. Wider minimal path mainline
   status=parked until all selected edges have executable artifacts
   boundary=build_module AST Literal Integer(0) only
   semantic_authority=mainline adoption policy; non_authority=smoke alone
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
queued: task-order SSOT を active next 3 + parked index へ圧縮
queued: mirbuilder_family_artifacts.py 分割
  boundary=behavior_preserving_split_only
queued: leaf projection validator 二重化を整理
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
