use crate::ast::ASTNode;

use super::model::{
    ParserCompositeResultSyntaxV1, ParserCompositeRootMethodCallV1, ParserCompositeRootTerminalV1,
    ParserCompositeSourceDispositionV1, ParserCompositeSourcePreservationV1,
    ParserCompositeTransformRejectV1,
};

pub(crate) fn validate_parser_composite_transform_v1(
    disposition: ParserCompositeSourceDispositionV1,
    initial: &ASTNode,
    transformed: &ASTNode,
) -> Result<ParserCompositeSourceDispositionV1, ParserCompositeTransformRejectV1> {
    let ParserCompositeSourceDispositionV1::Ready(source) = disposition else {
        return Ok(disposition);
    };
    validate_provider(&source, initial, transformed)?;
    validate_terminal(&source, initial, transformed)?;
    Ok(ParserCompositeSourceDispositionV1::Ready(source))
}

fn validate_provider(
    source: &ParserCompositeSourcePreservationV1,
    initial: &ASTNode,
    transformed: &ASTNode,
) -> Result<(), ParserCompositeTransformRejectV1> {
    let provider = source.provider();
    let initial_method =
        locate_provider_method(initial, provider.statement(), provider.method_inventory())
            .ok_or(ParserCompositeTransformRejectV1::ProviderChanged)?;
    let transformed_method = locate_provider_method(
        transformed,
        provider.statement(),
        provider.method_inventory(),
    )
    .ok_or(ParserCompositeTransformRejectV1::ProviderChanged)?;
    let (initial_name, initial_result, initial_static) =
        provider_shape(initial_method).ok_or(ParserCompositeTransformRejectV1::ProviderChanged)?;
    let (transformed_name, transformed_result, transformed_static) =
        provider_shape(transformed_method)
            .ok_or(ParserCompositeTransformRejectV1::ProviderChanged)?;
    if initial_name != provider.diagnostic_name()
        || transformed_name != initial_name
        || !initial_static
        || !transformed_static
    {
        return Err(ParserCompositeTransformRejectV1::ProviderChanged);
    }
    if !result_syntax_matches(provider.result_syntax(), &initial_result)
        || !result_syntax_matches(provider.result_syntax(), &transformed_result)
    {
        return Err(ParserCompositeTransformRejectV1::ProviderResultChanged);
    }
    Ok(())
}

fn locate_provider_method(
    ast: &ASTNode,
    statement: u32,
    method_inventory: u32,
) -> Option<&ASTNode> {
    let ASTNode::Program { statements, .. } = ast else {
        return None;
    };
    let ASTNode::BoxDeclaration { methods, .. } =
        statements.get(usize::try_from(statement).ok()?)?
    else {
        return None;
    };
    methods
        .iter_selected_declaration_order()
        .nth(usize::try_from(method_inventory).ok()?)
        .map(|entry| entry.declaration())
}

fn provider_shape(ast: &ASTNode) -> Option<(&str, Option<&str>, bool)> {
    let ASTNode::FunctionDeclaration {
        name,
        return_type_name,
        is_static,
        ..
    } = ast
    else {
        return None;
    };
    Some((name, return_type_name.as_deref(), *is_static))
}

fn result_syntax_matches(expected: &ParserCompositeResultSyntaxV1, actual: &Option<&str>) -> bool {
    match (expected, actual) {
        (ParserCompositeResultSyntaxV1::Implicit, None) => true,
        (ParserCompositeResultSyntaxV1::Explicit(expected), Some(actual)) => {
            expected.as_ref() == *actual
        }
        (ParserCompositeResultSyntaxV1::Implicit, Some(_))
        | (ParserCompositeResultSyntaxV1::Explicit(_), None) => false,
    }
}

fn validate_terminal(
    source: &ParserCompositeSourcePreservationV1,
    initial: &ASTNode,
    transformed: &ASTNode,
) -> Result<(), ParserCompositeTransformRejectV1> {
    let terminal = source.terminal();
    let initial_call = locate_terminal_call(initial, terminal)
        .ok_or(ParserCompositeTransformRejectV1::TerminalChanged)?;
    let transformed_call = locate_terminal_call(transformed, terminal)
        .ok_or(ParserCompositeTransformRejectV1::TerminalChanged)?;
    let (initial_method, initial_receiver, initial_arguments) =
        call_shape(initial_call).ok_or(ParserCompositeTransformRejectV1::RootCallChanged)?;
    let (transformed_method, transformed_receiver, transformed_arguments) =
        call_shape(transformed_call).ok_or(ParserCompositeTransformRejectV1::RootCallChanged)?;
    let expected_call = terminal.call();
    if initial_method != expected_call.method() || transformed_method != initial_method {
        return Err(ParserCompositeTransformRejectV1::RootCallChanged);
    }
    if !matches_receiver(expected_call, initial_receiver)
        || initial_receiver != transformed_receiver
    {
        return Err(ParserCompositeTransformRejectV1::ReceiverChanged);
    }
    let expected_arguments = expected_call.arguments();
    if initial_arguments.len() != expected_arguments.len()
        || transformed_arguments.len() != initial_arguments.len()
    {
        return Err(
            ParserCompositeTransformRejectV1::ArgumentCardinalityChanged {
                expected: u32::try_from(expected_arguments.len()).unwrap_or(u32::MAX),
                actual: u32::try_from(transformed_arguments.len()).unwrap_or(u32::MAX),
            },
        );
    }
    for (index, expected) in expected_arguments.iter().enumerate() {
        let actual_ordinal = u32::try_from(index).unwrap_or(u32::MAX);
        if expected.ordinal() != actual_ordinal {
            return Err(ParserCompositeTransformRejectV1::ArgumentOrderChanged {
                ordinal: expected.ordinal(),
            });
        }
        if initial_arguments[index] != transformed_arguments[index] {
            return Err(ParserCompositeTransformRejectV1::ArgumentChanged {
                ordinal: actual_ordinal,
            });
        }
    }
    if expected_call.result() != super::model::ParserCompositeCallResultV1::ThisMethodCall {
        return Err(ParserCompositeTransformRejectV1::ResultChanged);
    }
    Ok(())
}

fn locate_terminal_call<'ast>(
    ast: &'ast ASTNode,
    terminal: &ParserCompositeRootTerminalV1,
) -> Option<&'ast ASTNode> {
    let ASTNode::Program { statements, .. } = ast else {
        return None;
    };
    if statements.len() != usize::try_from(terminal.statement()).ok()?.checked_add(1)? {
        return None;
    }
    let statement = statements.get(usize::try_from(terminal.statement()).ok()?)?;
    match (terminal.is_root_return(), statement) {
        (false, ASTNode::MethodCall { .. }) => Some(statement),
        (
            true,
            ASTNode::Return {
                value: Some(value), ..
            },
        ) => match value.as_ref() {
            ASTNode::MethodCall { .. } => Some(value.as_ref()),
            _ => None,
        },
        (false, ASTNode::Return { .. })
        | (true, ASTNode::MethodCall { .. })
        | (true, ASTNode::Return { value: None, .. })
        | (false, _)
        | (true, _) => None,
    }
}

fn call_shape(call: &ASTNode) -> Option<(&str, &ASTNode, &[ASTNode])> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = call
    else {
        return None;
    };
    Some((method, object.as_ref(), arguments.as_slice()))
}

fn matches_receiver(expected: &ParserCompositeRootMethodCallV1, actual: &ASTNode) -> bool {
    matches!(
        actual,
        ASTNode::Variable { name, .. } if name == expected.receiver().diagnostic_name()
    )
}
