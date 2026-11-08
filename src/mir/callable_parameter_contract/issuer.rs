use std::collections::BTreeSet;

use crate::mir::callable_semantic_batch::{
    ResolvedCallableDeclarationModeV1, ResolvedCallableSemanticBatchLoanErrorV1,
    VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::exact_text_parameter_abi::ExactTextFormalAbiV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::resolved_semantics::{BindingKindV1, BindingOriginV1, SourceBindingSiteV1};

use super::model::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
    VerifiedCallableParameterContractCatalogV1, VerifiedCallableParameterContractDeclarationV1,
    VerifiedCallableParameterContractV1,
};

#[derive(Debug)]
pub(crate) enum CallableParameterContractIssueV1 {
    BatchLoan {
        _error: ResolvedCallableSemanticBatchLoanErrorV1,
    },
    DeclarationCoverage,
    UnsupportedReceiverTransfer { _declaration: u32 },
    UnsupportedResultType { _declaration: u32 },
    MissingParameterBinding {
        _declaration: u32,
        _parameter: u32,
    },
    ForeignParameterBinding {
        _declaration: u32,
        _parameter: u32,
    },
    ParameterRecordMismatch {
        _declaration: u32,
        _parameter: u32,
    },
    DuplicateParameterBinding {
        _declaration: u32,
        _parameter: u32,
    },
    UnsupportedTransfer {
        _declaration: u32,
        _parameter: u32,
    },
    UnsupportedDeclaredType {
        declaration: u32,
        parameter: u32,
    },
}

fn is_admitted_declared_box_name(source_type: &str) -> bool {
    matches!(source_type, "ArrayBox")
}

pub(crate) fn issue_callable_parameter_contract_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
) -> Result<VerifiedCallableParameterContractCatalogV1<'_>, CallableParameterContractIssueV1> {
    let declarations = batch
        .with_declaration_semantics(|semantic| {
            let mut bindings = BTreeSet::new();
            let mut declarations = Vec::with_capacity(semantic.declarations().len());
            for row in semantic.declarations() {
                if row.receiver_transfer() != crate::parser::callable_parameter_source::ParserTransferKindV1::Ordinary {
                    return Err(CallableParameterContractIssueV1::UnsupportedReceiverTransfer { _declaration: row.batch_slot() });
                }
                if row.declared_result_type().is_some_and(|reference| match reference {
                    crate::ast::TypeReferenceV1::Nullable(inner) => inner.name().is_none_or(|name|
                        !matches!(batch.ordinary_box_coverage().row_for(name), Ok(Some(_)))
                        || crate::mir::exact_trivial_scalar_abi::DeclaredResultKindV1::classify(name).is_some()),
                    _ => false,
                }) {
                    return Err(CallableParameterContractIssueV1::UnsupportedResultType { _declaration: row.batch_slot() });
                }
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
                    let kind = match source_parameter.declared_type_name() {
                        None => CallableParameterContractKindV1::OpaqueHandle,
                        Some(source_type) => {
                            if source_parameter.type_reference().and_then(crate::ast::TypeReferenceV1::name) != Some(source_type) {
                                return Err(CallableParameterContractIssueV1::UnsupportedDeclaredType { declaration, parameter });
                            }
                            // A user-declared box named `String` is the class authority;
                            // the ADR spelling only means Text when no such box exists.
                            let user_declared_string = source_type == "String"
                                && matches!(batch.ordinary_box_coverage().row_for(source_type), Ok(Some(_)));
                            if let Some(abi) = ExactTextFormalAbiV1::classify(source_type)
                                .filter(|_| !user_declared_string)
                            {
                                CallableParameterContractKindV1::ExactText(abi)
                            } else if let Some(abi) =
                                ExactTrivialParameterAbiV1::classify(source_type)
                            {
                                CallableParameterContractKindV1::ExactTrivial(abi)
                            } else if source_type == "MapBox" {
                                CallableParameterContractKindV1::Map
                            } else if is_admitted_declared_box_name(source_type) {
                                CallableParameterContractKindV1::DeclaredHandle
                            } else if matches!(
                                batch.ordinary_box_coverage().row_for(source_type),
                                Ok(Some(_))
                            ) {
                                // An explicit declaration that resolves
                                // to an ordinary box of this package is the
                                // class authority — the resolved identity is
                                // the declaration's own class, not a field
                                // inference. Unresolvable spellings keep the
                                // fail-closed `UnsupportedDeclaredType` edge.
                                CallableParameterContractKindV1::DeclaredObject(
                                    source_type.into(),
                                )
                            } else {
                                return Err(
                                    CallableParameterContractIssueV1::UnsupportedDeclaredType {
                                        declaration,
                                        parameter,
                                    },
                                );
                            }
                        }
                    };
                    let demand = kind.home_demand_for_transfer_v1(source_parameter.transfer())
                        .ok_or(CallableParameterContractIssueV1::UnsupportedTransfer {
                            _declaration: declaration, _parameter: parameter,
                        })?;
                    let site = SourceBindingSiteV1::Parameter { index: parameter };
                    let binding = function.declaration_binding(&site).ok_or(
                        CallableParameterContractIssueV1::MissingParameterBinding {
                            _declaration: declaration,
                            _parameter: parameter,
                        },
                    )?;
                    if binding.owner() != row.owner() {
                        return Err(CallableParameterContractIssueV1::ForeignParameterBinding {
                            _declaration: declaration,
                            _parameter: parameter,
                        });
                    }
                    let record = function.binding(binding).ok_or(
                        CallableParameterContractIssueV1::ForeignParameterBinding {
                            _declaration: declaration,
                            _parameter: parameter,
                        },
                    )?;
                    if record.kind() != (BindingKindV1::Parameter { index: parameter })
                        || record.origin() != &BindingOriginV1::Source(site)
                        || record.diagnostic_name() != source_parameter.name()
                    {
                        return Err(CallableParameterContractIssueV1::ParameterRecordMismatch {
                            _declaration: declaration,
                            _parameter: parameter,
                        });
                    }
                    if !bindings.insert(binding) {
                        return Err(
                            CallableParameterContractIssueV1::DuplicateParameterBinding {
                                _declaration: declaration,
                                _parameter: parameter,
                            },
                        );
                    }
                    parameters.push(VerifiedCallableParameterContractV1::new(
                        parameter, binding, kind, demand,
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
        .map_err(|error| CallableParameterContractIssueV1::BatchLoan { _error: error })??;

    Ok(VerifiedCallableParameterContractCatalogV1::new(
        batch,
        declarations,
    ))
}
