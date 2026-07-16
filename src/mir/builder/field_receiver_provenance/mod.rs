//! Bounded use-site proof for current-receiver Copy/Phi provenance.
//!
//! This module is disconnected in R0-DECLFIELD-PHI0-S0. It must not publish
//! origin/type metadata or become a general receiver-equivalence registry.

#[cfg(test)]
use super::BasicBlockId;
use super::{MirBuilder, ValueId};
use hakorune_mir_core::MirValueKind;

mod analysis;
mod cfg;
mod definitions;
#[cfg(test)]
mod tests;

#[derive(Debug)]
struct CurrentReceiverIdentitySealV1;

#[derive(Debug)]
struct SameRootReceiverValueSealV1;

#[derive(Debug)]
pub(crate) struct VerifiedCurrentReceiverIdentityV1 {
    receiver_parameter: ValueId,
    owner_box: String,
    _seal: CurrentReceiverIdentitySealV1,
}

impl VerifiedCurrentReceiverIdentityV1 {
    fn verify(builder: &MirBuilder) -> Result<Self, SameRootReceiverProofErrorV1> {
        let function = builder
            .scope_ctx
            .current_function
            .as_ref()
            .ok_or(SameRootReceiverProofErrorV1::NoCurrentFunction)?;
        let declaration = function
            .metadata
            .declared_param_decls
            .first()
            .ok_or(SameRootReceiverProofErrorV1::MissingImplicitReceiverMetadata)?;
        if declaration.name != "me" || !declaration.implicit_receiver {
            return Err(SameRootReceiverProofErrorV1::NotInstanceMethod);
        }
        let receiver_parameter = *function
            .params
            .first()
            .ok_or(SameRootReceiverProofErrorV1::MissingReceiverParameter)?;
        if builder.get_value_kind(receiver_parameter) != Some(MirValueKind::Parameter(0)) {
            return Err(SameRootReceiverProofErrorV1::ReceiverKindMismatch);
        }
        let signature_owner = match function.signature.params.first() {
            Some(crate::mir::MirType::Box(owner)) => owner,
            _ => return Err(SameRootReceiverProofErrorV1::ReceiverOwnerMismatch),
        };
        if builder.type_ctx.value_types.get(&receiver_parameter)
            != Some(&crate::mir::MirType::Box(signature_owner.clone()))
        {
            return Err(SameRootReceiverProofErrorV1::ReceiverOwnerMismatch);
        }
        if builder
            .type_ctx
            .value_origin_newbox
            .get(&receiver_parameter)
            != Some(signature_owner)
        {
            return Err(SameRootReceiverProofErrorV1::ReceiverOwnerMismatch);
        }
        if !builder
            .comp_ctx
            .user_box_field_decls
            .contains_key(signature_owner)
        {
            return Err(SameRootReceiverProofErrorV1::ReceiverRegistryMissing);
        }
        Ok(Self {
            receiver_parameter,
            owner_box: signature_owner.clone(),
            _seal: CurrentReceiverIdentitySealV1,
        })
    }

    pub(crate) fn receiver_parameter(&self) -> ValueId {
        self.receiver_parameter
    }

    pub(crate) fn owner_box(&self) -> &str {
        &self.owner_box
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedSameRootReceiverValueV1 {
    value: ValueId,
    receiver: VerifiedCurrentReceiverIdentityV1,
    _seal: SameRootReceiverValueSealV1,
}

impl VerifiedSameRootReceiverValueV1 {
    pub(crate) fn verify(
        builder: &MirBuilder,
        value: ValueId,
    ) -> Result<Self, SameRootReceiverProofErrorV1> {
        analysis::verify(builder, value, false).map(|result| result.proof)
    }

    pub(crate) fn value(&self) -> ValueId {
        self.value
    }

    pub(crate) fn receiver(&self) -> &VerifiedCurrentReceiverIdentityV1 {
        &self.receiver
    }
}

#[cfg(test)]
fn verify_with_normalized_test_view(
    builder: &MirBuilder,
    value: ValueId,
) -> Result<(VerifiedSameRootReceiverValueV1, String), SameRootReceiverProofErrorV1> {
    let result = analysis::verify(builder, value, true)?;
    Ok((
        result.proof,
        result
            .normalized
            .ok_or(SameRootReceiverProofErrorV1::TraversalBudgetExceeded)?,
    ))
}

#[cfg(test)]
fn verify_with_normalized_test_view_at(
    builder: &MirBuilder,
    value: ValueId,
    block: BasicBlockId,
    instruction_index: usize,
) -> Result<(VerifiedSameRootReceiverValueV1, String), SameRootReceiverProofErrorV1> {
    let result = analysis::verify_at(builder, value, true, block, instruction_index)?;
    Ok((
        result.proof,
        result
            .normalized
            .ok_or(SameRootReceiverProofErrorV1::TraversalBudgetExceeded)?,
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SameRootReceiverProofErrorV1 {
    NoCurrentFunction,
    NotInstanceMethod,
    MissingImplicitReceiverMetadata,
    MissingReceiverParameter,
    ReceiverKindMismatch,
    ReceiverOwnerMismatch,
    ReceiverRegistryMissing,
    MissingUseSite,
    SeedUnavailable,
    SeedTypeMissing,
    SeedTypeMismatch,
    ForeignOrigin,
    CfgSuccessorCacheMismatch,
    MissingCfgBlock,
    MissingDefinition,
    MultipleDefinition,
    UnsupportedDefinitionKind,
    ForeignParameter,
    CopySourceUnavailable,
    PhiUnreachable,
    PhiTooFewInputs,
    DuplicatePhiPredecessor,
    PhantomPhiPredecessor,
    MissingPhiPredecessor,
    UnreachablePhiPredecessor,
    PhiIncomingUnavailable,
    ValueDefinitionCycle,
    CfgCycleOrBackedge,
    TraversalBudgetExceeded,
}
