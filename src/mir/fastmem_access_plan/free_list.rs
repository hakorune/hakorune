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
    linked_list_plan(
        block,
        instruction_index,
        region,
        kind,
        operands,
        contract,
        facts,
        FastMemLinkedListFamily::LocalFree,
        dst,
        local_free_payload,
    )
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
    linked_list_plan(
        block,
        instruction_index,
        region,
        kind,
        operands,
        contract,
        facts,
        FastMemLinkedListFamily::FreeHead,
        dst,
        free_head_payload,
    )
}

fn linked_list_plan(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    kind: FastMemAccessPlanKind,
    operands: &[ValueId],
    contract: Option<&str>,
    facts: &FastMemFactStore<'_>,
    family: FastMemLinkedListFamily,
    dst: Option<ValueId>,
    payload: fn(ResolvedLinkedListPlanCore, Option<ValueId>) -> FastMemAccessPlanPayload,
) -> Option<FastMemAccessPlan> {
    let core = resolve_linked_list_plan_core(region, kind, operands, contract, facts, family)?;
    let status = core.status;
    let failure_reason = core.failure_reason.clone();

    Some(FastMemAccessPlan {
        block,
        instruction_index,
        region,
        kind,
        status,
        failure_reason,
        payload: payload(core, dst),
    })
}

fn local_free_payload(
    core: ResolvedLinkedListPlanCore,
    dst: Option<ValueId>,
) -> FastMemAccessPlanPayload {
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
        ..
    } = core;

    FastMemAccessPlanPayload::LocalFree(FastMemLocalFreeListPlan {
        page,
        block: block_value,
        result: dst,
        local_free_head: head_access.into_field_plan(),
        block_next: block_next_access.into_field_plan(),
        same_owner_proof_valid,
        block_next_proof_valid,
        non_empty_proof_valid,
        remote_owner_rejected,
        lowerable,
    })
}

fn free_head_payload(
    core: ResolvedLinkedListPlanCore,
    dst: Option<ValueId>,
) -> FastMemAccessPlanPayload {
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
        ..
    } = core;

    FastMemAccessPlanPayload::FreeHead(FastMemFreeHeadListPlan {
        page,
        block: block_value,
        result: dst,
        free_head: head_access.into_field_plan(),
        block_next: block_next_access.into_field_plan(),
        same_owner_proof_valid,
        block_next_proof_valid,
        non_empty_proof_valid,
        remote_owner_rejected,
        lowerable,
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
