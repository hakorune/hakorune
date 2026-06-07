use crate::mir::function::{FastMemFreeHeadNonEmptyFact, FastMemFreeHeadNonEmptyProofKind};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::{BasicBlockId, ValueId};

use super::fact_store::FastMemFactStore;
use super::linked_list::{
    resolve_linked_list_plan_core, FastMemLinkedListFamily, ResolvedLinkedListPlanCore,
};
use super::types::{
    FastMemAccessPlan, FastMemAccessPlanKind, FastMemAccessPlanPayload, FastMemFreeHeadListPlan,
    FastMemLocalFreeListPlan,
};

pub(super) fn local_free_plan(
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

pub(super) fn free_head_plan(
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

pub(super) fn maybe_add_derived_free_head_non_empty_fact(
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
