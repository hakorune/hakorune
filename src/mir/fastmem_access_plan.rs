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
        FastMemBlockNextFact, FastMemLocalFreeNonEmptyFact, FastMemRegionMetadata,
        FastMemSameOwnerFact, FastMemTableLengthFact, RangeIndexFact,
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
}

impl FastMemAccessPlanKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TableIndex => "table_index",
            Self::FieldLoad => "field_load",
            Self::FieldStore => "field_store",
            Self::LocalFreePush => "local_free_push",
            Self::LocalFreePop => "local_free_pop",
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

pub fn refresh_function_fastmem_access_plans(function: &mut MirFunction) {
    let mut plans = Vec::new();
    let regions = function.metadata.fastmem_regions.clone();
    let table_length_facts = function.metadata.fastmem_table_length_facts.clone();
    let same_owner_facts = function.metadata.fastmem_same_owner_facts.clone();
    let block_next_facts = function.metadata.fastmem_block_next_facts.clone();
    let local_free_non_empty_facts = function.metadata.fastmem_local_free_non_empty_facts.clone();
    let range_index_facts = function.metadata.range_index_facts.clone();

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
            let Some(plan) = plan_from_memop(
                block_id,
                instruction_index,
                *region,
                *kind,
                *dst,
                operands,
                access.as_ref(),
                region_contract(&regions, *region),
                &table_length_facts,
                &same_owner_facts,
                &block_next_facts,
                &local_free_non_empty_facts,
                &range_index_facts,
            ) else {
                continue;
            };
            plans.push(plan);
        }
    }

    let table_field_links = table_field_access_links(&mut plans);
    function.metadata.fastmem_access_plans = plans;
    function.metadata.fastmem_table_field_access_links = table_field_links;
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
    table_length_facts: &[FastMemTableLengthFact],
    same_owner_facts: &[FastMemSameOwnerFact],
    block_next_facts: &[FastMemBlockNextFact],
    local_free_non_empty_facts: &[FastMemLocalFreeNonEmptyFact],
    range_index_facts: &[RangeIndexFact],
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
            table_length_facts,
            range_index_facts,
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
            same_owner_facts,
            block_next_facts,
            local_free_non_empty_facts,
        ),
        MemOpKind::LocalFreePop => local_free_plan(
            block,
            instruction_index,
            region,
            FastMemAccessPlanKind::LocalFreePop,
            dst,
            operands,
            contract,
            same_owner_facts,
            block_next_facts,
            local_free_non_empty_facts,
        ),
        _ => None,
    }
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
    table_length_facts: &[FastMemTableLengthFact],
    range_index_facts: &[RangeIndexFact],
) -> Option<FastMemAccessPlan> {
    let access = access?;
    let table_id = access.table_id.as_ref()?.clone();
    let table = operands.first().copied()?;
    let index = operands.get(1).copied()?;
    let table_length_fact = table_length_fact(table_length_facts, region, &table_id, table);
    let bounds_proof = table_length_fact
        .and_then(|length_fact| range_bounds_proof(range_index_facts, block, index, length_fact));
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

fn range_bounds_proof(
    facts: &[RangeIndexFact],
    block: BasicBlockId,
    index_value: ValueId,
    length_fact: &FastMemTableLengthFact,
) -> Option<String> {
    facts.iter().find_map(|fact| {
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

fn table_length_fact<'a>(
    facts: &'a [FastMemTableLengthFact],
    region: FastMemRegionId,
    table_id: &str,
    table_value: ValueId,
) -> Option<&'a FastMemTableLengthFact> {
    facts.iter().find(|fact| {
        fact.region == region && fact.table_id == table_id && fact.table_value == table_value
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

fn local_free_plan(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    kind: FastMemAccessPlanKind,
    dst: Option<ValueId>,
    operands: &[ValueId],
    contract: Option<&str>,
    same_owner_facts: &[FastMemSameOwnerFact],
    block_next_facts: &[FastMemBlockNextFact],
    local_free_non_empty_facts: &[FastMemLocalFreeNonEmptyFact],
) -> Option<FastMemAccessPlan> {
    let page = operands.first().copied()?;
    let block_value = if kind == FastMemAccessPlanKind::LocalFreePush {
        operands.get(1).copied()
    } else {
        None
    };
    let resolved = contract.map(|contract| {
        resolve_fastmem_field_contract(contract, "local_free_head", FastMemFieldAccessMode::Load)
            .map_err(|err| err.reason())
    });
    let (
        failure_reason,
        layout_id,
        field_id,
        field_class,
        head_byte_offset,
        head_field_size,
        head_field_type,
        head_alignment,
    ) = match resolved {
        Some(Ok(resolved)) => (
            None,
            Some(resolved.layout_id),
            Some(resolved.field_id),
            Some(resolved.field_class),
            Some(resolved.byte_offset),
            Some(resolved.field_size),
            Some(resolved.field_type),
            Some(resolved.alignment),
        ),
        Some(Err(reason)) => (Some(reason), None, None, None, None, None, None, None),
        None => (
            Some("layout-field-contract-unresolved".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    };
    let same_owner_proof_valid = same_owner_fact(same_owner_facts, region, page)
        .map_or(false, |fact| fact.remote_owner_rejected);
    let remote_owner_rejected = same_owner_proof_valid;
    let non_empty_proof_valid = local_free_non_empty_fact(local_free_non_empty_facts, region, page)
        .map_or(false, |fact| fact.non_empty);
    let block_next_field_id = "next";
    let block_next_fact = block_value.and_then(|block_value| {
        block_next_fact(block_next_facts, region, block_value).filter(|fact| {
            fact.next_field_id == block_next_field_id && fact.writable && fact.provenance_valid
        })
    });
    let block_next_resolved = if let Some(fact) = block_next_fact {
        contract.and_then(|contract| {
            resolve_fastmem_block_next_contract(contract, &fact.next_field_id).ok()
        })
    } else if kind == FastMemAccessPlanKind::LocalFreePop && non_empty_proof_valid {
        contract.and_then(|contract| {
            resolve_fastmem_block_next_contract(contract, block_next_field_id).ok()
        })
    } else {
        None
    };
    let (
        block_next_layout_id,
        block_next_field_id,
        block_next_field_class,
        block_next_byte_offset,
        block_next_field_size,
        block_next_field_type,
        block_next_alignment,
    ) = if let Some(resolved) = block_next_resolved {
        (
            Some(resolved.layout_id),
            Some(resolved.field_id),
            Some(resolved.field_class),
            Some(resolved.byte_offset),
            Some(resolved.field_size),
            Some(resolved.field_type),
            Some(resolved.alignment),
        )
    } else {
        (None, None, None, None, None, None, None)
    };
    let block_next_proof_valid = block_next_fact.is_some()
        && block_next_layout_id.is_some()
        && block_next_field_id.is_some()
        && block_next_byte_offset.is_some()
        && block_next_field_size.is_some()
        && block_next_field_type.is_some()
        && block_next_alignment.is_some();
    let block_next_access_resolved = block_next_layout_id.is_some()
        && block_next_field_id.is_some()
        && block_next_byte_offset.is_some()
        && block_next_field_size.is_some()
        && block_next_field_type.is_some()
        && block_next_alignment.is_some();
    let common_lowerable = failure_reason.is_none()
        && same_owner_proof_valid
        && head_byte_offset.is_some()
        && head_field_size.is_some()
        && head_field_type.is_some()
        && head_alignment.is_some();
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
    let failure_reason = failure_reason.or_else(|| {
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
            local_free_head_layout_id: layout_id,
            local_free_head_field_id: field_id,
            local_free_head_field_class: field_class,
            local_free_head_byte_offset: head_byte_offset,
            local_free_head_field_size: head_field_size,
            local_free_head_field_type: head_field_type,
            local_free_head_alignment: head_alignment,
            block_next_layout_id,
            block_next_field_id,
            block_next_field_class,
            block_next_byte_offset,
            block_next_field_size,
            block_next_field_type,
            block_next_alignment,
            same_owner_proof_valid,
            block_next_proof_valid,
            non_empty_proof_valid,
            remote_owner_rejected,
            lowerable,
        }),
    })
}

fn same_owner_fact<'a>(
    facts: &'a [FastMemSameOwnerFact],
    region: FastMemRegionId,
    page_value: ValueId,
) -> Option<&'a FastMemSameOwnerFact> {
    facts
        .iter()
        .find(|fact| fact.region == region && fact.page_value == page_value)
}

fn block_next_fact<'a>(
    facts: &'a [FastMemBlockNextFact],
    region: FastMemRegionId,
    block_value: ValueId,
) -> Option<&'a FastMemBlockNextFact> {
    facts
        .iter()
        .find(|fact| fact.region == region && fact.block_value == block_value)
}

fn local_free_non_empty_fact<'a>(
    facts: &'a [FastMemLocalFreeNonEmptyFact],
    region: FastMemRegionId,
    page_value: ValueId,
) -> Option<&'a FastMemLocalFreeNonEmptyFact> {
    facts
        .iter()
        .find(|fact| fact.region == region && fact.page_value == page_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::function::{
        FastMemBlockNextFact, FastMemBlockNextProofKind, FastMemLocalFreeNonEmptyFact,
        FastMemLocalFreeNonEmptyProofKind, FastMemRegionMetadata, FastMemRegionOrigin,
        FastMemSameOwnerFact, FastMemSameOwnerProofKind, FastMemTableLengthFact,
        FastMemTableLengthPolicyKind, RangeIndexFact, RangeIndexFactOriginKind,
    };
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

    fn make_function(instructions: Vec<MirInstruction>) -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.fastmem/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let block = function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block");
        for instruction in instructions {
            block.add_instruction(instruction);
        }
        function
            .metadata
            .fastmem_regions
            .push(FastMemRegionMetadata {
                id: FastMemRegionId::new(0),
                contract: "PageMapV0".to_string(),
                source_span: Span::unknown(),
                origin: FastMemRegionOrigin::SourceFastMemBlock,
                body_statement_count: 1,
                emitted_memop_count: function
                    .blocks
                    .get(&BasicBlockId::new(0))
                    .map(|block| {
                        block
                            .instructions
                            .iter()
                            .filter(|instruction| {
                                matches!(instruction, MirInstruction::MemOp { .. })
                            })
                            .count()
                    })
                    .unwrap_or(0),
            });
        function
    }

    fn memop(
        kind: MemOpKind,
        dst: Option<ValueId>,
        operands: Vec<ValueId>,
        access: Option<MemOpAccess>,
    ) -> MirInstruction {
        MirInstruction::MemOp {
            region: FastMemRegionId::new(0),
            kind,
            dst,
            operands,
            access,
            effects: kind.effect_mask(),
        }
    }

    fn table_length_fact() -> FastMemTableLengthFact {
        FastMemTableLengthFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            table_id: "page_table".to_string(),
            table_value: ValueId::new(1),
            length_value: ValueId::new(50),
            resolved_length: Some(64),
            policy: FastMemTableLengthPolicyKind::ExplicitConstLen,
        }
    }

    fn range_index_fact(fact_id: u32, index_value: ValueId) -> RangeIndexFact {
        RangeIndexFact {
            fact_id,
            origin_kind: RangeIndexFactOriginKind::CountingLoop,
            index_value,
            lower_value: ValueId::new(40),
            upper_exclusive_value: ValueId::new(50),
            body_bb: BasicBlockId::new(0),
            step: 1,
            end_exclusive: true,
            index_body_read_only: true,
            loop_carried_writes_supported: false,
        }
    }

    #[test]
    fn refresh_verifies_page_meta_field_sites_and_rejects_unbounded_table() {
        let mut function = make_function(vec![
            memop(
                MemOpKind::TableIndex,
                Some(ValueId::new(10)),
                vec![ValueId::new(1), ValueId::new(2)],
                Some(MemOpAccess::table("page_table")),
            ),
            memop(
                MemOpKind::FieldLoad,
                Some(ValueId::new(11)),
                vec![ValueId::new(10)],
                Some(MemOpAccess::field("owner_id")),
            ),
            memop(
                MemOpKind::FieldStore,
                None,
                vec![ValueId::new(10), ValueId::new(3)],
                Some(MemOpAccess::field("local_free_head")),
            ),
        ]);

        refresh_function_fastmem_access_plans(&mut function);

        assert_eq!(function.metadata.fastmem_access_plans.len(), 3);
        assert_eq!(
            function.metadata.fastmem_access_plans[0].status,
            FastMemAccessPlanStatus::Rejected
        );
        assert_eq!(
            function.metadata.fastmem_access_plans[0]
                .failure_reason
                .as_deref(),
            Some("table-length-unresolved")
        );
        assert_eq!(
            function.metadata.fastmem_access_plans[1].status,
            FastMemAccessPlanStatus::Verified
        );
        assert_eq!(
            function.metadata.fastmem_access_plans[2].status,
            FastMemAccessPlanStatus::Verified
        );
        let FastMemAccessPlanPayload::Field(field) =
            &function.metadata.fastmem_access_plans[1].payload
        else {
            panic!("expected owner field plan");
        };
        assert_eq!(field.layout_id.as_deref(), Some("PageMetaLayoutV0"));
        assert_eq!(field.field_id, "owner_worker_id");
        assert_eq!(field.byte_offset, Some(0));
        assert_eq!(field.field_class.as_deref(), Some("plain_scalar"));
        let FastMemAccessPlanPayload::Table(table) =
            &function.metadata.fastmem_access_plans[0].payload
        else {
            panic!("expected table plan");
        };
        assert_eq!(table.element_layout_id.as_deref(), Some("PageMetaLayoutV0"));
        assert_eq!(table.element_repr.as_deref(), Some("pointer_to_element"));
        assert!(!table.proof.is_lowerable());
        assert!(!table.proof.table_length_resolved);
        assert!(!table.proof.bounds_proof_valid);
        assert!(table.proof.stride_resolved);
        assert!(table.proof.field_offset_resolved);
        assert!(!table.proof.overflow_proof_valid);
        assert!(table.proof.alignment_valid);
        assert!(table.proof.element_layout_verified);
        assert_eq!(
            table.proof.failure_reason.as_deref(),
            Some("table-length-unresolved")
        );
        assert_eq!(function.metadata.fastmem_table_field_access_links.len(), 2);
        let owner_link = &function.metadata.fastmem_table_field_access_links[0];
        assert_eq!(owner_link.table_instruction_index, 0);
        assert_eq!(owner_link.field_instruction_index, 1);
        assert_eq!(owner_link.table_result, ValueId::new(10));
        assert_eq!(owner_link.field_base, ValueId::new(10));
        assert_eq!(owner_link.field_id, "owner_worker_id");
        assert_eq!(owner_link.byte_offset, 0);
        assert_eq!(owner_link.field_size, 8);
        assert_eq!(owner_link.proof, "table_field_link:0:1");
    }

    #[test]
    fn refresh_consumes_explicit_table_length_fact_without_making_table_lowerable() {
        let mut function = make_function(vec![memop(
            MemOpKind::TableIndex,
            Some(ValueId::new(10)),
            vec![ValueId::new(1), ValueId::new(2)],
            Some(MemOpAccess::table("page_table")),
        )]);
        function
            .metadata
            .fastmem_table_length_facts
            .push(table_length_fact());

        refresh_function_fastmem_access_plans(&mut function);

        assert_eq!(function.metadata.fastmem_access_plans.len(), 1);
        let FastMemAccessPlanPayload::Table(table) =
            &function.metadata.fastmem_access_plans[0].payload
        else {
            panic!("expected table plan");
        };
        assert_eq!(table.length, Some(64));
        assert!(table.proof.table_length_resolved);
        assert_eq!(
            table.proof.table_length_policy.as_deref(),
            Some("explicit_const_len")
        );
        assert!(!table.proof.bounds_proof_valid);
        assert!(!table.proof.overflow_proof_valid);
        assert!(!table.proof.is_lowerable());
        assert_eq!(
            function.metadata.fastmem_access_plans[0]
                .failure_reason
                .as_deref(),
            Some("verified-table-access-proof-incomplete")
        );
    }

    #[test]
    fn refresh_consumes_range_index_fact_as_bounds_proof_after_length_fact() {
        let mut function = make_function(vec![memop(
            MemOpKind::TableIndex,
            Some(ValueId::new(10)),
            vec![ValueId::new(1), ValueId::new(2)],
            Some(MemOpAccess::table("page_table")),
        )]);
        function
            .metadata
            .fastmem_table_length_facts
            .push(table_length_fact());
        function
            .metadata
            .range_index_facts
            .push(range_index_fact(7, ValueId::new(2)));

        refresh_function_fastmem_access_plans(&mut function);

        let FastMemAccessPlanPayload::Table(table) =
            &function.metadata.fastmem_access_plans[0].payload
        else {
            panic!("expected table plan");
        };
        assert!(table.proof.table_length_resolved);
        assert!(table.proof.bounds_proof_valid);
        assert_eq!(table.proof.bounds_proof.as_deref(), Some("range_fact:7"));
        assert!(!table.proof.overflow_proof_valid);
        assert!(!table.proof.is_lowerable());
        assert_eq!(
            function.metadata.fastmem_access_plans[0]
                .failure_reason
                .as_deref(),
            Some("verified-table-access-proof-incomplete")
        );
    }

    #[test]
    fn refresh_sets_overflow_proof_from_length_bounds_and_table_field_link() {
        let mut function = make_function(vec![
            memop(
                MemOpKind::TableIndex,
                Some(ValueId::new(10)),
                vec![ValueId::new(1), ValueId::new(2)],
                Some(MemOpAccess::table("page_table")),
            ),
            memop(
                MemOpKind::FieldLoad,
                Some(ValueId::new(11)),
                vec![ValueId::new(10)],
                Some(MemOpAccess::field("capacity")),
            ),
        ]);
        function
            .metadata
            .fastmem_table_length_facts
            .push(table_length_fact());
        function
            .metadata
            .range_index_facts
            .push(range_index_fact(7, ValueId::new(2)));

        refresh_function_fastmem_access_plans(&mut function);

        let FastMemAccessPlanPayload::Table(table) =
            &function.metadata.fastmem_access_plans[0].payload
        else {
            panic!("expected table plan");
        };
        assert!(table.proof.table_length_resolved);
        assert!(table.proof.bounds_proof_valid);
        assert!(table.proof.field_offset_resolved);
        assert!(table.proof.overflow_proof_valid);
        assert!(table.proof.is_lowerable());
        assert!(table
            .proof
            .overflow_proof
            .as_deref()
            .unwrap_or_default()
            .contains("usize_mul_add_no_overflow+offset_within_object"));
        assert_eq!(
            function.metadata.fastmem_access_plans[0].status,
            FastMemAccessPlanStatus::Verified
        );
        assert_eq!(
            function.metadata.fastmem_access_plans[0].failure_reason,
            None
        );
        assert_eq!(function.metadata.fastmem_table_field_access_links.len(), 1);
        let link = &function.metadata.fastmem_table_field_access_links[0];
        assert_eq!(link.field_id, "capacity");
        assert!(link.byte_offset > 0);
        assert_eq!(link.field_size, 8);
        assert_eq!(link.field_access, FastMemFieldAccessMode::Load);
        assert_eq!(link.proof, "table_field_link:0:1");
    }

    #[test]
    fn refresh_keeps_overflow_proof_closed_without_bounds_proof() {
        let mut function = make_function(vec![
            memop(
                MemOpKind::TableIndex,
                Some(ValueId::new(10)),
                vec![ValueId::new(1), ValueId::new(2)],
                Some(MemOpAccess::table("page_table")),
            ),
            memop(
                MemOpKind::FieldLoad,
                Some(ValueId::new(11)),
                vec![ValueId::new(10)],
                Some(MemOpAccess::field("capacity")),
            ),
        ]);
        function
            .metadata
            .fastmem_table_length_facts
            .push(table_length_fact());

        refresh_function_fastmem_access_plans(&mut function);

        let FastMemAccessPlanPayload::Table(table) =
            &function.metadata.fastmem_access_plans[0].payload
        else {
            panic!("expected table plan");
        };
        assert!(table.proof.table_length_resolved);
        assert!(!table.proof.bounds_proof_valid);
        assert!(table.proof.field_offset_resolved);
        assert!(!table.proof.overflow_proof_valid);
        assert!(!table.proof.is_lowerable());
        assert_eq!(
            function.metadata.fastmem_access_plans[0]
                .failure_reason
                .as_deref(),
            Some("verified-table-access-proof-incomplete")
        );
    }

    #[test]
    fn refresh_does_not_link_field_access_before_table_index() {
        let mut function = make_function(vec![
            memop(
                MemOpKind::FieldLoad,
                Some(ValueId::new(11)),
                vec![ValueId::new(10)],
                Some(MemOpAccess::field("capacity")),
            ),
            memop(
                MemOpKind::TableIndex,
                Some(ValueId::new(10)),
                vec![ValueId::new(1), ValueId::new(2)],
                Some(MemOpAccess::table("page_table")),
            ),
        ]);

        refresh_function_fastmem_access_plans(&mut function);

        let FastMemAccessPlanPayload::Table(table) =
            &function.metadata.fastmem_access_plans[1].payload
        else {
            panic!("expected table plan");
        };
        assert!(!table.proof.field_offset_resolved);
        assert!(function
            .metadata
            .fastmem_table_field_access_links
            .is_empty());
    }

    #[test]
    fn refresh_does_not_consume_range_index_fact_without_matching_length_fact() {
        let mut function = make_function(vec![memop(
            MemOpKind::TableIndex,
            Some(ValueId::new(10)),
            vec![ValueId::new(1), ValueId::new(2)],
            Some(MemOpAccess::table("page_table")),
        )]);
        function
            .metadata
            .range_index_facts
            .push(range_index_fact(7, ValueId::new(2)));

        refresh_function_fastmem_access_plans(&mut function);

        let FastMemAccessPlanPayload::Table(table) =
            &function.metadata.fastmem_access_plans[0].payload
        else {
            panic!("expected table plan");
        };
        assert!(!table.proof.table_length_resolved);
        assert!(!table.proof.bounds_proof_valid);
        assert_eq!(table.proof.bounds_proof, None);
        assert_eq!(
            function.metadata.fastmem_access_plans[0]
                .failure_reason
                .as_deref(),
            Some("table-length-unresolved")
        );
    }

    #[test]
    fn refresh_rejects_range_index_fact_when_upper_does_not_match_length_value() {
        let mut function = make_function(vec![memop(
            MemOpKind::TableIndex,
            Some(ValueId::new(10)),
            vec![ValueId::new(1), ValueId::new(2)],
            Some(MemOpAccess::table("page_table")),
        )]);
        let mut range = range_index_fact(7, ValueId::new(2));
        range.upper_exclusive_value = ValueId::new(51);
        function
            .metadata
            .fastmem_table_length_facts
            .push(table_length_fact());
        function.metadata.range_index_facts.push(range);

        refresh_function_fastmem_access_plans(&mut function);

        let FastMemAccessPlanPayload::Table(table) =
            &function.metadata.fastmem_access_plans[0].payload
        else {
            panic!("expected table plan");
        };
        assert!(table.proof.table_length_resolved);
        assert!(!table.proof.bounds_proof_valid);
        assert_eq!(table.proof.bounds_proof, None);
        assert!(!table.proof.is_lowerable());
    }

    #[test]
    fn refresh_rejects_plain_store_to_atomic_remote_head() {
        let mut function = make_function(vec![memop(
            MemOpKind::FieldStore,
            None,
            vec![ValueId::new(10), ValueId::new(3)],
            Some(MemOpAccess::field("remote_head")),
        )]);

        refresh_function_fastmem_access_plans(&mut function);

        assert_eq!(function.metadata.fastmem_access_plans.len(), 1);
        let plan = &function.metadata.fastmem_access_plans[0];
        assert_eq!(plan.status, FastMemAccessPlanStatus::Rejected);
        assert_eq!(
            plan.failure_reason.as_deref(),
            Some("atomic-field-plain-store:remote_head")
        );
    }

    #[test]
    fn refresh_adds_nonlowerable_local_free_list_plans() {
        let mut function = make_function(vec![
            memop(
                MemOpKind::LocalFreePush,
                None,
                vec![ValueId::new(10), ValueId::new(11)],
                None,
            ),
            memop(
                MemOpKind::LocalFreePop,
                Some(ValueId::new(12)),
                vec![ValueId::new(10)],
                None,
            ),
        ]);

        refresh_function_fastmem_access_plans(&mut function);

        assert_eq!(function.metadata.fastmem_access_plans.len(), 2);
        for plan in &function.metadata.fastmem_access_plans {
            assert_eq!(plan.status, FastMemAccessPlanStatus::Rejected);
            assert_eq!(
                plan.failure_reason.as_deref(),
                Some("local-free-same-owner-proof-missing")
            );
            let FastMemAccessPlanPayload::LocalFree(local_free) = &plan.payload else {
                panic!("expected local free-list plan");
            };
            assert_eq!(
                local_free.local_free_head_field_id.as_deref(),
                Some("local_free_head")
            );
            assert_eq!(
                local_free.local_free_head_field_class.as_deref(),
                Some("local_free_head")
            );
            assert_eq!(local_free.local_free_head_byte_offset, Some(24));
            assert_eq!(local_free.local_free_head_field_size, Some(8));
            assert_eq!(
                local_free.local_free_head_field_type.as_deref(),
                Some("usize")
            );
            assert_eq!(local_free.local_free_head_alignment, Some(8));
            assert!(!local_free.same_owner_proof_valid);
            assert!(!local_free.block_next_proof_valid);
            assert!(!local_free.non_empty_proof_valid);
            assert!(!local_free.remote_owner_rejected);
            assert!(!local_free.lowerable);
        }
    }

    #[test]
    fn refresh_verifies_local_free_push_when_precondition_facts_exist() {
        let mut function = make_function(vec![
            memop(
                MemOpKind::LocalFreePush,
                None,
                vec![ValueId::new(10), ValueId::new(11)],
                None,
            ),
            memop(
                MemOpKind::LocalFreePop,
                Some(ValueId::new(12)),
                vec![ValueId::new(10)],
                None,
            ),
        ]);
        function
            .metadata
            .fastmem_same_owner_facts
            .push(FastMemSameOwnerFact {
                fact_id: 0,
                region: FastMemRegionId::new(0),
                page_value: ValueId::new(10),
                proof_value: ValueId::new(20),
                proof_kind: FastMemSameOwnerProofKind::SourceAssumeOwnerEq,
                remote_owner_rejected: true,
            });
        function
            .metadata
            .fastmem_block_next_facts
            .push(FastMemBlockNextFact {
                fact_id: 0,
                region: FastMemRegionId::new(0),
                block_value: ValueId::new(11),
                next_field_id: "next".to_string(),
                proof_kind: FastMemBlockNextProofKind::SourceAssumeLocalFreeBlockNext,
                writable: true,
                provenance_valid: true,
            });

        refresh_function_fastmem_access_plans(&mut function);

        assert_eq!(function.metadata.fastmem_access_plans.len(), 2);
        let push_plan = &function.metadata.fastmem_access_plans[0];
        assert_eq!(push_plan.kind, FastMemAccessPlanKind::LocalFreePush);
        assert_eq!(push_plan.status, FastMemAccessPlanStatus::Verified);
        assert_eq!(push_plan.failure_reason, None);
        let FastMemAccessPlanPayload::LocalFree(push) = &push_plan.payload else {
            panic!("expected local free-list push plan");
        };
        assert!(push.same_owner_proof_valid);
        assert!(push.block_next_proof_valid);
        assert!(push.remote_owner_rejected);
        assert!(push.lowerable);
        assert_eq!(
            push.local_free_head_layout_id.as_deref(),
            Some("PageMetaLayoutV0")
        );
        assert_eq!(
            push.local_free_head_field_id.as_deref(),
            Some("local_free_head")
        );
        assert_eq!(push.local_free_head_byte_offset, Some(24));
        assert_eq!(push.local_free_head_field_size, Some(8));
        assert_eq!(push.local_free_head_field_type.as_deref(), Some("usize"));
        assert_eq!(push.local_free_head_alignment, Some(8));
        assert!(!push.non_empty_proof_valid);
        assert_eq!(
            push.block_next_layout_id.as_deref(),
            Some("FreeBlockNodeLayoutV0")
        );
        assert_eq!(push.block_next_field_id.as_deref(), Some("next"));
        assert_eq!(
            push.block_next_field_class.as_deref(),
            Some("local_free_block_next")
        );
        assert_eq!(push.block_next_byte_offset, Some(0));
        assert_eq!(push.block_next_field_size, Some(8));
        assert_eq!(push.block_next_field_type.as_deref(), Some("usize"));
        assert_eq!(push.block_next_alignment, Some(8));

        let pop_plan = &function.metadata.fastmem_access_plans[1];
        assert_eq!(pop_plan.kind, FastMemAccessPlanKind::LocalFreePop);
        assert_eq!(pop_plan.status, FastMemAccessPlanStatus::Rejected);
        assert_eq!(
            pop_plan.failure_reason.as_deref(),
            Some("local-free-non-empty-proof-missing")
        );
        let FastMemAccessPlanPayload::LocalFree(pop) = &pop_plan.payload else {
            panic!("expected local free-list pop plan");
        };
        assert!(pop.same_owner_proof_valid);
        assert!(!pop.block_next_proof_valid);
        assert!(!pop.non_empty_proof_valid);
        assert!(pop.remote_owner_rejected);
        assert!(!pop.lowerable);
    }

    #[test]
    fn refresh_verifies_local_free_pop_preconditions_without_lowering() {
        let mut function = make_function(vec![memop(
            MemOpKind::LocalFreePop,
            Some(ValueId::new(12)),
            vec![ValueId::new(10)],
            None,
        )]);
        function
            .metadata
            .fastmem_same_owner_facts
            .push(FastMemSameOwnerFact {
                fact_id: 0,
                region: FastMemRegionId::new(0),
                page_value: ValueId::new(10),
                proof_value: ValueId::new(20),
                proof_kind: FastMemSameOwnerProofKind::SourceAssumeOwnerEq,
                remote_owner_rejected: true,
            });
        function
            .metadata
            .fastmem_local_free_non_empty_facts
            .push(FastMemLocalFreeNonEmptyFact {
                fact_id: 0,
                region: FastMemRegionId::new(0),
                page_value: ValueId::new(10),
                proof_kind: FastMemLocalFreeNonEmptyProofKind::SourceAssumeLocalFreeNonEmpty,
                non_empty: true,
            });

        refresh_function_fastmem_access_plans(&mut function);

        assert_eq!(function.metadata.fastmem_access_plans.len(), 1);
        let pop_plan = &function.metadata.fastmem_access_plans[0];
        assert_eq!(pop_plan.kind, FastMemAccessPlanKind::LocalFreePop);
        assert_eq!(pop_plan.status, FastMemAccessPlanStatus::Verified);
        assert_eq!(pop_plan.failure_reason, None);
        let FastMemAccessPlanPayload::LocalFree(pop) = &pop_plan.payload else {
            panic!("expected local free-list pop plan");
        };
        assert!(pop.same_owner_proof_valid);
        assert!(pop.non_empty_proof_valid);
        assert!(!pop.block_next_proof_valid);
        assert!(pop.remote_owner_rejected);
        assert!(pop.lowerable);
        assert_eq!(
            pop.local_free_head_layout_id.as_deref(),
            Some("PageMetaLayoutV0")
        );
        assert_eq!(
            pop.local_free_head_field_id.as_deref(),
            Some("local_free_head")
        );
        assert_eq!(pop.local_free_head_byte_offset, Some(24));
        assert_eq!(pop.local_free_head_field_size, Some(8));
        assert_eq!(pop.local_free_head_field_type.as_deref(), Some("usize"));
        assert_eq!(pop.local_free_head_alignment, Some(8));
        assert_eq!(
            pop.block_next_layout_id.as_deref(),
            Some("FreeBlockNodeLayoutV0")
        );
        assert_eq!(pop.block_next_field_id.as_deref(), Some("next"));
        assert_eq!(
            pop.block_next_field_class.as_deref(),
            Some("local_free_block_next")
        );
        assert_eq!(pop.block_next_byte_offset, Some(0));
        assert_eq!(pop.block_next_field_size, Some(8));
        assert_eq!(pop.block_next_field_type.as_deref(), Some("usize"));
        assert_eq!(pop.block_next_alignment, Some(8));
    }

    #[test]
    fn refresh_ignores_layout_table_memops_without_symbolic_ids() {
        let mut function = make_function(vec![memop(
            MemOpKind::FieldLoad,
            Some(ValueId::new(1)),
            vec![ValueId::new(0)],
            None,
        )]);

        refresh_function_fastmem_access_plans(&mut function);

        assert!(function.metadata.fastmem_access_plans.is_empty());
    }
}
