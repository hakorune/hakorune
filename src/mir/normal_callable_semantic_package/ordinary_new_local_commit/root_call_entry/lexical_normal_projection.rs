//! Normal completion of the same physical Call: a value projection or Unit.
//! A Unit continuation never creates a ValueId or an InvokeNormalResult.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;

#[derive(Debug)]
pub(super) enum NormalCallProjectionV1 {
    Value(Binding),
    Unit { normal_landing: BasicBlockId },
}
impl NormalCallProjectionV1 {
    pub(super) fn value(&self) -> Result<&Binding, String> {
        match self {
            Self::Value(binding) => Ok(binding),
            Self::Unit { .. } => Err(freeze("unit-call-has-no-value")),
        }
    }
    #[cfg(test)]
    pub(super) fn value_mut(&mut self) -> Result<&mut Binding, String> {
        match self {
            Self::Value(binding) => Ok(binding),
            Self::Unit { .. } => Err(freeze("unit-call-has-no-value")),
        }
    }
}

impl EmittedLexicalCallProjectionV1 {
    /// The original disposition must already prove Unit. This retains only
    /// its actual Invoke/Normal edge; it issues no semantic result evidence.
    pub(in crate::mir) fn from_unit_source(
        row: CallPacketSourceV1,
        prepared: PreparedLexicalCallProjectionV1,
        invoke: Binding,
        normal_landing: BasicBlockId,
    ) -> Result<Self, String> {
        if row.loan().result() != Some(InvokeCallResultKind::Unit)
            || !matches!(&invoke.1, MirInstruction::Invoke {
                operation: InvokeOperation::Call { result: InvokeCallResultKind::Unit, .. },
                normal_landing: actual, fault_landing, ..
            } if *actual == normal_landing && normal_landing != invoke.0
                && *fault_landing != normal_landing && *fault_landing != invoke.0)
        { return Err(freeze("unit-call/normal-source-drift")); }
        Ok(Self { row, prepared, invoke, projection: NormalCallProjectionV1::Unit { normal_landing } })
    }

    pub(super) fn validate_normal_recorded(
        &self, bindings: &[Binding], finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        match &self.projection {
            NormalCallProjectionV1::Value(binding) => require_recorded(bindings, binding, finishing),
            NormalCallProjectionV1::Unit { .. } => {
                let mapped = finishing.map(|finished| finished.binding(self.invoke.0, &self.invoke.1))
                    .transpose()?.flatten();
                let origin = mapped.as_ref().unwrap_or(&self.invoke).0;
                if bindings.iter().any(|(_, instruction)| matches!(instruction,
                    MirInstruction::InvokeNormalResult { invoke_block, .. } if *invoke_block == origin)) {
                    return Err(freeze("unit-call/unexpected-value-projection"));
                }
                Ok(())
            }
        }
    }

    /// Physical call order: receiver before argument calls, then Invoke/Normal.
    /// Reuse original packets; the postorder node visitor remains unchanged.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn ordered_call_bindings_in_context_v1(
        &self, owner: FunctionOwnerIdV1, source: &[LocalCallArgumentV1],
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        self.call_in_context(owner, source, context)?;
        let mut ordered = Vec::new();
        if let Some(read) = self.stored_receiver_read_v1() { ordered.push(read.clone()); }
        for (argument, original) in self.prepared.arguments.iter().zip(source) {
            if let LexicalCallArgumentProjectionV1::CallResult(inner) = argument {
                let LocalCallArgumentV1::CallResult(original) = original else {
                    return Err(freeze("lexical-i64/nested-source-missing"));
                };
                ordered.extend(inner.ordered_call_bindings_in_context_v1(owner,
                    original.arguments(), context)?);
            }
        }
        ordered.push(self.invoke.clone());
        if let NormalCallProjectionV1::Value(result) = &self.projection { ordered.push(result.clone()); }
        Ok(ordered)
    }

    pub(in crate::mir) fn value_in_context_v1(
        &self, owner: FunctionOwnerIdV1, source: &[LocalCallArgumentV1], context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        self.value_using(owner, source, Some(context))
    }

    pub(super) fn value_using(
        &self,
        owner: FunctionOwnerIdV1,
        source: &[LocalCallArgumentV1],
        context: Option<CallValidationContextV1<'_>>,
    ) -> Result<ValueId, String> {
        self.normal_result_using(owner, source, context)?
            .ok_or_else(|| freeze("unit-call-has-no-value"))
    }

    /// Validate the SAME target, receiver and ordered arguments for both
    /// result forms. Nested arguments still require an actual value.
    pub(super) fn normal_result_using(
        &self,
        owner: FunctionOwnerIdV1,
        source: &[LocalCallArgumentV1],
        context: Option<CallValidationContextV1<'_>>,
    ) -> Result<Option<ValueId>, String> {
        let ledger = context.map(CallValidationContextV1::ledger);
        if let CallPacketSourceLoanV1::Instance(row) = self.row.loan() {
            if row.source_target().borrowed_nullable_local_source().is_some() {
                ledger.ok_or_else(|| freeze("borrowed-nullable-call/ledger-missing"))?.corroborate_borrowed_nullable_call_v1(row)?;
            }
            if row.source_target().mixed_local_source().is_some() {
                let context = context.ok_or_else(|| freeze("mixed-call/ledger-missing"))?;
                context.ledger().corroborate_mixed_call_using_v1(row, context)?;
            }
            if row.result() == Some(InvokeCallResultKind::Unit) {
                ledger.ok_or_else(|| freeze("unit-call/ledger-missing"))?.corroborate_unit_discard_v1(row)?;
            }
        }
        let expected = self.prepared.materialize_in_context(owner, self.row.loan(), source, context)?;
        let MirInstruction::Invoke {
            operation: InvokeOperation::Call { call, result: emitted_result },
            normal_landing, ..
        } = &self.invoke.1 else {
            return Err(freeze("lexical-i64/nested-invoke-shape"));
        };
        if self.row.loan().result() != Some(*emitted_result) || *call != expected {
            return Err(freeze("lexical-i64/nested-producer-drift"));
        }
        match &self.projection {
            NormalCallProjectionV1::Value(projection) => {
                let MirInstruction::InvokeNormalResult { invoke_block, dst } = &projection.1 else {
                    return Err(freeze("lexical-i64/nested-projection-shape"));
                };
                if *emitted_result == InvokeCallResultKind::Unit
                    || *invoke_block != self.invoke.0 || *normal_landing != projection.0 {
                    return Err(freeze("lexical-i64/nested-producer-drift"));
                }
                Ok(Some(*dst))
            }
            NormalCallProjectionV1::Unit { normal_landing: original } => {
                if *emitted_result != InvokeCallResultKind::Unit || normal_landing != original {
                    return Err(freeze("unit-call/normal-source-drift"));
                }
                Ok(None)
            }
        }
    }
}
