/*!
 * Nyash MIR (Mid-level Intermediate Representation) - Stage 1 Implementation
 *
 * ChatGPT5-designed MIR infrastructure for native compilation support
 * Based on SSA form with effect tracking and Box-aware optimizations
 */

pub mod agg_local_scalarization; // generic agg_local scalarization owner seam folded from landed pilots
pub mod analysis; // analysis-only views (no AST rewrite)
#[cfg(feature = "aot-plan-import")]
pub mod aot_plan_import;
pub mod array_getset_micro_seed_plan; // MIR-owned route plan for temporary array get/set micro seed bridge
pub(crate) mod array_receiver_proof; // shared RuntimeDataBox -> ArrayBox receiver proof
pub mod array_rmw_add1_leaf_seed_plan; // MIR-owned route plan for temporary array RMW add1 leaf seed bridge
pub mod array_rmw_window_plan; // MIR-owned route plans for array get/+1/set windows
pub mod array_string_len_window_plan; // MIR-owned route plans for array get/length windows
pub mod array_string_store_micro_seed_plan; // MIR-owned route plan for temporary array string-store micro seed bridge
pub mod array_text_combined_region_plan; // MIR-owned combined array/text region route plans
pub mod array_text_edit_plan; // MIR-owned route plans for array/text same-cell edits
pub mod array_text_loopcarry_plan; // MIR-owned route plans for active array/text loopcarry lane
pub mod array_text_observer_plan; // MIR-owned route plans for generic array/text read-side observers
pub mod array_text_observer_region_contract; // Nested executor contract for observer-store regions
pub mod array_text_residence_session_plan; // MIR-owned residence session route plans
pub mod array_text_state_residence_plan; // MIR-owned route plan for generic array/text state residence
pub mod basic_block;
pub mod builder;
mod compiler;
pub mod concat_const_suffix_micro_seed_plan; // MIR-owned route plan for temporary concat const-suffix micro seed bridge
pub mod contracts; // backend-core instruction contracts (SSOT)
pub mod core_method_op; // MIR-side CoreMethodOp carrier vocabulary
pub mod definitions; // Unified MIR definitions (MirCall, Callee, etc.)
pub mod diagnostics; // freeze diagnostics helpers (SSOT)
pub mod effect;
pub mod escape_barrier; // escape operand-role vocabulary (SSOT)
pub mod exact_seed_backend_route; // function-level backend route tags for exact seed bridges
pub mod function;
pub(crate) mod generic_method_route_facts; // reusable facts for generic-method route planners
pub mod generic_method_route_plan; // MIR-owned generic method route policy plans
pub mod if_in_loop_phi; // Phase 187-2: Minimal if-in-loop PHI emitter (extracted from loop_builder)
pub mod indexof_search_micro_seed_plan; // MIR-owned route plan for temporary indexOf search micro seed bridge
pub mod instruction;
pub mod instruction_introspection; // Introspection helpers for tests (instruction names)
pub mod instruction_kinds; // small kind-specific metadata (Const/BinOp)
pub mod loop_api; // Minimal LoopBuilder facade (adapter-ready)
pub mod loop_canonicalizer; // Phase 1: Loop skeleton canonicalization (AST preprocessing)
pub mod map_lookup_fusion_plan; // MIR-owned MapGet/MapHas same-key fusion metadata
pub mod naming; // Static box / entry naming rules（NamingBox）
pub mod optimizer;
pub mod policies; // shared routing policies (SSOT)
pub mod ssot; // Shared helpers (SSOT) for instruction lowering
pub mod types; // core MIR enums (ConstValue, Ops, MirType)
pub mod utils; // Phase 15 control flow utilities for root treatment
               // pub mod lowerers; // reserved: Stage-3 loop lowering (while/for-range)
pub mod cfg_extractor; // Phase 154: CFG extraction for hako_check
pub mod control_form;
pub mod control_tree; // Phase 110: Structure-only SSOT (StepTree)
pub mod function_emission; // FunctionEmissionBox（MirFunction直編集の発行ヘルパ）
pub mod hints; // scaffold: zero-cost guidance (no-op)
pub mod join_ir; // Phase 26-H: 関数正規化IR（JoinIR）
pub mod join_ir_ops; // Phase 27.8: JoinIR 命令意味箱（ops box）
pub mod join_ir_runner; // Phase 27.2: JoinIR 実行器（実験用）
pub mod join_ir_vm_bridge; // Phase 27-shortterm S-4: JoinIR → Rust VM ブリッジ
pub mod join_ir_vm_bridge_dispatch; // Phase 30 F-4.4: JoinIR VM ブリッジ dispatch helper
pub mod loop_form; // ControlForm::LoopShape の薄いエイリアス
pub mod loop_route_detection; // Active module surface for loop route-shape detection
pub mod optimizer_passes; // optimizer passes (normalize/diagnostics)
pub mod optimizer_stats; // extracted stats struct
pub mod passes;
pub mod phi_core; // Phase 1 scaffold: unified PHI entry (re-exports only)
pub(crate) mod phi_query; // generic PHI base-relation seam for later relation consumers
pub mod placement_effect; // generic placement/effect owner seam folded from landed pilots
pub mod printer;
mod printer_helpers; // internal helpers extracted from printer.rs
pub mod query; // Phase 26-G: MIR read/write/CFGビュー (MirQuery)
pub mod region; // Phase 25.1l: Region/GC観測レイヤ（LoopForm v2 × RefKind）
pub mod semantic_refresh; // MIR semantic metadata refresh owner (SSOT)
pub mod slot_registry; // Phase 9.79b.1: method slot resolution (IDs)
mod spanned_instruction;
pub mod storage_class; // primitive / user-box storage-class inventory + refresh helper
pub mod string_corridor; // string canonical corridor facts + refresh helper
pub(crate) mod string_corridor_compat; // compat semantic recovery quarantined from canonical facts
pub(crate) mod string_corridor_names; // helper-name vocabulary quarantine
pub mod string_corridor_placement; // placement/effect scaffold over canonical string facts
pub(crate) mod string_corridor_recognizer; // shared pure shape recognizers for string corridor
pub mod string_corridor_relation; // string-corridor relation layer over generic PHI queries
pub mod string_direct_set_window_plan; // MIR-owned string direct-set source-window route plans
pub mod string_kernel_plan; // backend-consumable string plan seam derived from corridor candidates
pub mod substring_views_micro_seed_plan; // MIR-owned route plan for temporary substring views micro seed bridge
pub mod sum_placement; // sum-local proving slice for later generic placement/effect pass
pub mod sum_placement_layout; // LLVM-side payload-lane choices for selected local sums
pub mod sum_placement_selection; // selection pilot over sum-local placement facts
pub mod sum_variant_project_seed_plan; // MIR-owned route plan for temporary Sum variant_project exact seed bridge
pub mod sum_variant_tag_seed_plan; // MIR-owned route plan for temporary Sum variant_tag exact seed bridge
pub mod thin_entry; // thin-entry inventory for known local routes
pub mod thin_entry_selection; // manifest-driven thin-entry selection pilot
pub mod type_propagation; // Phase 279 P0: SSOT type propagation pipeline
pub mod userbox_known_receiver_method_seed_plan; // MIR-owned route plan for temporary UserBox known-receiver method exact seeds
pub mod userbox_local_scalar_seed_plan; // MIR-owned route plan for temporary UserBox local scalar exact seeds
pub mod userbox_loop_micro_seed_plan; // MIR-owned route plan for temporary UserBox loop micro exact seeds
pub mod value_consumer; // generic consumer capability facts for backend emitters
pub mod value_id;
pub mod value_kind; // Phase 26-A: ValueId型安全化
pub mod value_origin; // generic copy-root / alias-root owner (SSOT)
pub mod verification;
pub mod verification_types; // extracted error types // Optimization subpasses (e.g., type_hints) // Phase 25.1f: Loop/If 共通ビュー（ControlForm）

// Re-export main types for easy access
pub use basic_block::{BasicBlock, EdgeArgs, OutEdge};
pub use builder::MirBuilder;
pub use compiler::{MirCompileResult, MirCompiler};
pub use hakorune_mir_core::{BasicBlockId, BasicBlockIdGenerator, BindingId};

// Phase 140-P4-A: Re-export skip_whitespace shape detection for loop_canonicalizer
pub(crate) use builder::detect_skip_whitespace_shape;
// Phase 104: Re-export read_digits(loop(true)) shape detection for loop_canonicalizer
pub(crate) use builder::detect_read_digits_loop_true_shape;
// Phase 142-P1: Re-export continue shape detection for loop_canonicalizer
pub(crate) use builder::detect_continue_shape;
// Phase 143-P0: Re-export parse_number / parse_string shape detection for loop_canonicalizer
pub(crate) use builder::detect_parse_number_shape;
// Phase 143-P1:
pub(crate) use builder::detect_parse_string_shape;
// Phase 91 P5b: Re-export escape skip pattern detection for loop_canonicalizer
pub use agg_local_scalarization::{
    refresh_function_agg_local_scalarization_routes, refresh_module_agg_local_scalarization_routes,
    AggLocalScalarizationKind, AggLocalScalarizationRoute,
};
pub use array_getset_micro_seed_plan::{
    refresh_function_array_getset_micro_seed_route, refresh_module_array_getset_micro_seed_routes,
    ArrayGetSetMicroSeedRoute,
};
pub use array_rmw_add1_leaf_seed_plan::{
    refresh_function_array_rmw_add1_leaf_seed_route,
    refresh_module_array_rmw_add1_leaf_seed_routes, ArrayRmwAdd1LeafSeedRoute,
};
pub use array_rmw_window_plan::{
    refresh_function_array_rmw_window_routes, refresh_module_array_rmw_window_routes,
    ArrayRmwWindowProof, ArrayRmwWindowRoute,
};
pub use array_string_len_window_plan::{
    refresh_function_array_string_len_window_routes, refresh_module_array_string_len_window_routes,
    ArrayStringLenWindowMode, ArrayStringLenWindowProof, ArrayStringLenWindowRoute,
};
pub use array_string_store_micro_seed_plan::{
    refresh_function_array_string_store_micro_seed_route,
    refresh_module_array_string_store_micro_seed_routes, ArrayStringStoreMicroSeedRoute,
};
pub use array_text_combined_region_plan::{
    refresh_function_array_text_combined_region_routes,
    refresh_module_array_text_combined_region_routes, ArrayTextCombinedRegionByteBoundaryProof,
    ArrayTextCombinedRegionConsumerCapability, ArrayTextCombinedRegionEffect,
    ArrayTextCombinedRegionExecutionMode, ArrayTextCombinedRegionProof,
    ArrayTextCombinedRegionProofRegion, ArrayTextCombinedRegionRoute,
};
pub use array_text_edit_plan::{
    refresh_function_array_text_edit_routes, refresh_module_array_text_edit_routes,
    ArrayTextEditKind, ArrayTextEditProof, ArrayTextEditRoute, ArrayTextEditSplitPolicy,
};
pub use array_text_loopcarry_plan::{
    refresh_function_array_text_loopcarry_len_store_routes,
    refresh_module_array_text_loopcarry_len_store_routes, ArrayTextLoopCarryLenStoreProof,
    ArrayTextLoopCarryLenStoreRoute,
};
pub use array_text_observer_plan::{
    refresh_function_array_text_observer_routes, refresh_module_array_text_observer_routes,
    ArrayTextObserverArgRepr, ArrayTextObserverConsumerShape, ArrayTextObserverKind,
    ArrayTextObserverProofRegion, ArrayTextObserverPublicationBoundary,
    ArrayTextObserverResultRepr, ArrayTextObserverRoute,
};
pub use array_text_observer_region_contract::{
    ArrayTextObserverExecutorCarrier, ArrayTextObserverExecutorConsumerCapability,
    ArrayTextObserverExecutorContract, ArrayTextObserverExecutorEffect,
    ArrayTextObserverExecutorExecutionMode, ArrayTextObserverExecutorMaterializationPolicy,
    ArrayTextObserverExecutorProofRegion, ArrayTextObserverStoreRegionMapping,
};
pub use array_text_residence_session_plan::{
    refresh_function_array_text_residence_session_routes,
    refresh_module_array_text_residence_session_routes, ArrayTextResidenceSessionProof,
    ArrayTextResidenceSessionRoute, ArrayTextResidenceSessionScope,
};
pub use array_text_state_residence_plan::{
    refresh_function_array_text_state_residence_route,
    refresh_module_array_text_state_residence_routes, ArrayTextStateResidence,
    ArrayTextStateResidenceConsumerCapability, ArrayTextStateResidenceContract,
    ArrayTextStateResidenceIndexOfSeedPayload, ArrayTextStateResidenceKind,
    ArrayTextStateResidencePublicationBoundary, ArrayTextStateResidenceResultRepr,
    ArrayTextStateResidenceRoute,
};
pub(crate) use builder::detect_escape_skip_shape;
pub use cfg_extractor::extract_cfg_info; // Phase 154: CFG extraction
pub use concat_const_suffix_micro_seed_plan::{
    refresh_function_concat_const_suffix_micro_seed_route,
    refresh_module_concat_const_suffix_micro_seed_routes, ConcatConstSuffixMicroSeedRoute,
};
pub use core_method_op::{
    CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier, CoreMethodOpProof,
};
pub use definitions::{CallFlags, Callee, MirCall}; // Unified call definitions
pub use effect::{Effect, EffectMask};
pub use escape_barrier::{classify_escape_uses, EscapeBarrier, EscapeUse};
pub use exact_seed_backend_route::{
    refresh_function_exact_seed_backend_route, refresh_module_exact_seed_backend_routes,
    ExactSeedBackendRoute,
};
pub use function::{
    ClosureBodyId, FunctionSignature, MirEnumDecl, MirEnumVariantDecl, MirFunction, MirModule,
    UserBoxFieldDecl,
};
pub use generic_method_route_facts::{
    GenericMethodKeyRoute, GenericMethodPublicationPolicy, GenericMethodReturnShape,
    GenericMethodValueDemand,
};
pub use generic_method_route_plan::{
    refresh_function_generic_method_routes, refresh_module_generic_method_routes,
    GenericMethodRoute,
};
pub use instruction::MirInstruction;
pub use join_ir_runner::{run_joinir_function, JoinRuntimeError, JoinValue};
pub use map_lookup_fusion_plan::{
    refresh_function_map_lookup_fusion_routes, refresh_module_map_lookup_fusion_routes,
};
pub use optimizer::MirOptimizer;
pub use placement_effect::{
    refresh_function_placement_effect_routes, refresh_module_placement_effect_routes,
    PlacementEffectBorrowContract, PlacementEffectDecision, PlacementEffectDemand,
    PlacementEffectPublicationBoundary, PlacementEffectRoute, PlacementEffectSource,
    PlacementEffectState, PlacementEffectStringProof,
};
pub use printer::MirPrinter;
pub use query::{MirQuery, MirQueryBox};
pub use semantic_refresh::{
    refresh_function_semantic_metadata, refresh_function_string_corridor_metadata,
    refresh_module_semantic_metadata,
};
pub use slot_registry::{BoxTypeId, MethodSlot};
pub use spanned_instruction::{SpannedInstRef, SpannedInstruction};
pub use storage_class::{
    refresh_function_storage_class_facts, refresh_module_storage_class_facts, StorageClass,
};
pub use string_corridor::{
    refresh_function_string_corridor_facts, refresh_module_string_corridor_facts,
    StringCorridorBorrowContract, StringCorridorCarrier, StringCorridorFact, StringCorridorOp,
    StringCorridorRole, StringOutcomeFact, StringPlacementFact, StringPublishReason,
    StringPublishReprPolicy, StringStableViewProvenance,
};
pub use string_corridor_placement::{
    refresh_function_string_corridor_candidates, refresh_module_string_corridor_candidates,
    StringCorridorCandidate, StringCorridorCandidateKind, StringCorridorCandidatePlan,
    StringCorridorCandidateProof, StringCorridorCandidateState, StringCorridorPublicationBoundary,
    StringCorridorPublicationContract,
};
pub use string_corridor_relation::{
    refresh_function_string_corridor_relations, refresh_module_string_corridor_relations,
    StringCorridorRelation, StringCorridorRelationKind, StringCorridorWindowContract,
};
pub use string_direct_set_window_plan::{
    refresh_function_string_direct_set_window_routes,
    refresh_module_string_direct_set_window_routes, StringDirectSetWindowRoute,
};
pub use string_kernel_plan::{
    derive_string_kernel_plan, infer_string_kernel_text_consumer,
    refresh_function_string_kernel_plans, refresh_module_string_kernel_plans, StringKernelPlan,
    StringKernelPlanBorrowContract, StringKernelPlanCarrier, StringKernelPlanConsumer,
    StringKernelPlanFamily, StringKernelPlanLegality, StringKernelPlanPart,
    StringKernelPlanPublicationBoundary, StringKernelPlanPublicationContract,
    StringKernelPlanReadAliasFacts, StringKernelPlanRetainedForm, StringKernelPlanSlotHopSubstring,
    StringKernelPlanTextConsumer, StringKernelPlanVerifierOwner,
};
pub use substring_views_micro_seed_plan::{
    refresh_function_substring_views_micro_seed_route,
    refresh_module_substring_views_micro_seed_routes, SubstringViewsMicroSeedRoute,
};
pub use sum_placement::{
    refresh_function_sum_placement_facts, refresh_module_sum_placement_facts,
    SumObjectizationBarrier, SumPlacementFact, SumPlacementState,
};
pub use sum_placement_layout::{
    refresh_function_sum_placement_layouts, refresh_module_sum_placement_layouts,
    SumLocalAggregateLayout, SumPlacementLayout,
};
pub use sum_placement_selection::{
    refresh_function_sum_placement_selections, refresh_module_sum_placement_selections,
    SumPlacementPath, SumPlacementSelection,
};
pub use sum_variant_project_seed_plan::{
    refresh_function_sum_variant_project_seed_route,
    refresh_module_sum_variant_project_seed_routes, SumVariantProjectSeedKind,
    SumVariantProjectSeedPayload, SumVariantProjectSeedRoute,
};
pub use sum_variant_tag_seed_plan::{
    refresh_function_sum_variant_tag_seed_route, refresh_module_sum_variant_tag_seed_routes,
    SumVariantTagSeedKind, SumVariantTagSeedRoute,
};
pub use thin_entry::{
    refresh_function_thin_entry_candidates, refresh_module_thin_entry_candidates,
    ThinEntryCandidate, ThinEntryCurrentCarrier, ThinEntryDemand, ThinEntryPreferredEntry,
    ThinEntrySurface, ThinEntryValueClass,
};
pub use thin_entry_selection::{
    refresh_function_thin_entry_selections, refresh_module_thin_entry_selections,
    ThinEntrySelection, ThinEntrySelectionState,
};
pub use types::{
    BarrierOp, BinaryOp, CompareOp, ConstValue, MirType, TypeOpKind, UnaryOp, WeakRefOp,
};
pub use userbox_known_receiver_method_seed_plan::{
    refresh_module_userbox_known_receiver_method_seed_routes, UserBoxKnownReceiverMethodSeedKind,
    UserBoxKnownReceiverMethodSeedPayload, UserBoxKnownReceiverMethodSeedRoute,
};
pub use userbox_local_scalar_seed_plan::{
    refresh_function_userbox_local_scalar_seed_route,
    refresh_module_userbox_local_scalar_seed_routes, UserBoxLocalScalarSeedKind,
    UserBoxLocalScalarSeedPayload, UserBoxLocalScalarSeedRoute,
    UserBoxLocalScalarSeedSinglePayload,
};
pub use userbox_loop_micro_seed_plan::{
    refresh_function_userbox_loop_micro_seed_route, refresh_module_userbox_loop_micro_seed_routes,
    UserBoxLoopMicroSeedKind, UserBoxLoopMicroSeedRoute,
};
pub use value_consumer::{
    refresh_function_value_consumer_facts, refresh_module_value_consumer_facts, ValueConsumerFacts,
};
pub use value_id::{LocalId, ValueId, ValueIdGenerator};
pub use value_kind::{MirValueKind, TypedValueId}; // Phase 26-A: ValueId型安全化
pub use value_origin::{
    build_value_def_map, resolve_value_origin, resolve_value_origin_from_copy_parents,
    resolve_value_origin_from_parent_map, CopyParentMap, ParentMap, ValueDefMap,
};
pub use verification::MirVerifier;
pub use verification_types::VerificationError;
// Phase 29y.1: RC insertion pass (skeleton)
pub use passes::rc_insertion::{insert_rc_instructions, RcInsertionStats};
// Phase 15 control flow utilities (段階的根治戦略)
pub use utils::{
    capture_actual_predecessor_and_jump, collect_phi_incoming_if_reachable,
    execute_statement_with_termination_check, is_current_block_terminated,
};
