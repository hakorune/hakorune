//! Co-seal exact parameter ABI rows before body profile analysis.

use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::resolved_semantics::{BindingKindV1, BindingRefV1, SourceBindingSiteV1};

use super::coverage::{ResolvedFactCoverageDraftV1, TrivialProfileDraftV1};
use super::error::{
    stop, AnalysisResultV1, TrivialProfileContractErrorV1, TrivialProfileStopReasonV1,
    TrivialProfileStopSiteV1,
};
use super::product::TrivialRepresentationV1;

pub(super) fn seal_parameter_entries_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    draft: &mut TrivialProfileDraftV1,
    fact_coverage: &mut ResolvedFactCoverageDraftV1,
) -> AnalysisResultV1<Vec<(BindingRefV1, TrivialRepresentationV1)>> {
    let ASTNode::FunctionDeclaration {
        params,
        param_decls,
        ..
    } = input.source().root()
    else {
        return Err(TrivialProfileContractErrorV1::InvalidFunctionRoot.into());
    };
    if params.is_empty() && param_decls.is_empty() {
        return Ok(Vec::new());
    }
    if !params.is_empty() && param_decls.is_empty() {
        return stop(
            TrivialProfileStopSiteV1::Binding(SourceBindingSiteV1::Parameter { index: 0 }),
            TrivialProfileStopReasonV1::ParameterRepresentationUnavailable,
        );
    }
    if params.len() != param_decls.len()
        || param_decls
            .iter()
            .zip(params)
            .any(|(declaration, name)| declaration.name != *name)
    {
        return Err(TrivialProfileContractErrorV1::ParameterDeclarationShapeMismatch.into());
    }

    let mut entries = Vec::with_capacity(params.len());
    for (index, (source_name, declaration)) in params.iter().zip(param_decls).enumerate() {
        let formal_index = u32::try_from(index)
            .map_err(|_| TrivialProfileContractErrorV1::ParameterDeclarationShapeMismatch)?;
        let site = SourceBindingSiteV1::Parameter {
            index: formal_index,
        };
        let Some(declared_type_name) = declaration.declared_type_name.as_deref() else {
            return stop(
                TrivialProfileStopSiteV1::Binding(site),
                TrivialProfileStopReasonV1::ParameterRepresentationUnavailable,
            );
        };
        let Some(abi) = ExactTrivialParameterAbiV1::classify(declared_type_name) else {
            return stop(
                TrivialProfileStopSiteV1::Binding(site),
                TrivialProfileStopReasonV1::TypedSignatureOutsideProfile,
            );
        };
        let binding = fact_coverage.declaration_binding(input.function(), &site)?;
        let Some(record) = input.function().binding(binding) else {
            return Err(
                TrivialProfileContractErrorV1::ParameterEntryContractMismatch { formal_index }
                    .into(),
            );
        };
        if record.kind()
            != (BindingKindV1::Parameter {
                index: formal_index,
            })
            || record.diagnostic_name() != source_name
        {
            return Err(
                TrivialProfileContractErrorV1::ParameterEntryContractMismatch { formal_index }
                    .into(),
            );
        }
        draft.record_parameter_entry(site, binding, formal_index, source_name.clone(), abi)?;
        entries.push((binding, TrivialRepresentationV1::InlineI64));
    }
    Ok(entries)
}
