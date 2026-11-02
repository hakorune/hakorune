//! Typed eager-expression values and source-qualified non-owning read transport.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{CheckedExpressionReferenceV1 as Reference, NullableResultReferenceV1};

/// Internal evaluation values keep Bool separate from String/I64 operands.
/// Conversion borrows an existing domain; it never casts or stringifies Bool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EvaluatedExpressionValueV1 {
    Operand(Operand),
    Bool(ValueId),
    /// A non-owning From reference, never a numeric/String operand.
    DeclaredRead(ValueId),
}
impl From<Operand> for EvaluatedExpressionValueV1 {
    fn from(value: Operand) -> Self {
        Self::Operand(value)
    }
}
impl EvaluatedExpressionValueV1 {
    pub(in crate::mir) fn as_operand(&self) -> Result<&Operand, String> {
        match self {
            Self::Operand(value) => Ok(value),
            Self::Bool(_) | Self::DeclaredRead(_) => Err(freeze("expression/scalar-continuation-result")),
        }
    }
    pub(in crate::mir) fn value(&self) -> Option<ValueId> {
        match self {
            Self::Operand(value) => value.value(),
            Self::Bool(value) | Self::DeclaredRead(value) => Some(*value),
        }
    }
}

impl ExpressionReferenceReadV1 {
    pub(in crate::mir) fn declared_read_value_v1(&self,
        frame: &CheckedExpressionEvaluationV1, node: usize, context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        let Some(reference @ Reference::NullableResult(NullableResultReferenceV1::DeclaredFrom(view))) = frame.references().get(&node)
            else { return Err(freeze("expression/reference-source-node")); };
        if !reference.matches_node(frame.source(), node) { return Err(freeze("expression/reference-source-node")); }
        let (value, [_, normal]) = context.declared_result_read_v1(view, self.exact_read_v1())?;
        if self.definition() != Some(&normal) { return Err(freeze("expression/reference-source-node")); }
        Ok(value)
    }
}
