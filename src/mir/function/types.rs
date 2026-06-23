use super::metadata::FunctionMetadata;
use super::object_metadata::{
    ArrayRecordAutoUseEligibilityPlan, ArrayRecordMaterializationBoundaryPlan,
    ArrayRecordPackedAutoUsePilotPlan, ArrayRecordStoragePlan, DirectStatePlan,
    HakoAllocAlignedSmallPackedStorePilotPlan, HakoAllocHugePagePackedStorePilotPlan, RecordDecl,
    RecordLayoutPlan, RecordStateResidencePlan, SourcePackedArrayAutoUsePilotPlan,
    SourcePackedArrayDirectReadConsumptionPlan, TypedObjectPlan, UserBoxFieldDecl,
};
use crate::mir::{BasicBlock, BasicBlockId, ConstValue, EffectMask, MirType, ValueId};
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

/// MIR-side declared parameter metadata.
///
/// This preserves source annotation text without changing the canonical
/// `FunctionSignature.params` / `MirType` ABI surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirParamDecl {
    pub name: String,
    pub declared_type_name: Option<String>,
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

/// Function statistics for profiling and optimization
#[derive(Debug, Clone)]
pub struct FunctionStats {
    pub block_count: usize,
    pub instruction_count: usize,
    pub phi_count: usize,
    pub value_count: usize,
    pub is_pure: bool,
}

/// First exact numeric runtime-check contract vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactNumericRuntimeCheckContractKind {
    DynamicIntegerRange,
}

/// Function-local exact numeric runtime-check contract.
///
/// The contract is anchored to a `FieldSet` site. It does not add VM behavior
/// by itself; it only lets verifier/lowering agree on where a range check must
/// exist before a runtime-range-sensitive exact numeric write can be accepted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactNumericRuntimeCheckContract {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub field: String,
    pub value: ValueId,
    pub declared_type_name: String,
    pub kind: ExactNumericRuntimeCheckContractKind,
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

    /// Record declarations stay in their own lane until record lowering rows
    /// explicitly consume them.
    pub record_decls: BTreeMap<String, RecordDecl>,

    /// Backend-readable typed object layouts derived from user box field metadata.
    pub typed_object_plans: Vec<TypedObjectPlan>,

    /// Metadata-only direct-state candidates derived from user box field metadata.
    pub direct_state_plans: Vec<DirectStatePlan>,

    /// Metadata-only box-private record-state residence candidates.
    pub record_state_residence_plans: Vec<RecordStateResidencePlan>,

    /// Backend-readable record layouts derived from record declaration metadata.
    pub record_layout_plans: Vec<RecordLayoutPlan>,

    /// Metadata-only ArrayBox packed record storage descriptors.
    pub array_record_storage_plans: Vec<ArrayRecordStoragePlan>,

    /// Metadata-only eligibility rows for future ArrayBox inline-record auto-use.
    pub array_record_autouse_eligibility_plans: Vec<ArrayRecordAutoUseEligibilityPlan>,

    /// Metadata-only materialization/escape boundary for future ArrayBox auto-use.
    pub array_record_materialization_boundary_plans: Vec<ArrayRecordMaterializationBoundaryPlan>,

    /// Metadata-only non-escaping packed ArrayBox auto-use pilot rows.
    pub array_record_packed_autouse_pilot_plans: Vec<ArrayRecordPackedAutoUsePilotPlan>,

    /// Metadata-only source `PackedArray<T>` auto-use pilot rows.
    pub source_packed_array_autouse_pilot_plans: Vec<SourcePackedArrayAutoUsePilotPlan>,

    /// Metadata-only source `PackedArray<Record>` direct-read consumption rows.
    /// These rows expose per-record-field direct read slots for future consumers
    /// without enabling runtime/backend lowering.
    pub source_packed_array_direct_read_consumption_plans:
        Vec<SourcePackedArrayDirectReadConsumptionPlan>,

    /// Metadata-only aligned-small hako_alloc packed-store pilot rows.
    pub hako_alloc_aligned_small_packed_store_pilot_plans:
        Vec<HakoAllocAlignedSmallPackedStorePilotPlan>,

    /// Metadata-only huge-page hako_alloc packed-store pilot rows.
    pub hako_alloc_huge_page_packed_store_pilot_plans: Vec<HakoAllocHugePagePackedStorePilotPlan>,

    /// Backend-readable static readonly table rows.
    /// MIR owns this row shape; backend emitters only serialize rows.
    pub static_data_plans: Vec<StaticDataPlan>,

    /// Declared enum inventory for canonical sum lowering and runtime/codegen handoff.
    pub enum_decls: BTreeMap<String, MirEnumDecl>,

    /// Backend-readable boxed runtime ABI rows for enum values that cross
    /// function/container boundaries.
    pub boxed_sum_abi_plans: Vec<crate::mir::boxed_sum_abi_plan::BoxedSumAbiPlanV1>,

    /// NCL-1: Externalized closure bodies (`body_id -> AST body`).
    /// NewClosure keeps only a small descriptor and references this table.
    pub closure_bodies: BTreeMap<ClosureBodyId, Vec<crate::ast::ASTNode>>,

    /// NCL-1: Next stable id for `closure_bodies`.
    pub next_closure_body_id: ClosureBodyId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticDataPlan {
    pub source_name: String,
    pub symbol: String,
    pub element: String,
    pub align: u32,
    pub linkage: String,
    pub unnamed_addr: bool,
    pub values: Vec<u64>,
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
