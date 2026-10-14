//! One immutable original-source expression plan, shared by its qualified uses.
//! Nodes retain syntax and unresolved needs; this owner grants no execution.
use std::collections::BTreeSet;
use std::rc::Rc;

use super::expression_children::{
    expression_children_v1, ExpressionChildIssueV1, ExpressionChildrenV1,
};
use super::expression_root::{statement_expression_root_v1, StatementExpressionRoleV1};
use super::expression_source::{expression_source_v1, ResolvedExpressionSourceV1};
use crate::mir::resolved_semantics::{
    BodyExpressionShapeV1, CallableSemanticSourceLedgerView, FunctionOwnerIdV1,
    ResolvedAssignmentSourceV1, ResolvedBinaryExpressionSourceV1, ResolvedInitializerRelationV1,
    ResolvedLexicalRefV1, ResolvedLiteralSourceV1, ResolvedMethodCallReceiverSourceV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1,
    VerifiedResolvedBodyShapeInventoryV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExpressionPlanRootV1 {
    LocalInitializer {
        statement: SourceStmtSiteV1,
        initializer: ResolvedInitializerRelationV1,
    },
    Assignment {
        assignment: ResolvedAssignmentSourceV1,
    },
    Statement {
        statement: SourceStmtSiteV1,
        role: StatementExpressionRoleV1,
        value: SourceExprSiteV1,
    },
}

impl ExpressionPlanRootV1 {
    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        match self {
            Self::LocalInitializer { statement, .. } | Self::Statement { statement, .. } => {
                statement
            }
            Self::Assignment { assignment } => assignment.statement_site(),
        }
    }
    pub(crate) fn initializer(&self) -> Option<&ResolvedInitializerRelationV1> {
        match self {
            Self::LocalInitializer { initializer, .. } => Some(initializer),
            Self::Statement { .. } | Self::Assignment { .. } => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExpressionNodeSourceV1 {
    Binary(ResolvedBinaryExpressionSourceV1),
    Literal(ResolvedLiteralSourceV1),
    Reference(ResolvedLexicalRefV1),
    Unary,
    Conditional,
    /// Exact original sites remain requests to the existing call/input issuer.
    MethodCall,
    DirectCall,
    ReceiverQualification {
        call: SourceExprSiteV1,
        receiver: ResolvedMethodCallReceiverSourceV1,
    },
    /// Field/receiver/opaque structure is not a value-class proof.
    Shape(BodyExpressionShapeV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ExpressionSourceNodeV1 {
    site: SourceExprSiteV1,
    source: ExpressionNodeSourceV1,
    children: ExpressionChildrenV1,
}
impl ExpressionSourceNodeV1 {
    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }
    pub(crate) fn source(&self) -> &ExpressionNodeSourceV1 {
        &self.source
    }
    pub(crate) fn children(&self) -> &ExpressionChildrenV1 {
        &self.children
    }
    pub(crate) fn binary(&self) -> Option<&ResolvedBinaryExpressionSourceV1> {
        match &self.source {
            ExpressionNodeSourceV1::Binary(row) => Some(row),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ExpressionSourcePlanV1 {
    owner: FunctionOwnerIdV1,
    root: ExpressionPlanRootV1,
    nodes: Box<[ExpressionSourceNodeV1]>,
}
impl ExpressionSourcePlanV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn root(&self) -> &ExpressionPlanRootV1 {
        &self.root
    }
    pub(crate) fn nodes(&self) -> &[ExpressionSourceNodeV1] {
        &self.nodes
    }
    pub(crate) fn has_opaque_obligations(&self) -> bool {
        self.nodes
            .iter()
            .any(|node| matches!(node.children, ExpressionChildrenV1::Opaque { .. }))
    }
}

/// Borrow only the original initializer relation; no statement shape is needed
/// for the existing shape-independent scalar Local cohort.
pub(crate) fn local_expression_source_plan_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    shape: Option<&VerifiedResolvedBodyShapeInventoryV1>,
    initializer: &ResolvedInitializerRelationV1,
) -> Result<Rc<ExpressionSourcePlanV1>, ExpressionChildIssueV1> {
    use ExpressionChildIssueV1 as Issue;
    let SourceBindingSiteV1::Local { statement, .. } = initializer.declaration_site() else {
        return Err(Issue::ChildIdentity);
    };
    if initializer.binding().owner() != ledger.owner()
        || ledger.initializer(initializer.declaration_site()) != Some(initializer)
        || !ledger.source_site_inventory().contains_statement(statement)
    {
        return Err(Issue::ChildIdentity);
    }
    let site = initializer.initializer_site().ok_or(Issue::MissingSource)?;
    build_plan(
        ledger,
        shape,
        ExpressionPlanRootV1::LocalInitializer {
            statement: statement.clone(),
            initializer: initializer.clone(),
        },
        site,
    )
}

/// An assignment retains its original target, RHS and Plain/Compound form.
/// The RHS plan is not the stored-value proof of a compound assignment.
/// Assignment rows have no statement-to-Value relation; validate their actual
/// sealed row and both canonical expression sites instead of inventing one.
pub(crate) fn assignment_expression_source_plan_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    assignment: &ResolvedAssignmentSourceV1,
) -> Result<Rc<ExpressionSourcePlanV1>, ExpressionChildIssueV1> {
    use ExpressionChildIssueV1 as Issue;
    if shape.owner() != ledger.owner() {
        return Err(Issue::ForeignOwner);
    }
    let inventory = ledger.source_site_inventory();
    let mut rows = shape
        .assignment_sources()
        .iter()
        .filter(|row| row.statement_site() == assignment.statement_site());
    let path = SourcePathV1::from_node(assignment.statement_site().node());
    if rows.next() != Some(assignment)
        || rows.next().is_some()
        || !inventory.contains_statement(assignment.statement_site())
        || assignment.target_site() != &path.child(SourcePathSegmentV1::Target).expr()
        || assignment.value_site() != &path.child(SourcePathSegmentV1::Value).expr()
        || !inventory.contains_expression(assignment.target_site())
        || !inventory.contains_expression(assignment.value_site())
        || ledger.assignment_target(assignment.target_site()).is_none()
    {
        return Err(Issue::ChildIdentity);
    }
    build_plan(
        ledger,
        Some(shape),
        ExpressionPlanRootV1::Assignment {
            assignment: assignment.clone(),
        },
        assignment.value_site(),
    )
}

pub(crate) fn statement_expression_source_plan_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    statement: &SourceStmtSiteV1,
) -> Result<Option<Rc<ExpressionSourcePlanV1>>, ExpressionChildIssueV1> {
    let Some(root) = statement_expression_root_v1(ledger, shape, statement)? else {
        return Ok(None);
    };
    build_plan(
        ledger,
        Some(shape),
        ExpressionPlanRootV1::Statement {
            statement: statement.clone(),
            role: root.role,
            value: root.value.clone(),
        },
        root.value,
    )
    .map(Some)
}

fn build_plan(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    shape: Option<&VerifiedResolvedBodyShapeInventoryV1>,
    root: ExpressionPlanRootV1,
    site: &SourceExprSiteV1,
) -> Result<Rc<ExpressionSourcePlanV1>, ExpressionChildIssueV1> {
    use ExpressionChildIssueV1 as Issue;
    let mut pending = vec![site.clone()];
    let mut seen = BTreeSet::new();
    let mut nodes = Vec::new();
    while let Some(site) = pending.pop() {
        if !seen.insert(site.clone()) {
            return Err(Issue::ChildIdentity);
        }
        let children = expression_children_v1(ledger, shape, &site)?;
        let source = match expression_source_v1(ledger, shape, &site).ok_or(Issue::MissingSource)? {
            ResolvedExpressionSourceV1::Binary(row) => ExpressionNodeSourceV1::Binary(row.clone()),
            ResolvedExpressionSourceV1::Literal(row) => {
                ExpressionNodeSourceV1::Literal(row.clone())
            }
            ResolvedExpressionSourceV1::Reference(row) => ExpressionNodeSourceV1::Reference(row),
            ResolvedExpressionSourceV1::Unary(_) => ExpressionNodeSourceV1::Unary,
            ResolvedExpressionSourceV1::Conditional(_) => ExpressionNodeSourceV1::Conditional,
            ResolvedExpressionSourceV1::MethodCall(_) => ExpressionNodeSourceV1::MethodCall,
            ResolvedExpressionSourceV1::DirectCall => ExpressionNodeSourceV1::DirectCall,
            ResolvedExpressionSourceV1::ReceiverQualification(row) => {
                ExpressionNodeSourceV1::ReceiverQualification {
                    call: row.site().clone(),
                    receiver: row.receiver(),
                }
            }
            ResolvedExpressionSourceV1::Shape(row) => ExpressionNodeSourceV1::Shape(row.clone()),
        };
        if let ExpressionChildrenV1::Known(children) = &children {
            // Preorder retains original child order; branch reach remains on
            // each parent relation and is not flattened into unconditional use.
            pending.extend(children.iter().rev().map(|child| child.site.clone()));
        }
        // Opaque subtrees retain their original scope/prelude/tail obligations.
        // They are never traversed as if their statement effects were absent.
        nodes.push(ExpressionSourceNodeV1 {
            site,
            source,
            children,
        });
    }
    Ok(Rc::new(ExpressionSourcePlanV1 {
        owner: ledger.owner(),
        root,
        nodes: nodes.into_boxed_slice(),
    }))
}
