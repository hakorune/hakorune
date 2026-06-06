/*!
 * MIR-owned FastMemory layout/table access plans.
 *
 * `MemOpAccess` carries symbolic source ids. This module publishes the next
 * metadata seam: a function-local access-plan row for each layout/table MemOp
 * site. Verified rows are produced only by the memory-profile contract
 * resolver. LLVM GEP/load/store lowering remains closed until it consumes
 * verified rows without recomputing layout/table facts.
 */

use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};
use crate::mir::{
    fastmem_layout_contract::{
        resolve_fastmem_block_next_contract, resolve_fastmem_field_contract,
        resolve_fastmem_table_contract,
    },
    function::{
        FastMemBlockNextFact, FastMemBlockNextProofKind, FastMemFreeHeadNonEmptyFact,
        FastMemFreeHeadNonEmptyProofKind, FastMemLocalFreeNonEmptyFact, FastMemRegionMetadata,
        FastMemRemoteOwnerFact, FastMemSameOwnerFact, FastMemTableLengthFact, RangeIndexFact,
    },
};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemLocalFreeListPlan {
    pub page: ValueId,
    pub block: Option<ValueId>,
    pub result: Option<ValueId>,
    pub local_free_head_layout_id: Option<String>,
    pub local_free_head_field_id: Option<String>,
    pub local_free_head_field_class: Option<String>,
    pub local_free_head_byte_offset: Option<u32>,
    pub local_free_head_field_size: Option<u32>,
    pub local_free_head_field_type: Option<String>,
    pub local_free_head_alignment: Option<u32>,
    pub block_next_layout_id: Option<String>,
    pub block_next_field_id: Option<String>,
    pub block_next_field_class: Option<String>,
    pub block_next_byte_offset: Option<u32>,
    pub block_next_field_size: Option<u32>,
    pub block_next_field_type: Option<String>,
    pub block_next_alignment: Option<u32>,
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
    pub free_head_layout_id: Option<String>,
    pub free_head_field_id: Option<String>,
    pub free_head_field_class: Option<String>,
    pub free_head_byte_offset: Option<u32>,
    pub free_head_field_size: Option<u32>,
    pub free_head_field_type: Option<String>,
    pub free_head_alignment: Option<u32>,
    pub block_next_layout_id: Option<String>,
    pub block_next_field_id: Option<String>,
    pub block_next_field_class: Option<String>,
    pub block_next_byte_offset: Option<u32>,
    pub block_next_field_size: Option<u32>,
    pub block_next_field_type: Option<String>,
    pub block_next_alignment: Option<u32>,
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
    pub remote_head_layout_id: Option<String>,
    pub remote_head_field_id: Option<String>,
    pub remote_head_field_class: Option<String>,
    pub remote_head_byte_offset: Option<u32>,
    pub remote_head_field_size: Option<u32>,
    pub remote_head_field_type: Option<String>,
    pub remote_head_alignment: Option<u32>,
    pub block_next_layout_id: Option<String>,
    pub block_next_field_id: Option<String>,
    pub block_next_field_class: Option<String>,
    pub block_next_byte_offset: Option<u32>,
    pub block_next_field_size: Option<u32>,
    pub block_next_field_type: Option<String>,
    pub block_next_alignment: Option<u32>,
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
    pub local_free_head_layout_id: Option<String>,
    pub local_free_head_field_id: Option<String>,
    pub local_free_head_field_class: Option<String>,
    pub local_free_head_byte_offset: Option<u32>,
    pub local_free_head_field_size: Option<u32>,
    pub local_free_head_field_type: Option<String>,
    pub local_free_head_alignment: Option<u32>,
    pub block_next_layout_id: Option<String>,
    pub block_next_field_id: Option<String>,
    pub block_next_field_class: Option<String>,
    pub block_next_byte_offset: Option<u32>,
    pub block_next_field_size: Option<u32>,
    pub block_next_field_type: Option<String>,
    pub block_next_alignment: Option<u32>,
    pub block_next_access_resolved: bool,
    pub publication_order: String,
    pub lowerable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolvedHeadAccess {
    failure_reason: Option<String>,
    layout_id: Option<String>,
    field_id: Option<String>,
    field_class: Option<String>,
    byte_offset: Option<u32>,
    field_size: Option<u32>,
    field_type: Option<String>,
    alignment: Option<u32>,
}

impl ResolvedHeadAccess {
    fn is_resolved(&self) -> bool {
        self.failure_reason.is_none()
            && self.byte_offset.is_some()
            && self.field_size.is_some()
            && self.field_type.is_some()
            && self.alignment.is_some()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ResolvedBlockNextAccess {
    layout_id: Option<String>,
    field_id: Option<String>,
    field_class: Option<String>,
    byte_offset: Option<u32>,
    field_size: Option<u32>,
    field_type: Option<String>,
    alignment: Option<u32>,
}

impl ResolvedBlockNextAccess {
    fn is_resolved(&self) -> bool {
        self.layout_id.is_some()
            && self.field_id.is_some()
            && self.byte_offset.is_some()
            && self.field_size.is_some()
            && self.field_type.is_some()
            && self.alignment.is_some()
    }
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

struct FastMemFactStore<'a> {
    regions: &'a [FastMemRegionMetadata],
    table_length_facts: &'a [FastMemTableLengthFact],
    same_owner_facts: &'a [FastMemSameOwnerFact],
    remote_owner_facts: &'a [FastMemRemoteOwnerFact],
    block_next_facts: &'a [FastMemBlockNextFact],
    local_free_non_empty_facts: &'a [FastMemLocalFreeNonEmptyFact],
    free_head_non_empty_facts: &'a [FastMemFreeHeadNonEmptyFact],
    remote_drain_token_facts: &'a [FastMemRemoteDrainTokenFact],
    range_index_facts: &'a [RangeIndexFact],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FastMemRemoteDrainTokenFact {
    region: FastMemRegionId,
    page_value: ValueId,
    token_value: ValueId,
    block: BasicBlockId,
    instruction_index: usize,
}

impl<'a> FastMemFactStore<'a> {
    fn table_length(
        &self,
        region: FastMemRegionId,
        table_id: &str,
        table_value: ValueId,
    ) -> Option<&'a FastMemTableLengthFact> {
        self.table_length_facts.iter().find(|fact| {
            fact.region == region && fact.table_id == table_id && fact.table_value == table_value
        })
    }

    fn range_bounds_proof(
        &self,
        block: BasicBlockId,
        index_value: ValueId,
        length_fact: &FastMemTableLengthFact,
    ) -> Option<String> {
        self.range_index_facts.iter().find_map(|fact| {
            if fact.index_value == index_value
                && fact.upper_exclusive_value == length_fact.length_value
                && fact.body_bb == block
                && fact.step == 1
                && fact.end_exclusive
                && fact.index_body_read_only
                && !fact.loop_carried_writes_supported
            {
                Some(format!("range_fact:{}", fact.fact_id))
            } else {
                None
            }
        })
    }

    fn same_owner(
        &self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Option<&'a FastMemSameOwnerFact> {
        self.same_owner_facts
            .iter()
            .find(|fact| fact.region == region && fact.page_value == page_value)
    }

    fn remote_owner(
        &self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Option<&'a FastMemRemoteOwnerFact> {
        self.remote_owner_facts
            .iter()
            .find(|fact| fact.region == region && fact.page_value == page_value)
    }

    fn block_next(
        &self,
        region: FastMemRegionId,
        block_value: ValueId,
    ) -> Option<&'a FastMemBlockNextFact> {
        self.block_next_facts
            .iter()
            .find(|fact| fact.region == region && fact.block_value == block_value)
    }

    fn local_free_non_empty(
        &self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Option<&'a FastMemLocalFreeNonEmptyFact> {
        self.local_free_non_empty_facts
            .iter()
            .find(|fact| fact.region == region && fact.page_value == page_value)
    }

    fn free_head_non_empty(
        &self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Option<&'a FastMemFreeHeadNonEmptyFact> {
        self.free_head_non_empty_facts
            .iter()
            .find(|fact| fact.region == region && fact.page_value == page_value)
    }

    fn remote_drain_token(
        &self,
        region: FastMemRegionId,
        page_value: ValueId,
        token_value: ValueId,
    ) -> Option<&'a FastMemRemoteDrainTokenFact> {
        self.remote_drain_token_facts.iter().find(|fact| {
            fact.region == region
                && fact.page_value == page_value
                && fact.token_value == token_value
        })
    }
}

pub fn refresh_function_fastmem_access_plans(function: &mut MirFunction) {
    let mut plans = Vec::new();
    let regions = function.metadata.fastmem_regions.clone();
    let table_length_facts = function.metadata.fastmem_table_length_facts.clone();
    let same_owner_facts = function.metadata.fastmem_same_owner_facts.clone();
    let remote_owner_facts = function.metadata.fastmem_remote_owner_facts.clone();
    let block_next_facts = function.metadata.fastmem_block_next_facts.clone();
    let local_free_non_empty_facts = function.metadata.fastmem_local_free_non_empty_facts.clone();
    let mut free_head_non_empty_facts: Vec<_> = function
        .metadata
        .fastmem_free_head_non_empty_facts
        .iter()
        .filter(|fact| fact.proof_kind != FastMemFreeHeadNonEmptyProofKind::DerivedFromFreeHeadPush)
        .cloned()
        .collect();
    let range_index_facts = function.metadata.range_index_facts.clone();
    let remote_drain_token_facts = collect_remote_drain_token_facts(function);

    for block_id in function.block_ids() {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, sp) in block.all_spanned_instructions_enumerated() {
            let MirInstruction::MemOp {
                region,
                kind,
                dst,
                operands,
                access,
                ..
            } = sp.inst
            else {
                continue;
            };
            let fact_store = FastMemFactStore {
                regions: &regions,
                table_length_facts: &table_length_facts,
                same_owner_facts: &same_owner_facts,
                remote_owner_facts: &remote_owner_facts,
                block_next_facts: &block_next_facts,
                local_free_non_empty_facts: &local_free_non_empty_facts,
                free_head_non_empty_facts: &free_head_non_empty_facts,
                remote_drain_token_facts: &remote_drain_token_facts,
                range_index_facts: &range_index_facts,
            };
            let Some(plan) = plan_from_memop(
                block_id,
                instruction_index,
                *region,
                *kind,
                *dst,
                operands,
                access.as_ref(),
                region_contract(&regions, *region),
                &fact_store,
            ) else {
                continue;
            };
            maybe_add_derived_free_head_non_empty_fact(&plan, &mut free_head_non_empty_facts);
            plans.push(plan);
        }
    }

    let table_field_links = table_field_access_links(&mut plans);
    function.metadata.fastmem_access_plans = plans;
    function.metadata.fastmem_table_field_access_links = table_field_links;
    function.metadata.fastmem_free_head_non_empty_facts = free_head_non_empty_facts;
}

fn plan_from_memop(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    kind: MemOpKind,
    dst: Option<ValueId>,
    operands: &[ValueId],
    access: Option<&MemOpAccess>,
    contract: Option<&str>,
    facts: &FastMemFactStore<'_>,
) -> Option<FastMemAccessPlan> {
    match kind {
        MemOpKind::TableIndex => table_plan(
            block,
            instruction_index,
            region,
            dst,
            operands,
            access,
            contract,
            facts,
        ),
        MemOpKind::FieldLoad => field_plan(
            block,
            instruction_index,
            region,
            dst,
            operands,
            access,
            FastMemFieldAccessMode::Load,
            contract,
        ),
        MemOpKind::FieldStore => field_plan(
            block,
            instruction_index,
            region,
            dst,
            operands,
            access,
            FastMemFieldAccessMode::Store,
            contract,
        ),
        MemOpKind::LocalFreePush => local_free_plan(
            block,
            instruction_index,
            region,
            FastMemAccessPlanKind::LocalFreePush,
            dst,
            operands,
            contract,
            facts,
        ),
        MemOpKind::LocalFreePop => local_free_plan(
            block,
            instruction_index,
            region,
            FastMemAccessPlanKind::LocalFreePop,
            dst,
            operands,
            contract,
            facts,
        ),
        MemOpKind::FreeHeadPop => free_head_plan(
            block,
            instruction_index,
            region,
            FastMemAccessPlanKind::FreeHeadPop,
            dst,
            operands,
            contract,
            facts,
        ),
        MemOpKind::FreeHeadPush => free_head_plan(
            block,
            instruction_index,
            region,
            FastMemAccessPlanKind::FreeHeadPush,
            dst,
            operands,
            contract,
            facts,
        ),
        MemOpKind::AtomicRemoteHeadPush => atomic_remote_head_plan(
            block,
            instruction_index,
            region,
            FastMemAccessPlanKind::AtomicRemoteHeadPush,
            dst,
            operands,
            contract,
            facts,
        ),
        MemOpKind::AtomicRemoteHeadDrain => atomic_remote_head_plan(
            block,
            instruction_index,
            region,
            FastMemAccessPlanKind::AtomicRemoteHeadDrain,
            dst,
            operands,
            contract,
            facts,
        ),
        MemOpKind::DrainRemoteListToLocal => {
            drain_remote_list_to_local_plan(block, instruction_index, region, dst, operands, facts)
        }
        _ => None,
    }
}

fn collect_remote_drain_token_facts(function: &MirFunction) -> Vec<FastMemRemoteDrainTokenFact> {
    let mut facts = Vec::new();
    for block_id in function.block_ids() {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, sp) in block.all_spanned_instructions_enumerated() {
            let MirInstruction::MemOp {
                region,
                kind,
                dst,
                operands,
                ..
            } = sp.inst
            else {
                continue;
            };
            if *kind != MemOpKind::AtomicRemoteHeadDrain {
                continue;
            }
            let (Some(token_value), Some(page_value)) = (*dst, operands.first().copied()) else {
                continue;
            };
            facts.push(FastMemRemoteDrainTokenFact {
                region: *region,
                page_value,
                token_value,
                block: block_id,
                instruction_index,
            });
        }
    }
    facts
}

fn region_contract(regions: &[FastMemRegionMetadata], region: FastMemRegionId) -> Option<&str> {
    regions
        .iter()
        .find(|metadata| metadata.id == region)
        .map(|metadata| metadata.contract.as_str())
}

fn table_plan(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    dst: Option<ValueId>,
    operands: &[ValueId],
    access: Option<&MemOpAccess>,
    contract: Option<&str>,
    facts: &FastMemFactStore<'_>,
) -> Option<FastMemAccessPlan> {
    let access = access?;
    let table_id = access.table_id.as_ref()?.clone();
    let table = operands.first().copied()?;
    let index = operands.get(1).copied()?;
    let table_length_fact = facts.table_length(region, &table_id, table);
    let bounds_proof = table_length_fact
        .and_then(|length_fact| facts.range_bounds_proof(block, index, length_fact));
    let resolved = contract.map(|contract| {
        resolve_fastmem_table_contract(contract, &table_id).map_err(|err| err.reason())
    });
    let (
        status,
        mut failure_reason,
        element_layout_id,
        element_repr,
        element_stride,
        element_size,
        _contract_length,
        alignment,
        index_policy,
    ) = match resolved {
        Some(Ok(resolved)) if resolved.lowerable => (
            FastMemAccessPlanStatus::Verified,
            None,
            Some(resolved.element_layout_id),
            Some(resolved.element_repr),
            Some(resolved.element_stride),
            Some(resolved.element_size),
            resolved.length,
            Some(resolved.alignment),
            Some(resolved.index_policy),
        ),
        Some(Ok(resolved)) => (
            FastMemAccessPlanStatus::Rejected,
            resolved.non_lowerable_reason,
            Some(resolved.element_layout_id),
            Some(resolved.element_repr),
            Some(resolved.element_stride),
            Some(resolved.element_size),
            resolved.length,
            Some(resolved.alignment),
            Some(resolved.index_policy),
        ),
        Some(Err(reason)) => (
            FastMemAccessPlanStatus::Rejected,
            Some(reason),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        None => (
            FastMemAccessPlanStatus::SymbolicOnly,
            Some("layout-table-contract-unresolved".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    };
    let length = table_length_fact.and_then(|fact| fact.resolved_length);
    let table_length_policy = table_length_fact.map(|fact| fact.policy.as_str().to_string());
    if table_length_fact.is_some() && failure_reason.as_deref() == Some("table-length-unresolved") {
        failure_reason = None;
    }
    let proof = FastMemTableAccessProof {
        table_length_resolved: table_length_fact.is_some(),
        bounds_proof_valid: bounds_proof.is_some(),
        stride_resolved: element_stride.is_some(),
        field_offset_resolved: false,
        overflow_proof_valid: false,
        alignment_valid: alignment.is_some(),
        element_layout_verified: element_layout_id.is_some(),
        table_length_policy,
        bounds_proof,
        overflow_proof: None,
        failure_reason: failure_reason.clone(),
    };
    let status = if status == FastMemAccessPlanStatus::Verified && !proof.is_lowerable() {
        FastMemAccessPlanStatus::Rejected
    } else {
        status
    };
    let failure_reason = failure_reason.or_else(|| {
        if status == FastMemAccessPlanStatus::Rejected && !proof.is_lowerable() {
            Some("verified-table-access-proof-incomplete".to_string())
        } else {
            None
        }
    });

    Some(FastMemAccessPlan {
        block,
        instruction_index,
        region,
        kind: FastMemAccessPlanKind::TableIndex,
        status,
        failure_reason,
        payload: FastMemAccessPlanPayload::Table(FastMemTableAccessPlan {
            table_id,
            table,
            index,
            result: dst,
            element_layout_id,
            element_repr,
            element_stride,
            element_size,
            length,
            alignment,
            index_policy,
            proof,
        }),
    })
}

fn table_field_access_links(plans: &mut [FastMemAccessPlan]) -> Vec<FastMemTableFieldAccessLink> {
    let mut links = Vec::new();

    for table_index in 0..plans.len() {
        let Some((table_block, table_instruction_index, region, table_result)) =
            table_link_source(&plans[table_index])
        else {
            continue;
        };

        for field_plan in plans.iter() {
            let Some(field_link) = field_link_target(
                field_plan,
                table_block,
                table_instruction_index,
                region,
                table_result,
            ) else {
                continue;
            };
            links.push(field_link);
        }
    }

    for plan in plans.iter_mut() {
        let Some((table_block, table_instruction_index, region, table_result)) =
            table_link_source(plan)
        else {
            continue;
        };
        let has_link = links.iter().any(|link| {
            link.table_block == table_block
                && link.table_instruction_index == table_instruction_index
                && link.region == region
                && link.table_result == table_result
        });
        if has_link {
            if let FastMemAccessPlanPayload::Table(table) = &mut plan.payload {
                table.proof.field_offset_resolved = true;
                apply_table_overflow_proof(
                    table,
                    table_block,
                    table_instruction_index,
                    region,
                    &links,
                );
            }
            let lowerable = match &plan.payload {
                FastMemAccessPlanPayload::Table(table) => table.proof.is_lowerable(),
                FastMemAccessPlanPayload::Field(_) | FastMemAccessPlanPayload::LocalFree(_) => {
                    false
                }
                FastMemAccessPlanPayload::FreeHead(_)
                | FastMemAccessPlanPayload::AtomicRemoteHead(_)
                | FastMemAccessPlanPayload::DrainRemoteListToLocal(_) => false,
            };
            if lowerable {
                plan.status = FastMemAccessPlanStatus::Verified;
                plan.failure_reason = None;
                if let FastMemAccessPlanPayload::Table(table) = &mut plan.payload {
                    table.proof.failure_reason = None;
                }
            } else if plan.status == FastMemAccessPlanStatus::Verified {
                plan.status = FastMemAccessPlanStatus::Rejected;
            }
            if plan.status == FastMemAccessPlanStatus::Rejected && plan.failure_reason.is_none() {
                let reason = "verified-table-access-proof-incomplete".to_string();
                plan.failure_reason = Some(reason.clone());
                if let FastMemAccessPlanPayload::Table(table) = &mut plan.payload {
                    if table.proof.failure_reason.is_none() {
                        table.proof.failure_reason = Some(reason);
                    }
                }
            }
        }
    }

    links
}

fn apply_table_overflow_proof(
    table: &mut FastMemTableAccessPlan,
    table_block: BasicBlockId,
    table_instruction_index: usize,
    region: FastMemRegionId,
    links: &[FastMemTableFieldAccessLink],
) {
    if !(table.proof.table_length_resolved
        && table.proof.bounds_proof_valid
        && table.proof.stride_resolved
        && table.proof.field_offset_resolved
        && table.proof.alignment_valid
        && table.proof.element_layout_verified)
    {
        return;
    }

    let Some(table_result) = table.result else {
        return;
    };
    let Some(length) = table.length else {
        return;
    };
    let Some(stride) = table.element_stride else {
        return;
    };
    let Some(element_size) = table.element_size else {
        return;
    };
    let table_links = links
        .iter()
        .filter(|link| {
            link.table_block == table_block
                && link.table_instruction_index == table_instruction_index
                && link.region == region
                && link.table_result == table_result
        })
        .collect::<Vec<_>>();
    if table_links.is_empty() {
        return;
    }

    let target_max = target_usize_max();
    let Some(table_byte_len) = u128::from(length).checked_mul(u128::from(stride)) else {
        return;
    };
    if table_byte_len > target_max {
        return;
    }
    for link in &table_links {
        let Some(field_end) = u128::from(link.byte_offset).checked_add(u128::from(link.field_size))
        else {
            return;
        };
        if field_end > u128::from(element_size) || field_end > target_max {
            return;
        }
    }

    table.proof.overflow_proof_valid = true;
    table.proof.overflow_proof = Some(format!(
        "usize_mul_add_no_overflow+offset_within_object:len={}:stride={}:element_size={}:fields={}",
        length,
        stride,
        element_size,
        table_links
            .iter()
            .map(|link| link.field_id.as_str())
            .collect::<Vec<_>>()
            .join(",")
    ));
}

fn target_usize_max() -> u128 {
    if usize::BITS == 128 {
        u128::MAX
    } else {
        (1_u128 << usize::BITS) - 1
    }
}

fn table_link_source(
    plan: &FastMemAccessPlan,
) -> Option<(BasicBlockId, usize, FastMemRegionId, ValueId)> {
    let FastMemAccessPlanPayload::Table(table) = &plan.payload else {
        return None;
    };
    Some((
        plan.block,
        plan.instruction_index,
        plan.region,
        table.result?,
    ))
}

fn field_link_target(
    plan: &FastMemAccessPlan,
    table_block: BasicBlockId,
    table_instruction_index: usize,
    region: FastMemRegionId,
    table_result: ValueId,
) -> Option<FastMemTableFieldAccessLink> {
    if plan.status != FastMemAccessPlanStatus::Verified
        || plan.block != table_block
        || plan.region != region
        || plan.instruction_index <= table_instruction_index
    {
        return None;
    }
    let FastMemAccessPlanPayload::Field(field) = &plan.payload else {
        return None;
    };
    if field.base != table_result {
        return None;
    }
    Some(FastMemTableFieldAccessLink {
        table_block,
        table_instruction_index,
        field_block: plan.block,
        field_instruction_index: plan.instruction_index,
        region,
        table_result,
        field_base: field.base,
        field_id: field.field_id.clone(),
        field_access: field.mode,
        byte_offset: field.byte_offset?,
        field_size: field.field_size?,
        field_type: field.field_type.clone()?,
        alignment: field.alignment?,
        proof: format!(
            "table_field_link:{}:{}",
            table_instruction_index, plan.instruction_index
        ),
    })
}

fn field_plan(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    dst: Option<ValueId>,
    operands: &[ValueId],
    access: Option<&MemOpAccess>,
    mode: FastMemFieldAccessMode,
    contract: Option<&str>,
) -> Option<FastMemAccessPlan> {
    let access = access?;
    let field_id = access.field_id.as_ref()?.clone();
    let base = operands.first().copied()?;
    let value = if mode == FastMemFieldAccessMode::Store {
        operands.get(1).copied()
    } else {
        None
    };
    let resolved = contract.map(|contract| {
        resolve_fastmem_field_contract(contract, &field_id, mode).map_err(|err| err.reason())
    });
    let (
        status,
        failure_reason,
        layout_id,
        canonical_field_id,
        byte_offset,
        field_size,
        field_type,
        alignment,
        mutability,
        field_class,
    ) = match resolved {
        Some(Ok(resolved)) => (
            FastMemAccessPlanStatus::Verified,
            None,
            Some(resolved.layout_id),
            resolved.field_id,
            Some(resolved.byte_offset),
            Some(resolved.field_size),
            Some(resolved.field_type),
            Some(resolved.alignment),
            Some(resolved.mutability),
            Some(resolved.field_class),
        ),
        Some(Err(reason)) => (
            FastMemAccessPlanStatus::Rejected,
            Some(reason),
            access.layout_id.clone(),
            field_id,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        None => (
            FastMemAccessPlanStatus::SymbolicOnly,
            Some("layout-field-contract-unresolved".to_string()),
            access.layout_id.clone(),
            field_id,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    };
    Some(FastMemAccessPlan {
        block,
        instruction_index,
        region,
        kind: match mode {
            FastMemFieldAccessMode::Load => FastMemAccessPlanKind::FieldLoad,
            FastMemFieldAccessMode::Store => FastMemAccessPlanKind::FieldStore,
        },
        status,
        failure_reason,
        payload: FastMemAccessPlanPayload::Field(FastMemFieldAccessPlan {
            layout_id,
            field_id: canonical_field_id,
            base,
            value,
            result: dst,
            mode,
            byte_offset,
            field_size,
            field_type,
            alignment,
            mutability,
            field_class,
        }),
    })
}

fn resolve_head_access(
    contract: Option<&str>,
    field_id: &str,
    mode: FastMemFieldAccessMode,
) -> ResolvedHeadAccess {
    match contract.map(|contract| {
        resolve_fastmem_field_contract(contract, field_id, mode).map_err(|err| err.reason())
    }) {
        Some(Ok(resolved)) => ResolvedHeadAccess {
            failure_reason: None,
            layout_id: Some(resolved.layout_id),
            field_id: Some(resolved.field_id),
            field_class: Some(resolved.field_class),
            byte_offset: Some(resolved.byte_offset),
            field_size: Some(resolved.field_size),
            field_type: Some(resolved.field_type),
            alignment: Some(resolved.alignment),
        },
        Some(Err(reason)) => ResolvedHeadAccess {
            failure_reason: Some(reason),
            layout_id: None,
            field_id: None,
            field_class: None,
            byte_offset: None,
            field_size: None,
            field_type: None,
            alignment: None,
        },
        None => ResolvedHeadAccess {
            failure_reason: Some("layout-field-contract-unresolved".to_string()),
            layout_id: None,
            field_id: None,
            field_class: None,
            byte_offset: None,
            field_size: None,
            field_type: None,
            alignment: None,
        },
    }
}

fn resolve_block_next_access(contract: Option<&str>, field_id: &str) -> ResolvedBlockNextAccess {
    let Some(resolved) =
        contract.and_then(|contract| resolve_fastmem_block_next_contract(contract, field_id).ok())
    else {
        return ResolvedBlockNextAccess::default();
    };

    ResolvedBlockNextAccess {
        layout_id: Some(resolved.layout_id),
        field_id: Some(resolved.field_id),
        field_class: Some(resolved.field_class),
        byte_offset: Some(resolved.byte_offset),
        field_size: Some(resolved.field_size),
        field_type: Some(resolved.field_type),
        alignment: Some(resolved.alignment),
    }
}

fn local_free_plan(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    kind: FastMemAccessPlanKind,
    dst: Option<ValueId>,
    operands: &[ValueId],
    contract: Option<&str>,
    facts: &FastMemFactStore<'_>,
) -> Option<FastMemAccessPlan> {
    let page = operands.first().copied()?;
    let block_value = if kind == FastMemAccessPlanKind::LocalFreePush {
        operands.get(1).copied()
    } else {
        None
    };
    let head_access =
        resolve_head_access(contract, "local_free_head", FastMemFieldAccessMode::Load);
    let same_owner_proof_valid = facts
        .same_owner(region, page)
        .map_or(false, |fact| fact.remote_owner_rejected);
    let remote_owner_rejected = same_owner_proof_valid;
    let non_empty_proof_valid = facts
        .local_free_non_empty(region, page)
        .map_or(false, |fact| fact.non_empty);
    let block_next_field_id = "next";
    let block_next_fact = block_value.and_then(|block_value| {
        facts.block_next(region, block_value).filter(|fact| {
            fact.next_field_id == block_next_field_id && fact.writable && fact.provenance_valid
        })
    });
    let block_next_access = if let Some(fact) = block_next_fact {
        resolve_block_next_access(contract, &fact.next_field_id)
    } else if kind == FastMemAccessPlanKind::LocalFreePop && non_empty_proof_valid {
        resolve_block_next_access(contract, block_next_field_id)
    } else {
        ResolvedBlockNextAccess::default()
    };
    let block_next_proof_valid = block_next_fact.is_some() && block_next_access.is_resolved();
    let block_next_access_resolved = block_next_access.is_resolved();
    let common_lowerable = head_access.is_resolved() && same_owner_proof_valid;
    let lowerable_push =
        kind == FastMemAccessPlanKind::LocalFreePush && common_lowerable && block_next_proof_valid;
    let lowerable_pop = kind == FastMemAccessPlanKind::LocalFreePop
        && common_lowerable
        && non_empty_proof_valid
        && block_next_access_resolved;
    let lowerable = lowerable_push || lowerable_pop;
    let status = if lowerable {
        FastMemAccessPlanStatus::Verified
    } else {
        FastMemAccessPlanStatus::Rejected
    };
    let failure_reason = head_access.failure_reason.clone().or_else(|| {
        if !same_owner_proof_valid {
            Some("local-free-same-owner-proof-missing".to_string())
        } else if kind == FastMemAccessPlanKind::LocalFreePush && !block_next_proof_valid {
            Some("local-free-block-next-proof-missing".to_string())
        } else if kind == FastMemAccessPlanKind::LocalFreePop && !non_empty_proof_valid {
            Some("local-free-non-empty-proof-missing".to_string())
        } else if kind == FastMemAccessPlanKind::LocalFreePop && !block_next_access_resolved {
            Some("local-free-block-next-access-unresolved".to_string())
        } else {
            None
        }
    });

    Some(FastMemAccessPlan {
        block,
        instruction_index,
        region,
        kind,
        status,
        failure_reason,
        payload: FastMemAccessPlanPayload::LocalFree(FastMemLocalFreeListPlan {
            page,
            block: block_value,
            result: dst,
            local_free_head_layout_id: head_access.layout_id,
            local_free_head_field_id: head_access.field_id,
            local_free_head_field_class: head_access.field_class,
            local_free_head_byte_offset: head_access.byte_offset,
            local_free_head_field_size: head_access.field_size,
            local_free_head_field_type: head_access.field_type,
            local_free_head_alignment: head_access.alignment,
            block_next_layout_id: block_next_access.layout_id,
            block_next_field_id: block_next_access.field_id,
            block_next_field_class: block_next_access.field_class,
            block_next_byte_offset: block_next_access.byte_offset,
            block_next_field_size: block_next_access.field_size,
            block_next_field_type: block_next_access.field_type,
            block_next_alignment: block_next_access.alignment,
            same_owner_proof_valid,
            block_next_proof_valid,
            non_empty_proof_valid,
            remote_owner_rejected,
            lowerable,
        }),
    })
}

fn free_head_plan(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    kind: FastMemAccessPlanKind,
    dst: Option<ValueId>,
    operands: &[ValueId],
    contract: Option<&str>,
    facts: &FastMemFactStore<'_>,
) -> Option<FastMemAccessPlan> {
    let page = operands.first().copied()?;
    let block_value = if kind == FastMemAccessPlanKind::FreeHeadPush {
        operands.get(1).copied()
    } else {
        None
    };
    let head_access = resolve_head_access(contract, "free_head", FastMemFieldAccessMode::Load);
    let same_owner_proof_valid = facts
        .same_owner(region, page)
        .map_or(false, |fact| fact.remote_owner_rejected);
    let remote_owner_rejected = same_owner_proof_valid;
    let non_empty_proof_valid = facts
        .free_head_non_empty(region, page)
        .map_or(false, |fact| fact.non_empty);
    let block_next_field_id = "next";
    let block_next_fact = block_value.and_then(|block_value| {
        facts.block_next(region, block_value).filter(|fact| {
            fact.next_field_id == block_next_field_id && fact.writable && fact.provenance_valid
        })
    });
    let block_next_access = if let Some(fact) = block_next_fact {
        resolve_block_next_access(contract, &fact.next_field_id)
    } else if kind == FastMemAccessPlanKind::FreeHeadPop && non_empty_proof_valid {
        resolve_block_next_access(contract, block_next_field_id)
    } else {
        ResolvedBlockNextAccess::default()
    };
    let block_next_access_resolved = block_next_access.is_resolved();
    let block_next_proof_valid = block_next_fact.is_some() && block_next_access.is_resolved();
    let common_lowerable = head_access.is_resolved() && same_owner_proof_valid;
    let lowerable_push =
        kind == FastMemAccessPlanKind::FreeHeadPush && common_lowerable && block_next_proof_valid;
    let lowerable_pop = kind == FastMemAccessPlanKind::FreeHeadPop
        && common_lowerable
        && non_empty_proof_valid
        && block_next_access_resolved;
    let lowerable = lowerable_push || lowerable_pop;
    let status = if lowerable {
        FastMemAccessPlanStatus::Verified
    } else {
        FastMemAccessPlanStatus::Rejected
    };
    let failure_reason = head_access.failure_reason.clone().or_else(|| {
        if !same_owner_proof_valid {
            Some("free-head-same-owner-proof-missing".to_string())
        } else if kind == FastMemAccessPlanKind::FreeHeadPush && !block_next_proof_valid {
            Some("free-head-block-next-proof-missing".to_string())
        } else if kind == FastMemAccessPlanKind::FreeHeadPop && !non_empty_proof_valid {
            Some("free-head-non-empty-proof-missing".to_string())
        } else if kind == FastMemAccessPlanKind::FreeHeadPop && !block_next_access_resolved {
            Some("free-head-block-next-access-unresolved".to_string())
        } else {
            None
        }
    });

    Some(FastMemAccessPlan {
        block,
        instruction_index,
        region,
        kind,
        status,
        failure_reason,
        payload: FastMemAccessPlanPayload::FreeHead(FastMemFreeHeadListPlan {
            page,
            block: block_value,
            result: dst,
            free_head_layout_id: head_access.layout_id,
            free_head_field_id: head_access.field_id,
            free_head_field_class: head_access.field_class,
            free_head_byte_offset: head_access.byte_offset,
            free_head_field_size: head_access.field_size,
            free_head_field_type: head_access.field_type,
            free_head_alignment: head_access.alignment,
            block_next_layout_id: block_next_access.layout_id,
            block_next_field_id: block_next_access.field_id,
            block_next_field_class: block_next_access.field_class,
            block_next_byte_offset: block_next_access.byte_offset,
            block_next_field_size: block_next_access.field_size,
            block_next_field_type: block_next_access.field_type,
            block_next_alignment: block_next_access.alignment,
            same_owner_proof_valid,
            block_next_proof_valid,
            non_empty_proof_valid,
            remote_owner_rejected,
            lowerable,
        }),
    })
}

fn atomic_remote_head_plan(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    kind: FastMemAccessPlanKind,
    dst: Option<ValueId>,
    operands: &[ValueId],
    contract: Option<&str>,
    facts: &FastMemFactStore<'_>,
) -> Option<FastMemAccessPlan> {
    let page = operands.first().copied()?;
    let block_value = if kind == FastMemAccessPlanKind::AtomicRemoteHeadPush {
        operands.get(1).copied()
    } else {
        None
    };
    let head_access = resolve_head_access(contract, "remote_head", FastMemFieldAccessMode::Load);
    let block_next_field_id = "next";
    let block_next_fact = block_value.and_then(|block_value| {
        facts.block_next(region, block_value).filter(|fact| {
            fact.next_field_id == block_next_field_id
                && fact.writable
                && fact.provenance_valid
                && fact.proof_kind == FastMemBlockNextProofKind::SourceAssumeRemoteFreeBlockNext
        })
    });
    let block_next_access = if let Some(fact) = block_next_fact {
        resolve_block_next_access(contract, &fact.next_field_id)
    } else {
        ResolvedBlockNextAccess::default()
    };
    let block_next_proof_valid = block_next_fact.is_some() && block_next_access.is_resolved();
    let remote_owner_required = kind == FastMemAccessPlanKind::AtomicRemoteHeadPush;
    let remote_owner_proof_valid = if remote_owner_required {
        facts
            .remote_owner(region, page)
            .map_or(false, |fact| fact.same_owner_rejected)
    } else {
        false
    };
    let block_next_required = kind == FastMemAccessPlanKind::AtomicRemoteHeadPush;
    let memory_order_policy = if kind == FastMemAccessPlanKind::AtomicRemoteHeadDrain {
        "acquire_exchange".to_string()
    } else {
        "acq_rel".to_string()
    };
    let retry_attempt_limit = if kind == FastMemAccessPlanKind::AtomicRemoteHeadPush {
        3
    } else {
        0
    };
    let lowerable = if kind == FastMemAccessPlanKind::AtomicRemoteHeadDrain {
        head_access.is_resolved()
    } else {
        kind == FastMemAccessPlanKind::AtomicRemoteHeadPush
            && head_access.is_resolved()
            && remote_owner_proof_valid
            && block_next_proof_valid
    };
    let status = if lowerable {
        FastMemAccessPlanStatus::Verified
    } else {
        FastMemAccessPlanStatus::Rejected
    };
    let failure_reason = head_access.failure_reason.clone().or_else(|| {
        if lowerable {
            None
        } else if kind == FastMemAccessPlanKind::AtomicRemoteHeadDrain {
            Some("atomic-remote-head-drain-plan-not-lowerable".to_string())
        } else if !remote_owner_proof_valid {
            Some("atomic-remote-head-remote-owner-proof-missing".to_string())
        } else if !block_next_proof_valid {
            Some("atomic-remote-head-block-next-proof-missing".to_string())
        } else {
            Some("atomic-remote-head-cas-lowering-closed".to_string())
        }
    });

    Some(FastMemAccessPlan {
        block,
        instruction_index,
        region,
        kind,
        status,
        failure_reason,
        payload: FastMemAccessPlanPayload::AtomicRemoteHead(FastMemAtomicRemoteHeadPlan {
            page,
            block: block_value,
            result: dst,
            remote_head_layout_id: head_access.layout_id,
            remote_head_field_id: head_access.field_id,
            remote_head_field_class: head_access.field_class,
            remote_head_byte_offset: head_access.byte_offset,
            remote_head_field_size: head_access.field_size,
            remote_head_field_type: head_access.field_type,
            remote_head_alignment: head_access.alignment,
            block_next_layout_id: block_next_access.layout_id,
            block_next_field_id: block_next_access.field_id,
            block_next_field_class: block_next_access.field_class,
            block_next_byte_offset: block_next_access.byte_offset,
            block_next_field_size: block_next_access.field_size,
            block_next_field_type: block_next_access.field_type,
            block_next_alignment: block_next_access.alignment,
            remote_owner_required,
            remote_owner_proof_valid,
            block_next_required,
            block_next_proof_valid,
            memory_order_policy,
            retry_attempt_limit,
            lowerable,
        }),
    })
}

fn drain_remote_list_to_local_plan(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    dst: Option<ValueId>,
    operands: &[ValueId],
    facts: &FastMemFactStore<'_>,
) -> Option<FastMemAccessPlan> {
    if dst.is_some() {
        return None;
    }
    let page = operands.first().copied()?;
    let token = operands.get(1).copied()?;
    let token_fact = facts.remote_drain_token(region, page, token);
    let token_provenance_valid = token_fact.is_some();
    let page_operand_valid = token_provenance_valid;
    let contract = region_contract(facts.regions, region);
    let local_free_head =
        resolve_head_access(contract, "local_free_head", FastMemFieldAccessMode::Store);
    let block_next_access = resolve_block_next_access(contract, "next");
    let head_class_resolved = token_provenance_valid && local_free_head.is_resolved();
    let block_next_access_resolved = block_next_access.is_resolved();
    let local_list_head_class =
        head_class_resolved.then(|| "owner_local_free_or_free_head".to_string());
    let publication_order = if head_class_resolved {
        "verifier_owned_acquire_then_owner_local"
    } else {
        "closed"
    }
    .to_string();
    let lowerable = token_provenance_valid
        && page_operand_valid
        && head_class_resolved
        && block_next_access_resolved;
    let status = if lowerable {
        FastMemAccessPlanStatus::Verified
    } else {
        FastMemAccessPlanStatus::Rejected
    };
    let failure_reason = if lowerable {
        None
    } else if !token_provenance_valid {
        Some("drain-remote-list-token-provenance-missing".to_string())
    } else if !page_operand_valid {
        Some("drain-remote-list-page-operand-mismatch".to_string())
    } else if !head_class_resolved {
        Some("drain-remote-list-target-head-class-unresolved".to_string())
    } else if !block_next_access_resolved {
        Some("drain-remote-list-block-next-access-unresolved".to_string())
    } else {
        Some("drain-remote-list-to-local-lowering-closed".to_string())
    };

    Some(FastMemAccessPlan {
        block,
        instruction_index,
        region,
        kind: FastMemAccessPlanKind::DrainRemoteListToLocal,
        status,
        failure_reason,
        payload: FastMemAccessPlanPayload::DrainRemoteListToLocal(
            FastMemDrainRemoteListToLocalPlan {
                page,
                token,
                token_source_block: token_fact.map(|fact| fact.block),
                token_source_instruction_index: token_fact.map(|fact| fact.instruction_index),
                token_provenance_valid,
                page_operand_valid,
                head_class_resolved,
                local_list_head_class,
                local_free_head_layout_id: local_free_head.layout_id,
                local_free_head_field_id: local_free_head.field_id,
                local_free_head_field_class: local_free_head.field_class,
                local_free_head_byte_offset: local_free_head.byte_offset,
                local_free_head_field_size: local_free_head.field_size,
                local_free_head_field_type: local_free_head.field_type,
                local_free_head_alignment: local_free_head.alignment,
                block_next_layout_id: block_next_access.layout_id,
                block_next_field_id: block_next_access.field_id,
                block_next_field_class: block_next_access.field_class,
                block_next_byte_offset: block_next_access.byte_offset,
                block_next_field_size: block_next_access.field_size,
                block_next_field_type: block_next_access.field_type,
                block_next_alignment: block_next_access.alignment,
                block_next_access_resolved,
                publication_order,
                lowerable,
            },
        ),
    })
}

fn maybe_add_derived_free_head_non_empty_fact(
    plan: &FastMemAccessPlan,
    facts: &mut Vec<FastMemFreeHeadNonEmptyFact>,
) {
    if plan.kind != FastMemAccessPlanKind::FreeHeadPush || !plan.is_verified() {
        return;
    }
    let FastMemAccessPlanPayload::FreeHead(push) = &plan.payload else {
        return;
    };
    if !push.lowerable || !push.same_owner_proof_valid || !push.block_next_proof_valid {
        return;
    }
    if facts
        .iter()
        .any(|fact| fact.region == plan.region && fact.page_value == push.page && fact.non_empty)
    {
        return;
    }
    facts.push(FastMemFreeHeadNonEmptyFact {
        fact_id: facts.len() as u32,
        region: plan.region,
        page_value: push.page,
        proof_kind: FastMemFreeHeadNonEmptyProofKind::DerivedFromFreeHeadPush,
        non_empty: true,
    });
}

#[cfg(test)]
mod tests;
