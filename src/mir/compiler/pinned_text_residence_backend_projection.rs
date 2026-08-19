use std::collections::BTreeSet;

use crate::mir::{MirFunction, MirInstruction};

use super::pinned_text_residence_backend_carrier::PinnedTextResidenceBackendCarrierV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PinnedTextResidenceBackendCarrierProjectionErrorV1 {
    FrameContractMismatch,
    EnterSiteMismatch,
    TrapSiteMismatch,
    FinishSiteMismatch,
    FinishReturnOrderingMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PinnedTextResidenceBackendCarrierInstallV1 {
    Invalid(PinnedTextResidenceBackendCarrierProjectionErrorV1),
    AlreadyInstalled,
}

/// Validate one source-issued transport carrier against the exact detached
/// function receiving it. MIR is observed only here, at the projection seam;
/// it never becomes a carrier issuance authority.
pub(crate) fn verify_pinned_text_residence_backend_carrier_v1(
    carrier: &PinnedTextResidenceBackendCarrierV1,
    function: &MirFunction,
) -> Result<(), PinnedTextResidenceBackendCarrierProjectionErrorV1> {
    let view = carrier.projection_view();
    if function.entry_block != view.enter_source {
        return Err(PinnedTextResidenceBackendCarrierProjectionErrorV1::EnterSiteMismatch);
    }
    let frame = function
        .metadata
        .pinned_text_backend_frame_contract
        .as_ref()
        .ok_or(PinnedTextResidenceBackendCarrierProjectionErrorV1::FrameContractMismatch)?
        .borrow();
    if frame.owner() != view.owner
        || frame.invocation_ordinal() != view.invocation_ordinal
        || frame.plan_stamp() != view.plan.plan_stamp()
        || frame.target_profile_id() != view.target_profile_id
        || frame.target_triple() != view.target_triple
        || frame.target_data_layout() != view.target_data_layout
        || frame.residence_abi_revision() != view.residence_abi_revision
    {
        return Err(PinnedTextResidenceBackendCarrierProjectionErrorV1::FrameContractMismatch);
    }

    match function
        .get_block(view.enter_source)
        .and_then(|block| block.terminator.as_ref())
    {
        Some(MirInstruction::PinnedTextResidenceEnter {
            plan,
            normal_landing,
            trap_landing,
        }) if *plan == view.plan
            && *normal_landing == view.normal_landing
            && *trap_landing == view.trap_landing => {}
        _ => return Err(PinnedTextResidenceBackendCarrierProjectionErrorV1::EnterSiteMismatch),
    }

    let trap = function
        .get_block(view.trap_landing)
        .ok_or(PinnedTextResidenceBackendCarrierProjectionErrorV1::TrapSiteMismatch)?;
    if !trap.instructions.is_empty()
        || !matches!(
            trap.terminator.as_ref(),
            Some(MirInstruction::PinnedTextResidenceTrap { plan }) if *plan == view.plan
        )
    {
        return Err(PinnedTextResidenceBackendCarrierProjectionErrorV1::TrapSiteMismatch);
    }

    let expected = view.finish_blocks.iter().copied().collect::<BTreeSet<_>>();
    let mut actual = BTreeSet::new();
    let mut enter_count = 0usize;
    let mut trap_count = 0usize;
    let mut explicit_value_returns = BTreeSet::new();
    for (block_id, block) in &function.blocks {
        match block.terminator.as_ref() {
            Some(MirInstruction::PinnedTextResidenceEnter { .. }) => {
                enter_count = enter_count.saturating_add(1);
            }
            Some(MirInstruction::PinnedTextResidenceTrap { .. }) => {
                trap_count = trap_count.saturating_add(1);
            }
            Some(MirInstruction::Return { value: Some(_) }) => {
                explicit_value_returns.insert(*block_id);
            }
            _ => {}
        }
        let mut finish_count = 0usize;
        for instruction in &block.instructions {
            let MirInstruction::PinnedTextResidenceFinish { residence } = instruction else {
                continue;
            };
            if residence.owner() != view.owner
                || residence.plan_stamp() != view.plan.plan_stamp()
            {
                return Err(PinnedTextResidenceBackendCarrierProjectionErrorV1::FinishSiteMismatch);
            }
            finish_count = finish_count.saturating_add(1);
        }
        if finish_count == 0 {
            continue;
        }
        if finish_count != 1 || !expected.contains(block_id) || !actual.insert(*block_id) {
            return Err(PinnedTextResidenceBackendCarrierProjectionErrorV1::FinishSiteMismatch);
        }
        if !matches!(
            block.instructions.last(),
            Some(MirInstruction::PinnedTextResidenceFinish { .. })
        ) || !matches!(
            block.terminator.as_ref(),
            Some(MirInstruction::Return { value: Some(_) })
        ) {
            return Err(
                PinnedTextResidenceBackendCarrierProjectionErrorV1::FinishReturnOrderingMismatch,
            );
        }
    }
    if enter_count != 1 {
        return Err(PinnedTextResidenceBackendCarrierProjectionErrorV1::EnterSiteMismatch);
    }
    if trap_count != 1 {
        return Err(PinnedTextResidenceBackendCarrierProjectionErrorV1::TrapSiteMismatch);
    }
    if actual != expected
        || explicit_value_returns != expected
        || actual.len() != view.normal_exit_count as usize
    {
        return Err(PinnedTextResidenceBackendCarrierProjectionErrorV1::FinishSiteMismatch);
    }
    Ok(())
}

impl PinnedTextResidenceBackendCarrierV1 {
    pub(crate) fn verify_projected_function(
        &self,
        function: &MirFunction,
    ) -> Result<(), PinnedTextResidenceBackendCarrierProjectionErrorV1> {
        verify_pinned_text_residence_backend_carrier_v1(self, function)
    }
}

pub(crate) fn install_pinned_text_residence_backend_carrier_v1(
    carrier: PinnedTextResidenceBackendCarrierV1,
    function: &mut MirFunction,
) -> Result<(), PinnedTextResidenceBackendCarrierInstallV1> {
    verify_pinned_text_residence_backend_carrier_v1(&carrier, function)
        .map_err(PinnedTextResidenceBackendCarrierInstallV1::Invalid)?;
    if function
        .metadata
        .pinned_text_residence_backend_carrier
        .is_some()
    {
        return Err(PinnedTextResidenceBackendCarrierInstallV1::AlreadyInstalled);
    }
    function.metadata.pinned_text_residence_backend_carrier = Some(carrier);
    Ok(())
}
