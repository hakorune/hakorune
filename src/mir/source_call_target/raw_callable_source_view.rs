//! Catalog-branded structural source navigation for the Raw callable route.
//!
//! This is not a second navigation engine. All child-role selection, path
//! construction, and projection delegate to `resolved_semantics` SSOTs.

use std::ptr;

use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
    VerifiedSameModuleCallableDeclarationV1,
};
use crate::mir::resolved_semantics::{
    is_statement_expression_surface_v1, project_source_body_node_v1, BodyChildRoleV1,
    ExprChildRoleV1, ProjectedSourceNodeV1, SourceBodyKindV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathV1, SourceStmtSiteV1,
};

use super::RawSourceCursorErrorV1;

#[derive(Debug)]
pub(crate) struct VerifiedRawCallableSourceViewV1<'catalog> {
    catalog: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    caller: &'catalog CanonicalSameModuleCallableKeyV1,
    declaration: &'catalog VerifiedSameModuleCallableDeclarationV1,
    _seal: RawCallableSourceViewSealV1,
}

#[derive(Debug)]
pub(crate) struct RawLocatedBodyInputV1<'view, 'catalog> {
    view: &'view VerifiedRawCallableSourceViewV1<'catalog>,
    root: Option<SourceNodeSiteV1>,
    kind: SourceBodyKindV1,
    statements: &'catalog [ASTNode],
}

#[derive(Debug)]
pub(crate) struct RawLocatedStmtInputV1<'view, 'catalog> {
    view: &'view VerifiedRawCallableSourceViewV1<'catalog>,
    site: SourceStmtSiteV1,
    node: &'catalog ASTNode,
}

#[derive(Debug)]
pub(crate) struct RawLocatedExprInputV1<'view, 'catalog> {
    view: &'view VerifiedRawCallableSourceViewV1<'catalog>,
    site: SourceExprSiteV1,
    node: &'catalog ASTNode,
}

/// One exact MethodCall syntax input borrowed from the catalog-backed Raw
/// cursor. This is source-only: it carries neither lowering state nor a
/// physical result destination.
#[derive(Debug)]
pub(crate) struct RawLocatedMethodCallInputV1<'view, 'catalog> {
    view: &'view VerifiedRawCallableSourceViewV1<'catalog>,
    site: SourceExprSiteV1,
    node: &'catalog ASTNode,
}

#[derive(Debug)]
struct RawCallableSourceViewSealV1(());

impl RawCallableSourceViewSealV1 {
    const fn new() -> Self {
        Self(())
    }
}

impl<'catalog> VerifiedRawCallableSourceViewV1<'catalog> {
    /// Borrows one exact declaration row from the provided catalog allocation.
    ///
    /// The supplied key is lookup-only. The view retains the key embedded in
    /// the catalog row, preventing an equal-looking external key from becoming
    /// authority for later co-seals.
    pub(crate) fn verify(
        catalog: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
        caller: &CanonicalSameModuleCallableKeyV1,
    ) -> Result<Self, RawSourceCursorErrorV1> {
        let Some(declaration) = catalog.declaration(caller) else {
            return Err(RawSourceCursorErrorV1::CallerOutsideCatalog {
                caller: caller.clone(),
            });
        };
        Ok(Self {
            catalog,
            caller: declaration.key(),
            declaration,
            _seal: RawCallableSourceViewSealV1::new(),
        })
    }

    pub(crate) const fn catalog(&self) -> &'catalog VerifiedSameModuleCallableDeclarationCatalogV1 {
        self.catalog
    }

    pub(crate) const fn caller(&self) -> &'catalog CanonicalSameModuleCallableKeyV1 {
        self.caller
    }

    pub(crate) const fn declaration(&self) -> &'catalog VerifiedSameModuleCallableDeclarationV1 {
        self.declaration
    }

    pub(crate) fn root_body(&self) -> RawLocatedBodyInputV1<'_, 'catalog> {
        RawLocatedBodyInputV1 {
            view: self,
            root: None,
            kind: SourceBodyKindV1::Function,
            statements: self.declaration.body(),
        }
    }

    pub(crate) fn body_stmt<'view>(
        &'view self,
        body: &RawLocatedBodyInputV1<'view, 'catalog>,
        index: usize,
    ) -> Result<RawLocatedStmtInputV1<'view, 'catalog>, RawSourceCursorErrorV1> {
        self.require_view(body.view)?;
        let index = self.checked_index(index, "body_statement_index")?;
        let len = self.checked_index(body.statements.len(), "body_statement_length")?;
        if index >= len {
            return Err(RawSourceCursorErrorV1::BodyIndexOutOfBounds {
                caller: self.caller.clone(),
                index,
                len,
            });
        }
        let site = match &body.root {
            Some(root) => SourcePathV1::from_node(root)
                .child(body.kind.item_segment(index))
                .stmt(),
            None => SourcePathV1::root_body(index as usize).stmt(),
        };
        let node = self.project_node(site.node())?;
        Ok(RawLocatedStmtInputV1 {
            view: self,
            site,
            node,
        })
    }

    pub(crate) fn statement_expression<'view>(
        &'view self,
        statement: &RawLocatedStmtInputV1<'view, 'catalog>,
    ) -> Result<RawLocatedExprInputV1<'view, 'catalog>, RawSourceCursorErrorV1> {
        self.require_view(statement.view)?;
        if !is_statement_expression_surface_v1(statement.node) {
            return Err(RawSourceCursorErrorV1::StatementExpressionRequired {
                caller: self.caller.clone(),
                site: statement.site.clone(),
            });
        }
        Ok(RawLocatedExprInputV1 {
            view: self,
            site: SourceExprSiteV1::from_node(statement.site.node().clone()),
            node: statement.node,
        })
    }

    pub(crate) fn child_expr_from_stmt<'view>(
        &'view self,
        parent: &RawLocatedStmtInputV1<'view, 'catalog>,
        role: ExprChildRoleV1,
    ) -> Result<RawLocatedExprInputV1<'view, 'catalog>, RawSourceCursorErrorV1> {
        self.require_view(parent.view)?;
        self.child_expr(parent.site.node(), parent.node, role)
    }

    pub(crate) fn child_expr_from_expr<'view>(
        &'view self,
        parent: &RawLocatedExprInputV1<'view, 'catalog>,
        role: ExprChildRoleV1,
    ) -> Result<RawLocatedExprInputV1<'view, 'catalog>, RawSourceCursorErrorV1> {
        self.require_view(parent.view)?;
        self.child_expr(parent.site.node(), parent.node, role)
    }

    pub(crate) fn child_body_from_stmt<'view>(
        &'view self,
        parent: &RawLocatedStmtInputV1<'view, 'catalog>,
        role: BodyChildRoleV1,
    ) -> Result<RawLocatedBodyInputV1<'view, 'catalog>, RawSourceCursorErrorV1> {
        self.require_view(parent.view)?;
        self.child_body(parent.site.node(), parent.node, role)
    }

    pub(crate) fn child_body_from_expr<'view>(
        &'view self,
        parent: &RawLocatedExprInputV1<'view, 'catalog>,
        role: BodyChildRoleV1,
    ) -> Result<RawLocatedBodyInputV1<'view, 'catalog>, RawSourceCursorErrorV1> {
        self.require_view(parent.view)?;
        self.child_body(parent.site.node(), parent.node, role)
    }

    /// Seal an exact borrowed MethodCall from one located expression input.
    ///
    /// The view identity check prevents an equal-looking expression produced
    /// by another declaration-catalog allocation from becoming this Raw
    /// route's authority.
    pub(crate) fn method_call_input<'view>(
        &'view self,
        expression: &RawLocatedExprInputV1<'view, 'catalog>,
    ) -> Result<RawLocatedMethodCallInputV1<'view, 'catalog>, RawSourceCursorErrorV1> {
        self.require_view(expression.view())?;
        if !matches!(expression.node(), ASTNode::MethodCall { .. }) {
            return Err(RawSourceCursorErrorV1::MethodCallRequired {
                caller: self.caller.clone(),
                site: expression.site().clone(),
            });
        }
        Ok(RawLocatedMethodCallInputV1 {
            view: self,
            site: expression.site().clone(),
            node: expression.node(),
        })
    }

    fn child_expr<'view>(
        &'view self,
        parent_site: &SourceNodeSiteV1,
        parent_node: &ASTNode,
        role: ExprChildRoleV1,
    ) -> Result<RawLocatedExprInputV1<'view, 'catalog>, RawSourceCursorErrorV1> {
        let Some(segment) = role.segment_for(parent_node) else {
            return Err(RawSourceCursorErrorV1::ExpressionRoleParentMismatch {
                caller: self.caller.clone(),
                site: parent_site.clone(),
            });
        };
        let site = SourcePathV1::from_node(parent_site).child(segment).expr();
        let node = self.project_node(site.node())?;
        Ok(RawLocatedExprInputV1 {
            view: self,
            site,
            node,
        })
    }

    fn project_node(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Result<&'catalog ASTNode, RawSourceCursorErrorV1> {
        match project_source_body_node_v1(self.declaration.body(), site) {
            Some(ProjectedSourceNodeV1::Node(node)) => Ok(node),
            _ => Err(RawSourceCursorErrorV1::ProjectionExpectedNode {
                caller: self.caller.clone(),
                site: site.clone(),
            }),
        }
    }

    fn child_body<'view>(
        &'view self,
        parent_site: &SourceNodeSiteV1,
        parent_node: &ASTNode,
        role: BodyChildRoleV1,
    ) -> Result<RawLocatedBodyInputV1<'view, 'catalog>, RawSourceCursorErrorV1> {
        let Some(kind) = role.kind_for(parent_node) else {
            return Err(RawSourceCursorErrorV1::BodyRoleParentMismatch {
                caller: self.caller.clone(),
                site: parent_site.clone(),
            });
        };
        let Some(root_segment) = kind.root_segment() else {
            return Err(RawSourceCursorErrorV1::BodyRoleParentMismatch {
                caller: self.caller.clone(),
                site: parent_site.clone(),
            });
        };
        let body_root = SourcePathV1::from_node(parent_site)
            .child(root_segment)
            .node();
        let statements = match project_source_body_node_v1(self.declaration.body(), &body_root) {
            Some(ProjectedSourceNodeV1::Body(statements)) => statements,
            _ => {
                return Err(RawSourceCursorErrorV1::ProjectionExpectedBody {
                    caller: self.caller.clone(),
                    site: body_root,
                })
            }
        };
        Ok(RawLocatedBodyInputV1 {
            view: self,
            // Item sites are projected from the parent node with the existing
            // `SourceBodyKindV1` vocabulary. The `*BodyRoot` path above only
            // verifies the body slice; it is not an item-site prefix.
            root: Some(parent_site.clone()),
            kind,
            statements,
        })
    }

    fn require_view(
        &self,
        actual: &VerifiedRawCallableSourceViewV1<'catalog>,
    ) -> Result<(), RawSourceCursorErrorV1> {
        if ptr::eq(self, actual) {
            Ok(())
        } else {
            Err(RawSourceCursorErrorV1::ForeignView {
                caller: actual.caller().clone(),
            })
        }
    }

    fn checked_index(
        &self,
        value: usize,
        role: &'static str,
    ) -> Result<u32, RawSourceCursorErrorV1> {
        u32::try_from(value).map_err(|_| RawSourceCursorErrorV1::SourceIndexOverflow {
            caller: self.caller.clone(),
            value,
            role,
        })
    }
}

impl<'view, 'catalog> RawLocatedBodyInputV1<'view, 'catalog> {
    pub(crate) const fn view(&self) -> &'view VerifiedRawCallableSourceViewV1<'catalog> {
        self.view
    }

    pub(crate) const fn statements(&self) -> &'catalog [ASTNode] {
        self.statements
    }
}

impl<'view, 'catalog> RawLocatedStmtInputV1<'view, 'catalog> {
    pub(crate) const fn view(&self) -> &'view VerifiedRawCallableSourceViewV1<'catalog> {
        self.view
    }

    pub(crate) const fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(crate) const fn node(&self) -> &'catalog ASTNode {
        self.node
    }
}

impl<'view, 'catalog> RawLocatedExprInputV1<'view, 'catalog> {
    pub(crate) const fn view(&self) -> &'view VerifiedRawCallableSourceViewV1<'catalog> {
        self.view
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn node(&self) -> &'catalog ASTNode {
        self.node
    }
}

impl<'view, 'catalog> RawLocatedMethodCallInputV1<'view, 'catalog> {
    pub(crate) const fn view(&self) -> &'view VerifiedRawCallableSourceViewV1<'catalog> {
        self.view
    }

    pub(crate) const fn caller(&self) -> &'catalog CanonicalSameModuleCallableKeyV1 {
        self.view.caller()
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn node(&self) -> &'catalog ASTNode {
        self.node
    }

    pub(crate) fn receiver(&self) -> &'catalog ASTNode {
        let ASTNode::MethodCall { object, .. } = self.node else {
            unreachable!("RawLocatedMethodCallInputV1 seals only MethodCall syntax")
        };
        object
    }

    pub(crate) fn method(&self) -> &'catalog str {
        let ASTNode::MethodCall { method, .. } = self.node else {
            unreachable!("RawLocatedMethodCallInputV1 seals only MethodCall syntax")
        };
        method
    }

    pub(crate) fn arguments(&self) -> &'catalog [ASTNode] {
        let ASTNode::MethodCall { arguments, .. } = self.node else {
            unreachable!("RawLocatedMethodCallInputV1 seals only MethodCall syntax")
        };
        arguments
    }
}
