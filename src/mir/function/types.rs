use super::metadata::FunctionMetadata;
use super::object_metadata::{
    ArrayRecordAutoUseEligibilityPlan, ArrayRecordMaterializationBoundaryPlan,
    ArrayRecordPackedAutoUsePilotPlan, ArrayRecordStoragePlan, DirectStatePlan,
    HakoAllocAlignedSmallPackedStorePilotPlan, HakoAllocHugePagePackedStorePilotPlan, RecordDecl,
    RecordLayoutPlan, RecordStateResidencePlan, SourcePackedArrayAutoUsePilotPlan,
    SourcePackedArrayDirectReadConsumptionPlan, TypedObjectPlan, UserBoxFieldDecl,
    WeakFieldContractSpec,
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
    pub implicit_receiver: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterEntryContractKind {
    ExactNumeric,
}

/// Executable semantic contract for one explicit source parameter.
///
/// Function ownership supplies function identity and refresh freshness. The
/// row carries both call-boundary position and callee-body ValueId so entry
/// validation cannot silently drift from register binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterEntryContract {
    pub contract_id: String,
    pub formal_parameter_index: usize,
    pub source_parameter_index: usize,
    pub parameter_value_id: ValueId,
    pub source_parameter_name: String,
    pub declared_type_name: String,
    pub contract_kind: ParameterEntryContractKind,
    pub implicit_receiver: bool,
    pub runtime_check_required: bool,
    pub proof_elision_allowed: bool,
    pub backend_capability_required: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReturnExitContractKind {
    ExactNumeric,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReturnExitVoidPolicy {
    RejectVoid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReturnExitContractOwner {
    FunctionReturnContract,
}

/// Executable semantic contract for one explicit source return annotation.
///
/// Function ownership supplies identity and refresh freshness. Return operands
/// deliberately stay out of this carrier because the final runtime outcome is
/// the only value checked by the exit owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnExitContract {
    pub contract_id: String,
    pub declared_type_name: String,
    pub contract_kind: ReturnExitContractKind,
    pub void_policy: ReturnExitVoidPolicy,
    pub runtime_check_required: bool,
    pub proof_elision_allowed: bool,
    pub backend_capability_required: String,
    pub source_return_annotation_present: bool,
    pub owner: ReturnExitContractOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalContractWriteKind {
    Init,
    Reassign,
}

/// Executable semantic contract for one lexical local slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSlotContract {
    pub contract_id: String,
    pub local_slot_id: crate::mir::LocalSlotId,
    pub diagnostic_source_name: String,
    pub declared_type_name: String,
    pub runtime_check_required: bool,
    pub proof_elision_allowed: bool,
    pub backend_capability_required: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocalIdentityEvidence {
    pub local_slot_id: crate::mir::LocalSlotId,
    pub merge_value_id: ValueId,
    pub incoming_values: Vec<ValueId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordValueBoundaryKind {
    Construct,
    WithUpdate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordContractDisposition {
    AnyDefault,
    RuntimeCheckedContract,
    VerifierProvenContract { proof_id: String },
    UnsupportedFailFast,
}

/// One active record-field semantic contract at a value publication site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordFieldValueContract {
    pub field_index: usize,
    pub diagnostic_field_name: String,
    pub value_id: ValueId,
    pub declared_type_name: String,
    pub disposition: RecordContractDisposition,
}

/// Function-owned record construction/update contract rebuilt from semantic
/// operations and the module's source-owned RecordDecl inventory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordValueContract {
    pub contract_id: String,
    pub boundary: RecordValueBoundaryKind,
    pub diagnostic_record_name: String,
    pub schema_fingerprint: String,
    pub dst_value_id: ValueId,
    pub base_value_id: Option<ValueId>,
    pub fields: Vec<RecordFieldValueContract>,
    pub backend_capability_required: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ArrayStateTermId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayStateTermKind {
    Fresh { allocation_site: ValueId },
    SameAs { source: ValueId },
    Select { inputs: Vec<ValueId> },
    DynamicBoundary { value: ValueId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayStateTerm {
    pub term_id: ArrayStateTermId,
    pub value: ValueId,
    pub kind: ArrayStateTermKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayElementWriteWitness {
    pub site_id: crate::mir::ArrayWriteSiteId,
    pub kind: crate::mir::ArrayElementWriteKind,
    pub producer: crate::mir::ArrayWriteProducerKind,
    pub receiver: ValueId,
    pub index: Option<ValueId>,
    pub value: ValueId,
    pub state_term: ArrayStateTermId,
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

/// Typed rejection for publication that would replace an existing function.
///
/// Legacy module assembly still has an explicit overwrite-capable entry. The
/// canonical function transaction uses this error so duplicate publication
/// cannot silently destroy the first sealed draft.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionPublicationErrorV1 {
    pub function_name: String,
}

impl std::fmt::Display for FunctionPublicationErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][canonical_function_publication/duplicate] function={}",
            self.function_name
        )
    }
}

impl std::error::Error for FunctionPublicationErrorV1 {}

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

    /// Source-owned Weak field contracts rebuilt from typed box declarations.
    pub weak_field_contract_specs: Vec<WeakFieldContractSpec>,

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

    /// Source-owned readonly table contracts. Plans are derived from these rows.
    pub static_table_contract_specs: Vec<StaticTableContractSpec>,

    /// Rebuilt proof that source specs, plans, and load sites agree.
    pub verified_static_table_contracts: Vec<VerifiedStaticTableContract>,

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

    /// Explicit module-level canonical-recursion backend capability witness.
    /// P0c-MR-C0 defines the passive slot; only a later recursive-module
    /// transaction may install it.
    pub(crate) canonical_recursive_callable_module_capability:
        Option<crate::mir::canonical_recursive_callable_module_capability::CanonicalRecursiveCallableModuleCapabilityV1>,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StaticTableId {
    pub module_name: String,
    pub declaration_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticElementType {
    U16,
}

impl StaticElementType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::U16 => "u16",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticTableContractSpec {
    pub table_id: StaticTableId,
    pub diagnostic_name: String,
    pub element: StaticElementType,
    pub values: Vec<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticTableContractProof {
    SourceSpecAndPlanStructurallyMatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedStaticTableContract {
    pub table_id: StaticTableId,
    pub element: StaticElementType,
    pub len: u32,
    pub plan_symbol: String,
    pub proof: StaticTableContractProof,
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
