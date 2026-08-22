//! Source-only Dynamic operation relations for one callable Loop.
//!
//! This module proves the bounded carrier-lineage comparison and Add/rebind
//! rows before any Builder effect. It never derives Dynamic from a physical
//! type or emitted opcode, and it does not claim unrelated calls, locals,
//! exits, Tail, or Recipe operations.

use std::collections::BTreeMap;

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    project_source_node_v1, BindingRefV1, FunctionOwnerIdV1, ProjectedSourceNodeV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourcePathV1,
};

use super::normal_callable_dynamic_source::VerifiedSourceBackedDynamicCallableV1;
use super::normal_callable_loop_handoff::{
    CallableLoopBindingRoleV1, CallableLoopReadyBindingClassV1,
    VerifiedCallableSemanticLoopBindingScheduleV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DynamicLoopComparisonKindV1 {
    Less,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DynamicLoopOperationResultClassV1 {
    Dynamic,
    Bool,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct VerifiedDynamicLoopComparisonSourceV1 {
    operation: SourceExprSiteV1,
    carrier_read: SourceExprSiteV1,
    operand_read: SourceExprSiteV1,
    carrier: BindingRefV1,
    operand: BindingRefV1,
    kind: DynamicLoopComparisonKindV1,
    result: DynamicLoopOperationResultClassV1,
}

impl VerifiedDynamicLoopComparisonSourceV1 {
    pub(super) const fn operation(&self) -> &SourceExprSiteV1 {
        &self.operation
    }

    pub(super) const fn carrier(&self) -> BindingRefV1 {
        self.carrier
    }

    pub(super) const fn carrier_read(&self) -> &SourceExprSiteV1 {
        &self.carrier_read
    }

    pub(super) const fn operand_read(&self) -> &SourceExprSiteV1 {
        &self.operand_read
    }

    pub(super) const fn operand(&self) -> BindingRefV1 {
        self.operand
    }

    pub(super) const fn kind(&self) -> DynamicLoopComparisonKindV1 {
        self.kind
    }

    pub(super) const fn result(&self) -> DynamicLoopOperationResultClassV1 {
        self.result
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct VerifiedDynamicLoopAddRebindSourceV1 {
    operation: SourceExprSiteV1,
    carrier_read: SourceExprSiteV1,
    exact_literal: SourceExprSiteV1,
    target: SourceExprSiteV1,
    carrier: BindingRefV1,
    delta: i64,
    result: DynamicLoopOperationResultClassV1,
}

impl VerifiedDynamicLoopAddRebindSourceV1 {
    pub(super) const fn operation(&self) -> &SourceExprSiteV1 {
        &self.operation
    }

    pub(super) const fn target(&self) -> &SourceExprSiteV1 {
        &self.target
    }

    pub(super) const fn carrier_read(&self) -> &SourceExprSiteV1 {
        &self.carrier_read
    }

    pub(super) const fn exact_literal(&self) -> &SourceExprSiteV1 {
        &self.exact_literal
    }

    pub(super) const fn carrier(&self) -> BindingRefV1 {
        self.carrier
    }

    pub(super) const fn delta(&self) -> i64 {
        self.delta
    }

    pub(super) const fn result(&self) -> DynamicLoopOperationResultClassV1 {
        self.result
    }
}

/// Complete first-cohort operation/source relation set for one Loop.
///
/// The product is move-only. Its rows have no arbitrary constructors, so a
/// caller cannot combine a comparison from one Loop with a rebind from another.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct VerifiedDynamicLoopOperationSourceSetV1 {
    owner: FunctionOwnerIdV1,
    loop_site: SourceNodeSiteV1,
    comparison: VerifiedDynamicLoopComparisonSourceV1,
    add_rebind: VerifiedDynamicLoopAddRebindSourceV1,
}

impl VerifiedDynamicLoopOperationSourceSetV1 {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn loop_site(&self) -> &SourceNodeSiteV1 {
        &self.loop_site
    }

    pub(super) const fn comparison(&self) -> &VerifiedDynamicLoopComparisonSourceV1 {
        &self.comparison
    }

    pub(super) const fn add_rebind(&self) -> &VerifiedDynamicLoopAddRebindSourceV1 {
        &self.add_rebind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DynamicLoopOperationSourceIssueV1 {
    SourceProjection(String),
    OwnerMismatch,
    LoopSourceMismatch,
    CarrierCardinality,
    DynamicCarrierMismatch,
    ComparisonNoSafeSlice,
    ComparisonSourceMismatch,
    ComparisonOperandNotDynamic,
    RebindCardinality,
    RebindNoSafeSlice,
    RebindSourceMismatch,
    IncompleteSourceInventory,
}

pub(super) struct DynamicLoopOperationSourceIssuerV1;

impl DynamicLoopOperationSourceIssuerV1 {
    pub(super) fn issue(
        input: ResolvedFunctionLoweringInputV1<'_>,
        dynamic: &VerifiedSourceBackedDynamicCallableV1,
        schedule: &VerifiedCallableSemanticLoopBindingScheduleV1,
    ) -> Result<VerifiedDynamicLoopOperationSourceSetV1, DynamicLoopOperationSourceIssueV1> {
        if input.owner() != dynamic.owner() {
            return Err(DynamicLoopOperationSourceIssueV1::OwnerMismatch);
        }
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .map_err(|error| {
                DynamicLoopOperationSourceIssueV1::SourceProjection(format!("{error:?}"))
            })?;
        let dynamic_loop = dynamic
            .loops()
            .iter()
            .find(|row| row.membership().source().site().node() == schedule.loop_site())
            .ok_or(DynamicLoopOperationSourceIssueV1::LoopSourceMismatch)?;
        let [dynamic_carrier] = dynamic_loop.carriers() else {
            return Err(DynamicLoopOperationSourceIssueV1::DynamicCarrierMismatch);
        };
        let mut carrier_rows = schedule
            .rows()
            .iter()
            .filter(|row| row.class() == CallableLoopReadyBindingClassV1::Carrier);
        let carrier_row = carrier_rows
            .next()
            .ok_or(DynamicLoopOperationSourceIssueV1::CarrierCardinality)?;
        if carrier_rows.next().is_some() || carrier_row.binding() != dynamic_carrier.local() {
            return Err(DynamicLoopOperationSourceIssueV1::DynamicCarrierMismatch);
        }

        let dynamic_bindings = dynamic
            .formals()
            .iter()
            .map(|row| row.binding())
            .chain(
                dynamic
                    .local_initializations()
                    .iter()
                    .map(|row| row.local()),
            )
            .collect();
        let lexical_refs = ledger.variable_refs().collect::<BTreeMap<_, _>>();
        let assignment_targets = ledger.assignment_targets().collect::<BTreeMap<_, _>>();
        let comparison = issue_comparison(
            input.source().root(),
            &ledger,
            schedule,
            carrier_row.binding(),
            &dynamic_bindings,
            &lexical_refs,
        )?;
        let add_rebind = issue_add_rebind(
            input.source().root(),
            &ledger,
            schedule,
            carrier_row.binding(),
            &lexical_refs,
            &assignment_targets,
        )?;
        Ok(VerifiedDynamicLoopOperationSourceSetV1 {
            owner: input.owner(),
            loop_site: schedule.loop_site().clone(),
            comparison,
            add_rebind,
        })
    }
}

fn issue_comparison(
    function: &ASTNode,
    ledger: &crate::mir::resolved_semantics::CallableSemanticSourceLedgerView<'_>,
    schedule: &VerifiedCallableSemanticLoopBindingScheduleV1,
    carrier: BindingRefV1,
    dynamic_bindings: &std::collections::BTreeSet<BindingRefV1>,
    lexical_refs: &BTreeMap<&SourceExprSiteV1, &ResolvedLexicalRefV1>,
) -> Result<VerifiedDynamicLoopComparisonSourceV1, DynamicLoopOperationSourceIssueV1> {
    let operation = SourcePathV1::from_node(schedule.loop_site())
        .child(SourcePathSegmentV1::LoopCondition)
        .expr();
    let carrier_read = SourcePathV1::from_node(operation.node())
        .child(SourcePathSegmentV1::Lhs)
        .expr();
    let operand_read = SourcePathV1::from_node(operation.node())
        .child(SourcePathSegmentV1::Rhs)
        .expr();
    let Some(ProjectedSourceNodeV1::Node(ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        ..
    })) = project_source_node_v1(function, operation.node())
    else {
        return Err(DynamicLoopOperationSourceIssueV1::ComparisonNoSafeSlice);
    };
    if lexical_refs.get(&carrier_read) != Some(&&ResolvedLexicalRefV1::Local(carrier)) {
        return Err(DynamicLoopOperationSourceIssueV1::ComparisonSourceMismatch);
    }
    let Some(ResolvedLexicalRefV1::Local(operand)) =
        lexical_refs.get(&operand_read).map(|resolved| **resolved)
    else {
        return Err(DynamicLoopOperationSourceIssueV1::ComparisonSourceMismatch);
    };
    if !dynamic_bindings.contains(&operand)
        || !schedule.rows().iter().any(|row| {
            row.binding() == operand
                && row.class() == CallableLoopReadyBindingClassV1::ReadOnlyOperand
                && row.receipts().iter().any(|receipt| {
                    receipt.site() == operand_read.node()
                        && receipt.role() == CallableLoopBindingRoleV1::ConditionRead
                })
        })
    {
        return Err(DynamicLoopOperationSourceIssueV1::ComparisonOperandNotDynamic);
    }
    if !schedule.rows().iter().any(|row| {
        row.binding() == carrier
            && row.receipts().iter().any(|receipt| {
                receipt.site() == carrier_read.node()
                    && receipt.role() == CallableLoopBindingRoleV1::ConditionRead
            })
    }) {
        return Err(DynamicLoopOperationSourceIssueV1::ComparisonSourceMismatch);
    }
    require_inventory(ledger, [&operation, &carrier_read, &operand_read])?;
    Ok(VerifiedDynamicLoopComparisonSourceV1 {
        operation,
        carrier_read,
        operand_read,
        carrier,
        operand,
        kind: DynamicLoopComparisonKindV1::Less,
        result: DynamicLoopOperationResultClassV1::Bool,
    })
}

fn issue_add_rebind(
    function: &ASTNode,
    ledger: &crate::mir::resolved_semantics::CallableSemanticSourceLedgerView<'_>,
    schedule: &VerifiedCallableSemanticLoopBindingScheduleV1,
    carrier: BindingRefV1,
    lexical_refs: &BTreeMap<&SourceExprSiteV1, &ResolvedLexicalRefV1>,
    assignment_targets: &BTreeMap<&SourceExprSiteV1, &ResolvedAssignmentTargetV1>,
) -> Result<VerifiedDynamicLoopAddRebindSourceV1, DynamicLoopOperationSourceIssueV1> {
    let carrier_row = schedule
        .rows()
        .iter()
        .find(|row| row.binding() == carrier)
        .ok_or(DynamicLoopOperationSourceIssueV1::DynamicCarrierMismatch)?;
    let rebinds = carrier_row
        .receipts()
        .iter()
        .filter(|receipt| receipt.role() == CallableLoopBindingRoleV1::BodyRebind)
        .collect::<Vec<_>>();
    let [rebind] = rebinds.as_slice() else {
        return Err(DynamicLoopOperationSourceIssueV1::RebindCardinality);
    };
    let target = SourceExprSiteV1::from_node(rebind.site().clone());
    let segments = target.node().segments();
    if !matches!(segments.last(), Some(SourcePathSegmentV1::Target)) {
        return Err(DynamicLoopOperationSourceIssueV1::RebindSourceMismatch);
    }
    let statement = SourceNodeSiteV1::from_segments(segments[..segments.len() - 1].to_vec());
    let operation = SourcePathV1::from_node(&statement)
        .child(SourcePathSegmentV1::Value)
        .expr();
    let carrier_read = SourcePathV1::from_node(operation.node())
        .child(SourcePathSegmentV1::Lhs)
        .expr();
    let exact_literal = SourcePathV1::from_node(operation.node())
        .child(SourcePathSegmentV1::Rhs)
        .expr();
    let Some(ProjectedSourceNodeV1::Node(ASTNode::Assignment { value, .. })) =
        project_source_node_v1(function, &statement)
    else {
        return Err(DynamicLoopOperationSourceIssueV1::RebindNoSafeSlice);
    };
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        right,
        ..
    } = value.as_ref()
    else {
        return Err(DynamicLoopOperationSourceIssueV1::RebindNoSafeSlice);
    };
    let ASTNode::Literal {
        value: LiteralValue::Integer(delta),
        ..
    } = right.as_ref()
    else {
        return Err(DynamicLoopOperationSourceIssueV1::RebindNoSafeSlice);
    };
    if assignment_targets.get(&target) != Some(&&ResolvedAssignmentTargetV1::BindingRebind(carrier))
        || lexical_refs.get(&carrier_read) != Some(&&ResolvedLexicalRefV1::Local(carrier))
        || !carrier_row.receipts().iter().any(|receipt| {
            receipt.site() == carrier_read.node()
                && receipt.role() == CallableLoopBindingRoleV1::BodyRead
        })
    {
        return Err(DynamicLoopOperationSourceIssueV1::RebindSourceMismatch);
    }
    require_inventory(ledger, [&operation, &carrier_read, &exact_literal, &target])?;
    Ok(VerifiedDynamicLoopAddRebindSourceV1 {
        operation,
        carrier_read,
        exact_literal,
        target,
        carrier,
        delta: *delta,
        result: DynamicLoopOperationResultClassV1::Dynamic,
    })
}

fn require_inventory<const N: usize>(
    ledger: &crate::mir::resolved_semantics::CallableSemanticSourceLedgerView<'_>,
    sites: [&SourceExprSiteV1; N],
) -> Result<(), DynamicLoopOperationSourceIssueV1> {
    sites
        .into_iter()
        .all(|site| ledger.source_site_inventory().contains_expression(site))
        .then_some(())
        .ok_or(DynamicLoopOperationSourceIssueV1::IncompleteSourceInventory)
}

#[cfg(test)]
#[path = "normal_callable_dynamic_operation_source_tests.rs"]
mod tests;
