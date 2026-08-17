//! Detached Generic G0 physical-function skeleton reservation.
//!
//! This is deliberately Generic-only.  It consumes one complete entry-input
//! cohort, reissues the already-sealed Generic physical effect projection, and
//! reserves one unpublished `MirFunction` shell.  The shell's parameter
//! `ValueId`s are mechanical reservations only; entry adoption is a later
//! transaction.

use crate::mir::compiler::generic_g0_physical_function_effect::{
    issue_generic_g0_physical_function_effects_v1,
    GenericG0PhysicalFunctionEffectRejectV1,
    VerifiedGenericG0PhysicalFunctionEffectsV1,
};
use crate::mir::compiler::generic_g0_physical_function_entry_input::{
    GenericG0PhysicalFunctionEntryRejectV1, GenericG0PhysicalLaneRoleV1,
    GenericG0PhysicalParameterDescriptorV1, PreparedGenericG0PhysicalFunctionEntryInputV1,
};
use crate::mir::compiler::generic_g0_source_parent::GenericG0SourceParentRefV1;
use crate::mir::resolved_semantics::ReceiverPolicyV1;
use crate::mir::resolved_semantics::CanonicalCallableSymbolV1;
use crate::mir::function::MirParamDecl;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GenericG0PhysicalFunctionSkeletonRejectV1 {
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

/// One detached Generic shell and the exact source cohort that explains it.
/// Dropping this value is the complete rollback before Builder/session state.
pub(crate) struct PreparedGenericG0PhysicalFunctionSkeletonV1<'loan, 'source> {
    parent: GenericG0SourceParentRefV1<'loan, 'source>,
    effects: VerifiedGenericG0PhysicalFunctionEffectsV1,
    descriptors: Box<[GenericG0PhysicalParameterDescriptorV1]>,
    function: MirFunction,
}

impl<'loan, 'source> PreparedGenericG0PhysicalFunctionSkeletonV1<'loan, 'source> {
    pub(crate) fn function(&self) -> &MirFunction {
        &self.function
    }

    pub(crate) fn descriptors(&self) -> &[GenericG0PhysicalParameterDescriptorV1] {
        &self.descriptors
    }

    pub(crate) fn effects(&self) -> &VerifiedGenericG0PhysicalFunctionEffectsV1 {
        &self.effects
    }

    pub(crate) fn parent(&self) -> &GenericG0SourceParentRefV1<'loan, 'source> {
        &self.parent
    }
}

/// Reserve one unpublished physical shell from one Generic entry cohort.
/// `MirFunction::new` is the only mechanical allocation in this slice.
pub(crate) fn reserve_generic_g0_physical_function_skeleton<'loan, 'source>(
    prepared: PreparedGenericG0PhysicalFunctionEntryInputV1<'loan, 'source>,
) -> Result<
    PreparedGenericG0PhysicalFunctionSkeletonV1<'loan, 'source>,
    GenericG0PhysicalFunctionSkeletonRejectV1,
> {
    prepared.consume(|parent, descriptors| reserve_from_parts(parent, descriptors))
}

fn reserve_from_parts<'loan, 'source>(
    parent: GenericG0SourceParentRefV1<'loan, 'source>,
    descriptors: Box<[GenericG0PhysicalParameterDescriptorV1]>,
) -> Result<
    PreparedGenericG0PhysicalFunctionSkeletonV1<'loan, 'source>,
    GenericG0PhysicalFunctionSkeletonRejectV1,
> {
    let effects = issue_generic_g0_physical_function_effects_v1(&parent)
        .map_err(GenericG0PhysicalFunctionSkeletonRejectV1::PhysicalEffect)?;
    validate_cohort(&parent, &effects, &descriptors)?;

    let header = parent.declaration_header();
    let symbol = CanonicalCallableSymbolV1::from_name_arity(
        header.name(),
        header.parameters().len(),
    );
    let function = MirFunction::new(
        FunctionSignature {
            name: symbol.as_mir_name().to_owned(),
            params: vec![MirType::Integer; descriptors.len()],
            return_type: parent.result_abi().abi().mir_type(),
            effects: effects.effect_mask(),
        },
        BasicBlockId::new(0),
    );
    let mut function = function;
    function.metadata.declared_param_decls = descriptors
        .iter()
        .map(|row| MirParamDecl {
            name: row.diagnostic_name().to_owned(),
            declared_type_name: row.source_declared_type_name().map(str::to_owned),
            implicit_receiver: row.role() == GenericG0PhysicalLaneRoleV1::InstanceReceiver,
        })
        .collect();
    function.metadata.declared_return_type_name =
        Some(parent.result_abi().abi().source_type_name().to_owned());
    // The admitted Generic shape is exact-empty metadata.  Do not project
    // arbitrary attrs/uses from a physical or JSON representation.
    function.metadata.declared_capability_uses = Vec::new();
    function.metadata.runes = Vec::new();

    Ok(PreparedGenericG0PhysicalFunctionSkeletonV1 {
        parent,
        effects,
        descriptors,
        function,
    })
}

fn validate_cohort(
    parent: &GenericG0SourceParentRefV1<'_, '_>,
    effects: &VerifiedGenericG0PhysicalFunctionEffectsV1,
    descriptors: &[GenericG0PhysicalParameterDescriptorV1],
) -> Result<(), GenericG0PhysicalFunctionSkeletonRejectV1> {
    let header = parent.declaration_header();
    let storage = parent.storage_lane();
    let result = parent.result_abi();
    let context = parent.product().context();

    if header.name().is_empty() {
        return Err(GenericG0PhysicalFunctionSkeletonRejectV1::SourceNameEmpty);
    }
    if header.name().contains('/') {
        return Err(
            GenericG0PhysicalFunctionSkeletonRejectV1::SourceNameContainsPhysicalSeparator,
        );
    }
    if u32::try_from(header.parameters().len()).is_err() {
        return Err(GenericG0PhysicalFunctionSkeletonRejectV1::SourceArityOverflow);
    }
    if effects.owner() != parent.owner()
        || header.owner() != parent.owner()
        || storage.owner() != parent.owner()
        || result.owner() != parent.owner()
    {
        return Err(GenericG0PhysicalFunctionSkeletonRejectV1::OwnerMismatch);
    }
    if effects.origin() != header.origin()
        || storage.origin() != header.origin()
        || result.origin() != header.origin()
    {
        return Err(GenericG0PhysicalFunctionSkeletonRejectV1::OriginMismatch);
    }
    if effects.source_kind() != header.source_kind()
        || storage.source_kind() != header.source_kind()
        || result.source_kind() != header.source_kind()
    {
        return Err(GenericG0PhysicalFunctionSkeletonRejectV1::SourceKindMismatch);
    }
    if effects.body_root() != storage.body_root()
        || effects.body_root() != parent.body_shape().body_root()
    {
        return Err(GenericG0PhysicalFunctionSkeletonRejectV1::BodyRootMismatch);
    }
    if !effects.frame().matches(context.frame()) || !storage.frame().matches(context.frame()) {
        return Err(GenericG0PhysicalFunctionSkeletonRejectV1::FrameMismatch);
    }
    if !header.metadata_is_empty() || !header.attrs().is_empty() || !header.uses().is_empty()
        || !storage.attrs().is_empty()
        || !storage.uses().is_empty()
    {
        return Err(GenericG0PhysicalFunctionSkeletonRejectV1::MetadataNotEmpty);
    }
    if result.abi().source_type_name() != "i64"
        || result.abi().mir_type() != MirType::Integer
        || header.return_type_name() != Some("i64")
    {
        return Err(GenericG0PhysicalFunctionSkeletonRejectV1::ResultAbiMismatch);
    }
    if effects.effect_mask() != EffectMask::PURE {
        return Err(GenericG0PhysicalFunctionSkeletonRejectV1::EffectMismatch);
    }

    match storage.receiver_policy() {
        ReceiverPolicyV1::DeclaredInstance
            if header.is_static()
                || descriptors.first().map(|row| row.role())
                    != Some(GenericG0PhysicalLaneRoleV1::InstanceReceiver) =>
        {
            return Err(GenericG0PhysicalFunctionSkeletonRejectV1::ModeReceiverMismatch)
        }
        ReceiverPolicyV1::Absent
            if !header.is_static()
                || descriptors
                    .first()
                    .is_some_and(|row| row.role() == GenericG0PhysicalLaneRoleV1::InstanceReceiver) =>
        {
            return Err(GenericG0PhysicalFunctionSkeletonRejectV1::ModeReceiverMismatch)
        }
        ReceiverPolicyV1::StaticCurrentOwner => {
            return Err(GenericG0PhysicalFunctionSkeletonRejectV1::ModeReceiverMismatch)
        }
        _ => {}
    }

    let expected_lane_count = usize::try_from(storage.physical_callable_lane_count())
        .map_err(|_| GenericG0PhysicalFunctionSkeletonRejectV1::DescriptorCoverage)?;
    if descriptors.len() != expected_lane_count {
        return Err(GenericG0PhysicalFunctionSkeletonRejectV1::DescriptorCoverage);
    }
    let receiver_offset = usize::try_from(storage.receiver_lane_count())
        .map_err(|_| GenericG0PhysicalFunctionSkeletonRejectV1::DescriptorCoverage)?;
    for (index, row) in descriptors.iter().enumerate() {
        if row.physical_index() != u32::try_from(index).unwrap_or(u32::MAX) {
            return Err(GenericG0PhysicalFunctionSkeletonRejectV1::DescriptorIndexMismatch);
        }
        if row.carrier()
            != crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::ExistingCallableI64
        {
            return Err(GenericG0PhysicalFunctionSkeletonRejectV1::DescriptorCarrierMismatch);
        }
        if index < receiver_offset {
            if row.role() != GenericG0PhysicalLaneRoleV1::InstanceReceiver
                || row.logical_ordinal().is_some()
            {
                return Err(GenericG0PhysicalFunctionSkeletonRejectV1::DescriptorRoleMismatch);
            }
        } else {
            if row.role() != GenericG0PhysicalLaneRoleV1::OrdinaryScalar
                || row.logical_ordinal() != Some(u32::try_from(index - receiver_offset).unwrap_or(u32::MAX))
                || row.source_declared_type_name() != Some("i64")
            {
                return Err(GenericG0PhysicalFunctionSkeletonRejectV1::DescriptorOrdinalMismatch);
            }
        }
    }
    if descriptors
        .iter()
        .skip(receiver_offset)
        .any(|row| row.source_declared_type_name() != Some("i64"))
    {
        return Err(GenericG0PhysicalFunctionSkeletonRejectV1::DescriptorTypeMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::reserve_generic_g0_physical_function_skeleton;
    use crate::mir::compiler::generic_g0_physical_function_effect::
        issue_generic_g0_physical_function_effects_v1;
    use crate::mir::compiler::generic_g0_physical_function_entry_input::
        issue_generic_g0_physical_function_entry_input_v1;
    use crate::mir::compiler::generic_g0_source_parent::with_generic_g0_source_parent_v1;
    use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;
    use crate::mir::{BasicBlockId, EffectMask, MirType};

    #[test]
    fn reserves_detached_generic_shell_using_source_arity() {
        let (unit, selection) = generic_source_unit_and_selection_for_test();
        let input = unit.root_function_input().expect("root input");
        let result = with_generic_g0_source_parent_v1(input, selection, |parent| {
            let owner = parent.owner();
            let prepared = issue_generic_g0_physical_function_entry_input_v1(parent)
                .map_err(|_| "entry")?;
            let skeleton = reserve_generic_g0_physical_function_skeleton(prepared)
                .map_err(|_| "skeleton")?;
            assert_eq!(skeleton.function().signature.name, "generic_g0/2");
            assert_eq!(skeleton.function().signature.params.len(), 3);
            assert_eq!(skeleton.function().params.len(), 3);
            assert_eq!(skeleton.function().signature.return_type, MirType::Integer);
            assert_eq!(skeleton.function().signature.effects, EffectMask::PURE);
            assert_eq!(skeleton.function().entry_block, BasicBlockId::new(0));
            assert_eq!(skeleton.function().metadata.declared_param_decls.len(), 3);
            assert!(skeleton.function().metadata.declared_capability_uses.is_empty());
            assert!(skeleton.function().metadata.runes.is_empty());
            assert_eq!(skeleton.descriptors()[0].diagnostic_name(), "me");
            assert_eq!(skeleton.descriptors()[1].diagnostic_name(), "i");
            assert_eq!(skeleton.descriptors()[2].diagnostic_name(), "j");
            assert_eq!(skeleton.effects().effect_mask(), EffectMask::PURE);
            assert_eq!(skeleton.parent().owner(), owner);
            Ok::<(), &'static str>(())
        });
        result.expect("detached Generic skeleton").expect("source cohort");
    }

    #[test]
    fn descriptor_drift_rejects_before_detached_shell_creation() {
        let (unit, selection) = generic_source_unit_and_selection_for_test();
        let input = unit.root_function_input().expect("root input");
        let result = with_generic_g0_source_parent_v1(input, selection, |parent| {
            let prepared = issue_generic_g0_physical_function_entry_input_v1(parent)
                .map_err(|_| "entry")?;
            prepared.consume(|parent, mut descriptors| {
                let mut rows = descriptors.into_vec();
                rows.pop();
                descriptors = rows.into_boxed_slice();
                let effects = issue_generic_g0_physical_function_effects_v1(&parent)
                    .map_err(|_| "effect")?;
                assert_eq!(
                    super::validate_cohort(&parent, &effects, &descriptors),
                    Err(super::GenericG0PhysicalFunctionSkeletonRejectV1::DescriptorCoverage)
                );
                Ok::<(), &'static str>(())
            })
        });
        result.expect("source cohort").expect("descriptor rejection");
    }
}
