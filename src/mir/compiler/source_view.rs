//! Immutable function syntax views and exact parent-relative navigation.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, OwnedExprSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourcePathV1, VerifiedSemanticOwnerForestV1,
};

use super::located::{
    LocatedBodySuffixV1, LocatedBodyV1, LocatedExprV1, LocatedStmtV1, SourceBodyKindV1,
    SourceBodySiteV1,
};
use super::lowering_input::VerifiedResolvedSourceUnitV1;
use super::source_projection::{
    ProjectedSourceV1, SourceNavigationErrorV1, VerifiedSourceProjectionV1,
};

/// Unforgeable safe-code capability for constructing located carriers.
///
/// The type is visible to `located.rs`, but its field and constructor remain
/// private to this module. This keeps `FunctionSourceViewV1` as the sole
/// safe-code carrier factory without relying on call-site convention.
#[derive(Debug, Clone, Copy)]
pub(super) struct SourceViewSealV1(());

impl SourceViewSealV1 {
    const fn new() -> Self {
        Self(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExprChildRoleV1 {
    LocalInitializer(u32),
    AssignmentTarget,
    AssignmentValue,
    CompoundAssignmentTarget,
    CompoundAssignmentValue,
    PrintValue,
    NowaitValue,
    ReturnValue,
    UnaryOperand,
    AwaitOperand,
    BinaryLeft,
    BinaryRight,
    IfCondition,
    LoopCondition,
    BlockExprTail,
    Receiver,
    IndexTarget,
    IndexSubscript,
    CallCallee,
    CallArgument(u32),
    ArrayElement(u32),
    MapEntryValue(u32),
    RecordFieldValue(u32),
    RecordUpdateBase,
    RecordUpdateValue(u32),
    CheckItem(u32),
    NewFieldInitializer(u32),
    GroupedAssignmentValue,
}

impl ExprChildRoleV1 {
    fn segment_for(self, parent: &ASTNode) -> Option<SourcePathSegmentV1> {
        let segment = match (self, parent) {
            (Self::LocalInitializer(index), ASTNode::Local { .. }) => {
                SourcePathSegmentV1::Initializer(index)
            }
            (Self::AssignmentTarget, ASTNode::Assignment { .. }) => SourcePathSegmentV1::Target,
            (Self::AssignmentValue, ASTNode::Assignment { .. }) => SourcePathSegmentV1::Value,
            (Self::CompoundAssignmentTarget, ASTNode::CompoundAssignment { .. }) => {
                SourcePathSegmentV1::Target
            }
            (Self::CompoundAssignmentValue, ASTNode::CompoundAssignment { .. }) => {
                SourcePathSegmentV1::Value
            }
            (Self::PrintValue, ASTNode::Print { .. })
            | (Self::NowaitValue, ASTNode::Nowait { .. })
            | (Self::ReturnValue, ASTNode::Return { .. }) => SourcePathSegmentV1::Value,
            (Self::UnaryOperand, ASTNode::UnaryOp { .. })
            | (Self::AwaitOperand, ASTNode::AwaitExpression { .. }) => SourcePathSegmentV1::Operand,
            (Self::BinaryLeft, ASTNode::BinaryOp { .. }) => SourcePathSegmentV1::Lhs,
            (Self::BinaryRight, ASTNode::BinaryOp { .. }) => SourcePathSegmentV1::Rhs,
            (Self::IfCondition, ASTNode::If { .. }) => SourcePathSegmentV1::IfCondition,
            (Self::LoopCondition, ASTNode::Loop { .. }) => SourcePathSegmentV1::LoopCondition,
            (Self::BlockExprTail, ASTNode::BlockExpr { .. }) => SourcePathSegmentV1::BlockExprTail,
            (Self::Receiver, ASTNode::MethodCall { .. } | ASTNode::FieldAccess { .. }) => {
                SourcePathSegmentV1::Receiver
            }
            (Self::IndexTarget, ASTNode::Index { .. }) => SourcePathSegmentV1::Target,
            (Self::IndexSubscript, ASTNode::Index { .. }) => SourcePathSegmentV1::Argument(0),
            (Self::CallCallee, ASTNode::Call { .. }) => SourcePathSegmentV1::Callee,
            (
                Self::CallArgument(index),
                ASTNode::MethodCall { .. }
                | ASTNode::FunctionCall { .. }
                | ASTNode::FromCall { .. }
                | ASTNode::Call { .. }
                | ASTNode::New { .. },
            ) => SourcePathSegmentV1::Argument(index),
            (Self::ArrayElement(index), ASTNode::ArrayLiteral { .. }) => {
                SourcePathSegmentV1::Element(index)
            }
            (Self::MapEntryValue(index), ASTNode::MapLiteral { .. }) => {
                SourcePathSegmentV1::EntryValue(index)
            }
            (Self::RecordFieldValue(index), ASTNode::RecordLiteral { .. }) => {
                SourcePathSegmentV1::FieldValue(index)
            }
            (Self::RecordUpdateBase, ASTNode::RecordUpdate { .. }) => SourcePathSegmentV1::Base,
            (Self::RecordUpdateValue(index), ASTNode::RecordUpdate { .. }) => {
                SourcePathSegmentV1::UpdateValue(index)
            }
            (Self::CheckItem(index), ASTNode::CheckExpr { .. }) => {
                SourcePathSegmentV1::CheckItem(index)
            }
            (Self::NewFieldInitializer(index), ASTNode::New { .. }) => {
                SourcePathSegmentV1::Initializer(index)
            }
            (Self::GroupedAssignmentValue, ASTNode::GroupedAssignmentExpr { .. }) => {
                SourcePathSegmentV1::Value
            }
            _ => return None,
        };
        Some(segment)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BodyChildRoleV1 {
    FunctionBody,
    LambdaBody,
    ScopeBody,
    TaskScopeBody,
    FastMemBody,
    IfThen,
    IfElse,
    LoopBody,
    BlockExprPrelude,
}

impl BodyChildRoleV1 {
    fn kind_for(self, parent: &ASTNode) -> Option<SourceBodyKindV1> {
        match (self, parent) {
            (Self::FunctionBody, ASTNode::FunctionDeclaration { .. }) => {
                Some(SourceBodyKindV1::Function)
            }
            (Self::LambdaBody, ASTNode::Lambda { .. }) => Some(SourceBodyKindV1::Lambda),
            (Self::ScopeBody, ASTNode::ScopeBox { .. }) => Some(SourceBodyKindV1::Scope),
            (Self::TaskScopeBody, ASTNode::TaskScope { .. }) => Some(SourceBodyKindV1::TaskScope),
            (Self::FastMemBody, ASTNode::FastMemRegion { .. }) => Some(SourceBodyKindV1::FastMem),
            (Self::IfThen, ASTNode::If { .. }) => Some(SourceBodyKindV1::IfThen),
            (Self::IfElse, ASTNode::If { .. }) => Some(SourceBodyKindV1::IfElse),
            (Self::LoopBody, ASTNode::Loop { .. }) => Some(SourceBodyKindV1::Loop),
            (Self::BlockExprPrelude, ASTNode::BlockExpr { .. }) => {
                Some(SourceBodyKindV1::BlockExprPrelude)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FunctionSourceViewV1<'a> {
    owner: FunctionOwnerIdV1,
    unit_syntax_root: &'a ASTNode,
    owner_root: &'a ASTNode,
    forest: &'a VerifiedSemanticOwnerForestV1,
    projection: &'a VerifiedSourceProjectionV1,
}

impl VerifiedResolvedSourceUnitV1 {
    pub(crate) fn function_source_view(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<FunctionSourceViewV1<'_>, SourceNavigationErrorV1> {
        FunctionSourceViewV1::from_unit(self, owner)
    }
}

impl<'a> FunctionSourceViewV1<'a> {
    fn from_unit(
        unit: &'a VerifiedResolvedSourceUnitV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Self, SourceNavigationErrorV1> {
        if unit.forest().owner(owner).is_none() {
            return Err(SourceNavigationErrorV1::UnknownOwner(owner));
        }
        let owner_root = unit.projection().owner_root(unit.syntax_root(), owner)?;
        Ok(Self {
            owner,
            unit_syntax_root: unit.syntax_root(),
            owner_root,
            forest: unit.forest(),
            projection: unit.projection(),
        })
    }

    pub(crate) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn root(self) -> &'a ASTNode {
        self.owner_root
    }

    pub(crate) fn root_body(self) -> Result<LocatedBodyV1<'a>, SourceNavigationErrorV1> {
        let kind = BodyChildRoleV1::FunctionBody
            .kind_for(self.owner_root)
            .or_else(|| BodyChildRoleV1::LambdaBody.kind_for(self.owner_root))
            .ok_or_else(|| self.invalid_root_site("owner_root_has_no_body"))?;
        let seal = SourceViewSealV1::new();
        let site = SourceBodySiteV1::new_root(self.owner, kind, seal);
        let body = self.project_body(&site.root())?;
        Ok(LocatedBodyV1::new(site, body, seal))
    }

    pub(crate) fn body_stmt(
        self,
        body: &LocatedBodyV1<'a>,
        index: usize,
    ) -> Result<LocatedStmtV1<'a>, SourceNavigationErrorV1> {
        self.require_owner(body.site().owner())?;
        let len = body.statements().len();
        if index >= len {
            return Err(SourceNavigationErrorV1::BodyIndexOutOfBounds {
                owner: self.owner,
                body: body.site().root(),
                index: index as u32,
                len: len as u32,
            });
        }
        let seal = SourceViewSealV1::new();
        let site = body.site().statement(index as u32, seal);
        let node = self.project_node(site.node())?;
        Ok(LocatedStmtV1::new(self.owner, site, node, seal))
    }

    pub(crate) fn child_expr_from_stmt(
        self,
        parent: &LocatedStmtV1<'a>,
        role: ExprChildRoleV1,
    ) -> Result<LocatedExprV1<'a>, SourceNavigationErrorV1> {
        self.child_expr(parent.owner(), parent.site().node(), parent.node(), role)
    }

    pub(crate) fn child_expr_from_expr(
        self,
        parent: &LocatedExprV1<'a>,
        role: ExprChildRoleV1,
    ) -> Result<LocatedExprV1<'a>, SourceNavigationErrorV1> {
        self.child_expr(parent.owner(), parent.site().node(), parent.node(), role)
    }

    pub(crate) fn child_body_from_stmt(
        self,
        parent: &LocatedStmtV1<'a>,
        role: BodyChildRoleV1,
    ) -> Result<LocatedBodyV1<'a>, SourceNavigationErrorV1> {
        self.child_body(parent.owner(), parent.site().node(), parent.node(), role)
    }

    pub(crate) fn child_body_from_expr(
        self,
        parent: &LocatedExprV1<'a>,
        role: BodyChildRoleV1,
    ) -> Result<LocatedBodyV1<'a>, SourceNavigationErrorV1> {
        self.child_body(parent.owner(), parent.site().node(), parent.node(), role)
    }

    pub(crate) fn body_suffix(
        self,
        body: LocatedBodyV1<'a>,
        start_index: usize,
    ) -> Result<LocatedBodySuffixV1<'a>, SourceNavigationErrorV1> {
        self.require_owner(body.site().owner())?;
        let len = body.statements().len();
        if start_index > len {
            return Err(SourceNavigationErrorV1::SuffixStartOutOfBounds {
                owner: self.owner,
                body: body.site().root(),
                start: start_index as u32,
                len: len as u32,
            });
        }
        Ok(LocatedBodySuffixV1::new(
            body,
            start_index as u32,
            SourceViewSealV1::new(),
        ))
    }

    pub(crate) fn child_function(
        self,
        lambda: &LocatedExprV1<'a>,
    ) -> Result<Self, SourceNavigationErrorV1> {
        self.require_owner(lambda.owner())?;
        if !matches!(lambda.node(), ASTNode::Lambda { .. }) {
            return Err(SourceNavigationErrorV1::InvalidSite {
                owner: self.owner,
                site: lambda.site().node().clone(),
                reason: "child_owner_site_is_not_lambda",
            });
        }
        let owned_site = OwnedExprSiteV1::new(self.owner, lambda.site().clone());
        let child = self.forest.child_at(&owned_site).ok_or_else(|| {
            SourceNavigationErrorV1::InvalidSite {
                owner: self.owner,
                site: lambda.site().node().clone(),
                reason: "lambda_child_owner_missing",
            }
        })?;
        let owner_root = self.projection.owner_root(self.unit_syntax_root, child)?;
        Ok(Self {
            owner: child,
            unit_syntax_root: self.unit_syntax_root,
            owner_root,
            forest: self.forest,
            projection: self.projection,
        })
    }

    fn child_expr(
        self,
        actual_owner: FunctionOwnerIdV1,
        parent_site: &SourceNodeSiteV1,
        parent_node: &ASTNode,
        role: ExprChildRoleV1,
    ) -> Result<LocatedExprV1<'a>, SourceNavigationErrorV1> {
        self.require_owner(actual_owner)?;
        let segment =
            role.segment_for(parent_node)
                .ok_or_else(|| SourceNavigationErrorV1::InvalidSite {
                    owner: self.owner,
                    site: parent_site.clone(),
                    reason: "expression_role_parent_mismatch",
                })?;
        let site: SourceExprSiteV1 = SourcePathV1::from_node(parent_site).child(segment).expr();
        let node = self.project_node(site.node())?;
        Ok(LocatedExprV1::new(
            self.owner,
            site,
            node,
            SourceViewSealV1::new(),
        ))
    }

    fn child_body(
        self,
        actual_owner: FunctionOwnerIdV1,
        parent_site: &SourceNodeSiteV1,
        parent_node: &ASTNode,
        role: BodyChildRoleV1,
    ) -> Result<LocatedBodyV1<'a>, SourceNavigationErrorV1> {
        self.require_owner(actual_owner)?;
        let kind =
            role.kind_for(parent_node)
                .ok_or_else(|| SourceNavigationErrorV1::InvalidSite {
                    owner: self.owner,
                    site: parent_site.clone(),
                    reason: "body_role_parent_mismatch",
                })?;
        if matches!(kind, SourceBodyKindV1::Function | SourceBodyKindV1::Lambda) {
            return Err(SourceNavigationErrorV1::InvalidSite {
                owner: self.owner,
                site: parent_site.clone(),
                reason: "root_body_requested_as_child",
            });
        }
        let seal = SourceViewSealV1::new();
        let site = SourceBodySiteV1::new_child(self.owner, parent_site.clone(), kind, seal);
        let body = self.project_body(&site.root())?;
        Ok(LocatedBodyV1::new(site, body, seal))
    }

    fn project_node(self, site: &SourceNodeSiteV1) -> Result<&'a ASTNode, SourceNavigationErrorV1> {
        match self.projection.project(self.owner_root, self.owner, site)? {
            ProjectedSourceV1::Node(node) => Ok(node),
            ProjectedSourceV1::Body(_) => Err(SourceNavigationErrorV1::InvalidSite {
                owner: self.owner,
                site: site.clone(),
                reason: "expected_node_found_body",
            }),
            ProjectedSourceV1::SyntheticName => Err(SourceNavigationErrorV1::InvalidSite {
                owner: self.owner,
                site: site.clone(),
                reason: "synthetic_name_has_no_ast_node",
            }),
        }
    }

    fn project_body(
        self,
        site: &SourceNodeSiteV1,
    ) -> Result<&'a [ASTNode], SourceNavigationErrorV1> {
        match self.projection.project(self.owner_root, self.owner, site)? {
            ProjectedSourceV1::Body(body) => Ok(body),
            _ => Err(SourceNavigationErrorV1::InvalidSite {
                owner: self.owner,
                site: site.clone(),
                reason: "expected_body",
            }),
        }
    }

    fn require_owner(self, actual: FunctionOwnerIdV1) -> Result<(), SourceNavigationErrorV1> {
        if actual == self.owner {
            Ok(())
        } else {
            Err(SourceNavigationErrorV1::ForeignOwner {
                expected: self.owner,
                actual,
            })
        }
    }

    fn invalid_root_site(self, reason: &'static str) -> SourceNavigationErrorV1 {
        SourceNavigationErrorV1::InvalidSite {
            owner: self.owner,
            site: SourceNodeSiteV1::from_segments(Vec::new()),
            reason,
        }
    }
}
