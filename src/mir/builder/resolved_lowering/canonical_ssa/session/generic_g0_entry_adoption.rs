//! Generic G0 receiver/formal declaration adoption.

use std::collections::BTreeSet;

use crate::mir::builder::MirBuilder;
use crate::mir::compiler::generic_g0_physical_function_entry_input::{
    GenericG0PhysicalLaneRoleV1, GenericG0PhysicalParameterDescriptorV1,
};
use crate::mir::resolved_semantics::SourceBindingSiteV1;
use crate::mir::{MirType, ValueId};
use hakorune_mir_core::MirValueKind;

use super::CanonicalSsaFunctionSessionV2;

pub(super) fn adopt(
    session: &mut CanonicalSsaFunctionSessionV2<'_>,
    builder: &mut MirBuilder,
    descriptors: &[GenericG0PhysicalParameterDescriptorV1],
) -> Result<(), String> {
    if session.generic_entry_adopted {
        return Err("generic physical entry lanes were already adopted".to_owned());
    }
    validate_descriptors(session.owner, descriptors)?;
    let (entry, values) = reserved_values(builder, descriptors)?;

    let mut seen = BTreeSet::new();
    for (index, descriptor) in descriptors.iter().enumerate() {
        let site = match descriptor.role() {
            GenericG0PhysicalLaneRoleV1::InstanceReceiver => SourceBindingSiteV1::Receiver,
            GenericG0PhysicalLaneRoleV1::OrdinaryScalar => SourceBindingSiteV1::Parameter {
                index: descriptor
                    .logical_ordinal()
                    .ok_or_else(|| "generic physical entry ordinal missing".to_owned())?,
            },
        };
        if !seen.insert(descriptor.binding()) {
            return Err("generic physical entry duplicate binding".to_owned());
        }
        session.identity.publish_declaration_exact(
            &site,
            descriptor.binding(),
            entry,
            values[index],
        )?;
        builder.register_value_kind(
            values[index],
            MirValueKind::Parameter(
                u32::try_from(index)
                    .map_err(|_| "generic physical entry parameter overflow".to_owned())?,
            ),
        );
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(values[index], MirType::Integer);
    }
    session.generic_entry_adopted = true;
    Ok(())
}

fn validate_descriptors(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    descriptors: &[GenericG0PhysicalParameterDescriptorV1],
) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    for (index, descriptor) in descriptors.iter().enumerate() {
        let index = u32::try_from(index)
            .map_err(|_| "generic physical entry descriptor overflow".to_owned())?;
        if descriptor.physical_index() != index
            || descriptor.carrier()
                != crate::mir::compiler::common_v2_physical_function_entry_input::
                    PhysicalCallableLaneCarrierV1::ExistingCallableI64
            || descriptor.binding().owner() != owner
            || !seen.insert(descriptor.binding())
        {
            return Err("generic physical entry descriptor drift".to_owned());
        }
        match descriptor.role() {
            GenericG0PhysicalLaneRoleV1::InstanceReceiver
                if index != 0 || descriptor.logical_ordinal().is_some() =>
            {
                return Err("generic physical entry receiver drift".to_owned());
            }
            GenericG0PhysicalLaneRoleV1::OrdinaryScalar
                if descriptor.logical_ordinal().is_none() =>
            {
                return Err("generic physical entry formal drift".to_owned());
            }
            _ => {}
        }
    }
    if descriptors
        .iter()
        .filter(|descriptor| descriptor.role() == GenericG0PhysicalLaneRoleV1::InstanceReceiver)
        .count()
        > 1
    {
        return Err("generic physical entry duplicate receiver".to_owned());
    }
    Ok(())
}

fn reserved_values(
    builder: &MirBuilder,
    descriptors: &[GenericG0PhysicalParameterDescriptorV1],
) -> Result<(crate::mir::BasicBlockId, Vec<ValueId>), String> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| "generic physical entry function missing".to_owned())?;
    if builder.function_state.current_block != Some(function.entry_block)
        || function.params.len() != descriptors.len()
        || function.signature.params.len() != descriptors.len()
    {
        return Err("generic physical entry skeleton drift".to_owned());
    }
    let values = function
        .params
        .iter()
        .copied()
        .enumerate()
        .map(|(index, value)| {
            let expected = ValueId::new(
                u32::try_from(index)
                    .map_err(|_| "generic physical entry ValueId overflow".to_owned())?,
            );
            if value != expected || function.signature.params[index] != MirType::Integer {
                return Err("generic physical entry parameter carrier drift".to_owned());
            }
            Ok(value)
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((function.entry_block, values))
}
