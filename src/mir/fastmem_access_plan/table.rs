use crate::mir::fastmem_layout_contract::resolve_fastmem_table_contract;
use crate::mir::instruction::{FastMemRegionId, MemOpAccess};
use crate::mir::{BasicBlockId, ValueId};
use std::collections::HashMap;

use super::fact_store::FastMemFactStore;
use super::types::{
    FastMemAccessPlan, FastMemAccessPlanKind, FastMemAccessPlanPayload, FastMemAccessPlanStatus,
    FastMemTableAccessPlan, FastMemTableAccessProof, FastMemTableFieldAccessLink,
};

pub(super) fn table_plan(
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

pub(super) fn table_field_access_links(
    plans: &mut [FastMemAccessPlan],
    copy_aliases: &HashMap<ValueId, ValueId>,
) -> Vec<FastMemTableFieldAccessLink> {
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
                copy_aliases,
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
    copy_aliases: &HashMap<ValueId, ValueId>,
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
    if resolve_copy_alias(field.base, copy_aliases)
        != resolve_copy_alias(table_result, copy_aliases)
    {
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

fn resolve_copy_alias(value: ValueId, copy_aliases: &HashMap<ValueId, ValueId>) -> ValueId {
    let mut current = value;
    for _ in 0..32 {
        let Some(next) = copy_aliases.get(&current).copied() else {
            break;
        };
        if next == current {
            break;
        }
        current = next;
    }
    current
}
