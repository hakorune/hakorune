use crate::mir::instruction::FastMemRegionId;
use crate::mir::ValueId;

use super::fact_store::FastMemFactStore;
use super::head_access::{
    resolve_block_next_access, resolve_head_access, ResolvedBlockNextAccess, ResolvedHeadAccess,
};
use super::types::{FastMemAccessPlanKind, FastMemAccessPlanStatus, FastMemFieldAccessMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FastMemLinkedListFamily {
    LocalFree,
    FreeHead,
}

impl FastMemLinkedListFamily {
    fn head_field_id(self) -> &'static str {
        match self {
            Self::LocalFree => "local_free_head",
            Self::FreeHead => "free_head",
        }
    }

    fn push_kind(self) -> FastMemAccessPlanKind {
        match self {
            Self::LocalFree => FastMemAccessPlanKind::LocalFreePush,
            Self::FreeHead => FastMemAccessPlanKind::FreeHeadPush,
        }
    }

    fn pop_kind(self) -> FastMemAccessPlanKind {
        match self {
            Self::LocalFree => FastMemAccessPlanKind::LocalFreePop,
            Self::FreeHead => FastMemAccessPlanKind::FreeHeadPop,
        }
    }

    fn same_owner_failure(self) -> String {
        match self {
            Self::LocalFree => "local-free-same-owner-proof-missing",
            Self::FreeHead => "free-head-same-owner-proof-missing",
        }
        .to_string()
    }

    fn block_next_failure(self) -> String {
        match self {
            Self::LocalFree => "local-free-block-next-proof-missing",
            Self::FreeHead => "free-head-block-next-proof-missing",
        }
        .to_string()
    }

    fn non_empty_failure(self) -> String {
        match self {
            Self::LocalFree => "local-free-non-empty-proof-missing",
            Self::FreeHead => "free-head-non-empty-proof-missing",
        }
        .to_string()
    }

    fn block_next_access_failure(self) -> String {
        match self {
            Self::LocalFree => "local-free-block-next-access-unresolved",
            Self::FreeHead => "free-head-block-next-access-unresolved",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ResolvedLinkedListPlanCore {
    pub(super) page: ValueId,
    pub(super) block_value: Option<ValueId>,
    pub(super) head_access: ResolvedHeadAccess,
    pub(super) block_next_access: ResolvedBlockNextAccess,
    pub(super) same_owner_proof_valid: bool,
    pub(super) block_next_proof_valid: bool,
    pub(super) non_empty_proof_valid: bool,
    pub(super) remote_owner_rejected: bool,
    pub(super) lowerable: bool,
    pub(super) status: FastMemAccessPlanStatus,
    pub(super) failure_reason: Option<String>,
}

pub(super) fn resolve_linked_list_plan_core(
    region: FastMemRegionId,
    kind: FastMemAccessPlanKind,
    operands: &[ValueId],
    contract: Option<&str>,
    facts: &FastMemFactStore<'_>,
    family: FastMemLinkedListFamily,
) -> Option<ResolvedLinkedListPlanCore> {
    let page = operands.first().copied()?;
    let block_value = if kind == family.push_kind() {
        operands.get(1).copied()
    } else {
        None
    };
    let head_access = resolve_head_access(
        contract,
        family.head_field_id(),
        FastMemFieldAccessMode::Load,
    );
    let same_owner_proof_valid = facts
        .same_owner(region, page)
        .map_or(false, |fact| fact.remote_owner_rejected);
    let remote_owner_rejected = same_owner_proof_valid;
    let non_empty_proof_valid = match family {
        FastMemLinkedListFamily::LocalFree => facts
            .local_free_non_empty(region, page)
            .map_or(false, |fact| fact.non_empty),
        FastMemLinkedListFamily::FreeHead => facts
            .free_head_non_empty(region, page)
            .map_or(false, |fact| fact.non_empty),
    };
    let block_next_field_id = "next";
    let block_next_fact = block_value.and_then(|block_value| {
        facts.block_next(region, block_value).filter(|fact| {
            fact.next_field_id == block_next_field_id && fact.writable && fact.provenance_valid
        })
    });
    let block_next_access = if let Some(fact) = block_next_fact {
        resolve_block_next_access(contract, &fact.next_field_id)
    } else if kind == family.pop_kind() && non_empty_proof_valid {
        resolve_block_next_access(contract, block_next_field_id)
    } else {
        ResolvedBlockNextAccess::default()
    };
    let block_next_access_resolved = block_next_access.is_resolved();
    let block_next_proof_valid = block_next_fact.is_some() && block_next_access_resolved;
    let common_lowerable = head_access.is_resolved() && same_owner_proof_valid;
    let lowerable_push = kind == family.push_kind() && common_lowerable && block_next_proof_valid;
    let lowerable_pop = kind == family.pop_kind()
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
            Some(family.same_owner_failure())
        } else if kind == family.push_kind() && !block_next_proof_valid {
            Some(family.block_next_failure())
        } else if kind == family.pop_kind() && !non_empty_proof_valid {
            Some(family.non_empty_failure())
        } else if kind == family.pop_kind() && !block_next_access_resolved {
            Some(family.block_next_access_failure())
        } else {
            None
        }
    });

    Some(ResolvedLinkedListPlanCore {
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
    })
}
