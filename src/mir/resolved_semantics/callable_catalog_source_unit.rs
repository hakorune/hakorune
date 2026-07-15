//! Program-owned CAT0 header source-unit shell.
//!
//! The owned Program and its verified declaration sites cannot be paired by a
//! caller. CAT0-C0a/C0b will add candidate/header sealing through this shell;
//! CAT0-S0 itself issues no owners and builds no callable index.

use crate::ast::ASTNode;

use super::{
    CallableModuleHeaderSyntaxErrorV1, CallableModuleHeaderSyntaxViewV1,
    LocatedCallableHeaderSyntaxViewV1, SourceCallableDeclarationSiteV1,
};

#[derive(Debug)]
struct CanonicalProgramSyntaxOwnerV1 {
    program: ASTNode,
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableCatalogSourceUnitV1 {
    syntax: CanonicalProgramSyntaxOwnerV1,
    declaration_sites: Box<[SourceCallableDeclarationSiteV1]>,
}

impl VerifiedCallableCatalogSourceUnitV1 {
    pub(crate) fn seal_header_surface(
        program: ASTNode,
    ) -> Result<Self, CallableModuleHeaderSyntaxErrorV1> {
        let view = CallableModuleHeaderSyntaxViewV1::from_program(&program)?;
        let declaration_sites = view
            .declaration_sites()
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Ok(Self {
            syntax: CanonicalProgramSyntaxOwnerV1 { program },
            declaration_sites,
        })
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
}
