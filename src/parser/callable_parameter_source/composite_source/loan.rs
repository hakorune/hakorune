//! One scoped view of the parser-owned composite source token.
//!
//! The view exposes only the exact Program cursor and parser-issued role
//! evidence needed by source admission. The higher-ranked callback prevents
//! an AST reference from escaping into a derived product.

use crate::ast::ASTNode;

use super::model::{
    ParserCompositeIncompleteV1, ParserCompositeIntegrityIssueV1,
    ParserCompositeOutsideReasonV1, ParserCompositeSourceDispositionV1,
    ParserCompositeSourcePreservationV1, ParserCompositeSourceUnavailableV1,
};
use super::super::parser_invocation_witness::ParserInvocationWitnessV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserCompositeSourceLoanRejectV1 {
    Outside(ParserCompositeOutsideReasonV1),
    SourceAuthorityUnavailable(ParserCompositeSourceUnavailableV1),
    Incomplete(ParserCompositeIncompleteV1),
    IntegrityInvalid(ParserCompositeIntegrityIssueV1),
}

#[derive(Debug)]
pub(crate) struct ParserCompositeSourceLoanV1<'source> {
    source: &'source ParserCompositeSourcePreservationV1,
    statements: &'source [ASTNode],
}

#[derive(Debug)]
pub(crate) struct ParserCompositeProgramItemCursorV1<'source> {
    statements: std::slice::Iter<'source, ASTNode>,
    next_index: usize,
}

#[derive(Debug)]
pub(crate) struct ParserCompositeProgramItemLoanV1<'source> {
    index: usize,
    statement: &'source ASTNode,
}

pub(crate) fn with_parser_composite_source_loan<R>(
    disposition: &ParserCompositeSourceDispositionV1,
    ast: &ASTNode,
    callback: impl for<'source> FnOnce(ParserCompositeSourceLoanV1<'source>) -> R,
) -> Result<R, ParserCompositeSourceLoanRejectV1> {
    let ParserCompositeSourceDispositionV1::Ready(source) = disposition else {
        return Err(ParserCompositeSourceLoanRejectV1::from_disposition(disposition));
    };
    let ASTNode::Program { statements, .. } = ast else {
        return Err(ParserCompositeSourceLoanRejectV1::Incomplete(
            ParserCompositeIncompleteV1::ProgramBodyMissing,
        ));
    };
    Ok(callback(ParserCompositeSourceLoanV1 { source, statements }))
}

impl ParserCompositeSourceLoanRejectV1 {
    fn from_disposition(disposition: &ParserCompositeSourceDispositionV1) -> Self {
        match disposition {
            ParserCompositeSourceDispositionV1::Ready(_) => unreachable!(
                "ready composite source must enter the scoped source loan"
            ),
            ParserCompositeSourceDispositionV1::OutsideBoundedCohort(reason) => {
                Self::Outside(*reason)
            }
            ParserCompositeSourceDispositionV1::SourceAuthorityUnavailable(reason) => {
                Self::SourceAuthorityUnavailable(*reason)
            }
            ParserCompositeSourceDispositionV1::Incomplete(reason) => Self::Incomplete(*reason),
            ParserCompositeSourceDispositionV1::IntegrityInvalid(reason) => {
                Self::IntegrityInvalid(*reason)
            }
        }
    }
}

impl<'source> ParserCompositeSourceLoanV1<'source> {
    pub(crate) fn invocation_witness(&self) -> &ParserInvocationWitnessV1 {
        self.source.invocation()
    }

    pub(crate) fn provider_statement_index(&self) -> usize {
        self.source.provider().statement() as usize
    }

    pub(crate) fn terminal_statement_index(&self) -> usize {
        self.source.terminal().statement() as usize
    }

    pub(crate) fn terminal_is_root_return(&self) -> bool {
        self.source.terminal().is_root_return()
    }

    pub(crate) fn items(&self) -> ParserCompositeProgramItemCursorV1<'source> {
        ParserCompositeProgramItemCursorV1 {
            statements: self.statements.iter(),
            next_index: 0,
        }
    }
}

impl<'source> Iterator for ParserCompositeProgramItemCursorV1<'source> {
    type Item = ParserCompositeProgramItemLoanV1<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        let statement = self.statements.next()?;
        let item = ParserCompositeProgramItemLoanV1 {
            index: self.next_index,
            statement,
        };
        self.next_index += 1;
        Some(item)
    }
}

impl<'source> ParserCompositeProgramItemLoanV1<'source> {
    pub(crate) const fn index(&self) -> usize {
        self.index
    }

    pub(crate) fn statement(&self) -> &'source ASTNode {
        self.statement
    }
}
