//! Original call actuals joined to the caller's sealed entry, not metadata.
use super::*;

impl BorrowedCallUses {
    pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view::compiled_entry_contract::borrowed_call_incoming) fn call(
        &mut self,
        caller: &MirFunction,
        coordinate: Coordinate,
        actuals: &[PreparedBorrowedFormalActualV1],
        copies: &[(Binding, Coordinate)],
        caller_entry: &[(u32, BindingRefV1, ValueId)],
    ) -> Result<(), String> {
        let state = self
            .functions
            .entry(caller.signature.name.clone())
            .or_default();
        // The packet has independently validated every copy, including
        // ordinary scalar/receiver dependencies. This closure tracks only
        // descendants of the SAME source-issued borrowed entry roots.
        let mut borrowed_values: BTreeSet<_> = caller_entry.iter().map(|(_, _, value)| *value).collect();
        let mut admitted = BTreeSet::new();
        loop {
            let mut progressed = false;
            for (index, (original, finished)) in copies.iter().enumerate() {
                let MirInstruction::Copy { src, dst } = &original.1 else {
                    return Err(fault("borrowed-use/copy-instruction"));
                };
                if admitted.contains(&index) || !borrowed_values.contains(src) { continue; }
                state.copy(original, Some(*finished))?;
                borrowed_values.insert(*dst);
                admitted.insert(index);
                progressed = true;
            }
            if !progressed { break; }
        }
        for actual in actuals {
            if let Some((_, formal)) = actual.source.caller_formal_origin_v1() {
                // DeclaredObject also exists outside the selected borrowed
                // transport. Only the original finished caller entry can
                // connect it to this operand closure; visitation order cannot.
                if matches!(actual.source, BorrowedFormalActualSourceV1::DeclaredFormal { .. })
                    && !caller_entry.iter().any(|(_, root, _)| *root == formal)
                {
                    continue;
                }
                let key = (coordinate.0, coordinate.1, actual.ordinal as usize);
                if let Some(previous) = state.arguments.insert(key, formal) {
                    if previous != formal {
                        return Err(fault("borrowed-use/conflicting-argument"));
                    }
                }
            }
        }
        Ok(())
    }
}

impl BorrowedCallUses {
    pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view::compiled_entry_contract::borrowed_call_incoming) fn computed_call(
        &mut self, caller: &MirFunction, coordinate: Coordinate,
        actuals: &[PreparedBorrowedFormalActualV1], operands: &BTreeMap<u32, crate::mir::instruction::StringInvokeOperand>,
    ) -> Result<(), String> {
        let required: BTreeSet<_> = actuals.iter().filter(|actual|
            matches!(actual.source, BorrowedFormalActualSourceV1::Expression(_))).map(|actual| actual.ordinal).collect();
        if required != operands.keys().copied().collect() { return Err(fault("borrowed-text/source-coverage")); }
        let state = self.functions.entry(caller.signature.name.clone()).or_default();
        for (ordinal, operand) in operands {
            let actual = actuals.iter().find(|actual| actual.ordinal == *ordinal)
                .ok_or_else(|| fault("borrowed-text/source-coverage"))?;
            match (&actual.source, operand) {
                (BorrowedFormalActualSourceV1::Expression(source), crate::mir::instruction::StringInvokeOperand::NormalInteger(_))
                    if source.frame().is_i64() => continue,
                (BorrowedFormalActualSourceV1::Expression(source), crate::mir::instruction::StringInvokeOperand::OwnedString(_))
                    if !source.frame().is_i64() => {},
                _ => return Err(fault("borrowed-text/operand-correspondence")),
            }
            if state.text_arguments.insert((coordinate.0, coordinate.1, *ordinal as usize), operand.clone()).is_some() {
                return Err(fault("borrowed-text/duplicate-occurrence"));
            }
        }
        Ok(())
    }
}
impl FunctionUses {
    /// Both source consumers select physical Text occurrences independently;
    /// only the SAME packet image can authorize any selected occurrence.
    pub(super) fn verify_text_calls_v1(&self, indexed: &[(Coordinate, &MirInstruction)]) -> Result<(), String> {
        use crate::mir::instruction::StringInvokeOperand as Operand;
        let owned: BTreeSet<_> = indexed.iter().filter_map(|(_, instruction)| match instruction {
            MirInstruction::InvokeNormalResult { dst, invoke_block }
                if indexed.iter().any(|(coordinate, instruction)| coordinate.0 == *invoke_block
                    && matches!(instruction, MirInstruction::Invoke { operation: InvokeOperation::String(operation), .. } if operation.owns_normal_result())) => Some(*dst),
            _ => None,
        }).collect();
        let mut seen = BTreeSet::new();
        for (coordinate, instruction) in indexed {
            let call = match instruction {
                MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. } => call,
                MirInstruction::Call(call) if call.args.iter().any(|value| owned.contains(value)) => return Err(fault("borrowed-text/plain-call")),
                _ => continue,
            };
            for (ordinal, value) in call.args.iter().enumerate() {
                let key = (coordinate.0, coordinate.1, ordinal);
                if !owned.contains(value) && !self.text_arguments.contains_key(&key) { continue; }
                if !matches!(self.text_arguments.get(&key), Some(Operand::OwnedString(original)) if original == value && owned.contains(value))
                    || !seen.insert(key) {
                    return Err(fault("borrowed-text/operand-correspondence"));
                }
            }
        }
        if seen != self.text_arguments.keys().copied().collect() { return Err(fault("borrowed-text/occurrence-coverage")); }
        Ok(())
    }
}

impl FunctionUses {
    /// Both final scans corroborate readonly String operands against the same
    /// sealed Text entry domains; ordinary numeric/object uses stay refused.
    pub(super) fn verify_text_operation_v1(&self, tracked: &BTreeMap<ValueId, BindingRefV1>, views: &ViewScan,
        operation: &crate::mir::instruction::StringInvokeOperation, fault_frame: ValueId,
    ) -> Result<(), String> {
        use crate::mir::instruction::StringInvokeOperand as Operand;
        if tracked.contains_key(&fault_frame) || views.views.contains_key(&fault_frame) {
            return Err(fault("borrowed-use/string-fault-frame"));
        }
        for operand in operation.operands() {
            if let Operand::BorrowedText(value) = operand {
                let formal = tracked.get(value).ok_or_else(|| fault("borrowed-use/string-formal"))?;
                let mut roots = self.roots.iter().filter(|(_, original)| *original == formal);
                let (root, _) = roots.next().ok_or_else(|| fault("borrowed-use/string-formal"))?;
                if roots.next().is_some() || !self.domains.get(root).is_some_and(|domain| domain.text_only()) {
                    return Err(fault("borrowed-use/string-domain"));
                }
            }
        }
        for operand in operation.operands() {
            if operand.value().is_some_and(|value| tracked.contains_key(&value) || views.views.contains_key(&value))
                && !matches!(operand, Operand::BorrowedText(_)) {
                return Err(fault("borrowed-use/string-operand"));
            }
        }
        let scalar_values = match operation {
            crate::mir::instruction::StringInvokeOperation::Substring { start, end, .. } => vec![*start, *end],
            crate::mir::instruction::StringInvokeOperation::End { value } => vec![*value],
            _ => Vec::new(),
        };
        if scalar_values.iter().any(|value| tracked.contains_key(value) || views.views.contains_key(value)) {
            return Err(fault("borrowed-use/string-operand"));
        }
        Ok(())
    }
}
