//! Exact source literal at a Home-tracked return. The physical return value
//! and cleanup remain with the existing terminal and exit owners.
use super::*;

pub(super) fn classify_return_literal(
    input: ResolvedFunctionLoweringInputV1<'_>,
    exit: &SourceStmtSiteV1,
    value: &SourceExprSiteV1,
) -> Option<TerminalRelationV1> {
    let source = input.function().expression_source();
    match source.literal(value) {
        // `return void` is an explicit Unit return in Completion.
        Some(ResolvedLiteralSourceV1::Void) => Some(TerminalRelationV1::Unit(
            TerminalUnitReturnV1::issue(input.owner(), exit.clone()),
        )),
        Some(ResolvedLiteralSourceV1::Integer(number)) => Some(TerminalRelationV1::IntegerLiteral(
            TerminalIntegerLiteralReturnV1::issue(
                input.owner(),
                exit.clone(),
                value.clone(),
                *number,
            ),
        )),
        Some(ResolvedLiteralSourceV1::Bool(value_bool)) => Some(TerminalRelationV1::BoolLiteral(
            TerminalBoolLiteralReturnV1::issue(
                input.owner(),
                exit.clone(),
                value.clone(),
                *value_bool,
            ),
        )),
        _ => source.negative_integer_immediate(value).map(|number| {
            TerminalRelationV1::IntegerLiteral(TerminalIntegerLiteralReturnV1::issue(
                input.owner(),
                exit.clone(),
                value.clone(),
                number,
            ))
        }),
    }
}
