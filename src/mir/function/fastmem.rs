use crate::ast::Span;
use crate::mir::instruction::FastMemRegionId;
use crate::mir::ValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FastMemTableLengthPolicyKind {
    ExplicitConstLen,
}

impl FastMemTableLengthPolicyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitConstLen => "explicit_const_len",
        }
    }
}

/// FastMemory-owned table length fact for TableIndex access proofs.
///
/// Layout contracts own representation facts such as element layout, stride,
/// and alignment. Length facts live here so later page-map strategies and
/// range proofs can feed one FastMemory access-proof surface without teaching
/// MIRBuilder or lowering to invent table bounds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemTableLengthFact {
    pub fact_id: u32,
    pub region: FastMemRegionId,
    pub table_id: String,
    pub table_value: ValueId,
    pub length_value: ValueId,
    pub resolved_length: Option<u64>,
    pub policy: FastMemTableLengthPolicyKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastMemSameOwnerProofKind {
    SourceAssumeOwnerEq,
}

impl FastMemSameOwnerProofKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceAssumeOwnerEq => "source_assume_owner_eq",
        }
    }
}

/// FastMemory-owned proof that a PageMeta value is on the same allocator owner
/// route for a local free-list operation.
///
/// This is a verifier fact, not a lowering decision. The proof value is the
/// source-side equality token, typically produced by `mem.ownerEq(...)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemSameOwnerFact {
    pub fact_id: u32,
    pub region: FastMemRegionId,
    pub page_value: ValueId,
    pub proof_value: ValueId,
    pub proof_kind: FastMemSameOwnerProofKind,
    pub remote_owner_rejected: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastMemRemoteOwnerProofKind {
    SourceAssumeRemoteOwner,
}

impl FastMemRemoteOwnerProofKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceAssumeRemoteOwner => "source_assume_remote_owner",
        }
    }
}

/// FastMemory-owned proof that a PageMeta value must use the remote-owner
/// publication path rather than same-owner local mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemRemoteOwnerFact {
    pub fact_id: u32,
    pub region: FastMemRegionId,
    pub page_value: ValueId,
    pub proof_kind: FastMemRemoteOwnerProofKind,
    pub same_owner_rejected: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastMemBlockNextProofKind {
    SourceAssumeLocalFreeBlockNext,
    SourceAssumeFreeHeadBlockNext,
    SourceAssumeRemoteFreeBlockNext,
}

impl FastMemBlockNextProofKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceAssumeLocalFreeBlockNext => "source_assume_local_free_block_next",
            Self::SourceAssumeFreeHeadBlockNext => "source_assume_free_head_block_next",
            Self::SourceAssumeRemoteFreeBlockNext => "source_assume_remote_free_block_next",
        }
    }
}

/// FastMemory-owned proof that a candidate block has the free-list `next`
/// storage/provenance required by page-local free-list push routes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemBlockNextFact {
    pub fact_id: u32,
    pub region: FastMemRegionId,
    pub block_value: ValueId,
    pub next_field_id: String,
    pub proof_kind: FastMemBlockNextProofKind,
    pub writable: bool,
    pub provenance_valid: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastMemLocalFreeNonEmptyProofKind {
    SourceAssumeLocalFreeNonEmpty,
}

impl FastMemLocalFreeNonEmptyProofKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceAssumeLocalFreeNonEmpty => "source_assume_local_free_non_empty",
        }
    }
}

/// FastMemory-owned proof that a PageMeta local free-list has a pop candidate.
///
/// This is a source/verifier proof consumed by `LocalFreePop` plans. It is not
/// an ordinary `local_free_head` FieldLoad and does not open pop lowering by
/// itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemLocalFreeNonEmptyFact {
    pub fact_id: u32,
    pub region: FastMemRegionId,
    pub page_value: ValueId,
    pub proof_kind: FastMemLocalFreeNonEmptyProofKind,
    pub non_empty: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastMemFreeHeadNonEmptyProofKind {
    SourceAssumeFreeHeadNonEmpty,
    DerivedFromFreeHeadPush,
}

impl FastMemFreeHeadNonEmptyProofKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceAssumeFreeHeadNonEmpty => "source_assume_free_head_non_empty",
            Self::DerivedFromFreeHeadPush => "derived_from_free_head_push",
        }
    }
}

/// FastMemory-owned proof that a PageMeta ordinary free list has a pop
/// candidate.
///
/// This is consumed by `FreeHeadPop` plans. It is not an ordinary `free_head`
/// FieldLoad/FieldStore route and does not open pop lowering by itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemFreeHeadNonEmptyFact {
    pub fact_id: u32,
    pub region: FastMemRegionId,
    pub page_value: ValueId,
    pub proof_kind: FastMemFreeHeadNonEmptyProofKind,
    pub non_empty: bool,
}

/// Function-local site metadata for a source FastMemory field access.
#[derive(Debug, Clone, PartialEq)]
pub struct FastMemFieldAccessSite {
    pub site_id: String,
    pub source_span: Span,
    pub region: Option<FastMemRegionId>,
    pub base_value: ValueId,
    pub field_id: String,
    pub layout_id: Option<String>,
    pub access_kind: String,
    pub required_route: String,
    pub fallback_policy: String,
}

/// Function-local site metadata for a source FastMemory table access.
#[derive(Debug, Clone, PartialEq)]
pub struct FastMemIndexAccessSite {
    pub site_id: String,
    pub source_span: Span,
    pub region: Option<FastMemRegionId>,
    pub base_value: ValueId,
    pub index_value: ValueId,
    pub table_id: Option<String>,
    pub layout_id: Option<String>,
    pub access_kind: String,
    pub required_route: String,
    pub fallback_policy: String,
}

/// Function-local metadata for a source `fastmem ContractName { ... }` region.
///
/// This is side-table contract metadata, not an executable begin/end marker.
/// Executable fast-memory operations point back here through
/// `MirInstruction::MemOp.region`.
#[derive(Debug, Clone, PartialEq)]
pub struct FastMemRegionMetadata {
    pub id: FastMemRegionId,
    pub contract: String,
    pub source_span: crate::ast::Span,
    pub origin: FastMemRegionOrigin,
    pub body_statement_count: usize,
    pub emitted_memop_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastMemRegionOrigin {
    SourceFastMemBlock,
}
