//! Checked body of one original expression, borrowed by its purpose wrapper.
//! Acquisitions are source templates; only the physical Normal edge owns a value.
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::home_new_prefix::local_flow::PrefixLocalFlow;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourceExprSiteV1};
use crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1;
use std::collections::BTreeSet;

/// Templates for source-node acquisitions, not already acquired runtime owners.
/// Only the matching checked operation Normal may acquire an owned String.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedExpressionEvaluationV1 {
    pub(super) source: Rc<ExpressionSourcePlanV1>,
    pub(super) prior_homes: Box<[LiveHomeV1]>,
    pub(super) ownership: ExpressionOwnershipV1,
    pub(super) order: Box<[usize]>,
    pub(super) classes: Box<[ExpressionLeafClassV1]>,
    pub(super) calls: Box<[LocalCallObservationV1]>,
    pub(super) cores: std::collections::BTreeMap<
        usize,
        crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1,
    >,
    pub(super) references: std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>,
    pub(super) operations: Box<[Rc<CheckedExpressionOperationLoanV1>]>,
    pub(super) owned_results: Box<[usize]>,
    pub(super) transferred_inputs: BTreeSet<usize>,
}
impl CheckedExpressionEvaluationV1 {
    /// The retained source laws, not an emitted opcode, decide whether this
    /// evaluation or its terminal consumer can need a Fault frame.
    pub(crate) fn requires_fault_frame_v1(
        &self,
        consumer_frame_free_proved: bool,
        mut checked_field: impl FnMut(&OwnedExprSiteV1) -> Option<bool>,
    ) -> bool {
        use crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1;
        use crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1;
        if !consumer_frame_free_proved
            || !matches!(
                self.root_class(),
                ExpressionLeafClassV1::Value(
                    ExpressionValueClassV1::I64 | ExpressionValueClassV1::Bool
                )
            )
            || !self.calls.is_empty()
            || !self.owned_results.is_empty()
            || matches!(
                self.source.root(),
                ExpressionPlanRootV1::Statement {
                    role: StatementExpressionRoleV1::PrintValue,
                    ..
                }
            )
            || self
                .cores
                .values()
                .any(|core| core.source().target().frame_free_read_law_v1().is_none())
        {
            return true;
        }
        self.order.iter().any(|&index| {
            let node = &self.source.nodes()[index];
            let checked = if matches!(
                node.source(),
                ExpressionNodeSourceV1::Shape(
                    crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }
                )
            ) {
                checked_field(&OwnedExprSiteV1::new(
                    self.source.owner(),
                    node.site().clone(),
                ))
            } else {
                None
            };
            self.node_requires_fault_frame_v1(index, checked)
        })
    }

    /// One node's retained operation law, shared by frame and cleanup admission.
    pub(crate) fn node_requires_fault_frame_v1(
        &self,
        index: usize,
        checked_field: Option<bool>,
    ) -> bool {
        let Some(node) = self.source.nodes().get(index) else {
            return true;
        };
        if let Some(core) = self.cores.get(&index) {
            return core.source().target().frame_free_read_law_v1().is_none();
        }
        if matches!(node.source(), ExpressionNodeSourceV1::MethodCall) {
            return true;
        }
        if self.owned_results.contains(&index) {
            return true;
        }
        if let Some(loan) = self.operations.iter().find(|loan| loan.node() == index) {
            return loan.scalar_copy_v1().is_none()
                && loan.integer_operator_v1().is_none()
                && !matches!(self.comparison_plan_v1(index), Ok(Some(plan)) if !plan.requires_fault_frame());
        }
        matches!(
            node.source(),
            ExpressionNodeSourceV1::Shape(
                crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }
            )
        ) && checked_field != Some(false)
    }

    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        &self.source
    }
    pub(crate) fn ownership_v1(&self) -> &ExpressionOwnershipV1 {
        &self.ownership
    }

    pub(crate) fn prior_homes(&self) -> &[LiveHomeV1] {
        &self.prior_homes
    }
    pub(crate) fn ordered_nodes(&self) -> &[usize] {
        &self.order
    }
    pub(crate) fn calls(&self) -> &[LocalCallObservationV1] {
        &self.calls
    }
    pub(crate) fn cores(
        &self,
    ) -> &std::collections::BTreeMap<
        usize,
        crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1,
    > {
        &self.cores
    }
    /// A Dynamic local may act as an opaque String carrier only when this
    /// exact read is backed by its live, checked invocation-result Home.
    /// This keeps arbitrary Dynamic values out of the String operation domain.
    pub(crate) fn dynamic_local_carrier_read_v1(&self, node: usize) -> bool {
        let Some(reference @ CheckedExpressionReferenceV1::DynamicInvocationResult(initializer)) =
            self.references.get(&node)
        else {
            return false;
        };
        reference.matches_node(&self.source, node)
            && initializer.is_dynamic_value()
            && initializer.binding().owner() == self.source.owner()
            && initializer.source().owner() == self.source.owner()
            && !self.owned_result_nodes().contains(&node)
            && initializer
                .dynamic_home_binding_v1()
                .is_some_and(|owner| self.prior_homes.contains(&LiveHomeV1::Stable(owner)))
    }
    /// A Dynamic invocation-result local read on the String Add right side is
    /// consumed as an opaque carrier, never as a tagged formal.
    pub(crate) fn string_add_right_operand_v1(
        &self,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        operand: crate::mir::instruction::StringInvokeOperand,
    ) -> crate::mir::instruction::StringInvokeOperand {
        use crate::mir::instruction::StringInvokeOperand as Operand;
        let Some(node) = self
            .source
            .nodes()
            .iter()
            .position(|node| node.site() == site)
        else {
            return operand;
        };
        match operand {
            Operand::Dynamic(carrier) if self.dynamic_local_carrier_read_v1(node) => {
                Operand::DynamicCarrier(carrier)
            }
            Operand::NormalInteger(value)
                if matches!(self.references.get(&node),
                Some(CheckedExpressionReferenceV1::Scalar(read)) if read.kind() == super::super::SourceScalarKind::Bool) =>
            {
                Operand::NormalBool(value)
            }
            _ => operand,
        }
    }
    pub(crate) fn references(
        &self,
    ) -> &std::collections::BTreeMap<usize, CheckedExpressionReferenceV1> {
        &self.references
    }
    /// Install source-qualified Core receiver reads into the same reference
    /// map consumed by expression evaluation and original-use closure.
    pub(crate) fn retain_core_text_entry_reads_v1(&mut self) -> bool {
        let mut reads = std::collections::BTreeMap::new();
        for core in self.cores.values() {
            let Some(read) = core.text_entry_receiver_read() else {
                continue;
            };
            if !core.matches_plan(&self.source, core.node())
                || !Rc::ptr_eq(read.source_plan(), &self.source)
                || reads
                    .insert(read.node(), read.clone())
                    .is_some_and(|previous| previous != *read)
            {
                return false;
            }
        }
        for (node, read) in reads {
            match self.references.get(&node) {
                Some(CheckedExpressionReferenceV1::ArgumentEntry(previous))
                    if previous == &read => {}
                // Initializer qualification may already hold the stronger
                // finite-domain entry read for this exact source node. The
                // Core receiver contract is co-located in that proof's
                // requirement; retain one source reference, not two lanes
                // competing for the same evaluated read.
                Some(CheckedExpressionReferenceV1::FiniteEntry { read: finite, .. })
                    if finite.requirement() == &read => {}
                Some(_) => return false,
                None => {
                    self.references
                        .insert(node, CheckedExpressionReferenceV1::ArgumentEntry(read));
                }
            }
        }
        true
    }
    /// Every active value reference has exactly one retained original proof.
    /// Receiver identities and transferred ordinary-call inputs keep their own
    /// owners; neither becomes a reference permission in this evaluation.
    pub(crate) fn reference_coverage_is_exact(&self) -> bool {
        let expected: BTreeSet<_> = self
            .order
            .iter()
            .copied()
            .filter(|&node| {
                !self.transfers_node(node)
                    && !self.node_is_identity(node)
                    && matches!(
                        self.source.nodes()[node].source(),
                        ExpressionNodeSourceV1::Reference(_)
                    )
            })
            .collect();
        expected.len() == self.references.len()
            && self.references.iter().all(|(&node, proof)| {
                expected.contains(&node) && proof.matches_node(&self.source, node)
            })
    }
    pub(crate) fn operations(&self) -> &[Rc<CheckedExpressionOperationLoanV1>] {
        &self.operations
    }
    pub(crate) fn owned_result_nodes(&self) -> &[usize] {
        &self.owned_results
    }
    pub(crate) fn class(&self, node: usize) -> Option<ExpressionLeafClassV1> {
        self.classes.get(node).copied()
    }
    pub(crate) fn root_class(&self) -> ExpressionLeafClassV1 {
        self.classes[0]
    }
    /// Borrow only the original eager inputs of one String comparison. The
    /// enclosing Bool tree still owns conditional reach; no AST is re-rooted.
    pub(crate) fn text_comparison_input_order_v1(&self, comparison: usize) -> Option<Vec<usize>> {
        self.comparison_input_order_v1(comparison, false)
    }
    /// Bool conditions may additionally compare one exact owned Dynamic call
    /// result with String. The Loop recipe keeps its narrower text-only lane.
    pub(crate) fn bool_condition_comparison_input_order_v1(
        &self,
        comparison: usize,
    ) -> Option<Vec<usize>> {
        self.comparison_input_order_v1(comparison, true)
    }
    /// Operation eligibility is independent of a purpose's input schedule.
    /// The complete expression and a conditional subtree lend the same source loan.
    /// The same eager-input scope serves Text comparison and synchronous
    /// field presence. Presence currently requires a passive exact Text leaf:
    /// computed key effects must not move ahead of receiver field evaluation.
    pub(crate) fn scalar_invoke_input_order_v1(&self, root: usize) -> Option<Vec<usize>> {
        if let Some(plan) = self.comparison_plan_v1(root).ok().flatten() {
            if matches!(
                plan.kind(),
                ComparisonExecutionKindV1::Checked {
                    integer_carrier_left: Some(_)
                }
            ) {
                return Some(plan.operands()?.to_vec());
            }
        }
        if let Some(order) = self.bool_condition_comparison_input_order_v1(root) {
            return Some(order);
        }
        let core = self.cores.get(&root)?;
        if !core.matches_plan(&self.source, root)
            || core.field_map().is_none()
            || !core.source().target().checked_map_presence_law_v1()
        {
            return None;
        }
        let [argument] = core.source().source().arguments() else {
            return None;
        };
        let node = self
            .source
            .nodes()
            .iter()
            .position(|node| node.site() == argument.site())?;
        let original = &self.source.nodes()[node];
        if self.class(node) != Some(ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String))
            || self.transfers_node(node)
            || !matches!(original.children(), crate::mir::resolved_value_profile::expression_children::ExpressionChildrenV1::Known(children) if children.is_empty())
            || !matches!(original.source(), ExpressionNodeSourceV1::Reference(_)
                | ExpressionNodeSourceV1::Literal(super::super::ResolvedLiteralSourceV1::String(_))) { return None; }
        Some(vec![node])
    }
    pub(crate) fn transfers_node(&self, node: usize) -> bool {
        self.transferred_inputs.contains(&node)
    }
    pub(crate) fn owned_root_node_v1(&self) -> Option<usize> {
        let mut node = 0;
        while let Some(tail) = self.source.transparent_tail_node_v1(node) {
            node = tail;
        }
        self.owned_result_nodes().contains(&node).then_some(node)
    }
    /// Owned carrier and owned Text are distinct acquisitions. A proved Text
    /// payload still needs a snapshot before a Text destination can own it.
    pub(crate) fn text_snapshot_root_v1(&self) -> Option<usize> {
        let node = self.owned_root_node_v1()?;
        (self.class(node) == Some(ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String))
            && self.cores.get(&node).is_some_and(|core|
                core.is_array_dynamic_get_v1() && core.proves_text_result_v1())).then_some(node)
    }
    pub(crate) fn owned_text_root_node_v1(&self) -> Option<usize> {
        let node = self.owned_root_node_v1()?;
        (self.class(node) == Some(ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String))
            && self.text_snapshot_root_v1().is_none()).then_some(node)
    }
    pub(crate) fn direct_array_receiver_v1(
        &self,
        node: usize,
    ) -> Option<&crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1> {
        let site = self.source.nodes().get(node)?.site();
        let mut cores = self.cores.values().filter(|core| {
            core.matches_plan(&self.source, core.node())
                && core.field_array_access_v1().is_some_and(|array| {
                    array.receiver_alias_v1().is_none()
                        && array.receiver().site() == site
                        && array.matches_expression_receiver_v1(&self.source)
                })
        });
        let core = cores.next()?;
        cores.next().is_none().then_some(core)
    }
    pub(crate) fn node_is_identity(&self, node: usize) -> bool {
        self.classes.get(node) == Some(&ExpressionLeafClassV1::Identity)
    }
}

pub(super) fn owned_result_nodes(
    order: &[usize],
    add: &[usize],
    cores: &std::collections::BTreeMap<
        usize,
        crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1,
    >,
    calls: &std::collections::BTreeMap<usize, LocalCallObservationV1>,
) -> Box<[usize]> {
    order.iter().copied().filter(|node| add.contains(node) || cores.get(node).is_some_and(|core| core.owns_result())
        || calls.get(node).is_some_and(|call| call.result() == super::super::LocalCallResultClassV1::Text
            && matches!(call.result_source(), Some(super::super::LocalCallResultSourceV1::Ordinary(loan)) if loan.result() == super::super::LocalCallResultClassV1::Text))
        || calls.get(node).is_some_and(|call| call.dynamic_invocation_result_lifecycle_v1().is_some())).collect()
}

pub(crate) use super::references::{CheckedExpressionReferenceV1, CheckedScalarReferenceV1};
pub(super) use super::references::{seed_expression_references_v1, seed_current_expression_references_with_loop_text_v1};

/// Shared original Text evaluation for terminal values and passive Map keys.
/// The purpose chooses the source root; this function grants no result/Home ABI.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn qualify_text_evaluation_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: std::rc::Rc<
        crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
    >,
    purpose_site: &OwnedExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    homes: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
    calls: &[crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1],
    actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
    field_text_source: Option<
        &crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1,
    >,
) -> Result<
    Result<
        std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
        &'static str,
    >,
    E,
> {
    let (evaluation, fields) = match prepare_value_evaluation_v1(
        input,
        source,
        crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String,
        purpose_site,
        locals,
        homes,
        calls,
        actuals,
        field_text_source,
        &std::collections::BTreeMap::new(),
        None,
        &mut |_| None,
    )? {
        Ok(prepared) => prepared,
        Err(reason) => return Ok(Err(reason)),
    };
    Ok(if fields.is_empty() {
        Ok(evaluation)
    } else {
        Err("evaluation-requests-field-reads")
    })
}

#[path = "home_expression_value_preparation.rs"]
mod value_preparation;
pub(in crate::mir::resolved_semantics::home_new_prefix) use value_preparation::prepare_value_evaluation_v1;

#[path = "home_expression_comparison_nodes.rs"]
mod comparison_nodes;

pub(crate) use comparison_nodes::{ComparisonExecutionKindV1, ComparisonExecutionPlanV1};

#[path = "home_expression_ownership.rs"]
mod ownership;
pub(crate) use ownership::{
    attach_expression_entry_facts_v1, expression_entry_facts_at_v1, ExpressionCallEntryFactsV1,
    ExpressionOwnershipStateV1, ExpressionOwnershipV1,
};
