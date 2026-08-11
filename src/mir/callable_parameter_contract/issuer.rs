use std::collections::BTreeSet;

use crate::mir::callable_semantic_batch::{
    ResolvedCallableDeclarationModeV1, ResolvedCallableSemanticBatchLoanErrorV1,
    VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, SourceBindingSiteV1,
};

use super::model::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
    VerifiedCallableParameterContractCatalogV1,
    VerifiedCallableParameterContractDeclarationV1, VerifiedCallableParameterContractV1,
};

#[derive(Debug)]
pub(crate) enum CallableParameterContractIssueV1 {
    BatchLoan(ResolvedCallableSemanticBatchLoanErrorV1),
    DeclarationCoverage,
    MissingParameterBinding { declaration: u32, parameter: u32 },
    ForeignParameterBinding { declaration: u32, parameter: u32 },
    ParameterRecordMismatch { declaration: u32, parameter: u32 },
    DuplicateParameterBinding { declaration: u32, parameter: u32 },
    UnsupportedTransfer { declaration: u32, parameter: u32 },
    UnsupportedDeclaredType { declaration: u32, parameter: u32 },
}

pub(crate) fn issue_callable_parameter_contract_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
) -> Result<VerifiedCallableParameterContractCatalogV1<'_>, CallableParameterContractIssueV1> {
    let declarations = batch
        .with_declaration_semantics(|semantic| {
            let mut bindings = BTreeSet::new();
            let mut declarations = Vec::with_capacity(semantic.declarations().len());
            for row in semantic.declarations() {
                let Some(source_parameters) = row.parameters() else {
                    continue;
                };
                let declaration = row.batch_slot();
                let function = row.function();
                let actual_parameter_count = function
                    .declaration_sites()
                    .filter(|site| matches!(site, SourceBindingSiteV1::Parameter { .. }))
                    .count();
                if actual_parameter_count != source_parameters.len() {
                    return Err(CallableParameterContractIssueV1::DeclarationCoverage);
                }
                let mut parameters = Vec::with_capacity(source_parameters.len());
                for source_parameter in source_parameters {
                    let parameter = source_parameter.ordinal();
                    if !source_parameter.is_ordinary() {
                        return Err(CallableParameterContractIssueV1::UnsupportedTransfer {
                            declaration,
                            parameter,
                        });
                    }
                    let kind = match source_parameter.declared_type_name() {
                        None => CallableParameterContractKindV1::OpaqueHandle,
                        Some(source_type) => {
                            let Some(abi) = ExactTrivialParameterAbiV1::classify(source_type)
                            else {
                                return Err(
                                    CallableParameterContractIssueV1::UnsupportedDeclaredType {
                                        declaration,
                                        parameter,
                                    },
                                );
                            };
                            CallableParameterContractKindV1::ExactTrivial(abi)
                        }
                    };
                    let site = SourceBindingSiteV1::Parameter { index: parameter };
                    let binding = function.declaration_binding(&site).ok_or(
                        CallableParameterContractIssueV1::MissingParameterBinding {
                            declaration,
                            parameter,
                        },
                    )?;
                    if binding.owner() != row.owner() {
                        return Err(
                            CallableParameterContractIssueV1::ForeignParameterBinding {
                                declaration,
                                parameter,
                            },
                        );
                    }
                    let record = function.binding(binding).ok_or(
                        CallableParameterContractIssueV1::ForeignParameterBinding {
                            declaration,
                            parameter,
                        },
                    )?;
                    if record.kind() != (BindingKindV1::Parameter { index: parameter })
                        || record.origin() != &BindingOriginV1::Source(site)
                        || record.diagnostic_name() != source_parameter.name()
                    {
                        return Err(CallableParameterContractIssueV1::ParameterRecordMismatch {
                            declaration,
                            parameter,
                        });
                    }
                    if !bindings.insert(binding) {
                        return Err(CallableParameterContractIssueV1::DuplicateParameterBinding {
                            declaration,
                            parameter,
                        });
                    }
                    parameters.push(VerifiedCallableParameterContractV1::new(
                        parameter, binding, kind,
                    ));
                }
                declarations.push(VerifiedCallableParameterContractDeclarationV1::new(
                    declaration,
                    row.owner(),
                    row.function_origin(),
                    match row.mode() {
                        ResolvedCallableDeclarationModeV1::TopLevel => {
                            return Err(CallableParameterContractIssueV1::DeclarationCoverage)
                        }
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
        .map_err(CallableParameterContractIssueV1::BatchLoan)??;

    Ok(VerifiedCallableParameterContractCatalogV1::new(
        batch,
        declarations,
    ))
}
