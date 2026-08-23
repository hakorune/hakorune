use crate::ast::{ASTNode, BoxMethodSourceSelectionV1};

use super::super::catalog::{
    ParserCallableParameterSourceCatalogV1, ParserCallableParameterSourceDispositionV1,
};
use super::super::model::ParserCallableDeclarationKindV1;
use super::super::parser_invocation_witness::ParserInvocationWitnessV1;
use super::model::{
    ParserCompositeArgumentV1, ParserCompositeCallResultV1, ParserCompositeIncompleteV1,
    ParserCompositeIntegrityIssueV1, ParserCompositeOutsideReasonV1, ParserCompositeReceiverV1,
    ParserCompositeResultSyntaxV1, ParserCompositeRootMethodCallV1, ParserCompositeRootTerminalV1,
    ParserCompositeSourceDispositionV1, ParserCompositeSourcePreservationV1,
    ParserCompositeSourceUnavailableV1, ParserCompositeStaticProviderV1,
};
use crate::parser::callable_source_anchor::{
    DirectCallableDeclarationKindV1, PreparedCallableSourceV1,
};
use crate::parser::postpass_envelope::CompletedParserPostpassV1;
use crate::parser::source_path::SourceProgramCallablePathV1;

/// The sole issuer for the first parser-owned composite source token.
///
/// This issuer only co-seals source presence. It never resolves the receiver,
/// selects a target, or emits candidate/A/C/Recipe/physical meaning.
pub(crate) fn issue_parser_composite_source_v1(
    completed: &CompletedParserPostpassV1,
    parameter_source: &ParserCallableParameterSourceDispositionV1,
) -> ParserCompositeSourceDispositionV1 {
    if !completed.is_source_backed() {
        return ParserCompositeSourceDispositionV1::SourceAuthorityUnavailable(
            ParserCompositeSourceUnavailableV1::PostpassNotSourceBacked,
        );
    }
    let ParserCallableParameterSourceDispositionV1::Complete(catalog) = parameter_source else {
        return ParserCompositeSourceDispositionV1::SourceAuthorityUnavailable(
            ParserCompositeSourceUnavailableV1::ParameterSourceUnavailable,
        );
    };

    let Some(provider) = select_provider(catalog) else {
        return if catalog.declarations().is_empty() {
            ParserCompositeSourceDispositionV1::OutsideBoundedCohort(
                ParserCompositeOutsideReasonV1::NoStaticProvider,
            )
        } else {
            ParserCompositeSourceDispositionV1::OutsideBoundedCohort(
                ParserCompositeOutsideReasonV1::MultipleCallableProviders,
            )
        };
    };

    let Some(source_row) = find_source_row(completed, provider, catalog) else {
        return ParserCompositeSourceDispositionV1::IntegrityInvalid(
            ParserCompositeIntegrityIssueV1::ProviderAnchorMismatch,
        );
    };

    let ASTNode::Program { statements, .. } = completed.ast() else {
        return ParserCompositeSourceDispositionV1::Incomplete(
            ParserCompositeIncompleteV1::ProgramBodyMissing,
        );
    };
    let statement = provider.box_statement_ordinal();
    let Some(ASTNode::BoxDeclaration {
        name,
        methods,
        is_interface,
        is_record,
        is_sync,
        is_static,
        ..
    }) = statements.get(statement as usize)
    else {
        return ParserCompositeSourceDispositionV1::Incomplete(
            ParserCompositeIncompleteV1::ProviderDeclarationMissing,
        );
    };
    if !*is_static || *is_interface || *is_record || *is_sync || name == "Main" {
        return ParserCompositeSourceDispositionV1::OutsideBoundedCohort(
            ParserCompositeOutsideReasonV1::ProviderOutsideBoundedCohort,
        );
    }
    if methods.len() != 1 {
        return ParserCompositeSourceDispositionV1::OutsideBoundedCohort(
            ParserCompositeOutsideReasonV1::ProviderMethodCountOutsideBoundedCohort,
        );
    }

    let inventory = provider.inventory_ordinal().inventory_ordinal();
    let Some(entry) = methods
        .iter_selected_declaration_order()
        .nth(inventory as usize)
    else {
        return ParserCompositeSourceDispositionV1::IntegrityInvalid(
            ParserCompositeIntegrityIssueV1::ProviderPlacementMismatch,
        );
    };
    if entry.site() != provider.inventory_ordinal()
        || !matches!(
            entry.provenance().explicit_source_selection(),
            Some(BoxMethodSourceSelectionV1::Direct)
        )
    {
        return ParserCompositeSourceDispositionV1::IntegrityInvalid(
            ParserCompositeIntegrityIssueV1::ProviderPlacementMismatch,
        );
    }
    let ASTNode::FunctionDeclaration {
        name: method_name,
        return_type_name,
        is_static: method_is_static,
        ..
    } = entry.declaration()
    else {
        return ParserCompositeSourceDispositionV1::IntegrityInvalid(
            ParserCompositeIntegrityIssueV1::ProviderPlacementMismatch,
        );
    };
    if entry.name() != provider.diagnostic_name()
        || method_name != provider.diagnostic_name()
        || !*method_is_static
    {
        return ParserCompositeSourceDispositionV1::IntegrityInvalid(
            ParserCompositeIntegrityIssueV1::ProviderAnchorMismatch,
        );
    }
    let result_syntax = match return_type_name {
        Some(name) => ParserCompositeResultSyntaxV1::Explicit(name.clone().into_boxed_str()),
        None => ParserCompositeResultSyntaxV1::Implicit,
    };
    let provider = ParserCompositeStaticProviderV1::new(
        statement,
        inventory,
        source_row.anchor().identity(),
        provider.source_site().clone(),
        provider.diagnostic_name(),
        result_syntax,
    );

    let invocation = ParserInvocationWitnessV1::from_brand(catalog.parser_brand());
    let terminal = match issue_root_terminal(statements) {
        Ok(terminal) => terminal,
        Err(disposition) => return disposition,
    };
    ParserCompositeSourceDispositionV1::Ready(ParserCompositeSourcePreservationV1::issue(
        invocation, provider, terminal,
    ))
}

fn select_provider(
    catalog: &ParserCallableParameterSourceCatalogV1,
) -> Option<&super::super::model::ParserCallableParameterDeclarationSourceV1> {
    let mut static_rows = catalog
        .declarations()
        .iter()
        .filter(|row| matches!(row.kind(), ParserCallableDeclarationKindV1::StaticBoxMethod));
    let first = static_rows.next()?;
    if static_rows.next().is_some() {
        return None;
    }
    Some(first)
}

fn find_source_row<'a>(
    completed: &'a CompletedParserPostpassV1,
    provider: &super::super::model::ParserCallableParameterDeclarationSourceV1,
    catalog: &ParserCallableParameterSourceCatalogV1,
) -> Option<&'a PreparedCallableSourceV1> {
    if !provider.source_site().is_direct() {
        return None;
    }
    completed.callable_rows().iter().find(|row| {
        if !catalog.same_parser_brand(row.parser_brand()) {
            return false;
        }
        let Some(direct) = row.direct() else {
            return false;
        };
        if direct.kind() != DirectCallableDeclarationKindV1::StaticBoxMethod {
            return false;
        }
        if !direct
            .anchor()
            .identity()
            .same_as(provider.callable_identity())
        {
            return false;
        }
        let SourceProgramCallablePathV1::BoxMethod {
            declaration,
            gate_path,
            member_ordinal,
        } = direct.path()
        else {
            return false;
        };
        gate_path.is_empty()
            && declaration.compatibility_box_path() == provider.source_site().box_site().path()
            && *member_ordinal == provider.source_member_ordinal()
    })
}

fn issue_root_terminal(
    statements: &[ASTNode],
) -> Result<ParserCompositeRootTerminalV1, ParserCompositeSourceDispositionV1> {
    let Some((statement, last)) = statements.iter().enumerate().next_back() else {
        return Err(ParserCompositeSourceDispositionV1::Incomplete(
            ParserCompositeIncompleteV1::ProgramBodyMissing,
        ));
    };
    let statement = u32::try_from(statement).map_err(|_| {
        ParserCompositeSourceDispositionV1::Incomplete(
            ParserCompositeIncompleteV1::ProgramStatementOrdinalOverflow,
        )
    })?;
    match last {
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => {
            let call = issue_root_call(object, method, arguments)?;
            Ok(ParserCompositeRootTerminalV1::FinalSequence { statement, call })
        }
        ASTNode::Return { value, .. } => {
            let Some(value) = value.as_deref() else {
                return Err(ParserCompositeSourceDispositionV1::Incomplete(
                    ParserCompositeIncompleteV1::RootTerminalValueMissing,
                ));
            };
            let ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            } = value
            else {
                return Err(ParserCompositeSourceDispositionV1::OutsideBoundedCohort(
                    ParserCompositeOutsideReasonV1::TerminalOutsideBoundedCohort,
                ));
            };
            let call = issue_root_call(object, method, arguments)?;
            Ok(ParserCompositeRootTerminalV1::RootReturn { statement, call })
        }
        _ => Err(ParserCompositeSourceDispositionV1::OutsideBoundedCohort(
            ParserCompositeOutsideReasonV1::TerminalOutsideBoundedCohort,
        )),
    }
}

fn issue_root_call(
    object: &ASTNode,
    method: &str,
    arguments: &[ASTNode],
) -> Result<ParserCompositeRootMethodCallV1, ParserCompositeSourceDispositionV1> {
    let ASTNode::Variable { name, .. } = object else {
        return Err(ParserCompositeSourceDispositionV1::OutsideBoundedCohort(
            ParserCompositeOutsideReasonV1::ReceiverOutsideBoundedCohort,
        ));
    };
    let mut rows = Vec::with_capacity(arguments.len());
    for (ordinal, _) in arguments.iter().enumerate() {
        let ordinal = u32::try_from(ordinal).map_err(|_| {
            ParserCompositeSourceDispositionV1::Incomplete(
                ParserCompositeIncompleteV1::ArgumentOrdinalOverflow,
            )
        })?;
        rows.push(ParserCompositeArgumentV1::new(ordinal));
    }
    let call = ParserCompositeRootMethodCallV1::new(
        method,
        ParserCompositeReceiverV1::new(name.as_str()),
        rows.into_boxed_slice(),
    );
    debug_assert_eq!(call.result(), ParserCompositeCallResultV1::ThisMethodCall);
    Ok(call)
}
