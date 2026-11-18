//! CHILDREN0 source-bound static helper work.
//!

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};

use super::raw_source_projection::RawSourceLocatorV1;


#[derive(Debug)]
pub(in crate::mir) struct RawRootStaticChildWorkV1 {
    ordinal: usize,
    locator: RawSourceLocatorV1,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
    declaration: ASTNode,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RawRootStaticChildLoweringPartsV1 {
    pub(super) function_name: String,
    pub(super) params: Vec<String>,
    pub(super) param_decls: Vec<ParamDecl>,
    pub(super) return_type_name: Option<String>,
    pub(super) body: Vec<ASTNode>,
    pub(super) uses: Vec<String>,
    pub(super) attrs: DeclarationAttrs,
    pub(super) declaration: ASTNode,
}

impl RawRootStaticChildWorkV1 {
    pub(in crate::mir::builder) fn into_source_parts(
        self,
    ) -> (usize, RawSourceLocatorV1, RawRootStaticChildLoweringPartsV1) {
        let Self {
            ordinal,
            locator,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
            declaration,
        } = self;
        let function_name = locator.symbol().to_owned();
        (
            ordinal,
            locator,
            RawRootStaticChildLoweringPartsV1 {
                function_name,
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
                declaration,
            },
        )
    }

}

#[derive(Debug)]
pub(in crate::mir) struct RawCallableMainWorkV1 {
    inner: RawRootStaticChildWorkV1,
}

impl RawCallableMainWorkV1 {
    pub(in crate::mir::builder) fn into_inner(self) -> RawRootStaticChildWorkV1 {
        self.inner
    }
}
