---
Status: SSOT
Date: 2026-06-24
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
  GENERIC-ROUTE-DESCRIPTOR-FULL-GENERATION-001

current implementation task:
  Derive backend route contract fields from neutral generic-method route
  descriptors instead of handwritten C registry tuple copies.

producer responsibility stack:
  Source preparation
    -> Canonical MIR compile
    -> MIR finalize / semantic refresh
    -> MIR JSON serialization
    -> ny-llvmc consumption

selected source slice:
  spec/mir/generic_method_routes.toml

selected lowering:
  routes.c_need_kind / routes.emit_kind
    -> generated C need-kind / emit-kind registry fields

landed evidence:
  RegionObserver SlotMetadata LLVM/AOT green; mixed runtime value carrier,
  stale NyRT fail-fast, generated route descriptors, route mismatch
  diagnostics, generic read-fold decomposition, boxed-sum I64 payload ABI,
  MetadataContext region-parent EXE/AOT acceptance, boxed-sum site metadata,
  C shim payload_type fallback removal, boxed-sum const payload definition
  indexing, boxed-sum lowering facade, variant binding fact owner drain, and
  explicit boxed-sum value facts for same-module and generic-method results
  plus MIR-call route policy legacy generic_method_routes fallback removal,
  MIR-call need-name fallback auditing, object-storage plan name-inference
  drain, exact-seed route quarantine, same-module definition edge plans, and
  constructor birth LoweringPlan facts are landed.

selected next owner:
  GENERIC-ROUTE-DESCRIPTOR-FULL-GENERATION-001

selected transport:
  SlotMetadata / RefSlotKind output transport is selected:

  - RefSlotKind is native enum; SlotMetadata is semantic OwnedProduct.
  - Current transport is ArrayBox<SlotMetadataBox>; future transport may become
    InlineRecord / packed / SoA without changing read-fold semantics.

current fail-fast boundary:
  no Option-name, MetadataContext-name, payload_type spelling, or raw i64
  sign inference fallback. C shims may emit selected same-module helpers, but
  must not discover selected fusion windows from neighboring instructions.
  `__hako_sum_` box-name prefix may not be used as a new proof source.

latest design decision:
  Boxed-sum handle proof is explicit per-value representation metadata:

  - semantic authority is ValueRepresentationFact::BoxedSumHandle { abi_plan_id }.
  - variant_binding remains local tag/payload tracking only.
  - `__hako_sum_` prefix, enum_name, box_type spelling, and raw i64 sign are
    forbidden proofs.

forbidden:
  raw aggregate map return; read-view / lease framework; new Hako pointer
  syntax; source-name hardcode; runtime fallback
  generated-Hako source-shape workaround for runner MIR drift
  i64 sign-based value classification
  MapBox-only transport fixes that leave ArrayBox ambiguous
  backend-local route descriptor copies
  diagnostics that become a second route classifier
  `ReadFoldSlotMetadata` compatibility renderer reintroduction
```

Recent acceptance evidence:

```text
TypeContext string literal artifact regenerates deterministically
MapGetOption is reused; new operation kind = 0
producer-side emit_string is harness-only prefill, not converted
full map-value publication claim = 0
MIR/EXE/LLVM-AOT focused guard green
boxed I64 payload focused probe EXE/AOT green
unit enum and handle-payload regressions green
metadata_context_region_parent EXE/AOT focused guard green
```

Current mechanical status:

```text
comparator proof = VmExeAotAccepted
region_observer_slot_metadata = LLVM/AOT green
boxed_runtime_v1_make_tag_project = landed
boxed_enum_mapbox_option_roundtrip = landed
boxed_sum_i64_payload_abi = landed
metadata_context_region_parent_backend = landed
same_module_typed_field_rmw_fusion_plan = landed
same_module_result_capsule_reset_batch_plan = landed
same_module_sum_handle_fact_owner = accepted
explicit_boxed_sum_value_fact_same_module = landed
generic_method_boxed_sum_result_fact = landed
variant_binding_boxed_sum_plan_index = landed
c_abi_shim_responsibility_inventory = landed
mir_call_constructor_birth_fact_drain = landed
mir_call_constructor_name_fallback_retired = landed
mir_call_array_text_observer_need_drain = landed
mir_call_generic_method_result_origin_and_get_policy_publish_drain = landed
mir_call_generic_method_receiver_origin_drain = landed
mir_call_array_string_birth_promotion_prepass_drain = landed
mir_call_array_string_promotion_value_origin_drain = landed
generic_method_emit_global_print_substring_arg0_and_value_origin_drain = landed
mir_call_runtime_map_has_need_fallback_drain = landed
mir_call_extern_result_origin_and_redundant_prepass_drain = landed
mir_call_extern_string_route_specs = landed
mir_call_extern_string_name_fallback_retired = landed
mir_call_generic_method_emit_fallback_drain = landed
generic_method_match_emit_fallback_drain = landed
generic_method_legacy_route_scan_drain = landed
mir_call_route_policy_drain = landed
mir_call_prepass_fact_owner_drain = landed
mir_call_need_name_fallback_audit = landed
object_storage_plan_name_inference_drain = landed
exact_seed_route_quarantine = landed
same_module_definition_edge_plan = landed
slot_classifier_policy = verified operation data
collection_runtime_value_carrier = landed for MapBox and ArrayBox
nyrt_freshness_fail_fast = landed for --no-build AOT harness
generic_method_route_descriptor_ssot = landed for Rust/C/Python generated tables
generic_method_route_mismatch_diagnostics = landed for first descriptor field
generic_read_fold_operation_decomposition = landed
type_context_string_literal_leaf_projection = landed
task_hygiene_next3 = landed
ordering SSOT = docs/development/current/main/design/mirbuilder-ordering-capability-ssot.md
```

## Active Next 3

Keep this section short. Detailed landed rows belong in phase cards and git
history, not in this task-order SSOT.

```text
1. Generic route descriptor full generation
   status=selected
   boundary=C registry rows keep policy deltas; route contract fields come from
     spec/mir/generic_method_routes.toml route descriptors
   semantic_authority=neutral route descriptor manifest
   non_authority=handwritten C emit-kind / need-kind tuple copies

2. Helper-symbol override drain
   status=parked
   boundary=concrete helper variants move from row overrides to descriptor data
   semantic_authority=route descriptor manifest
   non_authority=C row helper_symbol override except temporary array-store split

3. Same-module / extern route descriptor generation
   status=parked
   boundary=non-generic route metadata gets equivalent generated descriptor rows
   semantic_authority=neutral route manifests
   non_authority=C-side name or tuple fallback classifiers
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

This is a BoxShape cleanup lane. It does not change the current converter
semantic claim, and it must not be used to add backend-local special cases.

Accepted finding:

```text
lang/c-abi/shims/*.inc is still accumulating responsibilities beyond
"consume selected MIR/lowering facts and emit C/LLVM glue".

The current boxed-sum I64 slice is behaviorally valid, but several shims now
derive or rediscover facts that should be owned upstream:

  - boxed sum payload storage is inferred from payload_type spelling
  - boxed sum I64 const payloads are recovered by scanning earlier MIR
  - boxed sum variant lowering is duplicated across generic and same-module
    emit paths
  - same-module RMW/window fusion is matched in C shims
  - generic route truth is duplicated across generated and hand-written tables
  - prepasses derive value/origin/variant/phi facts instead of only verifying
```

Initial inventory:

| Priority | File | Finding | Category | Intended owner |
| --- | --- | --- | --- | --- |
| P0 | `hako_llvmc_ffi_pure_compile_variant_dispatch.inc` | variant make/project still falls back from missing site metadata to `payload_type` spelling | `spelling_inference` | MIR JSON site fact |
| P0 | `hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc` | same-module variant make/project repeats the same payload spelling fallback | `spelling_inference` | MIR JSON site fact |
| P0 | `hako_llvmc_ffi_pure_compile_boxed_sum_emit.inc` | I64 payload make scans prior instructions to recover the defining const | `definition_discovery` | ValueId definition fact |
| P0 | `hako_llvmc_ffi_pure_compile_boxed_sum_emit.inc` | boxed-sum make/tag/project selection is duplicated behind generic and same-module dispatch | `route_policy` | boxed-sum lowering facade |
| P0 | `hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc` | variant binding / origin facts are derived in C before lowering | `value_class_derivation` | SemanticRefresh / LoweringPlan facts |
| P0 | `hako_llvmc_ffi_same_module_prepass.inc` | same-module path derives similar variant/origin facts separately | `value_class_derivation` | SemanticRefresh / LoweringPlan facts |
| P1 | `hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc` | get/binop/set fusion is rediscovered from neighboring ops | `fusion_window_discovery` | same-module fusion plan |
| P1 | `hako_llvmc_ffi_mir_call_prepass.inc` | call route need/origin facts are still assembled in C | `route_policy` | generated route descriptor / LoweringPlan row |
| P1 | `hako_llvmc_ffi_mir_call_surface_policy.inc` | call surfaces are classified from constructor/global/extern string names | `route_policy` | call surface normalization / LoweringPlan row |
| P1 | `hako_llvmc_ffi_mir_call_need_name_fallback.inc` | retired; builtin print need now comes from lowering_plan metadata | `route_policy` | keep deleted |
| P1 | `hako_llvmc_ffi_generic_method_policy.inc` | collection route policy still uses receiver origin and value class checks | `route_policy` | generic method route planner |
| P1 | `hako_llvmc_ffi_generic_method_match.inc` | route matching still depends on local value-shape flags | `value_class_derivation` | LoweringPlan route value-class row |
| P1 | `hako_llvmc_ffi_string_concat_window.inc` | string corridor discovers single-use / definition windows locally | `definition_discovery` | string corridor/window planner |
| P1 | `hako_llvmc_ffi_string_concat_match.inc` | string concat windows are discovered from neighboring MIR ops | `fusion_window_discovery` | string kernel lowering window routes |
| P1 | `hako_llvmc_ffi_string_chain_policy.inc` | concat chain route is classified from consumers and const suffix shape | `route_policy` | string route planner / value consumer facts |
| P1 | `hako_llvmc_ffi_object_storage_plan.inc` | object storage names are still used as interpretation inputs | `object_storage_inference` | object storage plan |
| P1 | `hako_llvmc_ffi_same_module_value_metadata.inc` | `__hako_sum_` prefix publishes sum handles | `value_class_derivation` | boxed-sum site/value fact |
| P2 | `hako_llvmc_ffi_pure_compile.inc` and seed emitters | exact seed tags dispatch legacy routes by string | `exact_seed_fallback` | exact-route rows or retirement |
| P2 | `hako_llvmc_ffi_generic_method_len_policy.inc` | length route uses known string length / placement windows before runtime len | `value_class_derivation` | string corridor facts / lowering route facts |

Pure glue / generated outputs:

```text
hako_llvmc_ffi_generic_method_route_registry.inc
  generated from spec/mir/generic_method_routes.toml; keep as generated output.

hako_llvmc_ffi_common.inc
  shared helpers and env parsing; not part of semantic route ownership unless
  a helper starts choosing backend policy.

hako_llvmc_ffi_lowering_plan_metadata.inc
  structured reader for published lowering plan rows.

hako_llvmc_ffi_mir_call_need_metadata_rules.inc
  consumes route metadata / registry rows and explicitly excludes name fallback.

hako_llvmc_ffi_typed_object_plan.inc
  typed object plan reader and storage tag mapper.

hako_llvmc_ffi_map_lookup_fusion_metadata.inc
  consumes published map_lookup_fusion_routes; it does not discover windows.

hako_llvmc_ffi_string_candidate_plan_readers.inc
  reader for value consumer / string corridor / kernel plans.
```

Cleanup task order:

```text
P0. BOXED-SUM-SITE-ABI-PLAN-ID-001
    current files:
      lang/c-abi/shims/hako_llvmc_ffi_boxed_sum_abi_plan.inc
      lang/c-abi/shims/hako_llvmc_ffi_pure_compile_boxed_sum_emit.inc
      lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
      lang/c-abi/shims/hako_llvmc_ffi_same_module_prepass.inc
    - MIR / LoweringPlan site facts carry resolved abi_plan_id and
      payload_storage=None|I64|Handle
    - C shims consume explicit plan rows only
    - remove boxed_sum_payload_storage_from_type_name()
    - no Option-name / MetadataContext-name / payload_type spelling inference
    - variant_make / tag / project plan lookup uses abi_plan_id, not
      enum_name + payload_storage search
    - VariantTag may only receive a single abi_plan_id when MIR-owned site
      facts prove the value resolves to a unique boxed sum shape; enum_name
      alone is not enough
    acceptance:
      backend-local payload class decision = 0
      enum-name-only boxed sum plan lookup = 0
      payload_type spelling fallback = 0
      unit / handle / I64 boxed-sum probes stay EXE/AOT green

P0. C-ABI-SHIM-RESPONSIBILITY-INVENTORY-001
    current files:
      lang/c-abi/shims/*.inc
    status=landed
    inventory result:
      route_policy:
        hako_llvmc_ffi_mir_call_route_policy.inc legacy generic_method_routes fallback
        hako_llvmc_ffi_generic_method_get_policy.inc / has_policy.inc emit-layer validation
      object_storage_inference:
        hako_llvmc_ffi_object_storage_plan.inc HakoAlloc alignment-result method/key mapping
      exact_seed_fallback:
        hako_llvmc_ffi_pure_compile.inc exact_seed_backend_route tag dispatch
        hako_llvmc_ffi_user_box_micro_seed_*.inc benchmark/userbox shape policy
      remaining boxed-sum local lookup:
        hako_llvmc_ffi_pure_compile_boxed_sum_emit.inc boxed_sum_unit_binding_plan_index enum-name lookup
      already drained:
        boxed-sum const payload definition scan
        boxed-sum lowering facade duplication
        prepass variant binding owner
        same-module typed-field/result-capsule fusion window discovery
        same-module/generic-method __hako_sum_ prefix proof
    selected next cleanup:
      EXACT-SEED-ROUTE-QUARANTINE-001
      owner=explicit exact route rows
      boundary=remove blind exact seed fallback attempts from C shim

P0. BOXED-SUM-CONST-PAYLOAD-DEF-INDEX-001
    card:
      docs/development/current/main/phases/phase-296x/296x-1654-BOXED-SUM-CONST-PAYLOAD-DEF-INDEX-001.md
    current file:
      lang/c-abi/shims/hako_llvmc_ffi_pure_compile_boxed_sum_emit.inc
    - replace boxed_sum_emit prior-instruction linear scans with
      ValueId -> const/definition lookup
    - owner may be MIR JSON value facts, lowering plan, or compiler-state
      prepass, but emit code must not scan for defining instructions
    - preserve Some(-1) vs handle(-1) ambiguity protection through value-class
      facts, not sign inference
    acceptance:
      emit_boxed_sum_i64_variant_make contains no for-k < ii definition scan
      const payload lookup is O(1) from a named owner
      missing class/const fact fails closed with a stable rejection reason

P0. BOXED-SUM-LOWERING-FACADE-001
    card:
      docs/development/current/main/phases/phase-296x/296x-1655-BOXED-SUM-LOWERING-FACADE-001.md
    current files:
      lang/c-abi/shims/hako_llvmc_ffi_pure_compile_variant_dispatch.inc
      lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc
      lang/c-abi/shims/hako_llvmc_ffi_pure_compile_boxed_sum_emit.inc
    - generic and same-module variant_make / variant_tag / variant_project
      dispatch use one boxed-sum lowering facade
    - boxed-sum plan lookup, local variant binding propagation, and
      make/tag/project selection are not repeated in each emitter
    - same-module typed-field / RMW emitter does not own boxed-sum ABI policy
    acceptance:
      one boxed-sum opcode-lowering entry per opcode surface
      same-module and generic paths share the same payload_storage behavior
      no duplicate payload_type inference or local-binding fallback branches

P1. SAME-MODULE-FUSION-PLAN-SSOT-001
    card:
      docs/development/current/main/phases/phase-296x/296x-1657-SAME-MODULE-FUSION-PLAN-SSOT-001.md
    current files:
      lang/c-abi/shims/hako_llvmc_ffi_same_module_prepass.inc
      lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc
    selected first window:
      typed_field_rmw get/binop/set -> exact_slot_rmw_add_u64 helper
    second window:
      result_capsule_reset_batch four-field reset helper = landed
    - upstream emits same_module_fusion_plan rows:
      selected sites, skipped instruction ids, helper symbol, slots, guards
    - C shims consume that plan only
    - remove get/binop/set window matching and function-name allowlists from
      same-module emit files
    drain_now:
      same_module_function_register_direct_use_count
      same_module_function_match_typed_field_rmw_fusion_plan_at
      same_module_function_match_typed_field_rmw_fusion_plans
      same_module_function_name_is_selected_facade_get_set_fusion_target
    drained:
      same_module_function_match_result_capsule_reset_batch_plan
      same_module_function_is_selected_result_capsule_reset_batch_target
    not_window_discovery:
      record-success helper bodies
    acceptance:
      same-module C shims do not discover fusion windows from neighboring ops
      helper/function allowlists live in plan generation, not emit code

P1. GENERIC-ROUTE-DESCRIPTOR-FULL-GENERATION-001
    current files:
      lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_op_dispatch.inc
      lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
    - generate same-module views, need-kind mapping, emit-kind mapping, and
      extern need mapping from the neutral route manifest
    - handwritten route/proof tuple copies become generated output checks only
    - backends switch on generated route ids / descriptor fields

P1. C-SHIM-PREPASS-FACT-OWNER-DRAIN-001
    card:
      docs/development/current/main/phases/phase-296x/296x-1656-C-SHIM-PREPASS-FACT-OWNER-DRAIN-001.md
    current files:
      lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
      lang/c-abi/shims/hako_llvmc_ffi_same_module_prepass.inc
      lang/c-abi/shims/hako_llvmc_ffi_mir_call_prepass.inc
    - upstream SemanticRefresh / LoweringPlan publishes ValueClass,
      OriginKind, NeedFlags, PhiType, and VariantBinding facts
    - C-side prepasses verify required facts exist
    - C-side prepasses stop being the semantic owner that derives those facts
    acceptance:
      prepass code verifies required published facts
      prepass code does not infer value class from box name or payload spelling

P1. MIR-CALL-ROUTE-POLICY-DRAIN-001
    current files:
      lang/c-abi/shims/hako_llvmc_ffi_mir_call_prepass.inc
      lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
    - route classification, declaration needs, result facts, receiver-origin
      mutation, and builtin declaration needs are separated
    status=landed
    - legacy generic_method_routes metadata scan is retired; route policy now
      consumes lowering_plan rows and generated route descriptors only
    - global print need fallback is retired; `builtin_global_call_routes` feeds
      lowering_plan `need_kind=printf` without joining target-shape routes
    acceptance:
      primary route policy comes from lowering_plan / generated descriptors
      C shim does not choose route by callee string when descriptor exists
      global print declaration need comes from lowering_plan, not callee name

P1. MIR-CALL-NEED-NAME-FALLBACK-AUDIT-001
    current files:
      lang/c-abi/shims/hako_llvmc_ffi_mir_call_prepass.inc
    status=landed
    - global `print` need-name fallback is deleted
    - declaration need is supplied by lowering_plan metadata
    - no C-side callee-name table remains for global print
    acceptance:
      route classification by callee spelling = 0
      global print fallback table = 0
      missing required descriptor fails closed instead of choosing a route name

P2. OBJECT-STORAGE-PLAN-NAME-INFERENCE-DRAIN-001
    status=landed
    current file:
      lang/c-abi/shims/hako_llvmc_ffi_object_storage_plan.inc
    - object_storage_plans carry explicit flattened-field keys,
      method/property mappings, and static key symbols
    - C shims do not infer object storage from HakoAlloc names

P2. SAME-MODULE-DEFINITION-EDGE-PLAN-001
    status=landed
    current file:
      lang/c-abi/shims/hako_llvmc_ffi_same_module_function_plan.inc
    - MIR finalize / lowering plan producer emits explicit same-module
      definition list and definition edges
    - C shim validates and emits listed definitions only
    - recursive function-list discovery is removed from C shim

P2. EXACT-SEED-ROUTE-QUARANTINE-001
    status=landed
    - legacy exact seed emitters require explicit exact route rows
    - blind fallback attempts are removed
    - benchmark/userbox seed paths are either quarantined or retired
```

Immediate recommendation:

```text
do not block BOXED-SUM-I64-PAYLOAD-ABI-001 closeout on this cleanup lane.
if boxed-sum ABI work continues after region-parent reopen, do P0 before adding
new boxed payload classes.
```

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
