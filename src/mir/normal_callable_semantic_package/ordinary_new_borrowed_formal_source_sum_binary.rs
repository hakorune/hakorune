//! Source-only binary transfer in the existing SUM; never an execution grant.
use super::SourceValueAlternativeV1 as Value;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, ResolvedBinaryExpressionSourceV1};
use crate::mir::resolved_value_profile::expression_class::{
    binary_expression_class_v1, ExpressionValueClassV1 as Class,
};
use std::collections::BTreeSet;

pub(super) struct SourceBinaryTransferV1 {
    pub(super) owner: FunctionOwnerIdV1,
    pub(super) source: ResolvedBinaryExpressionSourceV1,
    pub(super) operands: [usize; 2],
}

impl SourceBinaryTransferV1 {
    /// The result is text on every normal path when the left source is
    /// definitely text. The runtime still evaluates and checks the RHS; this
    /// only says its static value alternative is not needed to classify the
    /// result.
    pub(super) fn requires_rhs_domain(&self, lhs: &BTreeSet<Value>) -> bool {
        !(self.source.operator() == crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Add
            && !lhs.is_empty()
            && lhs.iter().all(|value| matches!(value, Value::Text)))
    }

    pub(super) fn transfer(
        &self,
        lhs: &BTreeSet<Value>,
        rhs: Option<&BTreeSet<Value>>,
    ) -> Option<BTreeSet<Value>> {
        if lhs.is_empty() {
            return None;
        }
        if !self.requires_rhs_domain(lhs)
            && self.source.operator()
                == crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Add
        {
            return Some(BTreeSet::from([Value::Text]));
        }
        let rhs = rhs?;
        if rhs.is_empty() {
            return None;
        }
        let class = |value: &Value| match value {
            Value::SignedI64 => Some(Class::I64),
            Value::Text => Some(Class::String),
            _ => None,
        };
        let dynamic_class = |value: &Value| match value {
            Value::TaggedDynamic => {
                Some(crate::mir::dynamic_operator_contract::DynamicOperatorValueClassV1::Dynamic)
            }
            Value::SignedI64 => {
                Some(crate::mir::dynamic_operator_contract::DynamicOperatorValueClassV1::I64)
            }
            Value::Text => {
                Some(crate::mir::dynamic_operator_contract::DynamicOperatorValueClassV1::String)
            }
            Value::Bool => {
                Some(crate::mir::dynamic_operator_contract::DynamicOperatorValueClassV1::Bool)
            }
            Value::Null => {
                Some(crate::mir::dynamic_operator_contract::DynamicOperatorValueClassV1::Null)
            }
            Value::Other | Value::Object(_) => None,
        };
        let mut values = BTreeSet::new();
        for lhs in lhs {
            for rhs in rhs {
                // `String + any` is a Text normal result even when the source
                // SUM deliberately retains an open `Other` right-hand tag.
                // This classifies only the result; runtime lowering still
                // requires the original untyped-formal proof and preserves
                // the tag/payload for checked conversion.
                if self.source.operator()
                    == crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Add
                    && matches!(lhs, Value::Text)
                {
                    values.insert(Value::Text);
                    continue;
                }
                let left_dynamic = dynamic_class(lhs)?;
                let right_dynamic = dynamic_class(rhs)?;
                if left_dynamic == crate::mir::dynamic_operator_contract::DynamicOperatorValueClassV1::Dynamic
                    || right_dynamic == crate::mir::dynamic_operator_contract::DynamicOperatorValueClassV1::Dynamic {
                    use crate::mir::dynamic_operator_contract::{
                        issue_dynamic_operator_execution_envelope_v1, DynamicOperatorDomainV1,
                        DynamicOperatorFamilyV1 as Family, DynamicOperatorNormalResultV1 as Result,
                    };
                    let family = match self.source.operator() {
                        crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Add => Family::Add,
                        crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Less => Family::Less,
                        _ => return None,
                    };
                    let envelope = issue_dynamic_operator_execution_envelope_v1(
                        DynamicOperatorDomainV1::new(family, left_dynamic, right_dynamic),
                    ).ok()?;
                    values.insert(match envelope.normal_result() {
                        Result::SelfContainedNonAliasingDynamicCarrier => Value::TaggedDynamic,
                        Result::TrivialBool => Value::Bool,
                        Result::NormalInteger => Value::SignedI64,
                        Result::OwnedStringValue => Value::Text,
                    });
                    continue;
                }
                let (left, right) = (class(lhs)?, class(rhs)?);
                values.insert(
                    match binary_expression_class_v1(self.source.operator(), left, right).ok()? {
                        Class::I64 => Value::SignedI64,
                        Class::String => Value::Text,
                        _ => return None,
                    },
                );
            }
        }
        Some(values)
    }
}
