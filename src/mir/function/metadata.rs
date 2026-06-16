use super::facts::{
    CountingLoopFact, DirectArrayExtentFact, FastPathObligation, LoopRangeFact, RangeIndexFact,
    RegionStabilityFact, RequiredFastPathRegion, SpanAccessPlan, SpanBorrowFact,
};
use super::fastmem::{
    FastMemBlockNextFact, FastMemBranchConditionFact, FastMemFieldAccessSite,
    FastMemFreeHeadNonEmptyFact, FastMemIndexAccessSite, FastMemLocalFreeNonEmptyFact,
    FastMemRegionMetadata, FastMemRemoteOwnerFact, FastMemSameOwnerFact, FastMemTableLengthFact,
};
use super::object_metadata::RecordStateFieldAccessPlan;
use super::types::{ExactNumericRuntimeCheckContract, MirParamDecl};
use crate::mir::{
    agg_local_scalarization::AggLocalScalarizationRoute,
    array_getset_micro_seed_plan::ArrayGetSetMicroSeedRoute,
    array_rmw_add1_leaf_seed_plan::ArrayRmwAdd1LeafSeedRoute,
    array_rmw_window_plan::ArrayRmwWindowRoute,
    array_string_len_window_plan::ArrayStringLenWindowRoute,
    array_string_store_micro_seed_plan::ArrayStringStoreMicroSeedRoute,
    array_text_combined_region_plan::ArrayTextCombinedRegionRoute,
    array_text_edit_plan::ArrayTextEditRoute,
    array_text_loop_session_plan::{ArrayTextIndexOfConstRegionPlan, ArrayTextLoopSessionPlan},
    array_text_loopcarry_plan::ArrayTextLoopCarryLenStoreRoute,
    array_text_observer_plan::ArrayTextObserverRoute,
    array_text_residence_session_plan::ArrayTextResidenceSessionRoute,
    array_text_state_residence_plan::ArrayTextStateResidenceRoute,
    concat_const_suffix_micro_seed_plan::ConcatConstSuffixMicroSeedRoute,
    direct_array_access_plan::DirectArrayAccessPlan,
    direct_exact_hotcore_call_plan::DirectExactHotCoreCallPlan,
    effect_capability_plan::{CapabilityPlan, EffectPlan},
    effect_summary::EffectSummary,
    exact_numeric_value_facts::{
        ExactNumericBinaryOpRouteFact, ExactNumericBinaryOpRouteRejection,
        ExactNumericCompareRouteFact, ExactNumericCompareRouteRejection, ExactNumericConstFact,
        ExactNumericReturnFact, ExactNumericShiftRouteFact, ExactNumericShiftRouteRejection,
        ExactNumericValueFact, ExactNumericValueFactRejection,
    },
    exact_seed_backend_route::ExactSeedBackendRoute,
    extern_call_route_plan::ExternCallRoute,
    fastmem_access_plan::{FastMemAccessPlan, FastMemTableFieldAccessLink},
    generic_method_route_plan::GenericMethodRoute,
    global_call_route_plan::GlobalCallRoute,
    hotcore_method_summary::HotCoreMethodSummary,
    inline_plan::InlinePlan,
    map_lookup_fusion_plan::MapLookupFusionRoute,
    map_repr_plan::{
        LocalI64MapDirectStoragePlan, LocalI64MapEntryValueTrackingPlan,
        LocalMapStorageRealizationPlan, MapReprPlan,
    },
    placement_effect::PlacementEffectRoute,
    receiver_snapshot_publication_plan::ReceiverSnapshotPublicationPlan,
    route_decision::RouteDecision,
    storage_class::StorageClass,
    string_corridor::StringCorridorFact,
    string_corridor_placement::StringCorridorCandidate,
    string_corridor_relation::StringCorridorRelation,
    string_direct_set_window_plan::StringDirectSetWindowRoute,
    string_kernel_plan::StringKernelPlan,
    substring_views_micro_seed_plan::SubstringViewsMicroSeedRoute,
    sum_placement::SumPlacementFact,
    sum_placement_layout::SumPlacementLayout,
    sum_placement_selection::SumPlacementSelection,
    sum_variant_project_seed_plan::SumVariantProjectSeedRoute,
    sum_variant_tag_seed_plan::SumVariantTagSeedRoute,
    thin_entry::ThinEntryCandidate,
    thin_entry_selection::ThinEntrySelection,
    user_box_method_route_plan::UserBoxMethodRoute,
    userbox_known_receiver_method_seed_plan::UserBoxKnownReceiverMethodSeedRoute,
    userbox_local_scalar_seed_plan::UserBoxLocalScalarSeedRoute,
    userbox_loop_micro_seed_plan::UserBoxLoopMicroSeedRoute,
    value_consumer::ValueConsumerFacts,
    MirType, ValueId,
};
use crate::object_storage_plan::LocalFastPathFact;
use std::collections::BTreeMap;

/// Metadata for MIR functions.
///
/// This module owns the function-level metadata catalog so `types.rs` can stay
/// focused on the core function/module containers. New derived facts and plans
/// should be added here unless they are module-level metadata.
#[derive(Debug, Clone, Default)]
pub struct FunctionMetadata {
    /// Source file location
    pub source_file: Option<String>,

    /// Line number in source
    pub line_number: Option<u32>,

    /// Whether this function is an entry point
    pub is_entry_point: bool,

    /// Whether this function is pure (no side effects)
    pub is_pure: bool,

    /// Optimization hints
    pub optimization_hints: Vec<String>,

    /// Optional per-value type map (for builders that annotate ValueId types)
    // Phase 25.1: HashMap -> BTreeMap（決定性確保）
    pub value_types: BTreeMap<ValueId, MirType>,

    /// Optional per-value origin caller map (diagnostic only)
    pub value_origin_callers: BTreeMap<ValueId, String>,

    /// Generic per-value consumer facts derived from canonical MIR.
    /// Backend emitters may consume these facts, but must not re-own consumer
    /// legality by scanning MIR JSON for semantic shape matches.
    pub value_consumer_facts: BTreeMap<ValueId, ValueConsumerFacts>,

    /// Stage1 LoopRange facts derived by the executable range-loop route.
    /// This metadata owns the index/bound/step contract for later verifier rows.
    pub loop_range_facts: Vec<LoopRangeFact>,

    /// While-style counting loop facts derived by the JSON v0 LoopForm route.
    /// These are producer facts only; fast-path consumers use `range_index_facts`.
    pub counting_loop_facts: Vec<CountingLoopFact>,

    /// Canonical range-index facts derived from loop producer facts.
    /// Consumers such as DirectArrayAccessPlan use this view instead of
    /// branching on source loop syntax.
    pub range_index_facts: Vec<RangeIndexFact>,

    /// DirectArray receiver extent facts consumed with `RangeIndexFact`.
    pub direct_array_extent_facts: Vec<DirectArrayExtentFact>,

    /// Region stability facts consumed with range/extent proofs.
    pub region_stability_facts: Vec<RegionStabilityFact>,

    /// No-escape Span borrow facts over stable regions.
    pub span_borrow_facts: Vec<SpanBorrowFact>,

    /// Metadata-only Span access plans over no-escape Span borrows.
    pub span_access_plans: Vec<SpanAccessPlan>,

    /// Metadata-only record-state candidate field access sites.
    pub record_state_field_access_plans: Vec<RecordStateFieldAccessPlan>,

    /// Required fast-path diagnostic regions.
    pub required_fastpath_regions: Vec<RequiredFastPathRegion>,

    /// Per-site obligations derived from required fast-path regions.
    pub fastpath_obligations: Vec<FastPathObligation>,

    /// Contract-bound fast-memory source regions.
    ///
    /// Region truth lives here. MIR instruction streams carry only `MemOp`
    /// executable operations with a `FastMemRegionId` back-reference.
    pub fastmem_regions: Vec<FastMemRegionMetadata>,

    /// FastMemory-owned table length facts consumed by TableIndex access plans.
    ///
    /// These rows are semantic memory-profile metadata. MIRBuilder must only
    /// preserve symbolic table ids and provenance; it must not invent lengths.
    pub fastmem_table_length_facts: Vec<FastMemTableLengthFact>,

    /// FastMemory same-owner facts consumed by local free-list access plans.
    pub fastmem_same_owner_facts: Vec<FastMemSameOwnerFact>,

    /// FastMemory remote-owner facts consumed by AtomicRemoteHead plans.
    pub fastmem_remote_owner_facts: Vec<FastMemRemoteOwnerFact>,

    /// FastMemory block-next facts consumed by LocalFreePush access plans.
    pub fastmem_block_next_facts: Vec<FastMemBlockNextFact>,

    /// FastMemory non-empty local free-list facts consumed by LocalFreePop.
    pub fastmem_local_free_non_empty_facts: Vec<FastMemLocalFreeNonEmptyFact>,

    /// FastMemory non-empty ordinary free-list facts consumed by FreeHeadPop.
    pub fastmem_free_head_non_empty_facts: Vec<FastMemFreeHeadNonEmptyFact>,

    /// Function-local FastMemory layout/table access plan rows.
    ///
    /// These rows connect symbolic `MemOpAccess` ids to the verifier-owned
    /// access contract that later LLVM lowering may consume. Rows may remain
    /// `symbolic_only` until canonical layout/table contracts are available.
    pub fastmem_access_plans: Vec<FastMemAccessPlan>,

    /// Function-local FastMemory field access-site metadata.
    ///
    /// These rows are the source-side site table for the current transitional
    /// dedicated lowerer and the future verified-direct route planner.
    pub fastmem_field_access_sites: Vec<FastMemFieldAccessSite>,

    /// Function-local FastMemory index access-site metadata.
    ///
    /// These rows are the source-side site table for table/index accesses and
    /// the future verified-direct route planner.
    pub fastmem_index_access_sites: Vec<FastMemIndexAccessSite>,

    /// Function-local FastMemory branch-condition proof facts.
    ///
    /// These rows record when a FastMemory branch condition is backed by the
    /// owner-equality proof required by the narrow branch route surface.
    pub fastmem_branch_condition_facts: Vec<FastMemBranchConditionFact>,

    /// Function-local outbox binding metadata.
    ///
    /// This records the narrow Stage1 transfer surface without introducing a
    /// richer ownership checker.
    pub outbox_bindings: Vec<String>,

    /// Verified v0 links from TableIndex results to same-block field accesses.
    ///
    /// These rows are the explicit source for `field_offset_resolved` in the
    /// table proof payload. They do not open overflow proof or lowering.
    pub fastmem_table_field_access_links: Vec<FastMemTableFieldAccessLink>,

    /// Metadata-only helper effect summaries.
    ///
    /// These inventory receiver/foreign reads, writes, handle publication, and
    /// hidden effect blockers for future narrow mixed-base helper recipes. They
    /// do not authorize Inline(required), call lowering, or publication routes.
    pub effect_summaries: Vec<EffectSummary>,

    /// Metadata-only narrow mixed-base publication recipe plans.
    ///
    /// v0 accepts scalar snapshot publication only. Foreign handle publication
    /// is reported but rejected until a barrier/lifetime policy lands.
    pub receiver_snapshot_publication_plans: Vec<ReceiverSnapshotPublicationPlan>,

    /// Metadata-only summaries for selected direct-exact hot-core methods.
    ///
    /// This does not authorize inline lowering. It only reports whether a
    /// selected multi-block callee keeps the expected scalar/no-fallback shape
    /// before a later call-plan/lowering consumer is allowed to use it.
    pub hotcore_method_summaries: Vec<HotCoreMethodSummary>,

    /// Report-only call-edge plans for selected direct-exact HotCore calls.
    ///
    /// This is not body inlining. It records that an already-known user-box
    /// method call can be explained as a static exact call candidate before a
    /// later lowering consumer is allowed to remove generic dispatch.
    pub direct_exact_hotcore_call_plans: Vec<DirectExactHotCoreCallPlan>,

    /// Declaration-local Rune attrs carried from AST/direct MIR routes.
    pub runes: Vec<crate::ast::RuneAttr>,

    /// Declaration-local `uses ...` capability names carried from source.
    /// RANDOM-CAP-001 only promotes `uses random` into metadata-only
    /// CapabilityPlan facts; broader capability checking remains a later row.
    pub declared_capability_uses: Vec<String>,

    /// MIR-owned inline metadata derived from advisory `Hint(...)` runes.
    /// This is preservation-only until an InlinePlan transform/verifier row lands.
    pub inline_plans: Vec<InlinePlan>,

    /// MIR-owned effect obligations derived from verifier-backed Contract runes.
    pub effect_plans: Vec<EffectPlan>,

    /// MIR-owned capability allowances. Empty until capability syntax/profile rows land.
    pub capability_plans: Vec<CapabilityPlan>,

    /// No-op string corridor facts attached to current MIR values.
    /// These facts inventory current carriers (`str.slice`, `str.len`, `freeze.str`)
    /// without introducing a second MIR dialect or changing runtime behavior.
    pub string_corridor_facts: BTreeMap<ValueId, StringCorridorFact>,

    /// No-op placement/effect candidate decisions derived from string corridor facts.
    /// These candidates are inspection-only scaffolding for the future placement/effect
    /// pass and must not change runtime behavior in this wave.
    pub string_corridor_candidates: BTreeMap<ValueId, Vec<StringCorridorCandidate>>,

    /// No-op relation metadata derived from canonical MIR plus generic PHI queries.
    /// These relations are structural continuity facts for later string corridor
    /// planning; they do not own PHI semantics and they must not mutate MIR.
    pub string_corridor_relations: BTreeMap<ValueId, Vec<StringCorridorRelation>>,

    /// No-op storage-class inventory derived from the current MIR value types.
    /// This is the first-step scaffold for primitive-family / user-box fast paths.
    pub value_storage_classes: BTreeMap<ValueId, StorageClass>,

    /// No-op thin-entry inventory derived from canonical MIR plus current metadata.
    /// This records where pass + manifest can later choose public vs thin internal
    /// physical entries without adding a second semantic call dialect.
    pub thin_entry_candidates: Vec<ThinEntryCandidate>,

    /// No-op thin-entry selection pilot derived from thin-entry inventory plus the
    /// current manifest rows.
    /// This binds the first public-vs-thin entry choice without mutating canonical
    /// MIR or changing runtime behavior in this wave.
    pub thin_entry_selections: Vec<ThinEntrySelection>,

    /// Sum-local placement/objectization facts for the current proving slice.
    /// This is sum-specific inspection metadata for now, but it should fold into a
    /// later generic placement/effect pass instead of becoming a permanent
    /// sum-only framework.
    pub sum_placement_facts: Vec<SumPlacementFact>,

    /// Selected sum-local placement routes derived from the current sum facts.
    /// This still does not mutate MIR or runtime behavior; it only distinguishes
    /// selected local aggregate routes from compat/runtime fallback routes so the
    /// later layout/lowering slices can stay thin.
    pub sum_placement_selections: Vec<SumPlacementSelection>,

    /// LLVM-side local aggregate layout choices for selected local sum routes.
    /// This remains metadata-only in the current slice so lowering can consume a
    /// single layout SSOT in the next step.
    pub sum_placement_layouts: Vec<SumPlacementLayout>,

    /// Folded agg_local scalarization routes derived from the landed pilot
    /// scaffolds. This is the first generic owner seam that reads the sum,
    /// thin-entry, and storage-class pilots together without changing runtime
    /// behavior.
    pub agg_local_scalarization_routes: Vec<AggLocalScalarizationRoute>,

    /// Folded generic placement/effect routes derived from the landed string,
    /// sum, and thin-entry pilots. This keeps the first cross-family route
    /// inventory in one owner seam without mutating MIR or lowering behavior.
    pub placement_effect_routes: Vec<PlacementEffectRoute>,

    /// Backend-consumable string kernel plans derived during MIR refresh.
    /// This is the first MIR-side generic placement/effect transform slice and
    /// stays a derived view over corridor candidates, not a new canonical
    /// semantic owner.
    pub string_kernel_plans: BTreeMap<ValueId, StringKernelPlan>,

    /// Backend-consumable string direct-set source-window route plans.
    /// These own the `substring + substring + substring_concat3_hhhii`
    /// direct-set legality proof in MIR so backend shims can only consume
    /// metadata and record the deferred piecewise route.
    pub string_direct_set_window_routes: Vec<StringDirectSetWindowRoute>,

    /// Backend-consumable generic method route plans.
    /// These own narrow method-surface policy decisions in MIR so backend
    /// shims can emit selected helpers without reclassifying method strings.
    pub generic_method_routes: Vec<GenericMethodRoute>,

    /// Metadata-only DirectArray access plans derived from Array get/set
    /// method routes.  The first slice records checked DirectArrayI64
    /// candidates only; unchecked proofs and backend consumption land later.
    pub direct_array_access_plans: Vec<DirectArrayAccessPlan>,

    /// Planner-owned fastpath-preferred route outcomes.
    ///
    /// This is a report-only view in v0. It records which route a planner
    /// selected for one site and which fallback policy applies. MIRBuilder
    /// must not produce these rows, and lowering must not re-decide them.
    pub route_decisions: Vec<RouteDecision>,

    /// Backend-consumable extern call route plans.
    /// These own narrow extern surface policy decisions in MIR so backend
    /// shims can emit selected runtime ABI calls without classifying `env.*`
    /// strings locally.
    pub extern_call_routes: Vec<ExternCallRoute>,

    /// Backend-facing unsupported global user-call route plans.
    /// These do not make global calls lowerable; they move the stop-line into
    /// LoweringPlan metadata so backend shims can fail from a typed plan.
    pub global_call_routes: Vec<GlobalCallRoute>,

    /// Backend-consumable typed user-box method route plans.
    /// The first accepted route is `birth` as a same-module uniform ABI
    /// function call; backends must consume this metadata instead of
    /// rediscovering user-box method semantics from raw names.
    pub user_box_method_routes: Vec<UserBoxMethodRoute>,

    /// Metadata-only MapGet/MapHas same-key fusion preflight routes.
    /// These are derived from `generic_method_routes` and do not change
    /// lowering; they only pin the exact pair a future plan may consume.
    pub map_lookup_fusion_routes: Vec<MapLookupFusionRoute>,

    /// Metadata-only MapBox representation plans.
    ///
    /// v0 records the current generic hash runtime surface derived from
    /// generic method routes. Later rows may promote fixed / enum / interned
    /// subsets into the same family without changing lowering by themselves.
    pub map_repr_plans: Vec<MapReprPlan>,

    /// Passive exact-AOT/local-first Map storage realization plans.
    ///
    /// These rows describe candidate representation before publication. They
    /// do not enable backend lowering, runtime helper routes, or product
    /// MapBox storage changes by themselves.
    pub local_map_storage_realization_plans: Vec<LocalMapStorageRealizationPlan>,

    /// Passive exact-AOT/local-first i64 Map direct storage descriptors.
    ///
    /// These rows name the selected closed-world storage representation but
    /// keep entry value tracking, backend lowering, runtime helpers, and
    /// product MapBox storage changes disabled.
    pub local_i64_map_direct_storage_plans: Vec<LocalI64MapDirectStoragePlan>,

    /// Passive set-site value tracking rows for local i64 Map direct storage.
    ///
    /// These rows record which key/value operands seeded the future local
    /// table. They do not materialize the table or enable backend lowering.
    pub local_i64_map_entry_value_tracking_plans: Vec<LocalI64MapEntryValueTrackingPlan>,

    /// Backend-consumable local fast path permissions.
    ///
    /// These rows must be positive facts only. Fallback evidence, observations,
    /// helper names, and source variable names do not belong in this surface.
    pub local_fastpath_facts: Vec<LocalFastPathFact>,

    /// Backend-consumable array RMW route plans.
    /// These own `array.get(i) -> + 1 -> array.set(i, ...)` legality in MIR
    /// so backend shims can emit/skip from metadata instead of scanning raw
    /// MIR JSON instruction windows.
    pub array_rmw_window_routes: Vec<ArrayRmwWindowRoute>,

    /// Backend-consumable array string length route plans.
    /// These own the len-only `array.get(i).length()` legality in MIR so
    /// backend shims can emit/skip from metadata instead of scanning raw MIR
    /// JSON instruction windows.
    pub array_string_len_window_routes: Vec<ArrayStringLenWindowRoute>,

    /// MIR-owned array/text loop-session proof plans.
    /// These connect an already-proven `array.get(i).length()` window with a
    /// loop-local index-domain proof. They are exported for inspection only
    /// until a later row explicitly enables backend consumption/lowering.
    pub array_text_loop_session_plans: Vec<ArrayTextLoopSessionPlan>,

    /// MIR-owned array/text indexOf-const region proof plans.
    /// These connect an already-proven `array.get(i).indexOf("const")`
    /// observer route with a loop-local found-predicate accumulator shape.
    /// They are exported for inspection only until a later row explicitly
    /// enables backend consumption/lowering.
    pub array_text_indexof_const_region_plans: Vec<ArrayTextIndexOfConstRegionPlan>,

    /// Backend-consumable array/text loopcarry route plans.
    /// These keep active fused store/len route recognition in MIR so the C
    /// backend can remain an emitter/transport consumer.
    pub array_text_loopcarry_len_store_routes: Vec<ArrayTextLoopCarryLenStoreRoute>,

    /// Backend-consumable array/text same-cell edit route plans.
    /// These own edit policy facts such as `source_len / 2` in MIR so backend
    /// shims do not re-prove length/split/substring legality from raw JSON.
    pub array_text_edit_routes: Vec<ArrayTextEditRoute>,

    /// Backend-consumable combined array/text region plans.
    /// These prove a bounded outer edit loop together with an already-proven
    /// nested observer-store region. They are metadata-only until lowering
    /// consumes them as a single begin-site executor contract.
    pub array_text_combined_regions: Vec<ArrayTextCombinedRegionRoute>,

    /// Backend-consumable array/text residence session plans.
    /// These are metadata-only until lowering consumes them; they prove where a
    /// future backend may hold a runtime-private text residence guard without
    /// asking runtime or `.inc` to rediscover legality.
    pub array_text_residence_sessions: Vec<ArrayTextResidenceSessionRoute>,

    /// Backend-consumable generic array/text observer route plans.
    /// These own read-side observer legality/provenance/consumer facts in MIR;
    /// backend shims may only map the metadata to local helper calls.
    pub array_text_observer_routes: Vec<ArrayTextObserverRoute>,

    /// Backend-consumable array/text state residence route plan.
    /// This keeps the generic residence contract separate from its explicit
    /// temporary payload, so backend consumers do not read a second exact route.
    pub array_text_state_residence_route: Option<ArrayTextStateResidenceRoute>,

    /// Backend-consumable exact array/string-store micro seed route.
    /// This quarantines the temporary kilo micro exact-shape bridge in MIR
    /// metadata so the C backend can select an emitter without re-planning raw
    /// MIR JSON.
    pub array_string_store_micro_seed_route: Option<ArrayStringStoreMicroSeedRoute>,

    /// Backend-consumable exact array get/set micro seed route.
    /// The inner RMW window proof remains in `array_rmw_window_routes`; this
    /// route carries the temporary whole-function exact seed payload for the C
    /// backend emitter selector.
    pub array_getset_micro_seed_route: Option<ArrayGetSetMicroSeedRoute>,

    /// Backend-consumable exact array RMW add1 leaf seed route.
    /// This route owns the current whole-function temporary exact bridge and
    /// references the inner `array_rmw_window_routes` proof instead of asking
    /// the C backend to rescan raw MIR JSON blocks.
    pub array_rmw_add1_leaf_seed_route: Option<ArrayRmwAdd1LeafSeedRoute>,

    /// Backend-consumable exact concat-const-suffix micro seed route.
    /// This keeps the current temporary exact bridge proof in MIR metadata so
    /// the C backend can remain an emitter selector instead of a route planner.
    pub concat_const_suffix_micro_seed_route: Option<ConcatConstSuffixMicroSeedRoute>,

    /// Backend-consumable exact substring-views micro seed route.
    /// Borrowed-slice windows stay in `string_kernel_plans`; this only carries
    /// the temporary emitter payload that generic plans do not expose yet.
    pub substring_views_micro_seed_route: Option<SubstringViewsMicroSeedRoute>,

    /// Backend-consumable exact Sum variant_tag seed route.
    /// Sum placement metadata owns the local aggregate proof; this route only
    /// owns the temporary whole-function exact seed payload for the C backend.
    pub sum_variant_tag_seed_route: Option<SumVariantTagSeedRoute>,

    /// Backend-consumable exact Sum variant_project seed route.
    /// Sum placement metadata owns the local aggregate proof; this route carries
    /// the temporary literal payload required by the exact backend helper.
    pub sum_variant_project_seed_route: Option<SumVariantProjectSeedRoute>,

    /// Backend-consumable exact UserBox local scalar seed route.
    /// Thin-entry metadata owns the primitive field surface proof; this route
    /// only carries the temporary Point local/copy exact seed payload for the C
    /// backend.
    pub userbox_local_scalar_seed_route: Option<UserBoxLocalScalarSeedRoute>,

    /// Backend-consumable exact UserBox loop micro seed route.
    /// Thin-entry metadata owns the primitive field surface proof; this route
    /// carries the current point-add / flag-toggle loop micro seed payload so
    /// the C backend can stay an emitter selector.
    pub userbox_loop_micro_seed_route: Option<UserBoxLoopMicroSeedRoute>,

    /// Backend-consumable exact UserBox known-receiver method seed route.
    /// Thin-entry metadata owns the method and primitive field surface proof;
    /// this route carries the current local/copy Counter.step and Point.sum
    /// exact seed payloads so the C backend can stay an emitter selector.
    pub userbox_known_receiver_method_seed_route: Option<UserBoxKnownReceiverMethodSeedRoute>,

    /// Function-level backend route tag for one already-proven exact seed.
    /// Payload legality remains owned by the selected `*_micro_seed_route`;
    /// this only lets the C boundary choose the first helper without walking
    /// the helper-specific ladder.
    pub exact_seed_backend_route: Option<ExactSeedBackendRoute>,

    /// Source-level declared parameter metadata carried into MIR without
    /// changing the callable ABI or `MirType` lane.
    pub declared_param_decls: Vec<MirParamDecl>,

    /// Source-level declared return annotation carried into MIR without
    /// forcing `FunctionSignature.return_type`.
    pub declared_return_type_name: Option<String>,

    /// MIR-owned exact numeric facts attached to values after field reads and
    /// conservative copy/control-merge propagation.
    ///
    /// These facts are reference-execution/lowering input metadata only. They
    /// do not change the legacy dynamic `Integer(i64)` lane by themselves.
    pub exact_numeric_value_facts: BTreeMap<ValueId, ExactNumericValueFact>,

    /// Builder-owned typed integer literal facts. Semantic refresh validates
    /// and copies these into `exact_numeric_value_facts` so downstream VM and
    /// backend rows can consume one per-value fact surface.
    pub exact_numeric_const_facts: BTreeMap<ValueId, ExactNumericConstFact>,

    /// Control-merge sites where exact numeric facts could not be propagated
    /// without mixing exact/dynamic values or mismatched exact source names.
    pub exact_numeric_value_fact_rejections: Vec<ExactNumericValueFactRejection>,

    /// Exact numeric binary-operation route facts. These do not execute or lower
    /// the operation; they only let later VM/backend rows consume MIR-owned
    /// exact operation routes.
    pub exact_numeric_binary_op_route_facts: Vec<ExactNumericBinaryOpRouteFact>,

    /// Binary-operation sites where exact numeric operation routes could not be
    /// published without mixing exact/dynamic values or mismatched exact source
    /// names.
    pub exact_numeric_binary_op_route_rejections: Vec<ExactNumericBinaryOpRouteRejection>,

    /// Exact numeric compare route facts. These prove the compare should use
    /// exact numeric ordering while still producing the canonical Bool result.
    pub exact_numeric_compare_route_facts: Vec<ExactNumericCompareRouteFact>,

    /// Compare sites where exact numeric compare routes could not be published
    /// without mixing exact/dynamic values or mismatched exact source names.
    pub exact_numeric_compare_route_rejections: Vec<ExactNumericCompareRouteRejection>,

    /// Exact numeric logical shift route facts. These require an exact unsigned
    /// left operand and keep the right operand as the dynamic shift count.
    pub exact_numeric_shift_route_facts: Vec<ExactNumericShiftRouteFact>,

    /// Shift sites where exact numeric logical shift routes could not be
    /// published, currently because the exact left operand is signed.
    pub exact_numeric_shift_route_rejections: Vec<ExactNumericShiftRouteRejection>,

    /// Function-level exact numeric return annotation fact.
    ///
    /// This is advisory metadata only until a later verifier/lowering row
    /// checks returned values against it.
    pub exact_numeric_return_fact: Option<ExactNumericReturnFact>,

    /// MIR-owned contracts proving that a dynamic value is range-checked before
    /// it is written into an exact numeric field. This is metadata only until
    /// runtime-check lowering consumes it.
    pub exact_numeric_runtime_check_contracts: Vec<ExactNumericRuntimeCheckContract>,
}
