//! Source-issued classification of the comparison nodes of a checked evaluation.
//! A comparison names its operand nodes from the checked operation loan only; no MIR fact
//! or runtime tag takes part in selecting an operation.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1;
use std::rc::Rc;

impl CheckedExpressionEvaluationV1 {
    /// The original source loan alone selects an exact, frame-free Text read.
    /// Invalid exact evidence refuses rather than degrading to checked Compare.
    pub(crate) fn frame_free_text_comparison_v1(
        &self,
        node: usize,
    ) -> Result<Option<crate::mir::CompareOp>, &'static str> {
        use crate::mir::dynamic_operator_contract::{
            DynamicOperatorFaultV1 as Fault, DynamicOperatorNormalResultV1 as Normal,
            DynamicOperatorSuspensionV1 as Suspension, DynamicOperatorValueClassV1 as Class,
        };
        use crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1 as ValueClass;
        let Some(binary) = self.source.nodes().get(node).and_then(|node| node.binary()) else {
            return Ok(None);
        };
        let Some(op) = binary.operator().compare_op() else {
            return Ok(None);
        };
        let operands = [binary.lhs(), binary.rhs()].map(|site| {
            self.source
                .nodes()
                .iter()
                .position(|row| row.site() == site)
        });
        let exact_classes = operands.iter().all(|index| {
            index.is_some_and(|index| {
                self.class(index) == Some(ExpressionLeafClassV1::Value(ValueClass::String))
            })
        });
        let loan = self.operations.iter().find(|loan| loan.node() == node);
        let envelope = loan.and_then(|loan| loan.single_envelope());
        let exact_domain = envelope.is_some_and(|envelope| {
            envelope.domain().left() == Class::String && envelope.domain().right() == Class::String
        });
        if !exact_classes && !exact_domain {
            return Ok(None);
        }
        let valid = loan.zip(envelope).is_some_and(|(loan, envelope)| {
            Rc::ptr_eq(loan.source(), &self.source) && exact_classes && exact_domain
            && envelope.normal_result() == Normal::TrivialBool
            && envelope.fault() == Fault::NoSourceFault
            && envelope.suspension() == Suspension::NonSuspending
            && envelope.lifecycle().is_none()
            && crate::mir::resolved_value_profile::expression_class::comparison_operator_family_v1(
                binary.operator()) == Some(envelope.domain().family())
            && operands.iter().all(|index| index.is_some_and(|index| !self.transfers_node(index)))
        });
        if !valid {
            return Err("bool-condition/text-comparison-source");
        }
        Ok(Some(op))
    }
    /// Source-issued Integer payload check for one opaque carrier comparison.
    /// The returned nodes are (carrier, I64, carrier-on-left); no MIR fact or
    /// runtime tag participates in selecting this operation.
    pub(crate) fn carrier_integer_comparison_nodes_v1(
        &self,
        root: usize,
    ) -> Option<(usize, usize, bool)> {
        use crate::mir::dynamic_operator_contract::{
            DynamicOperatorFamilyV1 as Family, DynamicOperatorNormalResultV1 as Normal,
            DynamicOperatorValueClassV1 as Class,
        };
        let binary = self.source.nodes().get(root)?.binary()?;
        let loan = self.operations.iter().find(|loan| loan.node() == root)?;
        let envelope = loan.single_envelope()?;
        if !Rc::ptr_eq(loan.source(), &self.source)
            || envelope.normal_result() != Normal::TrivialBool
        {
            return None;
        }
        let left = self
            .source
            .nodes()
            .iter()
            .position(|node| node.site() == binary.lhs())?;
        let right = self
            .source
            .nodes()
            .iter()
            .position(|node| node.site() == binary.rhs())?;
        let (carrier, integer, carrier_left) = match (
            envelope.domain().family(),
            envelope.domain().left(),
            envelope.domain().right(),
            binary.operator(),
        ) {
            (
                Family::Greater,
                Class::Dynamic,
                Class::I64,
                crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Greater,
            )
            | (
                Family::Less,
                Class::Dynamic,
                Class::I64,
                crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Less,
            ) => (left, right, true),
            (
                Family::Less,
                Class::I64,
                Class::Dynamic,
                crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Less,
            ) => (right, left, false),
            _ => return None,
        };
        let proof = self.references.get(&carrier)?;
        if !proof.matches_node(&self.source, carrier) {
            return None;
        }
        let owner = match proof {
            CheckedExpressionReferenceV1::DynamicInvocationResult(frame) => {
                frame.dynamic_home_binding_v1()?
            }
            CheckedExpressionReferenceV1::LoopDynamic(read) => {
                read.producer().invocation_home_v1()?
            }
            _ => return None,
        };
        if !self.prior_homes.contains(&LiveHomeV1::Stable(owner))
            || self.class(carrier) != Some(ExpressionLeafClassV1::Value(crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Dynamic))
            || self.class(integer) != Some(ExpressionLeafClassV1::Value(crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::I64))
            || [carrier, integer].iter().any(|&node| self.transfers_node(node)
                || !matches!(self.source.nodes()[node].source(), ExpressionNodeSourceV1::Reference(_)
                    | ExpressionNodeSourceV1::Literal(super::super::ResolvedLiteralSourceV1::Integer(_)))) {
            return None;
        }
        Some((carrier, integer, carrier_left))
    }
    /// R16: a comparison of two exact i64 operands is one `Compare` over the operand
    /// values; no Invoke or extraction takes part. The returned nodes are (left, right).
    /// The checked loan's envelope names the domain and the Bool result; no MIR fact participates.
    pub(crate) fn exact_integer_comparison_nodes_v1(&self, root: usize) -> Option<(usize, usize)> {
        use crate::mir::dynamic_operator_contract::{
            DynamicOperatorNormalResultV1 as Normal, DynamicOperatorValueClassV1 as Class,
        };
        let binary = self.source.nodes().get(root)?.binary()?;
        let loan = self.operations.iter().find(|loan| loan.node() == root)?;
        let envelope = loan.single_envelope()?;
        if !Rc::ptr_eq(loan.source(), &self.source)
            || envelope.normal_result() != Normal::TrivialBool
            || envelope.domain().left() != Class::I64
            || envelope.domain().right() != Class::I64
            || crate::mir::resolved_value_profile::expression_class::comparison_operator_family_v1(
                binary.operator(),
            ) != Some(envelope.domain().family())
        {
            return None;
        }
        let left = self
            .source
            .nodes()
            .iter()
            .position(|node| node.site() == binary.lhs())?;
        let right = self
            .source
            .nodes()
            .iter()
            .position(|node| node.site() == binary.rhs())?;
        let i64_class = Some(ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::I64,
        ));
        (self.class(left) == i64_class
            && self.class(right) == i64_class
            && !self.transfers_node(left)
            && !self.transfers_node(right))
        .then_some((left, right))
    }
}

impl CheckedExpressionEvaluationV1 {
    pub(super) fn comparison_input_order_v1(
        &self,
        comparison: usize,
        allow_dynamic_call: bool,
    ) -> Option<Vec<usize>> {
        self.text_comparison_operation_v1(comparison, allow_dynamic_call)?;
        let root = self.source.nodes().get(comparison)?;
        let prefix = root.site().node().segments();
        let order: Vec<_> = self
            .order
            .iter()
            .copied()
            .filter(|node| {
                *node != comparison
                    && self.source.nodes()[*node]
                        .site()
                        .node()
                        .segments()
                        .starts_with(prefix)
                    && (!allow_dynamic_call || !self.transfers_node(*node))
            })
            .collect();
        if allow_dynamic_call {
            let dynamic_positions: Vec<_> = order
                .iter()
                .enumerate()
                .filter_map(|(position, node)| {
                    self.calls
                        .iter()
                        .any(|call| {
                            call.site().site() == self.source.nodes()[*node].site()
                                && call.dynamic_invocation_result_lifecycle_v1().is_some()
                        })
                        .then_some(position)
                })
                .collect();
            if dynamic_positions.len() > 1
                || dynamic_positions.first().is_some_and(|position| {
                    order[*position + 1..].iter().any(|node| {
                        !matches!(
                            self.source.nodes()[*node].source(),
                            ExpressionNodeSourceV1::Reference(_)
                                | ExpressionNodeSourceV1::Literal(
                                    super::super::ResolvedLiteralSourceV1::String(_)
                                        | super::super::ResolvedLiteralSourceV1::Integer(_)
                                )
                        )
                    })
                })
            {
                return None;
            }
        }
        for &node in &order {
            let original = &self.source.nodes()[node];
            if self.transfers_node(node)
                || self.node_is_identity(node)
                || !matches!(original.children(), crate::mir::resolved_value_profile::expression_children::ExpressionChildrenV1::Known(children)
                    if children.iter().all(|child| child.reach == crate::mir::resolved_value_profile::expression_children::ExpressionChildReachV1::Always))
            {
                return None;
            }
            match original.source() {
                ExpressionNodeSourceV1::Reference(_) | ExpressionNodeSourceV1::Literal(
                    super::super::ResolvedLiteralSourceV1::Integer(_) | super::super::ResolvedLiteralSourceV1::String(_)) => {},
                ExpressionNodeSourceV1::MethodCall
                    if self.cores.get(&node).is_some_and(|core| core.matches_plan(&self.source, node))
                        || allow_dynamic_call && self.calls.iter().any(|call|
                            call.site().site() == self.source.nodes()[node].site()
                                && call.dynamic_invocation_result_lifecycle_v1().is_some()) => {},
                ExpressionNodeSourceV1::Binary(binary) if binary.operator() == crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Add
                    && self.class(node) == Some(ExpressionLeafClassV1::Value(
                        crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::I64)) => {},
                _ => return None,
            }
        }
        (!order.is_empty()).then_some(order)
    }
}
