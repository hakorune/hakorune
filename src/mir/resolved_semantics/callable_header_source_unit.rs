//! Program-owned CAT0 header source-unit shell.
//!
//! The owned Program and its verified declaration sites cannot be paired by a
//! caller. CAT0-C0a/C0b will add candidate/header sealing through this shell;
//! CAT0-S0 itself issues no owners and builds no callable index.

use crate::ast::ASTNode;

use super::{
    CallableFunctionSyntaxViewV1, CallableHeaderSyntaxViewV1, CallableModuleHeaderSyntaxErrorV1,
    CallableModuleHeaderSyntaxViewV1, LocatedCallableHeaderSyntaxViewV1,
    SourceCallableDeclarationSiteV1,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct EmbeddedCallableFunctionSyntaxViewV1<'a> {
    function_ast: &'a ASTNode,
    function: CallableFunctionSyntaxViewV1<'a>,
}

impl<'a> EmbeddedCallableFunctionSyntaxViewV1<'a> {
    pub(crate) const fn function_ast(self) -> &'a ASTNode {
        self.function_ast
    }

    pub(crate) const fn function(self) -> CallableFunctionSyntaxViewV1<'a> {
        self.function
    }
}

#[derive(Debug)]
struct CanonicalProgramSyntaxOwnerV1 {
    program: ASTNode,
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableHeaderSourceUnitV1 {
    syntax: CanonicalProgramSyntaxOwnerV1,
    declaration_sites: Box<[SourceCallableDeclarationSiteV1]>,
}

impl VerifiedCallableHeaderSourceUnitV1 {
    pub(crate) fn seal_header_surface(
        program: ASTNode,
    ) -> Result<Self, CallableModuleHeaderSyntaxErrorV1> {
        let view = CallableModuleHeaderSyntaxViewV1::from_program(&program)?;
        let declaration_sites = view
            .declaration_sites()
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self::seal_exact_sites(program, declaration_sites)
    }

    pub(crate) fn seal_exact_sites(
        program: ASTNode,
        declaration_sites: Box<[SourceCallableDeclarationSiteV1]>,
    ) -> Result<Self, CallableModuleHeaderSyntaxErrorV1> {
        Self::validate_exact_sites(&program, &declaration_sites)?;
        let mut declaration_sites = declaration_sites.into_vec();
        declaration_sites.sort_unstable();
        Ok(Self {
            syntax: CanonicalProgramSyntaxOwnerV1 { program },
            declaration_sites: declaration_sites.into_boxed_slice(),
        })
    }

    pub(crate) fn validate_exact_sites(
        program: &ASTNode,
        declaration_sites: &[SourceCallableDeclarationSiteV1],
    ) -> Result<(), CallableModuleHeaderSyntaxErrorV1> {
        let ASTNode::Program { statements, .. } = program else {
            return Err(CallableModuleHeaderSyntaxErrorV1::RootMustBeProgram {
                actual: program.node_type(),
            });
        };
        if declaration_sites.is_empty() {
            return Err(CallableModuleHeaderSyntaxErrorV1::EmptyCatalog);
        }
        let mut declaration_sites = declaration_sites.to_vec();
        declaration_sites.sort_unstable();
        for sites in declaration_sites.windows(2) {
            if sites[0] == sites[1] {
                return Err(
                    CallableModuleHeaderSyntaxErrorV1::DuplicateDeclarationSite { site: sites[0] },
                );
            }
        }
        for &site in &declaration_sites {
            let Some(statement) = statements.get(site.statement_index() as usize) else {
                return Err(CallableModuleHeaderSyntaxErrorV1::MissingProgramStatement { site });
            };
            if CallableHeaderSyntaxViewV1::from_function_ast(statement).is_none() {
                return Err(
                    CallableModuleHeaderSyntaxErrorV1::UnsupportedProgramStatement {
                        site,
                        actual: statement.node_type(),
                    },
                );
            }
        }
        Ok(())
    }

    pub(crate) fn declaration_sites(&self) -> &[SourceCallableDeclarationSiteV1] {
        &self.declaration_sites
    }

    pub(crate) fn located_header(
        &self,
        site: SourceCallableDeclarationSiteV1,
    ) -> Option<LocatedCallableHeaderSyntaxViewV1<'_>> {
        let ASTNode::Program { statements, .. } = &self.syntax.program else {
            return None;
        };
        let statement = statements.get(site.statement_index() as usize)?;
        LocatedCallableHeaderSyntaxViewV1::from_statement(site, statement)
    }

    pub(in crate::mir) fn embedded_function(
        &self,
        statement_index: usize,
        method_key: &str,
    ) -> Option<EmbeddedCallableFunctionSyntaxViewV1<'_>> {
        let ASTNode::Program { statements, .. } = &self.syntax.program else {
            return None;
        };
        let ASTNode::BoxDeclaration { methods, .. } = statements.get(statement_index)? else {
            return None;
        };
        let function_ast = methods.get(method_key)?;
        let function = CallableFunctionSyntaxViewV1::from_function_ast(function_ast)?;
        Some(EmbeddedCallableFunctionSyntaxViewV1 {
            function_ast,
            function,
        })
    }

    pub(super) fn function_ast(&self, site: SourceCallableDeclarationSiteV1) -> Option<&ASTNode> {
        let ASTNode::Program { statements, .. } = &self.syntax.program else {
            return None;
        };
        let statement = statements.get(site.statement_index() as usize)?;
        matches!(statement, ASTNode::FunctionDeclaration { .. }).then_some(statement)
    }
}
