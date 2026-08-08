use crate::ast::{
    ASTNode, BoxMethodDeclarationSiteV1, BoxMethodInventoryErrorV1, BoxMethodInventoryV1, Span,
};
use crate::parser::{NyashParser, ParseError};

/// An explicit method remains unpublished until its optional postfix syntax
/// has been consumed. This prevents mutation APIs on the ordered inventory.
pub(crate) struct PendingExplicitMethodV1 {
    name: String,
    declaration: ASTNode,
    diagnostic_span: Span,
}

impl PendingExplicitMethodV1 {
    pub(crate) fn new(name: String, declaration: ASTNode, diagnostic_span: Span) -> Self {
        Self {
            name,
            declaration,
            diagnostic_span,
        }
    }

    pub(crate) fn try_apply_postfix(
        &mut self,
        parser: &mut NyashParser,
    ) -> Result<bool, ParseError> {
        crate::parser::declarations::box_def::members::postfix::try_apply_to_pending_method(
            parser,
            &mut self.declaration,
        )
    }

    pub(crate) fn commit(
        self,
        inventory: &mut BoxMethodInventoryV1,
    ) -> Result<BoxMethodDeclarationSiteV1, ParseError> {
        inventory
            .try_push_explicit_source(self.name, self.declaration, self.diagnostic_span)
            .map_err(map_inventory_error)
    }
}

pub(crate) fn map_inventory_error(error: BoxMethodInventoryErrorV1) -> ParseError {
    match error {
        BoxMethodInventoryErrorV1::DuplicateMethod {
            name,
            first_span,
            duplicate_span,
        } => ParseError::DuplicateBoxMethod {
            name: name.into(),
            first_line: first_span.line,
            first_column: first_span.column,
            duplicate_line: duplicate_span.line,
            duplicate_column: duplicate_span.column,
        },
        other => ParseError::UnexpectedToken {
            found: crate::tokenizer::TokenType::IDENTIFIER("invalid method declaration".to_owned()),
            expected: other.to_string(),
            line: 0,
        },
    }
}

pub(crate) fn commit_pending_method(
    pending: &mut Option<PendingExplicitMethodV1>,
    inventory: &mut BoxMethodInventoryV1,
) -> Result<(), ParseError> {
    if let Some(method) = pending.take() {
        let _ = method.commit(inventory)?;
    }
    Ok(())
}
