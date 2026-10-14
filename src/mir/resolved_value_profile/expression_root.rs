//! Statement value roots from the same sealed source as expression children.
//! A root role is source structure, never permission to print or return a value.
use super::expression_children::ExpressionChildIssueV1;
use crate::mir::resolved_semantics::{
    BodyStatementShapeV1, CallableSemanticSourceLedgerView, SourceExprSiteV1,
    SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1, VerifiedResolvedBodyShapeInventoryV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StatementExpressionRoleV1 {
    PrintValue,
    ReturnValue,
}

#[derive(Debug)]
pub(crate) struct StatementExpressionRootV1<'a> {
    pub(crate) role: StatementExpressionRoleV1,
    pub(crate) value: &'a SourceExprSiteV1,
}

pub(crate) fn statement_expression_root_v1<'a>(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    shape: &'a VerifiedResolvedBodyShapeInventoryV1,
    statement: &SourceStmtSiteV1,
) -> Result<Option<StatementExpressionRootV1<'a>>, ExpressionChildIssueV1> {
    use ExpressionChildIssueV1 as Issue;
    if shape.owner() != ledger.owner() {
        return Err(Issue::ForeignOwner);
    }
    let inventory = ledger.source_site_inventory();
    if !inventory.contains_statement(statement) {
        return Err(Issue::MissingSource);
    }
    let row = shape
        .statements()
        .iter()
        .find(|row| row.site() == statement)
        .ok_or(Issue::MissingSource)?;
    let (role, value) = match row {
        BodyStatementShapeV1::Print { value, .. } => (StatementExpressionRoleV1::PrintValue, value),
        BodyStatementShapeV1::Return { value: Some(value), .. } => {
            (StatementExpressionRoleV1::ReturnValue, value)
        }
        BodyStatementShapeV1::SequenceItem { .. }
        | BodyStatementShapeV1::Return { value: None, .. } => return Ok(None),
    };
    let expected = SourcePathV1::from_node(statement.node())
        .child(SourcePathSegmentV1::Value)
        .expr();
    let mut relations = shape.relations().iter().filter(|relation| {
        relation.parent() == statement.node() && relation.role() == &SourcePathSegmentV1::Value
    });
    if value != &expected
        || !inventory.contains_expression(value)
        || relations.next().is_none_or(|relation| relation.child() != value)
        || relations.next().is_some()
    {
        return Err(Issue::ChildIdentity);
    }
    Ok(Some(StatementExpressionRootV1 { role, value }))
}
