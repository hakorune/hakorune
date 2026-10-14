//! Exact ordered child relations borrowed from the same expression source rows.
//! Reach records syntax dependence, never a Bool class or execution permission.
use super::expression_source::{expression_source_v1, ResolvedExpressionSourceV1};
use crate::mir::resolved_semantics::{
    BodyExpressionShapeV1, CallableSemanticSourceLedgerView, ResolvedBinaryExpressionSourceV1,
    ResolvedBinaryOperatorV1, SourceExprSiteV1, SourcePathSegmentV1, SourcePathV1,
    SourceStmtSiteV1, VerifiedResolvedBodyShapeInventoryV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExpressionChildReachV1 {
    Always,
    When {
        condition: SourceExprSiteV1,
        value: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpressionChildV1 {
    pub(crate) role: SourcePathSegmentV1,
    pub(crate) site: SourceExprSiteV1,
    pub(crate) reach: ExpressionChildReachV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExpressionChildIssueV1 {
    ForeignOwner,
    MissingSource,
    ChildIdentity,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExpressionChildrenV1 {
    Known(Vec<ExpressionChildV1>),
    Opaque {
        scope: SourceExprSiteV1,
        /// Original statement obligations, never replaced by an empty prelude.
        prelude: Vec<SourceStmtSiteV1>,
        tail: Option<SourceExprSiteV1>,
    },
}

/// Shared binary correspondence, including the original root and LHS/RHS order.
/// The caller still owns scalar classes, operator laws and Home availability.
pub(crate) fn binary_children_v1<'a>(
    root: &SourceExprSiteV1,
    row: &'a ResolvedBinaryExpressionSourceV1,
) -> Option<[&'a SourceExprSiteV1; 2]> {
    let path = SourcePathV1::from_node(root.node());
    (row.site() == root
        && row.lhs() == &path.child(SourcePathSegmentV1::Lhs).expr()
        && row.rhs() == &path.child(SourcePathSegmentV1::Rhs).expr())
        .then_some([row.lhs(), row.rhs()])
}

pub(crate) fn expression_children_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    shape: Option<&VerifiedResolvedBodyShapeInventoryV1>,
    root: &SourceExprSiteV1,
) -> Result<ExpressionChildrenV1, ExpressionChildIssueV1> {
    use ExpressionChildIssueV1 as Issue;
    use ExpressionChildReachV1::{Always, When};
    use SourcePathSegmentV1 as Role;
    if shape.is_some_and(|shape| shape.owner() != ledger.owner()) {
        return Err(Issue::ForeignOwner);
    }
    if !ledger.source_site_inventory().contains_expression(root) {
        return Err(Issue::MissingSource);
    }
    let source = expression_source_v1(ledger, shape, root).ok_or(Issue::MissingSource)?;
    let path = SourcePathV1::from_node(root.node());
    let mut children = Vec::new();
    let mut add = |role: Role, site: &SourceExprSiteV1, reach: ExpressionChildReachV1| {
        let expected = match role {
            Role::IfThen(0) => path.child(Role::IfThenBody).child(Role::IfThen(0)).expr(),
            Role::IfElse(0) => path.child(Role::IfElseBody).child(Role::IfElse(0)).expr(),
            _ => path.child(role.clone()).expr(),
        };
        if site != &expected || !ledger.source_site_inventory().contains_expression(site) {
            return Err(Issue::ChildIdentity);
        }
        children.push(ExpressionChildV1 {
            role,
            site: site.clone(),
            reach,
        });
        Ok(())
    };
    match source {
        ResolvedExpressionSourceV1::Binary(row) => {
            let [lhs, rhs] = binary_children_v1(root, row).ok_or(Issue::ChildIdentity)?;
            add(Role::Lhs, lhs, Always)?;
            let reach = match row.operator() {
                ResolvedBinaryOperatorV1::And => When {
                    condition: lhs.clone(),
                    value: true,
                },
                ResolvedBinaryOperatorV1::Or => When {
                    condition: lhs.clone(),
                    value: false,
                },
                _ => Always,
            };
            add(Role::Rhs, rhs, reach)?;
        }
        ResolvedExpressionSourceV1::Unary(row) => {
            if row.site() != root {
                return Err(Issue::ChildIdentity);
            }
            add(Role::Operand, row.operand(), Always)?;
        }
        ResolvedExpressionSourceV1::Conditional(row) => {
            if row.site() != root {
                return Err(Issue::ChildIdentity);
            }
            add(Role::IfCondition, row.condition(), Always)?;
            add(
                Role::IfThen(0),
                row.then_block(),
                When {
                    condition: row.condition().clone(),
                    value: true,
                },
            )?;
            add(
                Role::IfElse(0),
                row.else_block(),
                When {
                    condition: row.condition().clone(),
                    value: false,
                },
            )?;
            if row.then_tail()
                != &SourcePathV1::from_node(row.then_block().node())
                    .child(Role::BlockExprTail)
                    .expr()
                || row.else_tail()
                    != &SourcePathV1::from_node(row.else_block().node())
                        .child(Role::BlockExprTail)
                        .expr()
            {
                return Err(Issue::ChildIdentity);
            }
        }
        ResolvedExpressionSourceV1::MethodCall(row) => {
            if row.owner() != ledger.owner()
                || row.site() != root
                || row.arguments().len() != row.arity() as usize
            {
                return Err(Issue::ChildIdentity);
            }
            // This preserves a syntactic receiver. Its actual evaluation or
            // qualification is still decided by the original receiver issuer.
            add(Role::Receiver, row.receiver_site(), Always)?;
            for (ordinal, argument) in row.arguments().iter().enumerate() {
                if argument.ordinal() as usize != ordinal {
                    return Err(Issue::ChildIdentity);
                }
                add(Role::Argument(argument.ordinal()), argument.site(), Always)?;
            }
        }
        ResolvedExpressionSourceV1::DirectCall => {
            let row = ledger
                .direct_call_observation(root)
                .ok_or(Issue::MissingSource)?;
            if row.argument_sites().len() != row.arity() as usize {
                return Err(Issue::ChildIdentity);
            }
            for (ordinal, site) in row.argument_sites().iter().enumerate() {
                add(Role::Argument(ordinal as u32), site, Always)?;
            }
        }
        ResolvedExpressionSourceV1::Literal(_) | ResolvedExpressionSourceV1::Reference(_)
        | ResolvedExpressionSourceV1::ReceiverQualification(_) => {}
        ResolvedExpressionSourceV1::Shape(BodyExpressionShapeV1::FieldAccess {
            object, ..
        }) => {
            add(Role::Receiver, object, Always)?;
        }
        ResolvedExpressionSourceV1::Shape(
            BodyExpressionShapeV1::Me { .. } | BodyExpressionShapeV1::QualifiedReceiver { .. },
        ) => {}
        ResolvedExpressionSourceV1::Shape(BodyExpressionShapeV1::BlockExpr { .. }) => {
            let shape = shape.ok_or(Issue::MissingSource)?;
            let prelude: Vec<_> = shape
                .statements()
                .iter()
                .filter(|statement| {
                    matches!(statement.site().node().segments().split_last(),
                    Some((Role::BlockExprPrelude(_), prefix)) if prefix == root.node().segments())
                })
                .map(|statement| statement.site().clone())
                .collect();
            let tail = path.child(Role::BlockExprTail).expr();
            add(Role::BlockExprTail, &tail, Always)?;
            if !prelude.is_empty() {
                return Ok(ExpressionChildrenV1::Opaque {
                    scope: root.clone(),
                    prelude,
                    tail: Some(tail),
                });
            }
        }
        ResolvedExpressionSourceV1::Shape(_) => {
            return Ok(ExpressionChildrenV1::Opaque {
                scope: root.clone(),
                prelude: Vec::new(),
                tail: None,
            });
        }
    }
    Ok(ExpressionChildrenV1::Known(children))
}
