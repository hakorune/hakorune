//! Canonical entry adoption for expanded callable lanes.
//!
//! One logical ExactText binding owns one ordinary BindingSSA slot.  Its
//! adjacent generation lane is retained only in this skeleton-bound,
//! move-only sidecar; it is not a second semantic binding.

use crate::mir::compiler::common_v2_physical_function_entry_input::{
    PhysicalCallableLaneCarrierV1, PhysicalCallableParameterDescriptorV1,
};
use crate::mir::normal_callable_semantic_package::PhysicalCallableLaneRoleV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct PhysicalTextEntryLaneSidecarRowV1 {
    binding: BindingRefV1,
    logical_ordinal: u32,
    slot: ValueId,
    generation: ValueId,
    carrier: PhysicalCallableLaneCarrierV1,
}

impl PhysicalTextEntryLaneSidecarRowV1 {
    pub(in crate::mir::builder::resolved_lowering) const fn new(
        binding: BindingRefV1,
        logical_ordinal: u32,
        slot: ValueId,
        generation: ValueId,
        carrier: PhysicalCallableLaneCarrierV1,
    ) -> Self {
        Self {
            binding,
            logical_ordinal,
            slot,
            generation,
            carrier,
        }
    }

    pub(in crate::mir::builder::resolved_lowering) const fn binding(self) -> BindingRefV1 {
        self.binding
    }

    pub(in crate::mir::builder::resolved_lowering) const fn logical_ordinal(self) -> u32 {
        self.logical_ordinal
    }

    pub(in crate::mir::builder::resolved_lowering) const fn slot(self) -> ValueId {
        self.slot
    }

    pub(in crate::mir::builder::resolved_lowering) const fn generation(self) -> ValueId {
        self.generation
    }

    pub(in crate::mir::builder::resolved_lowering) const fn carrier(
        self,
    ) -> PhysicalCallableLaneCarrierV1 {
        self.carrier
    }
}

/// Move-only sidecar owned by the canonical physical session. It is scoped
/// to one function owner and live entry block; no raw pointer or runtime token
/// is stored here.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct PhysicalTextEntryLaneSidecarV1 {
    owner: FunctionOwnerIdV1,
    entry: BasicBlockId,
    rows: Box<[PhysicalTextEntryLaneSidecarRowV1]>,
}

impl PhysicalTextEntryLaneSidecarV1 {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        owner: FunctionOwnerIdV1,
        entry: BasicBlockId,
        rows: Vec<PhysicalTextEntryLaneSidecarRowV1>,
    ) -> Self {
        Self {
            owner,
            entry,
            rows: rows.into_boxed_slice(),
        }
    }

    pub(in crate::mir::builder::resolved_lowering) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder::resolved_lowering) const fn entry(&self) -> BasicBlockId {
        self.entry
    }

    pub(in crate::mir::builder::resolved_lowering) fn rows(
        &self,
    ) -> &[PhysicalTextEntryLaneSidecarRowV1] {
        &self.rows
    }
}

pub(in crate::mir::builder::resolved_lowering) fn validate_descriptor_sequence(
    descriptors: &[PhysicalCallableParameterDescriptorV1],
) -> Result<(), String> {
    let mut previous_text: Option<(u32, BindingRefV1, ValueId)> = None;
    let mut seen_bindings = std::collections::BTreeSet::new();
    for (index, descriptor) in descriptors.iter().enumerate() {
        let index = u32::try_from(index)
            .map_err(|_| "physical entry descriptor index overflow".to_owned())?;
        if descriptor.physical_index() != index {
            return Err("physical entry descriptor index drift".to_owned());
        }
        match descriptor.role() {
            PhysicalCallableLaneRoleV1::InstanceReceiver => {
                if index != 0 || descriptor.logical_ordinal().is_some() {
                    return Err("physical entry receiver prefix drift".to_owned());
                }
                if !seen_bindings.insert(descriptor.binding()) {
                    return Err("physical entry duplicate receiver binding".to_owned());
                }
                previous_text = None;
            }
            PhysicalCallableLaneRoleV1::OrdinaryScalar => {
                if descriptor.logical_ordinal().is_none()
                    || !seen_bindings.insert(descriptor.binding())
                {
                    return Err("physical entry ordinary binding drift".to_owned());
                }
                previous_text = None;
            }
            PhysicalCallableLaneRoleV1::ExactTextSlot => {
                let ordinal = descriptor
                    .logical_ordinal()
                    .ok_or_else(|| "physical entry ExactText ordinal missing".to_owned())?;
                if descriptor.carrier() != PhysicalCallableLaneCarrierV1::U64BitsOnI64
                    || !seen_bindings.insert(descriptor.binding())
                {
                    return Err("physical entry ExactText slot drift".to_owned());
                }
                previous_text = Some((ordinal, descriptor.binding(), ValueId::new(index)));
            }
            PhysicalCallableLaneRoleV1::ExactTextGeneration => {
                let Some((ordinal, binding, slot)) = previous_text.take() else {
                    return Err("physical entry generation without adjacent slot".to_owned());
                };
                if descriptor.logical_ordinal() != Some(ordinal)
                    || descriptor.binding() != binding
                    || descriptor.carrier() != PhysicalCallableLaneCarrierV1::U64BitsOnI64
                    || slot != ValueId::new(index.saturating_sub(1))
                {
                    return Err("physical entry ExactText pair drift".to_owned());
                }
            }
        }
    }
    if previous_text.is_some() {
        return Err("physical entry ExactText slot is missing generation".to_owned());
    }
    Ok(())
}
