//! Same-cohort, Builder-free physical parameter input for common V2.
//!
//! This module only joins already-issued package siblings.  It does not
//! create a function, allocate a block, publish a ValueId, or reinterpret a
//! source annotation as a physical ABI fact.

use std::collections::BTreeSet;

use crate::ast::ParamDecl;
use crate::mir::normal_callable_semantic_package::{
    PhysicalCallableLaneRoleV1, PhysicalCallableSignatureRowRefV1,
    S6CCommonV2PreSessionLoanRefV1,
};
use crate::mir::resolved_semantics::BindingRefV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum PhysicalCallableLaneCarrierV1 {
    ExistingCallableI64,
    U64BitsOnI64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct PhysicalCallableParameterDescriptorV1 {
    physical_index: u32,
    role: PhysicalCallableLaneRoleV1,
    logical_ordinal: Option<u32>,
    binding: BindingRefV1,
    diagnostic_name: Box<str>,
    source_declared_type_name: Option<Box<str>>,
    carrier: PhysicalCallableLaneCarrierV1,
}

impl PhysicalCallableParameterDescriptorV1 {
    pub(in crate::mir) const fn physical_index(&self) -> u32 {
        self.physical_index
    }

    pub(in crate::mir) const fn role(&self) -> PhysicalCallableLaneRoleV1 {
        self.role
    }

    pub(in crate::mir) const fn logical_ordinal(&self) -> Option<u32> {
        self.logical_ordinal
    }

    pub(in crate::mir) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(in crate::mir) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }

    pub(in crate::mir) fn source_declared_type_name(&self) -> Option<&str> {
        self.source_declared_type_name.as_deref()
    }

    pub(in crate::mir) const fn carrier(&self) -> PhysicalCallableLaneCarrierV1 {
        self.carrier
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum PhysicalFunctionEntryInputRejectV1 {
    SourceParameterCoverage,
    SourceNameMismatch,
    EmptySourceName,
    DuplicateSourceName,
    LaneCoverage,
    LaneIndex,
    ReceiverPolicy,
    LogicalOrdinal,
    BindingOwner,
    BindingCoverage,
    ExactTextPair,
    DescriptorNameCollision,
}

/// The one pre-effect compiler view for a selected S6C callable.
///
/// The installed loan is owned by this value so the envelope, source row,
/// signature, result, and effects cannot be re-paired after the callback.
/// Descriptor rows are mechanical copies of those borrowed facts; no new
/// semantic authority is issued here.
pub(in crate::mir) struct PreparedCanonicalFunctionEntryInputV1<'loan, 'source, 'join> {
    loan: S6CCommonV2PreSessionLoanRefV1<'loan, 'source, 'join>,
    descriptors: Box<[PhysicalCallableParameterDescriptorV1]>,
}

impl<'loan, 'source, 'join>
    PreparedCanonicalFunctionEntryInputV1<'loan, 'source, 'join>
{
    pub(in crate::mir) fn descriptors(&self) -> &[PhysicalCallableParameterDescriptorV1] {
        &self.descriptors
    }

    pub(in crate::mir) fn loan(&self) -> &S6CCommonV2PreSessionLoanRefV1<'loan, 'source, 'join> {
        &self.loan
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        S6CCommonV2PreSessionLoanRefV1<'loan, 'source, 'join>,
        Box<[PhysicalCallableParameterDescriptorV1]>,
    ) {
        (self.loan, self.descriptors)
    }
}

pub(in crate::mir) fn issue_common_v2_physical_function_entry_input<'loan, 'source, 'join>(
    loan: S6CCommonV2PreSessionLoanRefV1<'loan, 'source, 'join>,
) -> Result<
    PreparedCanonicalFunctionEntryInputV1<'loan, 'source, 'join>,
    PhysicalFunctionEntryInputRejectV1,
> {
    let callable = loan.callable();
    let storage = callable.storage_header();
    let signature = callable.signature();
    let descriptors = build_descriptors(
        storage.params(),
        storage.param_decls(),
        signature,
    )?;
    if callable.owner() != signature.owner()
        || callable.physical_effects().owner() != callable.owner()
    {
        return Err(PhysicalFunctionEntryInputRejectV1::BindingOwner);
    }
    Ok(PreparedCanonicalFunctionEntryInputV1 { loan, descriptors })
}

fn build_descriptors(
    params: &[String],
    param_decls: &[ParamDecl],
    signature: PhysicalCallableSignatureRowRefV1<'_>,
) -> Result<Box<[PhysicalCallableParameterDescriptorV1]>, PhysicalFunctionEntryInputRejectV1> {
    if params.len() != param_decls.len()
        || params.len() != usize::try_from(signature.source_logical_arity()).unwrap_or(usize::MAX)
    {
        return Err(PhysicalFunctionEntryInputRejectV1::SourceParameterCoverage);
    }
    let mut source_names = BTreeSet::new();
    for (name, decl) in params.iter().zip(param_decls) {
        if name.is_empty() || decl.name.is_empty() {
            return Err(PhysicalFunctionEntryInputRejectV1::EmptySourceName);
        }
        if name != &decl.name {
            return Err(PhysicalFunctionEntryInputRejectV1::SourceNameMismatch);
        }
        if !source_names.insert(name.as_str()) {
            return Err(PhysicalFunctionEntryInputRejectV1::DuplicateSourceName);
        }
    }

    let lanes = signature.lanes();
    if lanes.len()
        != usize::try_from(signature.physical_callable_lane_count()).unwrap_or(usize::MAX)
    {
        return Err(PhysicalFunctionEntryInputRejectV1::LaneCoverage);
    }
    for (expected, lane) in lanes.iter().enumerate() {
        if lane.index() != u32::try_from(expected).unwrap_or(u32::MAX)
            || lane.binding().owner() != signature.owner()
        {
            return Err(PhysicalFunctionEntryInputRejectV1::LaneIndex);
        }
    }

    let mut descriptors = Vec::with_capacity(lanes.len());
    let mut descriptor_names = BTreeSet::new();
    let mut seen_bindings = BTreeSet::new();
    let mut lane_index = 0usize;
    if signature.receiver_lane_count() == 1 {
        let Some(receiver) = lanes.get(lane_index) else {
            return Err(PhysicalFunctionEntryInputRejectV1::ReceiverPolicy);
        };
        if receiver.role() != PhysicalCallableLaneRoleV1::InstanceReceiver
            || receiver.logical_ordinal().is_some()
        {
            return Err(PhysicalFunctionEntryInputRejectV1::ReceiverPolicy);
        }
        push_descriptor(
            &mut descriptors,
            &mut descriptor_names,
            receiver.index(),
            receiver.role(),
            None,
            receiver.binding(),
            "me".into(),
            None,
            PhysicalCallableLaneCarrierV1::ExistingCallableI64,
        )?;
        if !seen_bindings.insert(receiver.binding()) {
            return Err(PhysicalFunctionEntryInputRejectV1::BindingCoverage);
        }
        lane_index += 1;
    } else if lanes
        .first()
        .is_some_and(|lane| lane.role() == PhysicalCallableLaneRoleV1::InstanceReceiver)
    {
        return Err(PhysicalFunctionEntryInputRejectV1::ReceiverPolicy);
    }

    for (ordinal, decl) in param_decls.iter().enumerate() {
        let ordinal = u32::try_from(ordinal)
            .map_err(|_| PhysicalFunctionEntryInputRejectV1::LogicalOrdinal)?;
        let Some(lane) = lanes.get(lane_index) else {
            return Err(PhysicalFunctionEntryInputRejectV1::LaneCoverage);
        };
        match lane.role() {
            PhysicalCallableLaneRoleV1::OrdinaryScalar => {
                if lane.logical_ordinal() != Some(ordinal)
                    || !seen_bindings.insert(lane.binding())
                {
                    return Err(PhysicalFunctionEntryInputRejectV1::LogicalOrdinal);
                }
                push_descriptor(
                    &mut descriptors,
                    &mut descriptor_names,
                    lane.index(),
                    lane.role(),
                    Some(ordinal),
                    lane.binding(),
                    decl.name.clone().into_boxed_str(),
                    decl.declared_type_name.clone().map(Into::into),
                    PhysicalCallableLaneCarrierV1::ExistingCallableI64,
                )?;
                lane_index += 1;
            }
            PhysicalCallableLaneRoleV1::ExactTextSlot => {
                let Some(generation) = lanes.get(lane_index + 1) else {
                    return Err(PhysicalFunctionEntryInputRejectV1::ExactTextPair);
                };
                if lane.logical_ordinal() != Some(ordinal)
                    || generation.role() != PhysicalCallableLaneRoleV1::ExactTextGeneration
                    || generation.logical_ordinal() != Some(ordinal)
                    || generation.index() != lane.index().saturating_add(1)
                    || generation.binding() != lane.binding()
                    || !seen_bindings.insert(lane.binding())
                {
                    return Err(PhysicalFunctionEntryInputRejectV1::ExactTextPair);
                }
                let slot_name = format!("{}.slot", decl.name).into_boxed_str();
                let generation_name = format!("{}.generation", decl.name).into_boxed_str();
                let source_type = decl.declared_type_name.clone().map(Into::into);
                push_descriptor(
                    &mut descriptors,
                    &mut descriptor_names,
                    lane.index(),
                    lane.role(),
                    Some(ordinal),
                    lane.binding(),
                    slot_name,
                    source_type.clone(),
                    PhysicalCallableLaneCarrierV1::U64BitsOnI64,
                )?;
                push_descriptor(
                    &mut descriptors,
                    &mut descriptor_names,
                    generation.index(),
                    generation.role(),
                    Some(ordinal),
                    generation.binding(),
                    generation_name,
                    source_type,
                    PhysicalCallableLaneCarrierV1::U64BitsOnI64,
                )?;
                lane_index += 2;
            }
            PhysicalCallableLaneRoleV1::InstanceReceiver
            | PhysicalCallableLaneRoleV1::ExactTextGeneration => {
                return Err(PhysicalFunctionEntryInputRejectV1::LogicalOrdinal);
            }
        }
    }
    if lane_index != lanes.len() {
        return Err(PhysicalFunctionEntryInputRejectV1::LaneCoverage);
    }
    Ok(descriptors.into_boxed_slice())
}

fn push_descriptor(
    descriptors: &mut Vec<PhysicalCallableParameterDescriptorV1>,
    names: &mut BTreeSet<String>,
    physical_index: u32,
    role: PhysicalCallableLaneRoleV1,
    logical_ordinal: Option<u32>,
    binding: BindingRefV1,
    diagnostic_name: Box<str>,
    source_declared_type_name: Option<Box<str>>,
    carrier: PhysicalCallableLaneCarrierV1,
) -> Result<(), PhysicalFunctionEntryInputRejectV1> {
    if !names.insert(diagnostic_name.to_string()) {
        return Err(PhysicalFunctionEntryInputRejectV1::DescriptorNameCollision);
    }
    descriptors.push(PhysicalCallableParameterDescriptorV1 {
        physical_index,
        role,
        logical_ordinal,
        binding,
        diagnostic_name,
        source_declared_type_name,
        carrier,
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        issue_common_v2_physical_function_entry_input, PhysicalCallableLaneCarrierV1,
    };
    use crate::mir::builder::CompilationContext;
    use crate::mir::normal_callable_semantic_package::
        issue_normal_callable_semantic_package_v1;
    use crate::mir::normal_callable_semantic_package::PhysicalCallableLaneRoleV1;
    use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
    use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

    fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            source,
            ParserBuildConfig::default(),
        )
        .expect("physical entry source");
        crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
                .expect("source-backed transform");
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
            else {
                panic!("fixture must remain source-backed")
            };
            source
        })
    }

    #[test]
    fn carrier_tags_keep_wire_meaning_out_of_mir_type() {
        assert_ne!(
            PhysicalCallableLaneCarrierV1::ExistingCallableI64,
            PhysicalCallableLaneCarrierV1::U64BitsOnI64
        );
    }

    #[test]
    fn same_cohort_entry_input_keeps_exact_text_lanes_adjacent() {
        let mut resolver = FunctionSemanticResolverSessionV1::new(982).expect("resolver");
        let package = issue_normal_callable_semantic_package_v1(
            &mut resolver,
            final_source(include_str!(
                "../../../apps/tests/scan_with_init_typed_ok_min.hako"
            )),
        )
        .expect("same-cohort package");
        let mut context = CompilationContext::new();
        let installed = package
            .prepare_install(&mut context)
            .expect("vacant catalog")
            .commit();
        let mut port = installed.begin_lowering(&context).expect("same catalog");

        port.with_s6c_common_v2_pre_session(|loan| {
            let prepared = issue_common_v2_physical_function_entry_input(loan)
                .expect("physical entry input");
            let descriptors = prepared.descriptors();
            assert_eq!(descriptors.len(), 4);
            assert_eq!(
                descriptors
                    .iter()
                    .map(|row| row.diagnostic_name())
                    .collect::<Vec<_>>(),
                vec!["s.slot", "s.generation", "ch.slot", "ch.generation"]
            );
            assert_eq!(
                descriptors
                    .iter()
                    .map(|row| row.physical_index())
                    .collect::<Vec<_>>(),
                vec![0, 1, 2, 3]
            );
            assert_eq!(
                descriptors
                    .iter()
                    .map(|row| row.role())
                    .collect::<Vec<_>>(),
                vec![
                    PhysicalCallableLaneRoleV1::ExactTextSlot,
                    PhysicalCallableLaneRoleV1::ExactTextGeneration,
                    PhysicalCallableLaneRoleV1::ExactTextSlot,
                    PhysicalCallableLaneRoleV1::ExactTextGeneration,
                ]
            );
            assert!(descriptors
                .iter()
                .all(|row| row.carrier() == PhysicalCallableLaneCarrierV1::U64BitsOnI64));
        })
        .expect("one callback-scoped input");
        port.complete().expect("selected child coverage");
    }
}
