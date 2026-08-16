//! Shadow-resolver adapter for the neutral body-shape inventory.
//!
//! Keeping traversal mutation separate from the sealed inventory keeps the
//! inventory module focused on its passive source rows and seal operations.

use super::assignment_source::record_shadow_assignment_source;
use super::body_shape::{
    BodyEffectKindV1, BodyShapeRelationV0, ShadowBodyShapeDraftV0, ShadowExpressionShapeV0,
    ShadowStatementShapeV0,
};
use super::source_site::{
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1,
};

impl<'ast, 'schema> super::shadow::resolver::ShadowResolverV0<'ast, 'schema> {
    pub(super) fn record_statement_shape(
        &mut self,
        statement: &crate::ast::ASTNode,
        site: SourceStmtSiteV1,
    ) {
        let shape = match statement {
            crate::ast::ASTNode::Return { value, .. } => ShadowStatementShapeV0::Return {
                site: site.clone(),
                value: value.as_deref().map(|_| {
                    SourcePathV1::from_node(site.node())
                        .child(SourcePathSegmentV1::Value)
                        .expr()
                }),
            },
            _ => ShadowStatementShapeV0::SequenceItem { site: site.clone() },
        };
        self.body_shape.statements.insert(site.clone(), shape);
        record_shadow_assignment_source(&mut self.body_shape, statement, site);
    }

    pub(super) fn record_expression_shape(
        &mut self,
        expression: &crate::ast::ASTNode,
        site: SourceExprSiteV1,
    ) {
        let shape = match expression {
            crate::ast::ASTNode::Variable { .. } => {
                ShadowExpressionShapeV0::Variable { site: site.clone() }
            }
            crate::ast::ASTNode::Me { .. } => ShadowExpressionShapeV0::Me { site: site.clone() },
            crate::ast::ASTNode::FieldAccess { field, .. } => {
                ShadowExpressionShapeV0::FieldAccess {
                    site: site.clone(),
                    object: SourcePathV1::from_node(site.node())
                        .child(SourcePathSegmentV1::Receiver)
                        .expr(),
                    field: field.clone().into_boxed_str(),
                }
            }
            crate::ast::ASTNode::MethodCall {
                method, arguments, ..
            } => ShadowExpressionShapeV0::MethodCall {
                site: site.clone(),
                object: SourcePathV1::from_node(site.node())
                    .child(SourcePathSegmentV1::Receiver)
                    .expr(),
                method: method.clone().into_boxed_str(),
                arity: arguments.len(),
            },
            crate::ast::ASTNode::BlockExpr { .. } => {
                ShadowExpressionShapeV0::BlockExpr { site: site.clone() }
            }
            _ => ShadowExpressionShapeV0::Other {
                site: site.clone(),
                kind: expression.node_type().into(),
            },
        };
        self.body_shape.expressions.insert(site, shape);
    }

    /// Records one assignment-place expression without reclassifying a plain
    /// binding target as a lexical value read.
    pub(super) fn record_assignment_target_shape(
        &mut self,
        target: &crate::ast::ASTNode,
        site: SourceExprSiteV1,
    ) {
        if matches!(target, crate::ast::ASTNode::Variable { .. }) {
            self.body_shape.expressions.insert(
                site.clone(),
                ShadowExpressionShapeV0::Other {
                    site,
                    kind: "BindingAssignmentTarget".into(),
                },
            );
        } else {
            self.record_expression_shape(target, site);
        }
    }

    pub(super) fn record_effect(&mut self, site: SourceExprSiteV1, kind: BodyEffectKindV1) {
        self.body_shape.effects.insert((site, kind));
    }

    pub(super) fn record_relation(
        &mut self,
        parent: SourceNodeSiteV1,
        role: SourcePathSegmentV1,
        child: SourceExprSiteV1,
    ) {
        self.body_shape.relations.push(BodyShapeRelationV0 {
            parent,
            role,
            child,
        });
    }
}
