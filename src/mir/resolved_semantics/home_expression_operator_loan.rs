//! Checked operator laws shared by all expression purposes.
use super::*;
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedExpressionOperationLoanV1 {
    pub(super) plan: Rc<ExpressionSourcePlanV1>,
    pub(super) node: usize,
    pub(super) proof: CheckedExpressionOperationProofV1,
}
/// Raw nullable identity is a separate closed law, never a tagged operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CheckedExpressionOperationProofV1 {
    ScalarCopy(super::super::SourceScalarKind),
    Dynamic(Box<[&'static VerifiedDynamicOperatorExecutionEnvelopeV1]>),
    Scalar(Rc<ExpressionScalarOperationLoanV1>),
    NullableAliasNull {
        reference: usize,
        null: usize,
        binding: crate::mir::resolved_semantics::BindingRefV1,
        root: crate::mir::resolved_semantics::BindingRefV1,
    },
    NullableFieldNull {
        reference: usize,
        null: usize,
        request: LocalFieldReadRequestV1,
    },
    NullableNull {
        reference: usize,
        null: usize,
        frame: NullableResultReferenceV1,
    },
}
impl CheckedExpressionOperationLoanV1 {
    pub(crate) fn scalar_copy_v1(&self) -> Option<(usize, super::super::SourceScalarKind)> {
        let CheckedExpressionOperationProofV1::ScalarCopy(kind) = self.proof else {
            return None;
        };
        Some((self.plan.copy_operand_node_v1(self.node)?, kind))
    }
    pub(crate) fn node(&self) -> usize {
        self.node
    }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        &self.plan
    }
    pub(crate) fn envelopes(&self) -> &[&'static VerifiedDynamicOperatorExecutionEnvelopeV1] {
        match &self.proof {
            CheckedExpressionOperationProofV1::Dynamic(rows) => rows,
            _ => &[],
        }
    }
    pub(crate) fn single_envelope(
        &self,
    ) -> Option<&'static VerifiedDynamicOperatorExecutionEnvelopeV1> {
        match self.envelopes() {
            [envelope] => Some(*envelope),
            _ => None,
        }
    }
    pub(crate) fn normal_result_v1(
        &self,
    ) -> Option<crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1> {
        use crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1 as ResultKind;
        match &self.proof {
            CheckedExpressionOperationProofV1::ScalarCopy(kind) => Some(match kind {
                super::super::SourceScalarKind::Integer => ResultKind::NormalInteger,
                super::super::SourceScalarKind::Bool => ResultKind::TrivialBool,
            }),
            CheckedExpressionOperationProofV1::Scalar(_) => self
                .integer_operator_v1()
                .map(|_| ResultKind::NormalInteger),
            CheckedExpressionOperationProofV1::Dynamic(envelopes) => {
                let result = envelopes.first()?.normal_result();
                envelopes
                    .iter()
                    .all(|envelope| envelope.normal_result() == result)
                    .then_some(result)
            }
            CheckedExpressionOperationProofV1::NullableAliasNull { .. } => {
                Some(ResultKind::TrivialBool)
            }
            CheckedExpressionOperationProofV1::NullableNull { .. }
            | CheckedExpressionOperationProofV1::NullableFieldNull { .. } => None,
        }
    }
    pub(crate) fn nullable_alias_null_v1(&self) -> Option<(usize, usize)> {
        let CheckedExpressionOperationProofV1::NullableAliasNull {
            reference,
            null,
            binding,
            root,
        } = &self.proof
        else {
            return None;
        };
        let binary = self.plan.nodes().get(self.node)?.binary()?;
        let read = self.plan.nodes().get(*reference)?;
        let empty = self.plan.nodes().get(*null)?;
        (binding.owner() == self.plan.owner() && root.owner() == self.plan.owner()
            && matches!(binary.operator(), ResolvedBinaryOperatorV1::Equal | ResolvedBinaryOperatorV1::NotEqual)
            && matches!(read.source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(original)) if original == binding)
            && matches!(empty.source(), ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::Null))
            && ((read.site() == binary.lhs() && empty.site() == binary.rhs())
                || (read.site() == binary.rhs() && empty.site() == binary.lhs())))
            .then_some((*reference, *null))
    }
    pub(crate) fn nullable_field_null_v1(
        &self,
    ) -> Option<(usize, usize, &LocalFieldReadRequestV1)> {
        let CheckedExpressionOperationProofV1::NullableFieldNull {
            reference,
            null,
            request,
        } = &self.proof
        else {
            return None;
        };
        let binary = self.plan.nodes().get(self.node)?.binary()?;
        let read = self.plan.nodes().get(*reference)?;
        let empty = self.plan.nodes().get(*null)?;
        (request.node_in_plan(&self.plan) == Some(*reference)
            && request.is_nullable_object_comparison_v1(&self.plan)
            && binary.operator() == ResolvedBinaryOperatorV1::Equal
            && matches!(
                empty.source(),
                ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::Null)
            )
            && ((read.site() == binary.lhs() && empty.site() == binary.rhs())
                || (read.site() == binary.rhs() && empty.site() == binary.lhs())))
        .then_some((*reference, *null, request))
    }
    pub(crate) fn nullable_null_v1(&self) -> Option<(usize, usize, &NullableResultReferenceV1)> {
        let CheckedExpressionOperationProofV1::NullableNull {
            reference,
            null,
            frame,
        } = &self.proof
        else {
            return None;
        };
        let binary = self.plan.nodes().get(self.node)?.binary()?;
        let read = self.plan.nodes().get(*reference)?;
        let empty = self.plan.nodes().get(*null)?;
        if binary.operator() != ResolvedBinaryOperatorV1::Equal
            || frame.site().owner() != self.plan.owner()
            || !matches!(read.source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == frame.binding())
            || !matches!(
                empty.source(),
                ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::Null)
            )
            || !((read.site() == binary.lhs() && empty.site() == binary.rhs())
                || (read.site() == binary.rhs() && empty.site() == binary.lhs()))
        {
            return None;
        }
        Some((*reference, *null, frame))
    }
    pub(crate) fn matches_nullable_null_reference_v1(
        &self,
        references: &std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>,
    ) -> bool {
        let Some((reference, _, frame)) = self.nullable_null_v1() else {
            return false;
        };
        matches!(references.get(&reference), Some(CheckedExpressionReferenceV1::NullableResult(original))
            if original == frame && references[&reference].matches_node(&self.plan, reference))
    }
    /// One canonical numeric operation, tied to its original source node.
    /// The writer and independent physical matcher consume this same loan.
    pub(crate) fn integer_operator_v1(&self) -> Option<crate::mir::BinaryOp> {
        use crate::mir::dynamic_operator_contract::{
            DynamicOperatorFamilyV1 as Family, DynamicOperatorNormalResultV1 as ResultKind,
            DynamicOperatorValueClassV1 as Class,
        };
        if let CheckedExpressionOperationProofV1::Scalar(source) = &self.proof {
            if !Rc::ptr_eq(source.plan(), &self.plan)
                || self
                    .plan
                    .nodes()
                    .get(self.node)
                    .and_then(|node| node.binary())
                    != Some(source.binary())
            {
                return None;
            }
            return source.integer_operator_v1();
        }
        let envelope = self.single_envelope()?;
        let domain = envelope.domain();
        if domain.left() != Class::NormalInteger
            || domain.right() != Class::NormalInteger
            || envelope.normal_result() != ResultKind::NormalInteger
        {
            return None;
        }
        match (
            self.plan.nodes().get(self.node)?.binary()?.operator(),
            domain.family(),
        ) {
            (ResolvedBinaryOperatorV1::Add, Family::Add) => Some(crate::mir::BinaryOp::Add),
            (ResolvedBinaryOperatorV1::Multiply, Family::Mul) => Some(crate::mir::BinaryOp::Mul),
            _ => None,
        }
    }
}
