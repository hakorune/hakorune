//! Exact function-only Program header surface.
//!
//! CAT0-S0 validates the whole top-level surface before exposing any row. It
//! deliberately offers no function-body accessor; body resolution starts in
//! MP0 only after the complete callable catalog has been sealed.

use crate::ast::ASTNode;

use super::CallableHeaderSyntaxViewV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SourceCallableDeclarationSiteV1 {
    statement_index: u32,
}

impl SourceCallableDeclarationSiteV1 {
    pub(crate) const fn statement_index(self) -> u32 {
        self.statement_index
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableModuleHeaderSyntaxErrorV1 {
    RootMustBeProgram {
        actual: &'static str,
    },
    EmptyCatalog,
    StatementIndexOverflow {
        index: usize,
    },
    UnsupportedProgramStatement {
        site: SourceCallableDeclarationSiteV1,
        actual: &'static str,
    },
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct LocatedCallableHeaderSyntaxViewV1<'a> {
    site: SourceCallableDeclarationSiteV1,
    header: CallableHeaderSyntaxViewV1<'a>,
}

impl<'a> LocatedCallableHeaderSyntaxViewV1<'a> {
    pub(super) fn from_statement(
        site: SourceCallableDeclarationSiteV1,
        statement: &'a ASTNode,
    ) -> Option<Self> {
        Some(Self {
            site,
            header: CallableHeaderSyntaxViewV1::from_function_ast(statement)?,
        })
    }

    pub(crate) const fn site(self) -> SourceCallableDeclarationSiteV1 {
        self.site
    }

    pub(crate) const fn header(self) -> CallableHeaderSyntaxViewV1<'a> {
        self.header
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CallableModuleHeaderSyntaxViewV1<'a> {
    statements: &'a [ASTNode],
    statement_count: u32,
}

impl<'a> CallableModuleHeaderSyntaxViewV1<'a> {
    pub(crate) fn from_program(
        program: &'a ASTNode,
    ) -> Result<Self, CallableModuleHeaderSyntaxErrorV1> {
        let ASTNode::Program { statements, .. } = program else {
            return Err(CallableModuleHeaderSyntaxErrorV1::RootMustBeProgram {
                actual: program.node_type(),
            });
        };
        if statements.is_empty() {
            return Err(CallableModuleHeaderSyntaxErrorV1::EmptyCatalog);
        }
        let statement_count = u32::try_from(statements.len()).map_err(|_| {
            CallableModuleHeaderSyntaxErrorV1::StatementIndexOverflow {
                index: statements.len() - 1,
            }
        })?;
        for (statement_index, statement) in (0..statement_count).zip(statements) {
            let site = SourceCallableDeclarationSiteV1 { statement_index };
            if CallableHeaderSyntaxViewV1::from_function_ast(statement).is_none() {
                return Err(
                    CallableModuleHeaderSyntaxErrorV1::UnsupportedProgramStatement {
                        site,
                        actual: statement.node_type(),
                    },
                );
            }
        }
        Ok(Self {
            statements,
            statement_count,
        })
    }

    pub(crate) fn declaration_sites(
        self,
    ) -> impl ExactSizeIterator<Item = SourceCallableDeclarationSiteV1> {
        (0..self.statement_count)
            .map(|statement_index| SourceCallableDeclarationSiteV1 { statement_index })
    }

    pub(crate) fn located_header(
        self,
        site: SourceCallableDeclarationSiteV1,
    ) -> Option<LocatedCallableHeaderSyntaxViewV1<'a>> {
        let statement = self.statements.get(site.statement_index() as usize)?;
        LocatedCallableHeaderSyntaxViewV1::from_statement(site, statement)
    }
}
