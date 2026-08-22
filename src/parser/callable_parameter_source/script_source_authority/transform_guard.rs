use crate::ast::ASTNode;

use super::super::composite_source::{
    validate_parser_composite_transform_v1, ParserCompositeTransformRejectV1,
};
use super::issuer::parser_program_body_syntax_kind;
use super::model::{
    ParserNormalProgramSourceAuthorityDispositionV1, ParserNormalProgramSourceAuthorityV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParserNormalProgramSourceTransformRejectV1 {
    InitialProgramMissing,
    TransformedProgramMissing,
    BodyCountChanged,
    BodyKindChanged { position: usize },
    Composite(ParserCompositeTransformRejectV1),
}

pub(crate) fn validate_parser_normal_program_source_transform_v1(
    disposition: ParserNormalProgramSourceAuthorityDispositionV1,
    initial: &ASTNode,
    transformed: &ASTNode,
) -> Result<
    ParserNormalProgramSourceAuthorityDispositionV1,
    ParserNormalProgramSourceTransformRejectV1,
> {
    let ParserNormalProgramSourceAuthorityDispositionV1::Ready(authority) = disposition else {
        return Ok(disposition);
    };
    let ASTNode::Program {
        statements: initial_statements,
        ..
    } = initial
    else {
        return Err(ParserNormalProgramSourceTransformRejectV1::InitialProgramMissing);
    };
    let ASTNode::Program {
        statements: transformed_statements,
        ..
    } = transformed
    else {
        return Err(ParserNormalProgramSourceTransformRejectV1::TransformedProgramMissing);
    };
    if authority.body_rows().len() != initial_statements.len()
        || transformed_statements.len() < authority.body_rows().len()
    {
        return Err(ParserNormalProgramSourceTransformRejectV1::BodyCountChanged);
    }
    for (position, ((row, initial_statement), transformed_statement)) in authority
        .body_rows()
        .iter()
        .zip(initial_statements)
        .zip(transformed_statements)
        .enumerate()
    {
        if row.kind() != parser_program_body_syntax_kind(initial_statement)
            || row.kind() != parser_program_body_syntax_kind(transformed_statement)
        {
            return Err(ParserNormalProgramSourceTransformRejectV1::BodyKindChanged { position });
        }
    }
    let (invocation, body_rows, composite, module_rows) = authority.into_parts();
    let composite = validate_parser_composite_transform_v1(composite, initial, transformed)
        .map_err(ParserNormalProgramSourceTransformRejectV1::Composite)?;
    Ok(ParserNormalProgramSourceAuthorityDispositionV1::Ready(
        ParserNormalProgramSourceAuthorityV1::new(invocation, body_rows, composite, module_rows),
    ))
}
