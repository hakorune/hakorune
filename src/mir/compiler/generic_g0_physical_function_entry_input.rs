//! Generic-only, pre-effect physical function-entry input.
//!
//! This is intentionally not the S6C physical descriptor.  It consumes one
//! complete Generic source parent and projects only the mechanical receiver
//! prefix and ordinary i64 formal rows needed by a later skeleton consumer.
//! No `MirFunction`, `ValueId`, `EffectMask`, Builder, or session state is
//! created here.

use std::collections::BTreeSet;

use super::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, ReceiverPolicyV1,
};

use super::generic_g0_source_parent::GenericG0SourceParentRefV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0PhysicalLaneRoleV1 {
    InstanceReceiver,
    OrdinaryScalar,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0PhysicalParameterDescriptorV1 {
    physical_index: u32,
    role: GenericG0PhysicalLaneRoleV1,
    logical_ordinal: Option<u32>,
    binding: BindingRefV1,
    diagnostic_name: Box<str>,
    source_declared_type_name: Option<Box<str>>,
    carrier: PhysicalCallableLaneCarrierV1,
}

impl GenericG0PhysicalParameterDescriptorV1 {
    pub(crate) const fn physical_index(&self) -> u32 {
        self.physical_index
    }

    pub(crate) const fn role(&self) -> GenericG0PhysicalLaneRoleV1 {
        self.role
    }

    pub(crate) const fn logical_ordinal(&self) -> Option<u32> {
        self.logical_ordinal
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }

    pub(crate) fn source_declared_type_name(&self) -> Option<&str> {
        self.source_declared_type_name.as_deref()
    }

    pub(crate) const fn carrier(&self) -> PhysicalCallableLaneCarrierV1 {
        self.carrier
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0PhysicalFunctionEntryRejectV1 {
    OwnerMismatch,
    OriginMismatch,
    SourceKindMismatch,
    BodyRootMismatch,
    FrameMismatch,
    HeaderModeMismatch,
    HeaderParameterCountMismatch,
    HeaderParameterOrdinalMismatch,
    HeaderParameterNameMismatch,
    HeaderParameterTypeMismatch,
    ResultAbiMismatch,
    EffectReceiptMismatch,
    CompletionMismatch,
    ReceiverMissing,
    ReceiverUnexpected,
    ReceiverDuplicate,
    FormalBindingDuplicate,
    FormalTypeMismatch,
    FormalAbiMismatch,
    DescriptorIndexOverflow,
    DescriptorCountMismatch,
    DescriptorNameCollision,
    UnsupportedReceiverPolicy,
}

/// One Generic-only pre-effect entry input.  The source parent remains
/// attached so its siblings cannot be re-paired or independently consumed.
pub(crate) struct PreparedGenericG0PhysicalFunctionEntryInputV1<'loan, 'source> {
    parent: GenericG0SourceParentRefV1<'loan, 'source>,
    descriptors: Box<[GenericG0PhysicalParameterDescriptorV1]>,
    physical_callable_lane_count: u32,
}

impl<'loan, 'source> PreparedGenericG0PhysicalFunctionEntryInputV1<'loan, 'source> {
    pub(crate) fn parent(&self) -> &GenericG0SourceParentRefV1<'loan, 'source> {
        &self.parent
    }

    pub(crate) fn descriptors(&self) -> &[GenericG0PhysicalParameterDescriptorV1] {
        &self.descriptors
    }

    pub(crate) const fn physical_callable_lane_count(&self) -> u32 {
        self.physical_callable_lane_count
    }

    /// Consume the complete parent and descriptor cohort together.  This is
    /// the only projection seam offered to the later skeleton consumer.
    pub(crate) fn consume<R>(
        self,
        callback: impl FnOnce(
            GenericG0SourceParentRefV1<'loan, 'source>,
            Box<[GenericG0PhysicalParameterDescriptorV1]>,
        ) -> R,
    ) -> R {
        callback(self.parent, self.descriptors)
    }
}

pub(crate) fn issue_generic_g0_physical_function_entry_input_v1<'loan, 'source>(
    parent: GenericG0SourceParentRefV1<'loan, 'source>,
) -> Result<
    PreparedGenericG0PhysicalFunctionEntryInputV1<'loan, 'source>,
    GenericG0PhysicalFunctionEntryRejectV1,
> {
    let header = parent.declaration_header();
    let storage = parent.storage_lane();
    let result = parent.result_abi();
    let effect = parent.function_effect();
    let completion = parent.completion();

    if storage.owner() != parent.owner()
        || header.owner() != parent.owner()
        || result.owner() != parent.owner()
        || effect.owner() != parent.owner()
        || completion.owner() != parent.owner()
    {
        return Err(GenericG0PhysicalFunctionEntryRejectV1::OwnerMismatch);
    }
    if storage.origin() != header.origin()
        || result.origin() != header.origin()
        || effect.origin() != header.origin()
    {
        return Err(GenericG0PhysicalFunctionEntryRejectV1::OriginMismatch);
    }
    if storage.source_kind() != header.source_kind()
        || result.source_kind() != header.source_kind()
        || effect.source_kind() != header.source_kind()
    {
        return Err(GenericG0PhysicalFunctionEntryRejectV1::SourceKindMismatch);
    }
    if effect.body_root() != storage.body_root() {
        return Err(GenericG0PhysicalFunctionEntryRejectV1::BodyRootMismatch);
    }
    if !storage.frame().matches(parent.product().context().frame())
        || !effect.root_frame().matches(parent.product().context().frame())
    {
        return Err(GenericG0PhysicalFunctionEntryRejectV1::FrameMismatch);
    }
    if header.return_type_name() != Some(result.abi().source_type_name())
        || result.abi().source_type_name() != "i64"
    {
        return Err(GenericG0PhysicalFunctionEntryRejectV1::ResultAbiMismatch);
    }
    if effect.local_write_count() != 2 || effect.tail_return_count() != 1 {
        return Err(GenericG0PhysicalFunctionEntryRejectV1::EffectReceiptMismatch);
    }
    if !completion.returns_value()
        || completion.explicit_sites().len() != 1
        || !completion.cleanup().crossed_scopes().is_empty()
    {
        return Err(GenericG0PhysicalFunctionEntryRejectV1::CompletionMismatch);
    }

    let receiver_policy = storage.receiver_policy();
    if matches!(receiver_policy, ReceiverPolicyV1::StaticCurrentOwner) {
        return Err(GenericG0PhysicalFunctionEntryRejectV1::UnsupportedReceiverPolicy);
    }
    match (receiver_policy, storage.receiver(), header.is_static()) {
        (ReceiverPolicyV1::DeclaredInstance, Some(_), false)
        | (ReceiverPolicyV1::Absent, None, true) => {}
        (ReceiverPolicyV1::DeclaredInstance, None, _) => {
            return Err(GenericG0PhysicalFunctionEntryRejectV1::ReceiverMissing)
        }
        (ReceiverPolicyV1::Absent, Some(_), _) => {
            return Err(GenericG0PhysicalFunctionEntryRejectV1::ReceiverUnexpected)
        }
        _ => return Err(GenericG0PhysicalFunctionEntryRejectV1::HeaderModeMismatch),
    }

    if header.parameters().len() != storage.formals().len()
        || header.parameters().len() != parent.entries().len()
    {
        return Err(GenericG0PhysicalFunctionEntryRejectV1::HeaderParameterCountMismatch);
    }

    let expected_descriptor_count = usize::try_from(storage.physical_callable_lane_count())
        .map_err(|_| GenericG0PhysicalFunctionEntryRejectV1::DescriptorCountMismatch)?;
    let mut descriptors = Vec::with_capacity(expected_descriptor_count);
    let mut seen_bindings = BTreeSet::new();
    let mut seen_names = BTreeSet::new();
    let mut physical_index = 0u32;

    if let Some(receiver) = storage.receiver() {
        if !seen_bindings.insert(receiver.binding()) {
            return Err(GenericG0PhysicalFunctionEntryRejectV1::ReceiverDuplicate);
        }
        let index = physical_index;
        physical_index = physical_index
            .checked_add(1)
            .ok_or(GenericG0PhysicalFunctionEntryRejectV1::DescriptorIndexOverflow)?;
        if !seen_names.insert("me") {
            return Err(GenericG0PhysicalFunctionEntryRejectV1::DescriptorNameCollision);
        }
        descriptors.push(GenericG0PhysicalParameterDescriptorV1 {
            physical_index: index,
            role: GenericG0PhysicalLaneRoleV1::InstanceReceiver,
            logical_ordinal: None,
            binding: receiver.binding(),
            diagnostic_name: "me".into(),
            source_declared_type_name: None,
            carrier: PhysicalCallableLaneCarrierV1::ExistingCallableI64,
        });
    }

    for ((formal, parameter), entry) in storage
        .formals()
        .iter()
        .zip(header.parameters())
        .zip(parent.entries())
    {
        if formal.ordinal() != parameter.ordinal() || formal.ordinal() != entry.parameter_index() {
            return Err(GenericG0PhysicalFunctionEntryRejectV1::HeaderParameterOrdinalMismatch);
        }
        let Some(header_type_name) = parameter.declared_type_name() else {
            return Err(GenericG0PhysicalFunctionEntryRejectV1::HeaderParameterTypeMismatch);
        };
        if formal.declared_type_name() != header_type_name {
            return Err(GenericG0PhysicalFunctionEntryRejectV1::HeaderParameterTypeMismatch);
        }
        if parameter.name().is_empty() {
            return Err(GenericG0PhysicalFunctionEntryRejectV1::HeaderParameterNameMismatch);
        }
        if formal.binding() != entry.binding() {
            return Err(GenericG0PhysicalFunctionEntryRejectV1::FormalBindingDuplicate);
        }
        if formal.declared_type_name() != "i64" {
            return Err(GenericG0PhysicalFunctionEntryRejectV1::FormalTypeMismatch);
        }
        if formal.abi() != crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1::I64 {
            return Err(GenericG0PhysicalFunctionEntryRejectV1::FormalAbiMismatch);
        }
        if !seen_bindings.insert(formal.binding()) {
            return Err(GenericG0PhysicalFunctionEntryRejectV1::FormalBindingDuplicate);
        }
        if !seen_names.insert(parameter.name()) {
            return Err(GenericG0PhysicalFunctionEntryRejectV1::DescriptorNameCollision);
        }
        let index = physical_index;
        physical_index = physical_index
            .checked_add(1)
            .ok_or(GenericG0PhysicalFunctionEntryRejectV1::DescriptorIndexOverflow)?;
        descriptors.push(GenericG0PhysicalParameterDescriptorV1 {
            physical_index: index,
            role: GenericG0PhysicalLaneRoleV1::OrdinaryScalar,
            logical_ordinal: Some(formal.ordinal()),
            binding: formal.binding(),
            diagnostic_name: parameter.name().into(),
            source_declared_type_name: parameter.declared_type_name().map(Into::into),
            carrier: PhysicalCallableLaneCarrierV1::ExistingCallableI64,
        });
    }

    if physical_index != storage.physical_callable_lane_count()
        || descriptors.len() != expected_descriptor_count
    {
        return Err(GenericG0PhysicalFunctionEntryRejectV1::DescriptorCountMismatch);
    }

    Ok(PreparedGenericG0PhysicalFunctionEntryInputV1 {
        parent,
        descriptors: descriptors.into_boxed_slice(),
        physical_callable_lane_count: physical_index,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        issue_generic_g0_physical_function_entry_input_v1, GenericG0PhysicalLaneRoleV1,
    };
    use crate::mir::compiler::generic_g0_source_parent::with_generic_g0_source_parent_v1;
    use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;

    #[test]
    fn generic_entry_input_projects_receiver_and_explicit_rows_without_effect() {
        let (unit, selection) = generic_source_unit_and_selection_for_test();
        let input = unit.root_function_input().expect("root input");
        let owner = input.owner();
        let consumed = with_generic_g0_source_parent_v1(input, selection, |parent| {
            let prepared = issue_generic_g0_physical_function_entry_input_v1(parent)?;
            assert_eq!(prepared.physical_callable_lane_count(), 3);
            assert_eq!(prepared.descriptors().len(), 3);
            assert_eq!(
                prepared.descriptors()[0].role(),
                GenericG0PhysicalLaneRoleV1::InstanceReceiver
            );
            assert_eq!(prepared.descriptors()[0].diagnostic_name(), "me");
            assert_eq!(prepared.descriptors()[1].logical_ordinal(), Some(0));
            assert_eq!(prepared.descriptors()[1].diagnostic_name(), "i");
            assert_eq!(prepared.descriptors()[2].logical_ordinal(), Some(1));
            assert_eq!(prepared.descriptors()[2].diagnostic_name(), "j");
            assert_eq!(prepared.parent().owner(), owner);
            Ok::<usize, super::GenericG0PhysicalFunctionEntryRejectV1>(prepared.consume(
                |parent, descriptors| {
                    assert_eq!(parent.owner(), owner);
                    descriptors.len()
                },
            ))
        });
        assert_eq!(
            consumed
                .expect("source cohort")
                .expect("generic physical entry input"),
            3
        );
    }
}
