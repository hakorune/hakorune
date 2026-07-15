//! Consuming MP0 view that opens function bodies only after CAT0 catalog seal.

use crate::ast::ASTNode;

use super::{
    CallableFunctionSyntaxViewV1, FunctionSyntaxViewV1, SourceCallableDeclarationSiteV1,
    VerifiedCallableCatalogSourceUnitV1, VerifiedCallableCatalogV1,
    VerifiedCallableHeaderSourceUnitV1,
};

#[derive(Debug, Clone, Copy)]
pub(in crate::mir) struct LocatedCallableResolutionViewV1<'a> {
    site: SourceCallableDeclarationSiteV1,
    root: &'a ASTNode,
    function: FunctionSyntaxViewV1<'a>,
}

impl<'a> LocatedCallableResolutionViewV1<'a> {
    pub(in crate::mir) const fn site(self) -> SourceCallableDeclarationSiteV1 {
        self.site
    }

    pub(in crate::mir) const fn root(self) -> &'a ASTNode {
        self.root
    }

    pub(in crate::mir) const fn function(self) -> FunctionSyntaxViewV1<'a> {
        self.function
    }
}

/// The exact Program and catalog remain inseparable while MP0 reads bodies.
#[derive(Debug)]
pub(in crate::mir) struct CallableCatalogResolutionSourceV1 {
    source: VerifiedCallableHeaderSourceUnitV1,
    catalog: VerifiedCallableCatalogV1,
}

impl CallableCatalogResolutionSourceV1 {
    pub(super) fn begin(source_unit: VerifiedCallableCatalogSourceUnitV1) -> Self {
        let (source, catalog) = source_unit.into_resolution_parts();
        Self { source, catalog }
    }

    pub(in crate::mir) fn declaration_sites(&self) -> &[SourceCallableDeclarationSiteV1] {
        self.source.declaration_sites()
    }

    pub(in crate::mir) const fn catalog(&self) -> &VerifiedCallableCatalogV1 {
        &self.catalog
    }

    pub(in crate::mir) fn located_function(
        &self,
        site: SourceCallableDeclarationSiteV1,
    ) -> Option<LocatedCallableResolutionViewV1<'_>> {
        let root = self.source.function_ast(site)?;
        let views = CallableFunctionSyntaxViewV1::from_function_ast(root)?;
        Some(LocatedCallableResolutionViewV1 {
            site,
            root,
            function: views.function(),
        })
    }

    pub(in crate::mir) fn finish(self) -> VerifiedCallableCatalogSourceUnitV1 {
        VerifiedCallableCatalogSourceUnitV1::restore_after_resolution(self.source, self.catalog)
    }
}
