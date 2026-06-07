/*!
 * MIR-owned FastMemory layout/table access plans.
 *
 * `MemOpAccess` carries symbolic source ids. This module publishes the next
 * metadata seam: a function-local access-plan row for each layout/table MemOp
 * site. Verified rows are produced only by the memory-profile contract
 * resolver. LLVM GEP/load/store lowering remains closed until it consumes
 * verified rows without recomputing layout/table facts.
 */

mod fact_store;
mod field;
mod linked_list;
mod table;
mod types;

use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};
use crate::mir::{
    fastmem_layout_contract::{
        resolve_fastmem_block_next_contract, resolve_fastmem_field_contract,
    },
    function::{
        FastMemBlockNextProofKind, FastMemFreeHeadNonEmptyFact, FastMemFreeHeadNonEmptyProofKind,
    },
};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use fact_store::{collect_remote_drain_token_facts, region_contract, FastMemFactStore};
use field::field_plan;
use linked_list::{
    resolve_linked_list_plan_core, FastMemLinkedListFamily, ResolvedLinkedListPlanCore,
};
use table::{table_field_access_links, table_plan};
pub use types::{
    FastMemAccessPlan, FastMemAccessPlanKind, FastMemAccessPlanPayload, FastMemAccessPlanStatus,
    FastMemAtomicRemoteHeadPlan, FastMemDrainRemoteListToLocalPlan, FastMemFieldAccessMode,
    FastMemFieldAccessPlan, FastMemFreeHeadListPlan, FastMemLocalFreeListPlan,
    FastMemTableAccessPlan, FastMemTableAccessProof, FastMemTableFieldAccessLink,
};

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
    let ResolvedLinkedListPlanCore {
        page,
        block_value,
        head_access,
        block_next_access,
        same_owner_proof_valid,
        block_next_proof_valid,
        non_empty_proof_valid,
        remote_owner_rejected,
        lowerable,
        status,
        failure_reason,
    } = resolve_linked_list_plan_core(
        region,
        kind,
        operands,
        contract,
        facts,
        FastMemLinkedListFamily::LocalFree,
    )?;

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
    let ResolvedLinkedListPlanCore {
        page,
        block_value,
        head_access,
        block_next_access,
        same_owner_proof_valid,
        block_next_proof_valid,
        non_empty_proof_valid,
        remote_owner_rejected,
        lowerable,
        status,
        failure_reason,
    } = resolve_linked_list_plan_core(
        region,
        kind,
        operands,
        contract,
        facts,
        FastMemLinkedListFamily::FreeHead,
    )?;

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
    let contract = facts.region_contract(region);
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
