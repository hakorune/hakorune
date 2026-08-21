use crate::ast::ASTNode;

use super::super::composite_source::{
    parser_composite_source_loan_from_statements, ParserCompositeSourceLoanRejectV1,
    ParserCompositeSourceLoanV1, ParserCompositeSourceUnavailableV1,
};
use super::model::{
    ParserNormalProgramBodySourceRowV1, ParserNormalProgramSourceAuthorityDispositionV1,
    ParserNormalProgramSourceAuthorityIncompleteV1,
    ParserNormalProgramSourceAuthorityIntegrityIssueV1,
    ParserNormalProgramSourceAuthorityUnavailableV1, ParserNormalProgramSourceAuthorityV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalProgramSourceLoanRejectV1 {
    SourceAuthorityUnavailable(ParserNormalProgramSourceAuthorityUnavailableV1),
    Incomplete(ParserNormalProgramSourceAuthorityIncompleteV1),
    IntegrityInvalid(ParserNormalProgramSourceAuthorityIntegrityIssueV1),
}

#[derive(Debug)]
pub(crate) struct ParserNormalProgramSourceLoanV1<'source> {
    authority: &'source ParserNormalProgramSourceAuthorityV1,
    statements: &'source [ASTNode],
}

#[derive(Debug)]
pub(crate) struct ParserNormalProgramStatementCursorV1<'source> {
    rows: std::slice::Iter<'source, ParserNormalProgramBodySourceRowV1>,
    statements: std::slice::Iter<'source, ASTNode>,
}

#[derive(Debug)]
pub(crate) struct ParserNormalProgramStatementLoanV1<'source> {
    row: &'source ParserNormalProgramBodySourceRowV1,
    statement: &'source ASTNode,
}

pub(crate) fn with_parser_normal_program_source_loan<R>(
    disposition: &ParserNormalProgramSourceAuthorityDispositionV1,
    ast: &ASTNode,
    callback: impl for<'source> FnOnce(ParserNormalProgramSourceLoanV1<'source>) -> R,
) -> Result<R, ParserNormalProgramSourceLoanRejectV1> {
    let ParserNormalProgramSourceAuthorityDispositionV1::Ready(authority) = disposition else {
        return Err(ParserNormalProgramSourceLoanRejectV1::from_disposition(disposition));
    };
    let ASTNode::Program { statements, .. } = ast else {
        return Err(ParserNormalProgramSourceLoanRejectV1::Incomplete(
            ParserNormalProgramSourceAuthorityIncompleteV1::ProgramBodyMissing,
        ));
    };
    if authority.body_rows().len() != statements.len() {
        return Err(ParserNormalProgramSourceLoanRejectV1::IntegrityInvalid(
            ParserNormalProgramSourceAuthorityIntegrityIssueV1::BodyCoverageMismatch,
        ));
    }
    for (row, statement) in authority.body_rows().iter().zip(statements) {
        if row.kind() != super::issuer::parser_program_body_syntax_kind(statement) {
            return Err(ParserNormalProgramSourceLoanRejectV1::IntegrityInvalid(
                ParserNormalProgramSourceAuthorityIntegrityIssueV1::BodyKindMismatch,
            ));
        }
    }
    Ok(callback(ParserNormalProgramSourceLoanV1 {
        authority,
        statements,
    }))
}

pub(crate) fn with_parser_composite_source_loan_from_normal_authority<R>(
    disposition: &ParserNormalProgramSourceAuthorityDispositionV1,
    ast: &ASTNode,
    callback: impl for<'source> FnOnce(ParserCompositeSourceLoanV1<'source>) -> R,
) -> Result<R, ParserCompositeSourceLoanRejectV1> {
    let ParserNormalProgramSourceAuthorityDispositionV1::Ready(authority) = disposition else {
        return Err(match disposition {
            ParserNormalProgramSourceAuthorityDispositionV1::Ready(_) => unreachable!(),
            ParserNormalProgramSourceAuthorityDispositionV1::SourceAuthorityUnavailable(reason) => {
                ParserCompositeSourceLoanRejectV1::SourceAuthorityUnavailable(match reason {
                    super::model::ParserNormalProgramSourceAuthorityUnavailableV1::PostpassNotSourceBacked => {
                        ParserCompositeSourceUnavailableV1::PostpassNotSourceBacked
                    }
                    super::model::ParserNormalProgramSourceAuthorityUnavailableV1::ParameterSourceUnavailable => {
                        ParserCompositeSourceUnavailableV1::ParameterSourceUnavailable
                    }
                })
            }
            ParserNormalProgramSourceAuthorityDispositionV1::Incomplete(_) => {
                ParserCompositeSourceLoanRejectV1::Incomplete(
                    super::super::composite_source::ParserCompositeIncompleteV1::ProgramBodyMissing,
                )
            }
            ParserNormalProgramSourceAuthorityDispositionV1::IntegrityInvalid(_) => {
                ParserCompositeSourceLoanRejectV1::IntegrityInvalid(
                    super::super::composite_source::ParserCompositeIntegrityIssueV1::CallTreeContradiction,
                )
            }
        });
    };
    let ASTNode::Program { statements, .. } = ast else {
        return Err(ParserCompositeSourceLoanRejectV1::Incomplete(
            super::super::composite_source::ParserCompositeIncompleteV1::ProgramBodyMissing,
        ));
    };
    let loan = parser_composite_source_loan_from_statements(
        authority.composite_source(),
        statements,
    )?;
    Ok(callback(loan))
}

impl ParserNormalProgramSourceLoanRejectV1 {
    fn from_disposition(
        disposition: &ParserNormalProgramSourceAuthorityDispositionV1,
    ) -> Self {
        match disposition {
            ParserNormalProgramSourceAuthorityDispositionV1::Ready(_) => {
                unreachable!("ready source authority must enter its scoped loan")
            }
            ParserNormalProgramSourceAuthorityDispositionV1::SourceAuthorityUnavailable(reason) => {
                Self::SourceAuthorityUnavailable(*reason)
            }
            ParserNormalProgramSourceAuthorityDispositionV1::Incomplete(reason) => {
                Self::Incomplete(*reason)
            }
            ParserNormalProgramSourceAuthorityDispositionV1::IntegrityInvalid(reason) => {
                Self::IntegrityInvalid(*reason)
            }
        }
    }
}

impl<'source> ParserNormalProgramSourceLoanV1<'source> {
    pub(crate) fn invocation_witness(
        &self,
    ) -> &crate::parser::ParserInvocationWitnessV1 {
        self.authority.invocation_witness()
    }

    pub(crate) fn statement_count(&self) -> usize {
        self.statements.len()
    }

    pub(crate) fn statements(&self) -> ParserNormalProgramStatementCursorV1<'source> {
        ParserNormalProgramStatementCursorV1 {
            rows: self.authority.body_rows().iter(),
            statements: self.statements.iter(),
        }
    }

    pub(crate) fn composite_loan(
        &self,
    ) -> Result<ParserCompositeSourceLoanV1<'source>, ParserCompositeSourceLoanRejectV1> {
        parser_composite_source_loan_from_statements(
            self.authority.composite_source(),
            self.statements,
        )
    }
}

impl<'source> Iterator for ParserNormalProgramStatementCursorV1<'source> {
    type Item = ParserNormalProgramStatementLoanV1<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(ParserNormalProgramStatementLoanV1 {
            row: self.rows.next()?,
            statement: self.statements.next()?,
        })
    }
}

impl<'source> ParserNormalProgramStatementLoanV1<'source> {
    pub(crate) const fn position(&self) -> u32 {
        self.row.position()
    }

    pub(crate) fn statement(&self) -> &'source ASTNode {
        self.statement
    }
}
