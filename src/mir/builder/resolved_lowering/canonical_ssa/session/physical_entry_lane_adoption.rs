//! Physical entry-lane adoption implementation kept outside the session shell.

use super::super::super::physical_entry_lane_adoption::{
    validate_descriptor_sequence, PhysicalTextEntryLaneSidecarRowV1, PhysicalTextEntryLaneSidecarV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableParameterDescriptorV1;
use crate::mir::normal_callable_semantic_package::PhysicalCallableLaneRoleV1;
use crate::mir::resolved_semantics::{BindingRefV1, SourceBindingSiteV1};
use crate::mir::{BasicBlockId, MirType, ValueId};

use super::CanonicalSsaFunctionSessionV2;

pub(super) fn adopt(
    session: &mut CanonicalSsaFunctionSessionV2<'_>,
    builder: &mut MirBuilder,
    descriptors: &[PhysicalCallableParameterDescriptorV1],
) -> Result<(), String> {
    if session.physical_entry_sidecar.is_some() {
        return Err("physical entry lanes were already adopted".to_owned());
    }
    validate_descriptor_sequence(descriptors)?;
    let (entry, values) = reserved_values(builder, descriptors)?;

    let mut sidecar_rows = Vec::new();
    let mut index = 0usize;
    while index < descriptors.len() {
        let descriptor = &descriptors[index];
        let value = values[index];
        let role = descriptor.role();
        let site = match role {
            PhysicalCallableLaneRoleV1::InstanceReceiver => SourceBindingSiteV1::Receiver,
            PhysicalCallableLaneRoleV1::OrdinaryScalar
            | PhysicalCallableLaneRoleV1::ExactTextSlot => {
                let ordinal = descriptor
                    .logical_ordinal()
                    .ok_or_else(|| "physical entry logical ordinal missing".to_owned())?;
                SourceBindingSiteV1::Parameter { index: ordinal }
            }
            PhysicalCallableLaneRoleV1::ExactTextGeneration => {
                index += 1;
                continue;
            }
        };
        if descriptor.binding().owner() != session.owner {
            return Err("physical entry binding owner drift".to_owned());
        }
        session
            .identity
            .publish_declaration_exact(&site, descriptor.binding(), entry, value)?;
        builder.register_value_kind(
            value,
            hakorune_mir_core::MirValueKind::Parameter(
                u32::try_from(index)
                    .map_err(|_| "physical entry parameter ordinal overflow".to_owned())?,
            ),
        );
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(value, MirType::Integer);

        if role == PhysicalCallableLaneRoleV1::ExactTextSlot {
            let generation = descriptors
                .get(index + 1)
                .ok_or_else(|| "physical entry generation lane missing".to_owned())?;
            if generation.role() != PhysicalCallableLaneRoleV1::ExactTextGeneration {
                return Err("physical entry generation lane drift".to_owned());
            }
            sidecar_rows.push(PhysicalTextEntryLaneSidecarRowV1::new(
                descriptor.binding(),
                descriptor
                    .logical_ordinal()
                    .ok_or_else(|| "physical entry ExactText ordinal missing".to_owned())?,
                value,
                values[index + 1],
                descriptor.carrier(),
            ));
        }
        index += 1;
    }
    session.physical_entry_sidecar = Some(PhysicalTextEntryLaneSidecarV1::new(
        session.owner,
        entry,
        sidecar_rows,
    ));
    Ok(())
}

pub(super) fn with_exact_text_sidecar_row<R>(
    session: &CanonicalSsaFunctionSessionV2<'_>,
    binding: BindingRefV1,
    logical_ordinal: u32,
    callback: impl FnOnce(&PhysicalTextEntryLaneSidecarRowV1) -> R,
) -> Result<R, String> {
    let sidecar = session
        .physical_entry_sidecar
        .as_ref()
        .ok_or_else(|| "physical entry ExactText sidecar is missing".to_owned())?;
    let mut rows = sidecar
        .rows()
        .iter()
        .filter(|row| row.binding() == binding && row.logical_ordinal() == logical_ordinal);
    let row = rows
        .next()
        .ok_or_else(|| "physical entry ExactText sidecar row is missing".to_owned())?;
    if rows.next().is_some() {
        return Err("physical entry ExactText sidecar row is duplicated".to_owned());
    }
    Ok(callback(row))
}

fn reserved_values(
    builder: &MirBuilder,
    descriptors: &[PhysicalCallableParameterDescriptorV1],
) -> Result<(BasicBlockId, Vec<ValueId>), String> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| "physical entry adoption requires current function".to_owned())?;
    if builder.function_state.current_block != Some(function.entry_block)
        || function.params.len() != descriptors.len()
        || function.signature.params.len() != descriptors.len()
    {
        return Err("physical entry skeleton drift".to_owned());
    }
    let values = function
        .params
        .iter()
        .copied()
        .enumerate()
        .map(|(index, value)| {
            let expected = ValueId::new(
                u32::try_from(index).map_err(|_| "physical entry ValueId overflow".to_owned())?,
            );
            if value != expected || function.signature.params[index] != MirType::Integer {
                return Err("physical entry parameter carrier drift".to_owned());
            }
            Ok(value)
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((function.entry_block, values))
}
