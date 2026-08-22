//! Source-backed Generic G0 physical shell facts.
//!
//! The validator is shared by the prephysical emitter admission.  It does
//! not allocate `MirFunction` state; detached shell construction is retired.

use crate::mir::compiler::generic_g0_physical_function_effect::{
    GenericG0PhysicalFunctionEffectRejectV1, VerifiedGenericG0PhysicalFunctionEffectsV1,
};
use crate::mir::compiler::generic_g0_physical_function_entry_input::{
    GenericG0PhysicalFunctionEntryRejectV1, GenericG0PhysicalLaneRoleV1,
    GenericG0PhysicalParameterDescriptorV1,
};
use crate::mir::compiler::generic_g0_source_parent::{
    physical_emitter_source_parts::GenericG0PhysicalEmitterSourcePartsRejectV1,
    GenericG0SourceParentRefV1,
};
use crate::mir::resolved_semantics::ReceiverPolicyV1;
use crate::mir::{EffectMask, MirType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GenericG0PhysicalShellFactsRejectV1 {
    EntryInput(GenericG0PhysicalFunctionEntryRejectV1),
    PhysicalEffect(GenericG0PhysicalFunctionEffectRejectV1),
    SourceNameEmpty,
    SourceNameContainsPhysicalSeparator,
    SourceArityOverflow,
    OwnerMismatch,
    OriginMismatch,
    SourceKindMismatch,
    BodyRootMismatch,
    FrameMismatch,
    MetadataNotEmpty,
    ModeReceiverMismatch,
    DescriptorCoverage,
    DescriptorIndexMismatch,
    DescriptorRoleMismatch,
    DescriptorOrdinalMismatch,
    DescriptorCarrierMismatch,
    DescriptorTypeMismatch,
    ResultAbiMismatch,
    EffectMismatch,
}

pub(in crate::mir::compiler) fn validate_generic_g0_physical_function_shell_facts(
    parent: &GenericG0SourceParentRefV1<'_, '_>,
    effects: &VerifiedGenericG0PhysicalFunctionEffectsV1,
    descriptors: &[GenericG0PhysicalParameterDescriptorV1],
) -> Result<(), GenericG0PhysicalShellFactsRejectV1> {
    let parts = parent.physical_emitter_source_parts();
    parts
        .validate_shared_axes()
        .map_err(map_source_parts_reject)?;
    let header = parent.declaration_header();
    let storage = parent.storage_lane();
    let result = parent.result_abi();
    let context = parent.product().context();

    if header.name().is_empty() {
        return Err(GenericG0PhysicalShellFactsRejectV1::SourceNameEmpty);
    }
    if header.name().contains('/') {
        return Err(GenericG0PhysicalShellFactsRejectV1::SourceNameContainsPhysicalSeparator);
    }
    if u32::try_from(header.parameters().len()).is_err() {
        return Err(GenericG0PhysicalShellFactsRejectV1::SourceArityOverflow);
    }
    if effects.owner() != parent.owner() {
        return Err(GenericG0PhysicalShellFactsRejectV1::OwnerMismatch);
    }
    if effects.origin() != parent.product().context().origin() {
        return Err(GenericG0PhysicalShellFactsRejectV1::OriginMismatch);
    }
    if effects.source_kind() != parent.product().context().source_kind() {
        return Err(GenericG0PhysicalShellFactsRejectV1::SourceKindMismatch);
    }
    if effects.body_root() != storage.body_root() {
        return Err(GenericG0PhysicalShellFactsRejectV1::BodyRootMismatch);
    }
    if !effects.frame().matches(context.frame()) || !storage.frame().matches(context.frame()) {
        return Err(GenericG0PhysicalShellFactsRejectV1::FrameMismatch);
    }
    if !header.metadata_is_empty()
        || !header.attrs().is_empty()
        || !header.uses().is_empty()
        || !storage.attrs().is_empty()
        || !storage.uses().is_empty()
    {
        return Err(GenericG0PhysicalShellFactsRejectV1::MetadataNotEmpty);
    }
    if result.abi().source_type_name() != "i64"
        || result.abi().mir_type() != MirType::Integer
        || header.return_type_name() != Some("i64")
    {
        return Err(GenericG0PhysicalShellFactsRejectV1::ResultAbiMismatch);
    }
    if effects.effect_mask() != EffectMask::PURE {
        return Err(GenericG0PhysicalShellFactsRejectV1::EffectMismatch);
    }

    match storage.receiver_policy() {
        ReceiverPolicyV1::DeclaredInstance
            if header.is_static()
                || descriptors.first().map(|row| row.role())
                    != Some(GenericG0PhysicalLaneRoleV1::InstanceReceiver) =>
        {
            return Err(GenericG0PhysicalShellFactsRejectV1::ModeReceiverMismatch)
        }
        ReceiverPolicyV1::Absent
            if !header.is_static()
                || descriptors
                    .first()
                    .is_some_and(|row| row.role() == GenericG0PhysicalLaneRoleV1::InstanceReceiver) =>
        {
            return Err(GenericG0PhysicalShellFactsRejectV1::ModeReceiverMismatch)
        }
        ReceiverPolicyV1::StaticCurrentOwner => {
            return Err(GenericG0PhysicalShellFactsRejectV1::ModeReceiverMismatch)
        }
        _ => {}
    }

    let expected_lane_count = usize::try_from(storage.physical_callable_lane_count())
        .map_err(|_| GenericG0PhysicalShellFactsRejectV1::DescriptorCoverage)?;
    if descriptors.len() != expected_lane_count {
        return Err(GenericG0PhysicalShellFactsRejectV1::DescriptorCoverage);
    }
    let receiver_offset = usize::try_from(storage.receiver_lane_count())
        .map_err(|_| GenericG0PhysicalShellFactsRejectV1::DescriptorCoverage)?;
    for (index, row) in descriptors.iter().enumerate() {
        if row.physical_index() != u32::try_from(index).unwrap_or(u32::MAX) {
            return Err(GenericG0PhysicalShellFactsRejectV1::DescriptorIndexMismatch);
        }
        if row.carrier()
            != crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::ExistingCallableI64
        {
            return Err(GenericG0PhysicalShellFactsRejectV1::DescriptorCarrierMismatch);
        }
        if index < receiver_offset {
            if row.role() != GenericG0PhysicalLaneRoleV1::InstanceReceiver
                || row.logical_ordinal().is_some()
            {
                return Err(GenericG0PhysicalShellFactsRejectV1::DescriptorRoleMismatch);
            }
        } else if row.role() != GenericG0PhysicalLaneRoleV1::OrdinaryScalar
            || row.logical_ordinal()
                != Some(u32::try_from(index - receiver_offset).unwrap_or(u32::MAX))
            || row.source_declared_type_name() != Some("i64")
        {
            return Err(GenericG0PhysicalShellFactsRejectV1::DescriptorOrdinalMismatch);
        }
    }
    if descriptors
        .iter()
        .skip(receiver_offset)
        .any(|row| row.source_declared_type_name() != Some("i64"))
    {
        return Err(GenericG0PhysicalShellFactsRejectV1::DescriptorTypeMismatch);
    }
    Ok(())
}

fn map_source_parts_reject(
    reject: GenericG0PhysicalEmitterSourcePartsRejectV1,
) -> GenericG0PhysicalShellFactsRejectV1 {
    match reject {
        GenericG0PhysicalEmitterSourcePartsRejectV1::OwnerMismatch => {
            GenericG0PhysicalShellFactsRejectV1::OwnerMismatch
        }
        GenericG0PhysicalEmitterSourcePartsRejectV1::OriginMismatch => {
            GenericG0PhysicalShellFactsRejectV1::OriginMismatch
        }
        GenericG0PhysicalEmitterSourcePartsRejectV1::SourceKindMismatch => {
            GenericG0PhysicalShellFactsRejectV1::SourceKindMismatch
        }
        GenericG0PhysicalEmitterSourcePartsRejectV1::BodyRootMismatch => {
            GenericG0PhysicalShellFactsRejectV1::BodyRootMismatch
        }
        GenericG0PhysicalEmitterSourcePartsRejectV1::FrameMismatch => {
            GenericG0PhysicalShellFactsRejectV1::FrameMismatch
        }
    }
}
