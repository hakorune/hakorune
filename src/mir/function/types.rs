use crate::mir::{
    agg_local_scalarization::AggLocalScalarizationRoute,
    array_rmw_add1_leaf_seed_plan::ArrayRmwAdd1LeafSeedRoute,
    array_rmw_window_plan::ArrayRmwWindowRoute,
    array_string_len_window_plan::ArrayStringLenWindowRoute,
    array_string_store_micro_seed_plan::ArrayStringStoreMicroSeedRoute,
    array_text_combined_region_plan::ArrayTextCombinedRegionRoute,
    array_text_edit_plan::ArrayTextEditRoute,
    array_text_loopcarry_plan::ArrayTextLoopCarryLenStoreRoute,
    array_text_observer_plan::ArrayTextObserverRoute,
    array_text_residence_session_plan::ArrayTextResidenceSessionRoute,
    array_text_state_residence_plan::ArrayTextStateResidenceRoute,
    concat_const_suffix_micro_seed_plan::ConcatConstSuffixMicroSeedRoute,
    exact_seed_backend_route::ExactSeedBackendRoute, generic_method_route_plan::GenericMethodRoute,
    placement_effect::PlacementEffectRoute, storage_class::StorageClass,
    string_corridor::StringCorridorFact, string_corridor_placement::StringCorridorCandidate,
    string_corridor_relation::StringCorridorRelation,
    string_direct_set_window_plan::StringDirectSetWindowRoute,
    string_kernel_plan::StringKernelPlan,
    substring_views_micro_seed_plan::SubstringViewsMicroSeedRoute, sum_placement::SumPlacementFact,
    sum_placement_layout::SumPlacementLayout, sum_placement_selection::SumPlacementSelection,
    sum_variant_project_seed_plan::SumVariantProjectSeedRoute,
    sum_variant_tag_seed_plan::SumVariantTagSeedRoute, thin_entry::ThinEntryCandidate,
    thin_entry_selection::ThinEntrySelection, value_consumer::ValueConsumerFacts, BasicBlock,
    BasicBlockId, ConstValue, EffectMask, MirType, ValueId,
};
use std::collections::{BTreeMap, HashMap, HashSet};

/// Stable identifier for externalized closure bodies in module metadata.
pub type ClosureBodyId = u32;

/// Function signature for MIR functions
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    /// Function name
    pub name: String,

    /// Parameter types
    pub params: Vec<MirType>,

    /// Return type
    pub return_type: MirType,

    /// Overall effect mask for the function
    pub effects: EffectMask,
}

/// A MIR function in SSA form
#[derive(Debug, Clone)]
pub struct MirFunction {
    /// Function signature
    pub signature: FunctionSignature,

    /// Basic blocks indexed by ID
    pub blocks: HashMap<BasicBlockId, BasicBlock>,

    /// Entry basic block ID
    pub entry_block: BasicBlockId,

    /// Local variable declarations (before SSA conversion)
    pub locals: Vec<MirType>,

    /// Parameter value IDs
    pub params: Vec<ValueId>,

    /// Next available value ID
    pub next_value_id: u32,

    /// Function-level metadata
    pub metadata: FunctionMetadata,
}

/// Metadata for MIR functions
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

    /// Declaration-local Rune attrs carried from AST/direct MIR routes.
    pub runes: Vec<crate::ast::RuneAttr>,

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

    /// Function-level backend route tag for one already-proven exact seed.
    /// Payload legality remains owned by the selected `*_micro_seed_route`;
    /// this only lets the C boundary choose the first helper without walking
    /// the helper-specific ladder.
    pub exact_seed_backend_route: Option<ExactSeedBackendRoute>,
}

/// Function statistics for profiling and optimization
#[derive(Debug, Clone)]
pub struct FunctionStats {
    pub block_count: usize,
    pub instruction_count: usize,
    pub phi_count: usize,
    pub value_count: usize,
    pub is_pure: bool,
}

/// Typed field declaration metadata carried alongside names-only user box decls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserBoxFieldDecl {
    pub name: String,
    pub declared_type_name: Option<String>,
    pub is_weak: bool,
}

/// Declared variant inventory for first-class enum/sum metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirEnumVariantDecl {
    pub name: String,
    pub payload_type_name: Option<String>,
}

/// Declared enum inventory carried alongside MIR modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirEnumDecl {
    pub type_parameters: Vec<String>,
    pub variants: Vec<MirEnumVariantDecl>,
}

/// A MIR module containing multiple functions
#[derive(Debug, Clone)]
pub struct MirModule {
    /// Module name
    pub name: String,

    /// Functions in this module (BTreeMap for deterministic iteration order)
    pub functions: BTreeMap<String, MirFunction>,

    /// Global constants/statics
    pub globals: HashMap<String, ConstValue>,

    /// Module metadata
    pub metadata: ModuleMetadata,
}

/// Metadata for MIR modules
#[derive(Debug, Clone, Default)]
pub struct ModuleMetadata {
    /// Source file this module was compiled from
    pub source_file: Option<String>,

    /// Compilation timestamp
    pub compiled_at: Option<String>,

    /// Compiler version
    pub compiler_version: Option<String>,

    /// Optimization level used
    pub optimization_level: u32,

    /// Dev idempotence markers for passes (optional; default empty)
    /// Key format suggestion: "pass_name:function_name"
    pub dev_processed_markers: HashSet<String>,

    /// Phase 285LLVM-1.1: User-defined box declarations with fields
    /// HashMap: box name -> field names (empty Vec for static boxes)
    pub user_box_decls: HashMap<String, Vec<String>>,

    /// Typed field declarations for user-defined boxes.
    /// This stays parallel to `user_box_decls` so names-only compatibility remains intact.
    pub user_box_field_decls: HashMap<String, Vec<UserBoxFieldDecl>>,

    /// Declared enum inventory for canonical sum lowering and runtime/codegen handoff.
    pub enum_decls: BTreeMap<String, MirEnumDecl>,

    /// NCL-1: Externalized closure bodies (`body_id -> AST body`).
    /// NewClosure keeps only a small descriptor and references this table.
    pub closure_bodies: BTreeMap<ClosureBodyId, Vec<crate::ast::ASTNode>>,

    /// NCL-1: Next stable id for `closure_bodies`.
    pub next_closure_body_id: ClosureBodyId,
}

/// Module statistics
#[derive(Debug, Clone)]
pub struct ModuleStats {
    pub function_count: usize,
    pub global_count: usize,
    pub total_blocks: usize,
    pub total_instructions: usize,
    pub total_values: usize,
    pub pure_functions: usize,
}
