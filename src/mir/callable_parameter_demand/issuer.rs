use std::collections::BTreeSet;

use crate::mir::callable_semantic_batch::{
    ResolvedCallableDeclarationModeV1, ResolvedCallableSemanticBatchLoanErrorV1,
    VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, HomeDemandV1, SourceBindingSiteV1,
};

use super::model::{
    CallableParameterDeclarationModeV1, VerifiedCallableParameterDemandCatalogV1,
    VerifiedCallableParameterDemandDeclarationV1, VerifiedCallableParameterDemandV1,
};

#[derive(Debug)]
pub(crate) enum CallableParameterDemandIssueV1 {
    BatchLoan(ResolvedCallableSemanticBatchLoanErrorV1),
    DeclarationCoverage,
    MissingParameterBinding { declaration: u32, parameter: u32 },
    ForeignParameterBinding { declaration: u32, parameter: u32 },
    ParameterRecordMismatch { declaration: u32, parameter: u32 },
    DuplicateParameterBinding { declaration: u32, parameter: u32 },
    UnsupportedTransfer { declaration: u32, parameter: u32 },
}

pub(crate) fn issue_callable_parameter_demands_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
) -> Result<VerifiedCallableParameterDemandCatalogV1<'_>, CallableParameterDemandIssueV1> {
    let declarations = batch
        .with_declaration_semantics(|semantic| {
            let mut bindings = BTreeSet::new();
            let mut declarations = Vec::with_capacity(semantic.declarations().len());
            for row in semantic.declarations() {
                let declaration = row.source_row_index();
                let function = row.function();
                let actual_parameter_count = function
                    .declaration_sites()
                    .filter(|site| matches!(site, SourceBindingSiteV1::Parameter { .. }))
                    .count();
                if actual_parameter_count != row.parameters().len() {
                    return Err(CallableParameterDemandIssueV1::DeclarationCoverage);
                }
                let mut parameters = Vec::with_capacity(row.parameters().len());
                for source_parameter in row.parameters() {
                    let parameter = source_parameter.ordinal();
                    if !source_parameter.is_ordinary() {
                        return Err(CallableParameterDemandIssueV1::UnsupportedTransfer {
                            declaration,
                            parameter,
                        });
                    }
                    let site = SourceBindingSiteV1::Parameter { index: parameter };
                    let binding = function.declaration_binding(&site).ok_or(
                        CallableParameterDemandIssueV1::MissingParameterBinding {
                            declaration,
                            parameter,
                        },
                    )?;
                    if binding.owner() != row.owner() {
                        return Err(CallableParameterDemandIssueV1::ForeignParameterBinding {
                            declaration,
                            parameter,
                        });
                    }
                    let record = function.binding(binding).ok_or(
                        CallableParameterDemandIssueV1::ForeignParameterBinding {
                            declaration,
                            parameter,
                        },
                    )?;
                    if record.kind() != (BindingKindV1::Parameter { index: parameter })
                        || record.origin() != &BindingOriginV1::Source(site)
                        || record.diagnostic_name() != source_parameter.name()
                    {
                        return Err(CallableParameterDemandIssueV1::ParameterRecordMismatch {
                            declaration,
                            parameter,
                        });
                    }
                    if !bindings.insert(binding) {
                        return Err(CallableParameterDemandIssueV1::DuplicateParameterBinding {
                            declaration,
                            parameter,
                        });
                    }
                    parameters.push(VerifiedCallableParameterDemandV1::new(
                        parameter,
                        binding,
                        HomeDemandV1::Handle,
                    ));
                }
                declarations.push(VerifiedCallableParameterDemandDeclarationV1::new(
                    declaration,
                    row.owner(),
                    row.function_origin(),
                    match row.mode() {
                        ResolvedCallableDeclarationModeV1::StaticBoxMethod => {
                            CallableParameterDeclarationModeV1::StaticBoxMethod
                        }
                        ResolvedCallableDeclarationModeV1::InstanceBoxMethod => {
                            CallableParameterDeclarationModeV1::InstanceBoxMethod
                        }
                    },
                    parameters.into_boxed_slice(),
                ));
            }
            Ok(declarations.into_boxed_slice())
        })
        .map_err(CallableParameterDemandIssueV1::BatchLoan)??;

    Ok(VerifiedCallableParameterDemandCatalogV1::new(
        batch,
        declarations,
    ))
}
