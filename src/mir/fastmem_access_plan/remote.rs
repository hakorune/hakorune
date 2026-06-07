use crate::mir::function::FastMemBlockNextProofKind;
use crate::mir::instruction::FastMemRegionId;
use crate::mir::{BasicBlockId, ValueId};

use super::fact_store::FastMemFactStore;
use super::head_access::{resolve_block_next_access, resolve_head_access, ResolvedBlockNextAccess};
use super::types::{
    FastMemAccessPlan, FastMemAccessPlanKind, FastMemAccessPlanPayload, FastMemAccessPlanStatus,
    FastMemAtomicRemoteHeadPlan, FastMemDrainRemoteListToLocalPlan, FastMemFieldAccessMode,
};

pub(super) fn atomic_remote_head_plan(
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
    let (block_next_access, block_next_proof_valid) =
        remote_block_next_access(region, block_value, contract, facts);
    let remote_owner_required = kind == FastMemAccessPlanKind::AtomicRemoteHeadPush;
    let remote_owner_proof_valid =
        remote_owner_required && has_remote_owner_proof(region, page, facts);
    let block_next_required = kind == FastMemAccessPlanKind::AtomicRemoteHeadPush;
    let memory_order_policy = atomic_remote_memory_order_policy(kind).to_string();
    let retry_attempt_limit = atomic_remote_retry_attempt_limit(kind);
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
    let failure_reason = atomic_remote_failure_reason(
        kind,
        lowerable,
        head_access.failure_reason.clone(),
        remote_owner_proof_valid,
        block_next_proof_valid,
    );

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
            remote_head: head_access.into_field_plan(),
            block_next: block_next_access.into_field_plan(),
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

fn remote_block_next_access(
    region: FastMemRegionId,
    block_value: Option<ValueId>,
    contract: Option<&str>,
    facts: &FastMemFactStore<'_>,
) -> (ResolvedBlockNextAccess, bool) {
    let block_next_field_id = "next";
    let block_next_fact = block_value.and_then(|block_value| {
        facts.block_next(region, block_value).filter(|fact| {
            fact.next_field_id == block_next_field_id
                && fact.writable
                && fact.provenance_valid
                && fact.proof_kind == FastMemBlockNextProofKind::SourceAssumeRemoteFreeBlockNext
        })
    });
    let access = if let Some(fact) = block_next_fact {
        resolve_block_next_access(contract, &fact.next_field_id)
    } else {
        ResolvedBlockNextAccess::default()
    };
    let proof_valid = block_next_fact.is_some() && access.is_resolved();
    (access, proof_valid)
}

fn has_remote_owner_proof(
    region: FastMemRegionId,
    page: ValueId,
    facts: &FastMemFactStore<'_>,
) -> bool {
    facts
        .remote_owner(region, page)
        .map_or(false, |fact| fact.same_owner_rejected)
}

fn atomic_remote_memory_order_policy(kind: FastMemAccessPlanKind) -> &'static str {
    if kind == FastMemAccessPlanKind::AtomicRemoteHeadDrain {
        "acquire_exchange"
    } else {
        "acq_rel"
    }
}

fn atomic_remote_retry_attempt_limit(kind: FastMemAccessPlanKind) -> u32 {
    if kind == FastMemAccessPlanKind::AtomicRemoteHeadPush {
        3
    } else {
        0
    }
}

fn atomic_remote_failure_reason(
    kind: FastMemAccessPlanKind,
    lowerable: bool,
    head_failure_reason: Option<String>,
    remote_owner_proof_valid: bool,
    block_next_proof_valid: bool,
) -> Option<String> {
    head_failure_reason.or_else(|| {
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
    })
}

pub(super) fn drain_remote_list_to_local_plan(
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
                local_free_head: local_free_head.into_field_plan(),
                block_next: block_next_access.into_field_plan(),
                block_next_access_resolved,
                publication_order,
                lowerable,
            },
        ),
    })
}
