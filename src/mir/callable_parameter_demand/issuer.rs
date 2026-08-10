use std::collections::BTreeSet;

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1,
    HomeDemandV1, ReceiverPolicyV1, ResolveOwnerForestErrorV1,
    ResolveSelectedCallableForestsOutcomeV1, SemanticOwnerRootProfileV1, SourceBindingSiteV1,
    VerifiedSemanticOwnerForestV1,
};
use crate::parser::{
    ParsedProgramWithCallableParameterSourceV1, ParserCallableParameterSourceCatalogV1,
    ParserCallableSyntaxLoanErrorV1,
};

use super::model::{
    CallableParameterDeclarationModeV1, VerifiedCallableParameterDemandCatalogV1,
    VerifiedCallableParameterDemandDeclarationV1, VerifiedCallableParameterDemandV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableParameterDemandIssueV1 {
    ParserSyntax(ParserCallableSyntaxLoanErrorV1),
    SourceLoanCoverage,
    Resolver(ResolveOwnerForestErrorV1),
    ResolverDeferred,
    DeclarationCoverage,
    MissingRoot,
    RootProfileMismatch,
    DuplicateOwner,
    MissingParameterBinding { declaration: u32, parameter: u32 },
    ForeignParameterBinding { declaration: u32, parameter: u32 },
    ParameterRecordMismatch { declaration: u32, parameter: u32 },
    DuplicateParameterBinding { declaration: u32, parameter: u32 },
    UnsupportedTransfer { declaration: u32, parameter: u32 },
}

pub(crate) fn issue_callable_parameter_demands_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    parsed: ParsedProgramWithCallableParameterSourceV1,
) -> Result<VerifiedCallableParameterDemandCatalogV1, CallableParameterDemandIssueV1> {
    parsed
        .with_callable_declaration_syntax(|catalog, loan| {
            let mut views = Vec::with_capacity(loan.declarations().len());
            for (expected, syntax) in loan.declarations().iter().enumerate() {
                let expected_row = u32::try_from(expected)
                    .map_err(|_| CallableParameterDemandIssueV1::SourceLoanCoverage)?;
                if syntax.source_row_index() != expected_row {
                    return Err(CallableParameterDemandIssueV1::SourceLoanCoverage);
                }
                let source = catalog
                    .declarations()
                    .get(expected as usize)
                    .ok_or(CallableParameterDemandIssueV1::SourceLoanCoverage)?;
                let ASTNode::FunctionDeclaration { params, body, .. } = syntax.declaration() else {
                    return Err(CallableParameterDemandIssueV1::SourceLoanCoverage);
                };
                let receiver = if source.is_static() {
                    ReceiverPolicyV1::StaticCurrentOwner
                } else {
                    ReceiverPolicyV1::DeclaredInstance
                };
                views.push(FunctionSyntaxViewV1::from_borrowed_function_parts(
                    params, body, receiver,
                ));
            }
            let forests = match resolver
                .resolve_selected_callable_forests(&views)
                .map_err(CallableParameterDemandIssueV1::Resolver)?
            {
                ResolveSelectedCallableForestsOutcomeV1::Complete(forests) => forests,
                ResolveSelectedCallableForestsOutcomeV1::Deferred => {
                    return Err(CallableParameterDemandIssueV1::ResolverDeferred)
                }
            };
            seal_demands(catalog, forests)
        })
        .map_err(CallableParameterDemandIssueV1::ParserSyntax)?
}

fn seal_demands(
    source: ParserCallableParameterSourceCatalogV1,
    forests: Box<[VerifiedSemanticOwnerForestV1]>,
) -> Result<VerifiedCallableParameterDemandCatalogV1, CallableParameterDemandIssueV1> {
    if source.declarations().len() != forests.len() {
        return Err(CallableParameterDemandIssueV1::DeclarationCoverage);
    }
    let mut owners = BTreeSet::new();
    let mut bindings = BTreeSet::new();
    let mut declarations = Vec::with_capacity(forests.len());
    for (index, (source_row, forest)) in
        source.declarations().iter().zip(forests.iter()).enumerate()
    {
        let declaration = u32::try_from(index)
            .map_err(|_| CallableParameterDemandIssueV1::DeclarationCoverage)?;
        let [owner] = forest.roots() else {
            return Err(CallableParameterDemandIssueV1::MissingRoot);
        };
        let function = forest
            .owner(*owner)
            .ok_or(CallableParameterDemandIssueV1::MissingRoot)?;
        let expected_receiver = if source_row.is_static() {
            ReceiverPolicyV1::StaticCurrentOwner
        } else {
            ReceiverPolicyV1::DeclaredInstance
        };
        if !matches!(
            function.root_profile(),
            SemanticOwnerRootProfileV1::DeclaredFunction { receiver_policy }
                if receiver_policy == expected_receiver
        ) {
            return Err(CallableParameterDemandIssueV1::RootProfileMismatch);
        }
        if !owners.insert(*owner) {
            return Err(CallableParameterDemandIssueV1::DuplicateOwner);
        }
        let actual_parameter_count = function
            .declaration_sites()
            .filter(|site| matches!(site, SourceBindingSiteV1::Parameter { .. }))
            .count();
        if actual_parameter_count != source_row.parameters().len() {
            return Err(CallableParameterDemandIssueV1::DeclarationCoverage);
        }
        let mut parameters = Vec::with_capacity(source_row.parameters().len());
        for source_parameter in source_row.parameters() {
            let parameter = source_parameter.ordinal();
            if !source_parameter.transfer().is_ordinary() {
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
            if binding.owner() != *owner {
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
            *owner,
            function.function_origin(),
            if source_row.is_static() {
                CallableParameterDeclarationModeV1::StaticBoxMethod
            } else {
                CallableParameterDeclarationModeV1::InstanceBoxMethod
            },
            parameters.into_boxed_slice(),
        ));
    }
    Ok(VerifiedCallableParameterDemandCatalogV1::new(
        source,
        forests,
        declarations.into_boxed_slice(),
    ))
}
