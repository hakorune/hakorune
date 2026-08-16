//! Caller-zero physical function skeleton reservation for common V2.
//!
//! This module consumes the already sealed same-cohort entry input and creates
//! one unpublished `MirFunction` shell.  It does not install the function in a
//! `MirBuilder`, publish a binding, or perform ExactText lane adoption.

use crate::mir::builder::{
    SameModuleCallableNamespaceV1, SelectedNormalCallableKeyV1,
};
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::compiler::common_v2_physical_function_entry_input::{
    PhysicalCallableLaneCarrierV1, PhysicalCallableParameterDescriptorV1,
    PreparedCanonicalFunctionEntryInputV1,
};
use crate::mir::normal_callable_semantic_package::S6CCommonV2PreSessionLoanRefV1;
use crate::mir::{BasicBlockId, FunctionSignature, MirFunction, MirType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PhysicalFunctionSkeletonRejectV1 {
    SelectedStorageKeyMismatch,
    ModeNamespaceMismatch,
    DescriptorCoverage,
    UnsupportedCarrier,
}

/// One unpublished physical function shell plus the descriptor rows that
/// explain its reserved parameter lanes.  The retained loan prevents a later
/// consumer from pairing the shell with a foreign source/header cohort.
pub(crate) struct PreparedPhysicalFunctionSkeletonV1<'loan, 'source, 'join> {
    loan: S6CCommonV2PreSessionLoanRefV1<'loan, 'source, 'join>,
    function: MirFunction,
    descriptors: Box<[PhysicalCallableParameterDescriptorV1]>,
}

impl<'loan, 'source, 'join> PreparedPhysicalFunctionSkeletonV1<'loan, 'source, 'join> {
    pub(crate) fn function(&self) -> &MirFunction {
        &self.function
    }

    pub(crate) fn descriptors(&self) -> &[PhysicalCallableParameterDescriptorV1] {
        &self.descriptors
    }

    pub(crate) fn loan(&self) -> &S6CCommonV2PreSessionLoanRefV1<'loan, 'source, 'join> {
        &self.loan
    }

    /// Move the retained cohort, detached shell, and descriptor rows into the
    /// one adoption transaction. No caller can recover a second shell after
    /// this handoff.
    pub(crate) fn into_parts(
        self,
    ) -> (
        S6CCommonV2PreSessionLoanRefV1<'loan, 'source, 'join>,
        MirFunction,
        Box<[PhysicalCallableParameterDescriptorV1]>,
    ) {
        (self.loan, self.function, self.descriptors)
    }
}

/// Reserve a fresh, unpublished physical skeleton from one accepted entry
/// input.  `MirFunction::new` reserves one mechanical i64 lane per descriptor;
/// ExactText's pair is retained as descriptor metadata until the separate lane
/// adoption slice decides how one logical BindingRef owns the sidecar.
pub(crate) fn reserve_common_v2_physical_function_skeleton<'loan, 'source, 'join>(
    prepared: PreparedCanonicalFunctionEntryInputV1<'loan, 'source, 'join>,
) -> Result<PreparedPhysicalFunctionSkeletonV1<'loan, 'source, 'join>, PhysicalFunctionSkeletonRejectV1>
{
    let (loan, descriptors) = prepared.into_parts();
    let callable = loan.callable();
    let storage = callable.storage_header();
    let signature_row = callable.signature();

    match callable.selected().selected_key() {
        SelectedNormalCallableKeyV1::Cataloged(key) if key == storage.key() => {}
        _ => return Err(PhysicalFunctionSkeletonRejectV1::SelectedStorageKeyMismatch),
    }

    let mode_matches = match (
        signature_row.mode(),
        storage.key().namespace(),
    ) {
        (
            CallableParameterDeclarationModeV1::StaticBoxMethod,
            SameModuleCallableNamespaceV1::StaticBoxMethod,
        )
        | (
            CallableParameterDeclarationModeV1::InstanceBoxMethod,
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
        ) => true,
        _ => false,
    };
    if !mode_matches {
        return Err(PhysicalFunctionSkeletonRejectV1::ModeNamespaceMismatch);
    }

    validate_descriptor_rows(signature_row, &descriptors)?;

    let function = MirFunction::new(
        FunctionSignature {
            name: storage.key().mir_symbol_projection(),
            params: vec![MirType::Integer; descriptors.len()],
            return_type: callable.result().mir_type(),
            effects: callable.physical_effects().effect_mask(),
        },
        // The shell is unpublished; its local entry id is intentionally not
        // allocated from a live Builder/module counter.
        BasicBlockId::new(0),
    );
    let mut function = function;
    function.metadata.declared_param_decls = storage
        .param_decls()
        .iter()
        .map(|decl| crate::mir::function::MirParamDecl {
            name: decl.name.clone(),
            declared_type_name: decl.declared_type_name.clone(),
            implicit_receiver: false,
        })
        .collect();
    function.metadata.declared_return_type_name =
        storage.return_type_name().map(str::to_owned);
    function.metadata.declared_capability_uses = storage.uses().to_vec();
    function.metadata.runes = storage.attrs().runes.clone();

    Ok(PreparedPhysicalFunctionSkeletonV1 {
        loan,
        function,
        descriptors,
    })
}

fn validate_descriptor_rows(
    signature: crate::mir::normal_callable_semantic_package::PhysicalCallableSignatureRowRefV1<'_>,
    descriptors: &[PhysicalCallableParameterDescriptorV1],
) -> Result<(), PhysicalFunctionSkeletonRejectV1> {
    let expected_lane_count = usize::try_from(signature.physical_callable_lane_count())
        .map_err(|_| PhysicalFunctionSkeletonRejectV1::DescriptorCoverage)?;
    if descriptors.len() != expected_lane_count
        || descriptors
            .iter()
            .enumerate()
            .any(|(index, row)| row.physical_index() != u32::try_from(index).unwrap_or(u32::MAX))
    {
        return Err(PhysicalFunctionSkeletonRejectV1::DescriptorCoverage);
    }
    if descriptors.iter().any(|row| {
        !matches!(
            row.carrier(),
            PhysicalCallableLaneCarrierV1::ExistingCallableI64
                | PhysicalCallableLaneCarrierV1::U64BitsOnI64
        )
    }) {
        return Err(PhysicalFunctionSkeletonRejectV1::UnsupportedCarrier);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::reserve_common_v2_physical_function_skeleton;
    use crate::mir::builder::CompilationContext;
    use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
    use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
    use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

    fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            source,
            ParserBuildConfig::default(),
        )
        .expect("physical skeleton source");
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
    fn reserves_four_mechanical_lanes_without_builder_publication() {
        let mut resolver = FunctionSemanticResolverSessionV1::new(983).expect("resolver");
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
            let prepared =
                crate::mir::compiler::common_v2_physical_function_entry_input::issue_common_v2_physical_function_entry_input(loan)
                    .expect("physical entry input");
            let skeleton = reserve_common_v2_physical_function_skeleton(prepared)
                .expect("fresh physical skeleton");
            assert_eq!(skeleton.function().signature.name, "Main.find_ok/2");
            assert_eq!(skeleton.function().signature.params.len(), 4);
            assert_eq!(skeleton.function().params.len(), 4);
            assert_eq!(skeleton.function().params[0].as_u32(), 0);
            assert_eq!(skeleton.function().params[3].as_u32(), 3);
            assert_eq!(skeleton.descriptors().len(), 4);
            assert_eq!(skeleton.function().entry_block, crate::mir::BasicBlockId::new(0));
        })
        .expect("one callback-scoped skeleton");
        port.complete().expect("selected child coverage");
    }

    #[test]
    fn descriptor_count_drift_rejects_before_shell_creation() {
        let mut resolver = FunctionSemanticResolverSessionV1::new(984).expect("resolver");
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
            let prepared =
                crate::mir::compiler::common_v2_physical_function_entry_input::issue_common_v2_physical_function_entry_input(loan)
                    .expect("physical entry input");
            let signature = prepared.loan().callable().signature();
            let mut descriptors = prepared.descriptors().to_vec();
            descriptors.pop();
            assert_eq!(
                super::validate_descriptor_rows(signature, &descriptors),
                Err(super::PhysicalFunctionSkeletonRejectV1::DescriptorCoverage)
            );
        })
        .expect("one callback-scoped input");
        port.complete().expect("selected child coverage");
    }
}
