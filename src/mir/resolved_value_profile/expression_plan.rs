//! One immutable original-source expression plan, shared by its qualified uses.
//! Nodes retain syntax and unresolved needs; this owner grants no execution.
use std::collections::BTreeSet;
use std::rc::Rc;

#[path = "expression_field_path.rs"]
mod field_path;
pub(crate) use field_path::OriginalExpressionFieldPathV1;

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
    CallArgument {
        statement: SourceStmtSiteV1,
        call: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
        ordinal: u32,
    },
    LocalInitializer {
        statement: SourceStmtSiteV1,
        initializer: ResolvedInitializerRelationV1,
    },
    Assignment {
        assignment: ResolvedAssignmentSourceV1,
    },
    IfCondition {
        statement: SourceStmtSiteV1,
        condition: SourceExprSiteV1,
        bundle: crate::mir::resolved_semantics::ResolvedIfRegionBundleV1,
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
            Self::CallArgument { statement, .. } | Self::LocalInitializer { statement, .. } | Self::Statement { statement, .. }
            | Self::IfCondition { statement, .. } => {
                statement
            }
            Self::Assignment { assignment } => assignment.statement_site(),
        }
    }
    pub(crate) fn initializer(&self) -> Option<&ResolvedInitializerRelationV1> {
        match self {
            Self::LocalInitializer { initializer, .. } => Some(initializer),
            Self::CallArgument { .. } | Self::Statement { .. } | Self::Assignment { .. } | Self::IfCondition { .. } => None,
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
    /// Children precede their parent in declared LHS/RHS/argument order.
    /// This is structural order only: conditional child reach remains on the
    /// SAME plan and must never be promoted into eager execution permission.
    pub(crate) fn ordered_postorder_v1(&self) -> Result<Vec<usize>, ExpressionChildIssueV1> {
        use ExpressionChildIssueV1 as Issue;
        if self.nodes.is_empty() { return Err(Issue::MissingSource); }
        let indices: std::collections::BTreeMap<_, _> = self.nodes.iter().enumerate()
            .map(|(index, node)| (node.site().clone(), index)).collect();
        if indices.len() != self.nodes.len() { return Err(Issue::ChildIdentity); }
        let mut pending = vec![(0, false)];
        let mut seen = BTreeSet::new();
        let mut ordered = Vec::with_capacity(self.nodes.len());
        while let Some((index, finishing)) = pending.pop() {
            if finishing { ordered.push(index); continue; }
            if !seen.insert(index) { return Err(Issue::ChildIdentity); }
            let ExpressionChildrenV1::Known(children) = self.nodes[index].children()
                else { return Err(Issue::MissingSource); };
            pending.push((index, true));
            for child in children.iter().rev() {
                pending.push((*indices.get(&child.site).ok_or(Issue::ChildIdentity)?, false));
            }
        }
        if ordered.len() != self.nodes.len() { return Err(Issue::ChildIdentity); }
        Ok(ordered)
    }

    /// Exact root-to-leaf original field nodes. This is source structure only:
    /// each edge still needs its own class, declaration and live-root proof.
    pub(crate) fn field_path_v1(
        &self, site: &SourceExprSiteV1,
    ) -> Option<(crate::mir::resolved_semantics::BindingRefV1, Vec<usize>)> {
        let mut current = site;
        let mut fields = Vec::new();
        loop {
            let (index, node) = self.nodes.iter().enumerate()
                .find(|(_, node)| node.site() == current)?;
            match node.source() {
                ExpressionNodeSourceV1::Shape(BodyExpressionShapeV1::FieldAccess { object, .. }) => {
                    let expected = SourcePathV1::from_node(current.node())
                        .child(SourcePathSegmentV1::Receiver).expr();
                    let ExpressionChildrenV1::Known(children) = node.children() else { return None; };
                    if object != &expected || children.len() != 1
                        || children[0].role != SourcePathSegmentV1::Receiver
                        || children[0].site != *object || fields.contains(&index)
                    { return None; }
                    fields.push(index);
                    current = object;
                }
                ExpressionNodeSourceV1::Reference(ResolvedLexicalRefV1::Local(binding))
                | ExpressionNodeSourceV1::Shape(BodyExpressionShapeV1::Me {
                    receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(binding), ..
                }) if binding.owner() == self.owner && !fields.is_empty() => {
                    fields.reverse();
                    return Some((*binding, fields));
                }
                _ => return None,
            }
        }
    }

    /// The existing direct-read adapter retains its one-edge restriction.
    /// A nested path never gains direct receiver/Home permission here.
    pub(crate) fn field_read_node_v1(
        &self, site: &SourceExprSiteV1, receiver_site: &SourceExprSiteV1,
        receiver: crate::mir::resolved_semantics::BindingRefV1, name: &str,
    ) -> Option<usize> {
        let (root, fields) = self.field_path_v1(site)?;
        let [index] = fields.as_slice() else { return None; };
        let ExpressionNodeSourceV1::Shape(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
            self.nodes[*index].source() else { return None; };
        (root == receiver && object == receiver_site && field.as_ref() == name).then_some(*index)
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

/// The resolver's exact If bundle owns this root; the shared builder retains
/// every original child and conditional reach without granting execution.
pub(crate) fn condition_expression_source_plan_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
) -> Result<Rc<ExpressionSourcePlanV1>, ExpressionChildIssueV1> {
    use ExpressionChildIssueV1 as Issue;
    let bundle = *input.function().if_region_bundle(statement).map_err(|_| Issue::MissingSource)?;
    let condition = SourcePathV1::from_node(statement.node())
        .child(SourcePathSegmentV1::IfCondition).expr();
    let exact = input.function().with_if_region_for_condition(&condition, |row|
        row.site() == statement && row.bundle() == bundle).map_err(|_| Issue::MissingSource)?;
    let shape = input.body_shape().ok_or(Issue::MissingSource)?;
    let ledger = input.forest().callable_source_ledger(input.owner()).map_err(|_| Issue::ForeignOwner)?;
    if !exact || shape.owner() != input.owner() || ledger.owner() != input.owner() {
        return Err(Issue::ForeignOwner);
    }
    let inventory = ledger.source_site_inventory();
    if !inventory.contains_statement(statement) || !inventory.contains_expression(&condition) {
        return Err(Issue::MissingSource);
    }
    let mut relations = shape.relations().iter().filter(|row|
        row.parent() == statement.node() && row.role() == &SourcePathSegmentV1::IfCondition);
    if relations.next().is_none_or(|row| row.child() != &condition) || relations.next().is_some() {
        return Err(Issue::ChildIdentity);
    }
    build_plan(&ledger, Some(shape), ExpressionPlanRootV1::IfCondition {
        statement: statement.clone(), condition: condition.clone(), bundle,
    }, &condition)
}

/// Source purpose for one exact original actual. The nearest registered
/// statement is topology, never a claim that its Prefix is covered.
pub(crate) fn call_argument_expression_source_plan_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    call: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    ordinal: u32,
) -> Result<Rc<ExpressionSourcePlanV1>, ExpressionChildIssueV1> {
    use ExpressionChildIssueV1 as Issue;
    let ledger = input.forest().callable_source_ledger(input.owner()).map_err(|_| Issue::ForeignOwner)?;
    let inventory = ledger.source_site_inventory();
    if call.owner() != input.owner() || ledger.owner() != input.owner()
        || !ledger.method_call_source_v1(call.site()).is_some_and(|original| std::sync::Arc::ptr_eq(original, call))
        || !inventory.contains_expression(call.site())
    { return Err(Issue::ChildIdentity); }
    let argument = call.arguments().get(ordinal as usize).filter(|argument| argument.ordinal() == ordinal)
        .ok_or(Issue::ChildIdentity)?;
    let expected = SourcePathV1::from_node(call.site().node()).child(SourcePathSegmentV1::Argument(ordinal)).expr();
    if argument.site() != &expected || !inventory.contains_expression(argument.site()) {
        return Err(Issue::ChildIdentity);
    }
    let statement = inventory.containing_statement(call.site()).ok_or(Issue::MissingSource)?;
    build_plan(&ledger, input.body_shape(), ExpressionPlanRootV1::CallArgument {
        statement: statement.clone(), call: std::sync::Arc::clone(call), ordinal,
    }, argument.site())
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
