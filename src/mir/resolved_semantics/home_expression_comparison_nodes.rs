//! Source-issued classification of the comparison nodes of a checked evaluation.
//! A comparison names its operand nodes from the checked operation loan only; no MIR fact
//! or runtime tag takes part in selecting an operation.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1;
use std::rc::Rc;

impl CheckedExpressionEvaluationV1 {
    /// Source-issued Integer payload check for one opaque carrier comparison.
    /// The returned nodes are (carrier, I64, carrier-on-left); no MIR fact or
    /// runtime tag participates in selecting this operation.
    pub(crate) fn carrier_integer_comparison_nodes_v1(&self, root: usize) -> Option<(usize, usize, bool)> {
        use crate::mir::dynamic_operator_contract::{DynamicOperatorFamilyV1 as Family,
            DynamicOperatorValueClassV1 as Class, DynamicOperatorNormalResultV1 as Normal};
        let binary = self.source.nodes().get(root)?.binary()?;
        let loan = self.operations.iter().find(|loan| loan.node() == root)?;
        let envelope = loan.single_envelope()?;
        if !Rc::ptr_eq(loan.source(), &self.source) || envelope.normal_result() != Normal::TrivialBool { return None; }
        let left = self.source.nodes().iter().position(|node| node.site() == binary.lhs())?;
        let right = self.source.nodes().iter().position(|node| node.site() == binary.rhs())?;
        let (carrier, integer, carrier_left) = match (envelope.domain().family(), envelope.domain().left(), envelope.domain().right(), binary.operator()) {
            (Family::Greater, Class::Dynamic, Class::I64, crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Greater)
            | (Family::Less, Class::Dynamic, Class::I64, crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Less) => (left, right, true),
            (Family::Less, Class::I64, Class::Dynamic, crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Less) => (right, left, false),
            _ => return None,
        };
        let proof = self.references.get(&carrier)?;
        if !proof.matches_node(&self.source, carrier) { return None; }
        let owner = match proof {
            CheckedExpressionReferenceV1::DynamicInvocationResult(frame) => frame.dynamic_home_binding_v1()?,
            CheckedExpressionReferenceV1::LoopDynamic(read) => read.producer().invocation_home_v1()?,
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
        use crate::mir::dynamic_operator_contract::{DynamicOperatorNormalResultV1 as Normal,
            DynamicOperatorValueClassV1 as Class};
        let binary = self.source.nodes().get(root)?.binary()?;
        let loan = self.operations.iter().find(|loan| loan.node() == root)?;
        let envelope = loan.single_envelope()?;
        if !Rc::ptr_eq(loan.source(), &self.source)
            || envelope.normal_result() != Normal::TrivialBool
            || envelope.domain().left() != Class::NormalInteger
            || envelope.domain().right() != Class::NormalInteger
            || crate::mir::resolved_value_profile::expression_class::comparison_operator_family_v1(binary.operator()) != Some(envelope.domain().family())
        {
            return None;
        }
        let left = self.source.nodes().iter().position(|node| node.site() == binary.lhs())?;
        let right = self.source.nodes().iter().position(|node| node.site() == binary.rhs())?;
        let i64_class = Some(ExpressionLeafClassV1::Value(crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::I64));
        (self.class(left) == i64_class && self.class(right) == i64_class
            && !self.transfers_node(left) && !self.transfers_node(right)).then_some((left, right))
    }
}
