use crate::mir::instruction::FastMemRegionId;
use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastMemAccessPlanKind {
    TableIndex,
    FieldLoad,
    FieldStore,
    LocalFreePush,
    LocalFreePop,
    FreeHeadPush,
    FreeHeadPop,
    AtomicRemoteHeadPush,
    AtomicRemoteHeadDrain,
    DrainRemoteListToLocal,
}

impl FastMemAccessPlanKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TableIndex => "table_index",
            Self::FieldLoad => "field_load",
            Self::FieldStore => "field_store",
            Self::LocalFreePush => "local_free_push",
            Self::LocalFreePop => "local_free_pop",
            Self::FreeHeadPush => "free_head_push",
            Self::FreeHeadPop => "free_head_pop",
            Self::AtomicRemoteHeadPush => "atomic_remote_head_push",
            Self::AtomicRemoteHeadDrain => "atomic_remote_head_drain",
            Self::DrainRemoteListToLocal => "drain_remote_list_to_local",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastMemAccessPlanStatus {
    SymbolicOnly,
    Verified,
    Rejected,
}

impl FastMemAccessPlanStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SymbolicOnly => "symbolic_only",
            Self::Verified => "verified",
            Self::Rejected => "rejected",
        }
    }

    pub fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastMemFieldAccessMode {
    Load,
    Store,
}

impl FastMemFieldAccessMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Load => "load",
            Self::Store => "store",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemFieldAccessPlan {
    pub layout_id: Option<String>,
    pub field_id: String,
    pub base: ValueId,
    pub value: Option<ValueId>,
    pub result: Option<ValueId>,
    pub mode: FastMemFieldAccessMode,
    pub byte_offset: Option<u32>,
    pub field_size: Option<u32>,
    pub field_type: Option<String>,
    pub alignment: Option<u32>,
    pub mutability: Option<String>,
    pub field_class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemTableAccessPlan {
    pub table_id: String,
    pub table: ValueId,
    pub index: ValueId,
    pub result: Option<ValueId>,
    pub element_layout_id: Option<String>,
    pub element_repr: Option<String>,
    pub element_stride: Option<u32>,
    pub element_size: Option<u32>,
    pub length: Option<u64>,
    pub alignment: Option<u32>,
    pub index_policy: Option<String>,
    pub proof: FastMemTableAccessProof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemTableFieldAccessLink {
    pub table_block: BasicBlockId,
    pub table_instruction_index: usize,
    pub field_block: BasicBlockId,
    pub field_instruction_index: usize,
    pub region: FastMemRegionId,
    pub table_result: ValueId,
    pub field_base: ValueId,
    pub field_id: String,
    pub field_access: FastMemFieldAccessMode,
    pub byte_offset: u32,
    pub field_size: u32,
    pub field_type: String,
    pub alignment: u32,
    pub proof: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FastMemResolvedFieldPlan {
    pub layout_id: Option<String>,
    pub field_id: Option<String>,
    pub field_class: Option<String>,
    pub byte_offset: Option<u32>,
    pub field_size: Option<u32>,
    pub field_type: Option<String>,
    pub alignment: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemLocalFreeListPlan {
    pub page: ValueId,
    pub block: Option<ValueId>,
    pub result: Option<ValueId>,
    pub local_free_head: FastMemResolvedFieldPlan,
    pub block_next: FastMemResolvedFieldPlan,
    pub same_owner_proof_valid: bool,
    pub block_next_proof_valid: bool,
    pub non_empty_proof_valid: bool,
    pub remote_owner_rejected: bool,
    pub lowerable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemFreeHeadListPlan {
    pub page: ValueId,
    pub block: Option<ValueId>,
    pub result: Option<ValueId>,
    pub free_head: FastMemResolvedFieldPlan,
    pub block_next: FastMemResolvedFieldPlan,
    pub same_owner_proof_valid: bool,
    pub block_next_proof_valid: bool,
    pub non_empty_proof_valid: bool,
    pub remote_owner_rejected: bool,
    pub lowerable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemAtomicRemoteHeadPlan {
    pub page: ValueId,
    pub block: Option<ValueId>,
    pub result: Option<ValueId>,
    pub remote_head: FastMemResolvedFieldPlan,
    pub block_next: FastMemResolvedFieldPlan,
    pub remote_owner_required: bool,
    pub remote_owner_proof_valid: bool,
    pub block_next_required: bool,
    pub block_next_proof_valid: bool,
    pub memory_order_policy: String,
    pub retry_attempt_limit: u32,
    pub lowerable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemDrainRemoteListToLocalPlan {
    pub page: ValueId,
    pub token: ValueId,
    pub token_source_block: Option<BasicBlockId>,
    pub token_source_instruction_index: Option<usize>,
    pub token_provenance_valid: bool,
    pub page_operand_valid: bool,
    pub head_class_resolved: bool,
    pub local_list_head_class: Option<String>,
    pub local_free_head: FastMemResolvedFieldPlan,
    pub block_next: FastMemResolvedFieldPlan,
    pub block_next_access_resolved: bool,
    pub publication_order: String,
    pub lowerable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemTableAccessProof {
    pub table_length_resolved: bool,
    pub bounds_proof_valid: bool,
    pub stride_resolved: bool,
    pub field_offset_resolved: bool,
    pub overflow_proof_valid: bool,
    pub alignment_valid: bool,
    pub element_layout_verified: bool,
    pub table_length_policy: Option<String>,
    pub bounds_proof: Option<String>,
    pub overflow_proof: Option<String>,
    pub failure_reason: Option<String>,
}

impl FastMemTableAccessProof {
    pub fn is_lowerable(&self) -> bool {
        self.table_length_resolved
            && self.bounds_proof_valid
            && self.stride_resolved
            && self.field_offset_resolved
            && self.overflow_proof_valid
            && self.alignment_valid
            && self.element_layout_verified
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FastMemAccessPlanPayload {
    Field(FastMemFieldAccessPlan),
    Table(FastMemTableAccessPlan),
    LocalFree(FastMemLocalFreeListPlan),
    FreeHead(FastMemFreeHeadListPlan),
    AtomicRemoteHead(FastMemAtomicRemoteHeadPlan),
    DrainRemoteListToLocal(FastMemDrainRemoteListToLocalPlan),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemAccessPlan {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub region: FastMemRegionId,
    pub kind: FastMemAccessPlanKind,
    pub status: FastMemAccessPlanStatus,
    pub failure_reason: Option<String>,
    pub payload: FastMemAccessPlanPayload,
}

impl FastMemAccessPlan {
    pub fn is_verified(&self) -> bool {
        self.status.is_verified()
    }
}
