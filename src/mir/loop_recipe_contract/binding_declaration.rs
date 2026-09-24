//! Resolver-function declaration lookup shared by Recipe producers.
//!
//! Recipe producers own the Recipe-key vocabulary but not the declaration
//! source table.  This cell is the single mechanical lookup that maps one
//! resolver-issued source binding to its declaration (and initializer when
//! the declaration has one) inside the verified function.  It issues no
//! Receipt, mints no identity, and never falls back to name matching.

use crate::mir::resolved_semantics::{
    BindingRefV1, SourceBindingSiteV1, SourceExprSiteV1, VerifiedResolvedFunctionV1,
};

/// One exact declaration-site pair resolved from the function authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedLoopBindingDeclarationV1 {
    pub(crate) declaration: SourceBindingSiteV1,
    pub(crate) initializer: Option<SourceExprSiteV1>,
}

/// Resolve the declaration site (and initializer site when present) for one
/// resolver-issued source binding inside the verified function.
pub(crate) fn resolve_loop_binding_declaration_v1(
    function: &VerifiedResolvedFunctionV1,
    binding: BindingRefV1,
) -> Option<ResolvedLoopBindingDeclarationV1> {
    let declaration = function
        .declaration_sites()
        .find(|site| function.declaration_binding(site) == Some(binding))?
        .clone();
    let initializer = function
        .expression_source()
        .initializer(&declaration)
        .and_then(|relation| relation.initializer_site().cloned());
    Some(ResolvedLoopBindingDeclarationV1 {
        declaration,
        initializer,
    })
}
