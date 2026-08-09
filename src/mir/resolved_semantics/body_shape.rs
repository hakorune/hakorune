//! Neutral, AST-free body-shape inventory issued beside one resolved owner.
//!
//! The shadow rows in this module are construction-local.  They may borrow no
//! AST after the shadow traversal finishes.  The sealed inventory is deliberately
//! weaker than a body Facts product: it records source shape and resolver-owned
//! lexical identity, but it does not classify Query/Home/ABI behavior.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use super::ids::{BindingRefV1, FunctionOwnerIdV1};
use super::owner_root_profile::SemanticOwnerRootProfileV1;
use super::product::VerifiedResolvedFunctionV1;
use super::records::ResolvedLexicalRefV1;
use super::source_site::{
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BodyStatementShapeV1 {
    SequenceItem {
        site: SourceStmtSiteV1,
    },
    Return {
        site: SourceStmtSiteV1,
        value: Option<SourceExprSiteV1>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BodyExpressionShapeV1 {
    Variable {
        site: SourceExprSiteV1,
        resolved: ResolvedLexicalRefV1,
    },
    Me {
        site: SourceExprSiteV1,
        receiver: BodyMeReceiverV1,
    },
    FieldAccess {
        site: SourceExprSiteV1,
        object: SourceExprSiteV1,
        field: Box<str>,
    },
    MethodCall {
        site: SourceExprSiteV1,
        object: SourceExprSiteV1,
        method: Box<str>,
        arity: u32,
    },
    Other {
        site: SourceExprSiteV1,
        kind: Box<str>,
    },
}

/// Resolver-owned meaning of one `me` expression.
///
/// Static-box current-owner syntax has no lexical receiver binding. Keeping
/// that case explicit prevents the neutral body inventory from fabricating a
/// `BindingRefV1` merely to satisfy instance-only consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BodyMeReceiverV1 {
    Lexical(BindingRefV1),
    StaticCurrentOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum BodyEffectKindV1 {
    Write,
    Allocation,
    Call,
    Await,
    QMark,
    Throw,
    NonLocalControl,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct BodyEffectShapeV1 {
    pub(crate) site: SourceExprSiteV1,
    pub(crate) kind: BodyEffectKindV1,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct BodyShapeRelationV1 {
    pub(crate) parent: SourceNodeSiteV1,
    pub(crate) role: SourcePathSegmentV1,
    pub(crate) child: SourceExprSiteV1,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct ShadowBodyShapeDraftV0 {
    pub(crate) statements: BTreeMap<SourceStmtSiteV1, ShadowStatementShapeV0>,
    pub(crate) expressions: BTreeMap<SourceExprSiteV1, ShadowExpressionShapeV0>,
    pub(crate) effects: BTreeSet<(SourceExprSiteV1, BodyEffectKindV1)>,
    pub(crate) relations: BTreeSet<BodyShapeRelationV0>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ShadowStatementShapeV0 {
    SequenceItem {
        site: SourceStmtSiteV1,
    },
    Return {
        site: SourceStmtSiteV1,
        value: Option<SourceExprSiteV1>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ShadowExpressionShapeV0 {
    Variable {
        site: SourceExprSiteV1,
    },
    Me {
        site: SourceExprSiteV1,
    },
    FieldAccess {
        site: SourceExprSiteV1,
        object: SourceExprSiteV1,
        field: Box<str>,
    },
    MethodCall {
        site: SourceExprSiteV1,
        object: SourceExprSiteV1,
        method: Box<str>,
        arity: u32,
    },
    Other {
        site: SourceExprSiteV1,
        kind: Box<str>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct BodyShapeRelationV0 {
    pub(crate) parent: SourceNodeSiteV1,
    pub(crate) role: SourcePathSegmentV1,
    pub(crate) child: SourceExprSiteV1,
}

#[derive(Debug)]
pub(crate) struct VerifiedResolvedBodyShapeInventoryV1 {
    owner: FunctionOwnerIdV1,
    body_root: SourcePathSegmentV1,
    statements: Box<[BodyStatementShapeV1]>,
    expressions: Box<[BodyExpressionShapeV1]>,
    effects: Box<[BodyEffectShapeV1]>,
    relations: Box<[BodyShapeRelationV1]>,
}

/// One resolver-issued function plus its neutral body-shape authority.
///
/// The shape inventory is a sibling product, not a new FunctionOwner issuer;
/// both products are sealed from the same shadow traversal and owner maps.
#[derive(Debug)]
pub(crate) struct ResolvedFunctionBodyShapeProductV1 {
    function: Arc<VerifiedResolvedFunctionV1>,
    body_shape: VerifiedResolvedBodyShapeInventoryV1,
}

impl ResolvedFunctionBodyShapeProductV1 {
    pub(crate) fn from_parts(
        function: Arc<VerifiedResolvedFunctionV1>,
        body_shape: VerifiedResolvedBodyShapeInventoryV1,
    ) -> Self {
        Self {
            function,
            body_shape,
        }
    }

    pub(crate) fn function(&self) -> &Arc<VerifiedResolvedFunctionV1> {
        &self.function
    }

    pub(crate) fn body_shape(&self) -> &VerifiedResolvedBodyShapeInventoryV1 {
        &self.body_shape
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        Arc<VerifiedResolvedFunctionV1>,
        VerifiedResolvedBodyShapeInventoryV1,
    ) {
        (self.function, self.body_shape)
    }
}

impl VerifiedResolvedBodyShapeInventoryV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn body_root(&self) -> &SourcePathSegmentV1 {
        &self.body_root
    }

    pub(crate) fn statements(&self) -> &[BodyStatementShapeV1] {
        &self.statements
    }

    pub(crate) fn expressions(&self) -> &[BodyExpressionShapeV1] {
        &self.expressions
    }

    pub(crate) fn effects(&self) -> &[BodyEffectShapeV1] {
        &self.effects
    }

    pub(crate) fn relations(&self) -> &[BodyShapeRelationV1] {
        &self.relations
    }
}

pub(crate) fn seal_shadow_body_shape(
    owner: FunctionOwnerIdV1,
    root_profile: SemanticOwnerRootProfileV1,
    draft: ShadowBodyShapeDraftV0,
    variable_refs: &BTreeMap<SourceExprSiteV1, ResolvedLexicalRefV1>,
    statement_sites: &BTreeSet<SourceStmtSiteV1>,
    expression_sites: &BTreeSet<SourceExprSiteV1>,
) -> Result<VerifiedResolvedBodyShapeInventoryV1, &'static str> {
    if draft.statements.len() != statement_sites.len()
        || draft
            .statements
            .keys()
            .any(|site| !statement_sites.contains(site))
        || draft.expressions.len() != expression_sites.len()
        || draft
            .expressions
            .keys()
            .any(|site| !expression_sites.contains(site))
    {
        return Err("body shape coverage does not match resolver source inventory");
    }

    let statements = draft
        .statements
        .into_values()
        .map(|row| match row {
            ShadowStatementShapeV0::SequenceItem { site } => {
                BodyStatementShapeV1::SequenceItem { site }
            }
            ShadowStatementShapeV0::Return { site, value } => {
                BodyStatementShapeV1::Return { site, value }
            }
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let expressions = draft
        .expressions
        .into_values()
        .map(|row| match row {
            ShadowExpressionShapeV0::Variable { site } => {
                let resolved = variable_refs
                    .get(&site)
                    .copied()
                    .ok_or("body variable shape lacks lexical resolution")?;
                Ok(BodyExpressionShapeV1::Variable { site, resolved })
            }
            ShadowExpressionShapeV0::Me { site } => {
                let receiver = match variable_refs.get(&site).copied() {
                    Some(ResolvedLexicalRefV1::Local(receiver)) => {
                        BodyMeReceiverV1::Lexical(receiver)
                    }
                    None if matches!(
                        root_profile,
                        SemanticOwnerRootProfileV1::DeclaredFunction {
                            receiver_policy:
                                super::function_view::ReceiverPolicyV1::StaticCurrentOwner,
                        }
                    ) =>
                    {
                        BodyMeReceiverV1::StaticCurrentOwner
                    }
                    _ => return Err("body Me shape lacks exact receiver authority"),
                };
                Ok(BodyExpressionShapeV1::Me { site, receiver })
            }
            ShadowExpressionShapeV0::FieldAccess {
                site,
                object,
                field,
            } => Ok(BodyExpressionShapeV1::FieldAccess {
                site,
                object,
                field,
            }),
            ShadowExpressionShapeV0::MethodCall {
                site,
                object,
                method,
                arity,
            } => Ok(BodyExpressionShapeV1::MethodCall {
                site,
                object,
                method,
                arity,
            }),
            ShadowExpressionShapeV0::Other { site, kind } => {
                Ok(BodyExpressionShapeV1::Other { site, kind })
            }
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();

    let effects = draft
        .effects
        .into_iter()
        .map(|(site, kind)| BodyEffectShapeV1 { site, kind })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let relations = draft
        .relations
        .into_iter()
        .map(|row| BodyShapeRelationV1 {
            parent: row.parent,
            role: row.role,
            child: row.child,
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();

    Ok(VerifiedResolvedBodyShapeInventoryV1 {
        owner,
        body_root: root_profile.body_root(),
        statements,
        expressions,
        effects,
        relations,
    })
}

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
                    super::source_site::SourcePathV1::from_node(site.node())
                        .child(SourcePathSegmentV1::Value)
                        .expr()
                }),
            },
            _ => ShadowStatementShapeV0::SequenceItem { site: site.clone() },
        };
        self.body_shape.statements.insert(site, shape);
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
                    object: super::source_site::SourcePathV1::from_node(site.node())
                        .child(SourcePathSegmentV1::Receiver)
                        .expr(),
                    field: field.clone().into_boxed_str(),
                }
            }
            crate::ast::ASTNode::MethodCall {
                method, arguments, ..
            } => ShadowExpressionShapeV0::MethodCall {
                site: site.clone(),
                object: super::source_site::SourcePathV1::from_node(site.node())
                    .child(SourcePathSegmentV1::Receiver)
                    .expr(),
                method: method.clone().into_boxed_str(),
                arity: arguments.len() as u32,
            },
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
        self.body_shape.relations.insert(BodyShapeRelationV0 {
            parent,
            role,
            child,
        });
    }
}
