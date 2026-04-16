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
pub mod basic_block;
pub mod builder;
mod compiler;
pub mod contracts; // backend-core instruction contracts (SSOT)
pub mod definitions; // Unified MIR definitions (MirCall, Callee, etc.)
pub mod diagnostics; // freeze diagnostics helpers (SSOT)
pub mod effect;
pub mod escape_barrier; // escape operand-role vocabulary (SSOT)
pub mod function;
pub mod if_in_loop_phi; // Phase 187-2: Minimal if-in-loop PHI emitter (extracted from loop_builder)
pub mod instruction;
pub mod instruction_introspection; // Introspection helpers for tests (instruction names)
pub mod instruction_kinds; // small kind-specific metadata (Const/BinOp)
pub mod loop_api; // Minimal LoopBuilder facade (adapter-ready)
pub mod loop_canonicalizer; // Phase 1: Loop skeleton canonicalization (AST preprocessing)
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
pub mod string_corridor_placement; // placement/effect scaffold over canonical string facts
pub(crate) mod string_corridor_recognizer; // shared pure shape recognizers for string corridor
pub mod string_corridor_relation; // string-corridor relation layer over generic PHI queries
pub mod string_kernel_plan; // backend-consumable string plan seam derived from corridor candidates
pub mod sum_placement; // sum-local proving slice for later generic placement/effect pass
pub mod sum_placement_layout; // LLVM-side payload-lane choices for selected local sums
pub mod sum_placement_selection; // selection pilot over sum-local placement facts
pub mod thin_entry; // thin-entry inventory for known local routes
pub mod thin_entry_selection; // manifest-driven thin-entry selection pilot
pub mod type_propagation; // Phase 279 P0: SSOT type propagation pipeline
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
pub(crate) use builder::detect_escape_skip_shape;
pub use cfg_extractor::extract_cfg_info; // Phase 154: CFG extraction
pub use definitions::{CallFlags, Callee, MirCall}; // Unified call definitions
pub use effect::{Effect, EffectMask};
pub use escape_barrier::{classify_escape_uses, EscapeBarrier, EscapeUse};
pub use function::{
    ClosureBodyId, FunctionSignature, MirEnumDecl, MirEnumVariantDecl, MirFunction, MirModule,
    UserBoxFieldDecl,
};
pub use instruction::MirInstruction;
pub use join_ir_runner::{run_joinir_function, JoinRuntimeError, JoinValue};
pub use optimizer::MirOptimizer;
pub use placement_effect::{
    refresh_function_placement_effect_routes, refresh_module_placement_effect_routes,
    PlacementEffectDecision, PlacementEffectPublicationBoundary, PlacementEffectRoute,
    PlacementEffectSource, PlacementEffectState, PlacementEffectStringProof,
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
    StringCorridorCarrier, StringCorridorFact, StringCorridorOp, StringCorridorRole,
    StringOutcomeFact, StringPlacementFact,
};
pub use string_corridor_placement::{
    refresh_function_string_corridor_candidates, refresh_module_string_corridor_candidates,
    StringCorridorCandidate, StringCorridorCandidateKind, StringCorridorCandidatePlan,
    StringCorridorCandidateProof, StringCorridorCandidateState,
    StringCorridorPublicationBoundary, StringCorridorPublicationContract,
};
pub use string_corridor_relation::{
    refresh_function_string_corridor_relations, refresh_module_string_corridor_relations,
    StringCorridorRelation, StringCorridorRelationKind, StringCorridorWindowContract,
};
pub use string_kernel_plan::{
    derive_string_kernel_plan, refresh_function_string_kernel_plans,
    refresh_module_string_kernel_plans, StringKernelPlan, StringKernelPlanConsumer,
    StringKernelPlanFamily, StringKernelPlanLegality, StringKernelPlanPart,
    StringKernelPlanPublicationBoundary, StringKernelPlanPublicationContract,
    StringKernelPlanRetainedForm,
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
pub use thin_entry::{
    refresh_function_thin_entry_candidates, refresh_module_thin_entry_candidates,
    ThinEntryCandidate, ThinEntryCurrentCarrier, ThinEntryPreferredEntry, ThinEntrySurface,
    ThinEntryValueClass,
};
pub use thin_entry_selection::{
    refresh_function_thin_entry_selections, refresh_module_thin_entry_selections,
    ThinEntrySelection, ThinEntrySelectionState,
};
pub use types::{
    BarrierOp, BinaryOp, CompareOp, ConstValue, MirType, TypeOpKind, UnaryOp, WeakRefOp,
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
