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

/// One structural MethodCall argument relation issued by the same Raw source
/// view that owns its parent. Both carrier inputs borrow only that external
/// view, so this product is not self-referential.
#[derive(Debug)]
pub(crate) struct VerifiedRawLocatedCallArgumentV1<'view, 'catalog> {
    parent: RawLocatedMethodCallInputV1<'view, 'catalog>,
    index: u32,
    child: RawLocatedExprInputV1<'view, 'catalog>,
    _seal: RawLocatedCallArgumentSealV1,
}

/// A source-only argument rejection retains its consumed parent. No caller can
/// retry from a bare parent input or reconstruct a child by ordinal.
#[derive(Debug)]
pub(crate) struct RejectedRawLocatedCallArgumentV1<'view, 'catalog> {
    parent: RawLocatedMethodCallInputV1<'view, 'catalog>,
    stage: RawLocatedCallArgumentStageV1,
    cause: RawSourceCursorErrorV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawLocatedCallArgumentStageV1 {
    ParentView,
    ArgumentIndex,
    ChildProjection,
}

#[derive(Debug)]
struct RawCallableSourceViewSealV1(());

#[derive(Debug)]
struct RawLocatedCallArgumentSealV1(());

impl RawCallableSourceViewSealV1 {
    const fn new() -> Self {
        Self(())
    }
}

impl RawLocatedCallArgumentSealV1 {
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

    /// Consume one located MethodCall and derive exactly one argument child
    /// through the shared path/projector authority. This is intentionally not
    /// an accessor returning a bare AST argument.
    pub(crate) fn method_call_argument<'view>(
        &'view self,
        parent: RawLocatedMethodCallInputV1<'view, 'catalog>,
        index: usize,
    ) -> Result<
        VerifiedRawLocatedCallArgumentV1<'view, 'catalog>,
        RejectedRawLocatedCallArgumentV1<'view, 'catalog>,
    > {
        if !ptr::eq(self, parent.view()) {
            return Err(RejectedRawLocatedCallArgumentV1 {
                parent,
                stage: RawLocatedCallArgumentStageV1::ParentView,
                cause: RawSourceCursorErrorV1::ForeignView {
                    caller: self.caller.clone(),
                },
            });
        }

        let index = match self.checked_index(index, "method_call_argument_index") {
            Ok(index) => index,
            Err(cause) => {
                return Err(reject_call_argument(
                    parent,
                    RawLocatedCallArgumentStageV1::ArgumentIndex,
                    cause,
                ))
            }
        };
        let len = match self.checked_index(parent.arguments().len(), "method_call_argument_length")
        {
            Ok(len) => len,
            Err(cause) => {
                return Err(reject_call_argument(
                    parent,
                    RawLocatedCallArgumentStageV1::ArgumentIndex,
                    cause,
                ))
            }
        };
        if index >= len {
            let site = parent.site().clone();
            return Err(reject_call_argument(
                parent,
                RawLocatedCallArgumentStageV1::ArgumentIndex,
                RawSourceCursorErrorV1::MethodCallArgumentIndexOutOfBounds {
                    caller: self.caller.clone(),
                    site,
                    index,
                    len,
                },
            ));
        }

        let child = match self.child_expr(
            parent.site().node(),
            parent.node(),
            ExprChildRoleV1::CallArgument(index),
        ) {
            Ok(child) => child,
            Err(cause) => {
                return Err(reject_call_argument(
                    parent,
                    RawLocatedCallArgumentStageV1::ChildProjection,
                    cause,
                ))
            }
        };
        Ok(VerifiedRawLocatedCallArgumentV1 {
            parent,
            index,
            child,
            _seal: RawLocatedCallArgumentSealV1::new(),
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

impl<'view, 'catalog> VerifiedRawLocatedCallArgumentV1<'view, 'catalog> {
    pub(crate) const fn parent(&self) -> &RawLocatedMethodCallInputV1<'view, 'catalog> {
        &self.parent
    }

    pub(crate) const fn index(&self) -> u32 {
        self.index
    }

    pub(crate) const fn child(&self) -> &RawLocatedExprInputV1<'view, 'catalog> {
        &self.child
    }

    pub(crate) fn discard(self) {}
}

impl<'view, 'catalog> RejectedRawLocatedCallArgumentV1<'view, 'catalog> {
    pub(crate) const fn stage(&self) -> RawLocatedCallArgumentStageV1 {
        self.stage
    }

    pub(crate) const fn cause(&self) -> &RawSourceCursorErrorV1 {
        &self.cause
    }

    pub(crate) fn discard(self) {}
}

fn reject_call_argument<'view, 'catalog>(
    parent: RawLocatedMethodCallInputV1<'view, 'catalog>,
    stage: RawLocatedCallArgumentStageV1,
    cause: RawSourceCursorErrorV1,
) -> RejectedRawLocatedCallArgumentV1<'view, 'catalog> {
    RejectedRawLocatedCallArgumentV1 {
        parent,
        stage,
        cause,
    }
}
