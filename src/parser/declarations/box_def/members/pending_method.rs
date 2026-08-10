use crate::ast::{ASTNode, BoxMethodInventoryErrorV1, BoxMethodInventoryOrdinalV1, Span};
use crate::parser::callable_parameter_source::ParsedCallableParameterListV1;
use crate::parser::source_authority::ExplicitMethodSink;
use crate::parser::{NyashParser, ParseError};

/// An explicit method remains unpublished until its optional postfix syntax
/// has been consumed. This prevents mutation APIs on the ordered inventory.
pub(crate) struct PendingExplicitMethodV1 {
    name: String,
    declaration: ASTNode,
    diagnostic_span: Span,
    parameter_source: Option<ParsedCallableParameterListV1>,
}

impl PendingExplicitMethodV1 {
    pub(crate) fn new(name: String, declaration: ASTNode, diagnostic_span: Span) -> Self {
        Self {
            name,
            declaration,
            diagnostic_span,
            parameter_source: None,
        }
    }

    pub(crate) fn with_parameter_source(
        name: String,
        declaration: ASTNode,
        diagnostic_span: Span,
        parameter_source: ParsedCallableParameterListV1,
    ) -> Self {
        Self {
            name,
            declaration,
            diagnostic_span,
            parameter_source: Some(parameter_source),
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
        sink: &mut impl ExplicitMethodSink,
    ) -> Result<CommittedExplicitMethodV1, ParseError> {
        let diagnostic_name = self.name.clone();
        let inventory_ordinal = sink.commit_explicit_method_at_current(
            self.name,
            self.declaration,
            self.diagnostic_span,
        )?;
        Ok(CommittedExplicitMethodV1 {
            inventory_ordinal,
            diagnostic_name,
            parameter_source: self.parameter_source,
        })
    }
}

pub(crate) struct CommittedExplicitMethodV1 {
    #[allow(dead_code)]
    inventory_ordinal: BoxMethodInventoryOrdinalV1,
    diagnostic_name: String,
    parameter_source: Option<ParsedCallableParameterListV1>,
}

impl CommittedExplicitMethodV1 {
    pub(crate) fn into_parameter_source(
        self,
    ) -> Option<(
        BoxMethodInventoryOrdinalV1,
        String,
        ParsedCallableParameterListV1,
    )> {
        self.parameter_source
            .map(|source| (self.inventory_ordinal, self.diagnostic_name, source))
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
