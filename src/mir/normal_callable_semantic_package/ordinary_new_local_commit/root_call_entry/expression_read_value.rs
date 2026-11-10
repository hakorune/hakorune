//! Typed eager-expression values and source-qualified non-owning read transport.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{
    CheckedExpressionReferenceV1 as Reference, NullableResultReferenceV1,
};

/// Internal evaluation values keep Bool separate from String/I64 operands.
/// Conversion borrows an existing domain; it never casts or stringifies Bool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EvaluatedExpressionValueV1 {
    Operand(Operand),
    Bool(ValueId),
    /// A source-qualified non-owning reference or sentinel, never a numeric/String operand.
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
            Self::Bool(_) | Self::DeclaredRead(_) => {
                Err(freeze("expression/scalar-continuation-result"))
            }
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
    pub(in crate::mir) fn declared_read_value_v1(
        &self,
        frame: &CheckedExpressionEvaluationV1,
        node: usize,
        context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        let reference = frame
            .references()
            .get(&node)
            .ok_or_else(|| freeze("expression/reference-source-node"))?;
        if !reference.matches_node(frame.source(), node) {
            return Err(freeze("expression/reference-source-node"));
        }
        if let Reference::Alias(_) = reference {
            let (initialized, _) = self
                .exact_read_v1()
                .loan_local_initializer_v1(reference.binding())?;
            if self.definition().and_then(|(_, row)| row.dst_value()) != Some(initialized) {
                return Err(freeze("expression/reference-source-node"));
            }
            return self
                .exact_read_v1()
                .value_for(
                    frame.source().owner(),
                    frame.source().nodes()[node].site().node(),
                    reference.binding(),
                )
                .map_err(|error| error.to_string());
        }
        let Reference::NullableResult(NullableResultReferenceV1::DeclaredFrom(view)) = reference
        else {
            return Err(freeze("expression/reference-source-node"));
        };
        let (value, normal) = context.declared_value_read_v1(view, self.exact_read_v1())?;
        if self.definition() != Some(&normal) {
            return Err(freeze("expression/reference-source-node"));
        }
        Ok(value)
    }
}
