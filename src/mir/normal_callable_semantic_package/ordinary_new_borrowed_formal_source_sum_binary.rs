//! Source-only binary transfer in the existing SUM; never an execution grant.
use super::SourceValueAlternativeV1 as Value;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, ResolvedBinaryExpressionSourceV1,
};
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
    pub(super) fn transfer(
        &self,
        lhs: &BTreeSet<Value>,
        rhs: &BTreeSet<Value>,
    ) -> Option<BTreeSet<Value>> {
        if lhs.is_empty() || rhs.is_empty() {
            return None;
        }
        let class = |value: &Value| match value {
            Value::SignedI64 => Some(Class::I64),
            Value::Text => Some(Class::String),
            _ => None,
        };
        let mut values = BTreeSet::new();
        for lhs in lhs {
            for rhs in rhs {
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
