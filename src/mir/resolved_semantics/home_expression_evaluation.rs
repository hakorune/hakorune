//! Checked body of one original expression, borrowed by its purpose wrapper.
//! Acquisitions are source templates; only the physical Normal edge owns a value.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1;
use crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1;
use std::collections::BTreeSet;

/// Templates for source-node acquisitions, not already acquired runtime owners.
/// Only checked Add Normal may acquire an owned String at one of these nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedExpressionEvaluationV1 {
    pub(super) source: Rc<ExpressionSourcePlanV1>,
    pub(super) prior_homes: Box<[crate::mir::resolved_semantics::BindingRefV1]>,
    pub(super) order: Box<[usize]>,
    pub(super) classes: Box<[ExpressionLeafClassV1]>,
    pub(super) calls: Box<[LocalCallObservationV1]>,
    pub(super) operations: Box<[Rc<CheckedExpressionOperationLoanV1>]>,
    pub(super) owned_results: Box<[usize]>,
    pub(super) transferred_inputs: BTreeSet<usize>,
}
impl CheckedExpressionEvaluationV1 {
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> { &self.source }
    pub(crate) fn prior_homes(&self) -> &[crate::mir::resolved_semantics::BindingRefV1] { &self.prior_homes }
    pub(crate) fn ordered_nodes(&self) -> &[usize] { &self.order }
    pub(crate) fn calls(&self) -> &[LocalCallObservationV1] { &self.calls }
    pub(crate) fn operations(&self) -> &[Rc<CheckedExpressionOperationLoanV1>] { &self.operations }
    pub(crate) fn owned_result_nodes(&self) -> &[usize] { &self.owned_results }
    pub(crate) fn root_class(&self) -> ExpressionLeafClassV1 { self.classes[0] }
    pub(crate) fn transfers_node(&self, node: usize) -> bool { self.transferred_inputs.contains(&node) }
    pub(crate) fn node_is_identity(&self, node: usize) -> bool {
        self.classes.get(node) == Some(&ExpressionLeafClassV1::Identity)
    }
}

