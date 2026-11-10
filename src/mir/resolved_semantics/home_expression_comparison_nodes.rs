//! One source-derived comparison plan; physical writers bind its ordered operands.
//! This conversion owns representation selection, never source admission or MIR validation.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ComparisonExecutionKindV1 {
    Scalar(super::super::super::SourceScalarKind),
    Null,
    Text,
    /// An opaque integer extraction needs the original carrier orientation;
    /// otherwise the existing checked Text operation supplies the Bool directly.
    Checked { integer_carrier_left: Option<bool> },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ComparisonExecutionPlanV1 {
    operator: crate::mir::CompareOp,
    operands: Option<[usize; 2]>,
    kind: ComparisonExecutionKindV1,
}
impl ComparisonExecutionPlanV1 {
    pub(crate) fn operator(self) -> crate::mir::CompareOp { self.operator }
    pub(crate) fn operands(self) -> Option<[usize; 2]> { self.operands }
    pub(crate) fn kind(self) -> ComparisonExecutionKindV1 { self.kind }
    pub(crate) fn requires_fault_frame(self) -> bool {
        matches!(self.kind, ComparisonExecutionKindV1::Checked { .. })
    }
}

impl CheckedExpressionEvaluationV1 {
    pub(crate) fn comparison_plan_v1(&self, node: usize) -> Result<Option<ComparisonExecutionPlanV1>, &'static str> {
        comparison_execution_plan_v1(ComparisonEvidenceV1::Checked(self, node))
    }
    /// The only execution-form conversion. Immutable source laws and exact
    /// operand evidence are required together; neither MIR nor ABI chooses a form.
    fn checked_comparison_plan_v1(
        &self, node: usize,
    ) -> Result<Option<ComparisonExecutionPlanV1>, &'static str> {
        use crate::mir::dynamic_operator_contract::{
            DynamicOperatorFaultV1 as Fault, DynamicOperatorNormalResultV1 as Normal,
            DynamicOperatorSuspensionV1 as Suspension, DynamicOperatorValueClassV1 as Class,
        };
        use crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1 as Value;
        use ComparisonExecutionKindV1 as Kind;
        let Some(binary) = self.source.nodes().get(node).and_then(|row| row.binary()) else { return Ok(None); };
        let Some(operator) = binary.operator().compare_op() else { return Ok(None); };
        let mut loans = self.operations.iter().filter(|loan| loan.node() == node);
        let Some(loan) = loans.next() else { return Ok(None); };
        let error = "bool-condition/text-comparison-source";
        if loans.next().is_some() || !Rc::ptr_eq(loan.source(), &self.source) { return Err(error); }
        let index = |site| self.source.nodes().iter().position(|row| row.site() == site).ok_or(error);
        let operands = [index(binary.lhs())?, index(binary.rhs())?];
        let plan = |kind| Some(ComparisonExecutionPlanV1 { operator, operands: Some(operands), kind });
        if loan.nullable_field_null_v1().is_some() || loan.nullable_null_v1().is_some() {
            return Ok(plan(Kind::Null));
        }
        let Some(envelope) = loan.single_envelope() else { return Ok(None); };
        let domain = envelope.domain();
        if envelope.normal_result() != Normal::TrivialBool
            || crate::mir::resolved_value_profile::expression_class::comparison_operator_family_v1(binary.operator()) != Some(domain.family()) {
            return Err(error);
        }
        let exact = match (domain.left(), domain.right()) {
            (Class::I64, Class::I64) => Some((Value::I64, Kind::Scalar(super::super::super::SourceScalarKind::Integer))),
            (Class::Bool, Class::Bool) => Some((Value::Bool, Kind::Scalar(super::super::super::SourceScalarKind::Bool))),
            (Class::String, Class::String) => Some((Value::String, Kind::Text)),
            _ => None,
        };
        if let Some((class, kind)) = exact {
            if envelope.fault() != Fault::NoSourceFault
                || envelope.suspension() != Suspension::NonSuspending
                || envelope.lifecycle().is_some()
                || operands.iter().any(|&node| self.class(node) != Some(ExpressionLeafClassV1::Value(class)) || self.transfers_node(node)) {
                return Err(error);
            }
            return Ok(plan(kind));
        }
        if matches!((domain.left(), domain.right()), (Class::Dynamic, Class::Null) | (Class::Null, Class::Dynamic)) {
            if operator != crate::mir::CompareOp::Eq || envelope.fault() != Fault::NoSourceFault
                || envelope.suspension() != Suspension::NonSuspending || envelope.lifecycle().is_some()
                || operands.iter().filter(|&&node| matches!(self.source.nodes()[node].source(),
                    ExpressionNodeSourceV1::Literal(super::super::super::ResolvedLiteralSourceV1::Null))).count() != 1 {
                return Err(error);
            }
            return Ok(plan(Kind::Null));
        }
        let carrier_left = match (domain.left(), domain.right(), operator) {
            (Class::Dynamic, Class::I64, crate::mir::CompareOp::Lt | crate::mir::CompareOp::Gt) => Some(true),
            (Class::I64, Class::Dynamic, crate::mir::CompareOp::Lt) => Some(false),
            _ => None,
        };
        if let Some(carrier_left) = carrier_left {
            let [left, right] = operands;
            let (carrier, integer) = if carrier_left { (left, right) } else { (right, left) };
            return Ok(self.checked_integer_carrier_home_v1(carrier, integer).is_some()
                .then(|| plan(Kind::Checked { integer_carrier_left: Some(carrier_left) })).flatten());
        }
        if matches!((domain.left(), domain.right()), (Class::String, Class::Dynamic) | (Class::Dynamic, Class::String)) {
            return Ok(plan(Kind::Checked { integer_carrier_left: None }));
        }
        Ok(None)
    }
    /// Validate the existing opaque Integer view's retained Home and original reads.
    fn checked_integer_carrier_home_v1(&self, carrier: usize, integer: usize) -> Option<()> {
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
        Some(())
    }
}

impl CheckedExpressionEvaluationV1 {
    pub(super) fn comparison_input_order_v1(
        &self,
        comparison: usize,
        allow_dynamic_call: bool,
    ) -> Option<Vec<usize>> {
        let plan = self.comparison_plan_v1(comparison).ok()??;
        if !matches!(plan.kind(), ComparisonExecutionKindV1::Text)
            && !(allow_dynamic_call && matches!(plan.kind(), ComparisonExecutionKindV1::Checked { integer_carrier_left: None })) {
            return None;
        }
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


/// Original syntax facts do not grant carrier admission or a source loan.
enum ComparisonEvidenceV1<'a> {
    Checked(&'a CheckedExpressionEvaluationV1, usize),
    Original(&'a crate::mir::resolved_semantics::ResolvedExpressionSourceInventoryV1,
        &'a crate::mir::resolved_semantics::SourceExprSiteV1),
}
fn comparison_execution_plan_v1(evidence: ComparisonEvidenceV1<'_>)
    -> Result<Option<ComparisonExecutionPlanV1>, &'static str> {
    match evidence {
        ComparisonEvidenceV1::Checked(frame, node) => frame.checked_comparison_plan_v1(node),
        ComparisonEvidenceV1::Original(source, site) => {
            let Some(binary) = source.binary(site) else { return Ok(None); };
            let Some(operator) = binary.operator().compare_op()
                .filter(|op| matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne)) else { return Ok(None); };
            let null = |site| matches!(source.literal(site), Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Null));
            Ok((null(binary.lhs()) ^ null(binary.rhs())).then_some(ComparisonExecutionPlanV1 {
                operator, operands: None, kind: ComparisonExecutionKindV1::Null,
            }))
        }
    }
}
impl ComparisonExecutionPlanV1 {
    /// The old raw source comparison keeps its executable admission with the
    /// existing owner. Only its original exact-null representation is selected here.
    pub(crate) fn original_null_v1(source: &crate::mir::resolved_semantics::ResolvedExpressionSourceInventoryV1,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<Option<Self>, &'static str> {
        comparison_execution_plan_v1(ComparisonEvidenceV1::Original(source, site))
    }
    pub(crate) fn instruction_v1(self, dst: crate::mir::ValueId, lhs: crate::mir::ValueId, rhs: crate::mir::ValueId)
        -> Option<crate::mir::MirInstruction> {
        match self.kind {
            ComparisonExecutionKindV1::Null => Some(crate::mir::MirInstruction::NullCompare { dst, op: self.operator, lhs, rhs }),
            ComparisonExecutionKindV1::Scalar(_) => Some(crate::mir::MirInstruction::Compare { dst, op: self.operator, lhs, rhs }),
            _ => None,
        }
    }
}
