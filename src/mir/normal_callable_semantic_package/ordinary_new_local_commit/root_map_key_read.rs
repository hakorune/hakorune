//! Shared passive Map key/receiver physical observations; no source classifier.
use super::*;
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;

/// Passive Text-key observations shared by exact Map reads and installs.
/// Source membership and operation policy remain with each source-issued consumer.
#[derive(Debug)]
pub(crate) struct FieldMapKeyReadEmissionV1 {
    pub(super) key:
        std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
    pub(super) receiver: crate::mir::builder::ExactLexicalReadV1,
    pub(super) values: Vec<Option<EvaluatedExpressionValueV1>>,
    pub(super) reads: std::collections::BTreeMap<usize, super::ExpressionReferenceReadV1>,
    pub(super) entry: (BasicBlockId, MirInstruction),
    pub(super) terminal: (BasicBlockId, MirInstruction),
    pub(super) continuation: (BasicBlockId, MirInstruction),
    pub(super) original: Vec<(BasicBlockId, MirInstruction)>,
}
impl FieldMapKeyReadEmissionV1 {
    pub(in crate::mir) fn issue(
        key: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        >,
        receiver: crate::mir::builder::ExactLexicalReadV1,
        values: Vec<Option<EvaluatedExpressionValueV1>>,
        reads: std::collections::BTreeMap<usize, super::ExpressionReferenceReadV1>,
        entry: (BasicBlockId, MirInstruction),
        terminal: (BasicBlockId, MirInstruction),
        continuation: (BasicBlockId, MirInstruction),
        original: Vec<(BasicBlockId, MirInstruction)>,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<Self, String> {
        if !key.calls().is_empty()
            || !key.cores().is_empty()
            || !key.operations().is_empty()
            || !key.owned_result_nodes().is_empty()
        {
            return Err(freeze("map-get-shape"));
        }
        let read = Self {
            key,
            receiver,
            values,
            reads,
            entry,
            terminal,
            continuation,
            original,
        };
        read.expression()
            .validate_source(CallValidationContextV1::Lowering(ledger))?;
        Ok(read)
    }
    pub(super) fn copy_dependencies(&self) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let mut rows = self.receiver.original_copy_dependencies_v1()?.to_vec();
        for read in self.reads.values() {
            rows.extend_from_slice(read.original_copy_dependencies_v1()?);
        }
        Ok(rows)
    }
    pub(super) fn expression(&self) -> super::ExpressionPhysicalProjectionRefV1<'_> {
        super::ExpressionPhysicalProjectionRefV1::new(
            &self.key,
            &self.values,
            &[],
            &[],
            &self.reads,
            &self.original,
        )
    }
    pub(super) fn validate(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        expected: &InvokeOperation,
        invoke: &(BasicBlockId, MirInstruction),
        projection: &(BasicBlockId, MirInstruction),
    ) -> Result<crate::mir::ValueId, String> {
        let ledger = context.ledger();
        if !matches!(&invoke.1, MirInstruction::Invoke { operation, .. } if operation == expected) {
            return Err(freeze("map-get-shape"));
        }
        // Both coordinates belong to this exact Invoke. A Local consumer may
        // continue after Normal, so root-exit graph checks cannot supply this.
        let MirInstruction::Invoke {
            normal_landing,
            fault_landing,
            ..
        } = invoke.1
        else {
            return Err(freeze("map-get-shape"));
        };
        let MirInstruction::InvokeNormalResult {
            invoke_block,
            dst: value,
        } = projection.1
        else {
            return Err(freeze("map-get-binding-drift"));
        };
        if invoke_block != invoke.0
            || projection.0 != normal_landing
            || normal_landing == fault_landing
            || normal_landing == invoke.0
        {
            return Err(freeze("map-get-binding-drift"));
        }
        for (block, row) in [invoke, projection] {
            if !super::super::physical_boundary::check_binding(function, finishing, *block, row)? {
                return Err(freeze("map-get-binding-drift"));
            }
        }
        let expression = self.expression();
        let MirInstruction::Invoke { fault_frame, .. } = invoke.1 else {
            return Err(freeze("map-get-frame-drift"));
        };
        let key_frame = expression.require_unique(
            |row| matches!(row, MirInstruction::FaultFrameEnter { dst, .. } if *dst == fault_frame),
        )?;
        let actual_frame = match finishing {
            Some(p) => p
                .binding(key_frame.0, &key_frame.1)?
                .ok_or_else(|| freeze("map-get-frame-drift"))?,
            None => key_frame.clone(),
        };
        if !super::super::physical_boundary::check_binding(
            function,
            None,
            actual_frame.0,
            &actual_frame.1,
        )? {
            return Err(freeze("map-get-frame-drift"));
        }
        let actual = finishing.map(|p| p.bindings(&self.original)).transpose()?;
        expression.validate_recorded(actual.as_deref().unwrap_or(&self.original), finishing)?;
        let covered = expression.validate_local_continuation_v1(
            context,
            function,
            finishing,
            &self.entry,
            &self.terminal,
            &self.continuation,
            None,
            None,
        )?;
        let actual_invoke = match finishing {
            Some(p) => p
                .binding(invoke.0, &invoke.1)?
                .ok_or_else(|| freeze("map-get-binding-missing"))?,
            None => invoke.clone(),
        };
        let MirInstruction::Jump {
            target,
            edge_args: None,
        } = self.continuation.1
        else {
            return Err(freeze("map-get-binding-drift"));
        };
        let target = match finishing {
            Some(p) => p
                .destination(target)
                .ok_or_else(|| freeze("map-get-binding-missing"))?,
            None => target,
        };
        if target != actual_invoke.0 {
            return Err(freeze("map-get-binding-drift"));
        }
        for original in self.receiver.original_copy_dependencies_v1()? {
            let actual = match finishing {
                Some(p) => p.relation_binding(function, original)?,
                None => Some(original.clone()),
            };
            if let Some((block, row)) = actual {
                if !super::super::physical_boundary::check_binding(function, None, block, &row)? {
                    return Err(freeze("map-get-binding-drift"));
                }
            }
        }
        if finishing.is_none() {
            let mut ordered = expression.ordered_bindings_in_context(context)?;
            ordered.push(self.terminal.clone());
            expression.validate_original_scope(
                ledger,
                function,
                &covered,
                &self.entry,
                &self.terminal,
                &self.continuation,
                &ordered,
            )?;
        }
        Ok(value)
    }
}
