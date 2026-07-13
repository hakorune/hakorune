//! Borrowed nested-owner syntax captured during one owner traversal.

use std::collections::BTreeMap;

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{FunctionSyntaxViewV1, SourceExprSiteV1};

use super::{ShadowBindingOrdinalV0, ShadowScopeIdV0};

/// Construction-only nested-function descriptor.
///
/// The canonical AST remains the sole syntax authority. This descriptor owns
/// no node and cannot fabricate a Lambda view from unrelated slices.
#[derive(Debug)]
pub(crate) struct ShadowLambdaSyntaxV0<'ast> {
    pub(crate) definition_site: SourceExprSiteV1,
    pub(crate) parent_scope: ShadowScopeIdV0,
    pub(crate) visible_bindings: BTreeMap<Box<str>, ShadowBindingOrdinalV0>,
    lambda: &'ast ASTNode,
}

impl<'ast> ShadowLambdaSyntaxV0<'ast> {
    pub(crate) fn new(
        definition_site: SourceExprSiteV1,
        parent_scope: ShadowScopeIdV0,
        visible_bindings: BTreeMap<Box<str>, ShadowBindingOrdinalV0>,
        lambda: &'ast ASTNode,
    ) -> Self {
        debug_assert!(matches!(lambda, ASTNode::Lambda { .. }));
        Self {
            definition_site,
            parent_scope,
            visible_bindings,
            lambda,
        }
    }

    pub(crate) fn syntax_view(&self) -> FunctionSyntaxViewV1<'ast> {
        FunctionSyntaxViewV1::from_lambda_ast(self.lambda).expect("inventory admits only Lambda")
    }
}
