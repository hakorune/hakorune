//! A real Get/Normal projection; never a received Home or lexical binding.
use super::*;
use crate::mir::builder::ExactLexicalReadV1;
use crate::mir::instruction::OwnedArrayInvokeOperation as Array;
use crate::mir::normal_callable_semantic_package::OwnedArrayReadSourceLoanV1;
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(crate) struct EmittedOwnedArrayReadV1 {
    source: OwnedArrayReadSourceLoanV1,
    call: OwnedExprSiteV1,
    receiver: ExactLexicalReadV1,
    index: Binding,
    invoke: Binding,
    normal: Binding,
    fault_bindings: Vec<Binding>,
}
impl EmittedOwnedArrayReadV1 {
    pub(crate) fn new(
        source: OwnedArrayReadSourceLoanV1,
        call: OwnedExprSiteV1,
        receiver: ExactLexicalReadV1,
        index: Binding,
        invoke: Binding,
        normal: Binding,
        fault_bindings: Vec<Binding>,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<Self, String> {
        let read = Self {
            source,
            call,
            receiver,
            index,
            invoke,
            normal,
            fault_bindings,
        };
        read.value_with_source_v1(&read.call, &read.source, ledger)?;
        Ok(read)
    }
    pub(crate) fn source(&self) -> &OwnedArrayReadSourceLoanV1 {
        &self.source
    }
    pub(crate) fn receiver(&self) -> &ExactLexicalReadV1 {
        &self.receiver
    }
    pub(crate) fn producer_bindings(&self) -> impl Iterator<Item = &Binding> {
        [&self.index, &self.invoke, &self.normal].into_iter()
    }
    pub(crate) fn bindings(&self) -> impl Iterator<Item = &Binding> {
        self.fault_bindings.iter().chain(self.producer_bindings())
    }
    pub(crate) fn value_with_source_v1(
        &self,
        call: &OwnedExprSiteV1,
        source: &OwnedArrayReadSourceLoanV1,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<ValueId, String> {
        if call != &self.call || source != &self.source || call.owner() != source.site().owner() {
            return Err(freeze("owned-array-read-projection-source-drift"));
        }
        let array = ledger.live_owned_array_read_value_v1(source)?;
        if self
            .receiver
            .value_for(call.owner(), source.receiver()?.node(), source.holder())
            .map_err(|issue| format!("{}: {issue:?}", freeze("owned-array-read-receiver")))?
            != array
        {
            return Err(freeze("owned-array-read-receiver-value-drift"));
        }
        let MirInstruction::Const {
            dst: index,
            value: crate::mir::ConstValue::Integer(integer),
        } = &self.index.1
        else {
            return Err(freeze("owned-array-read-index-producer"));
        };
        if *integer != source.index()? || self.index.0 != self.invoke.0 {
            return Err(freeze("owned-array-read-original-index-drift"));
        }
        let MirInstruction::Invoke {
            operation:
                InvokeOperation::OwnedArray(Array::Get {
                    array: actual,
                    descriptor,
                    index: read_index,
                }),
            fault_frame,
            normal_landing,
            fault_landing,
        } = &self.invoke.1
        else {
            return Err(freeze("owned-array-read-invoke-missing"));
        };
        if *actual != array
            || *descriptor != source.descriptor()
            || read_index != index
            || self.normal.0 != *normal_landing
            || normal_landing == fault_landing
        {
            return Err(freeze("owned-array-read-invoke-operands"));
        }
        let MirInstruction::InvokeNormalResult { invoke_block, dst } = &self.normal.1 else {
            return Err(freeze("owned-array-read-normal-missing"));
        };
        if *invoke_block != self.invoke.0 {
            return Err(freeze("owned-array-read-normal-origin"));
        }
        let expected = ledger.owned_array_read_fault_unwind_v1(call, source)?;
        super::owned_array::check_fault_chain(
            &self.fault_bindings,
            *fault_landing,
            *fault_frame,
            None,
            &expected,
        )?;
        Ok(*dst)
    }

    /// Source identity is checked first; finishing may only map the recorded
    /// coordinates. Its validated boundary owns legal Jump contraction and CFG
    /// correspondence, while this check binds Get Normal to its actual use.
    pub(super) fn validate_finished_v1(
        &self,
        function: &MirFunction,
        projection: &super::physical_boundary::FinishedBindings,
        ledger: &OrdinaryNewClaimLedgerV1,
        call_site: &OwnedExprSiteV1,
        outer: &Binding,
        call_coordinate: (BasicBlockId, usize),
        ordinal: u32,
    ) -> Result<(), String> {
        let value = self.value_with_source_v1(call_site, &self.source, ledger)?;
        self.check_physical_v1(function, Some(projection), ledger)?;
        let mapped = |original: &Binding| -> Result<Binding, String> {
            let binding = projection.binding(original.0, &original.1)?
                .ok_or_else(|| freeze("owned-array-read-finished-producer-removed"))?;
            if !projection.recorded().contains(&binding) {
                return Err(freeze("owned-array-read-finished-unrecorded"));
            }
            Ok(binding)
        };
        let index = projection.relation_binding(function, &self.index)?
            .ok_or_else(|| freeze("owned-array-read-finished-index-removed"))?;
        let invoke = mapped(&self.invoke)?;
        let normal = projection.relation_binding(function, &self.normal)?
            .ok_or_else(|| freeze("owned-array-read-finished-normal-removed"))?;
        let outer = mapped(outer)?;
        let locate = |binding: &Binding| -> Result<(BasicBlockId, usize), String> {
            let block = function.blocks.get(&binding.0)
                .ok_or_else(|| freeze("owned-array-read-finished-block"))?;
            let mut found = block.all_instructions().enumerate()
                .filter(|(_, actual)| *actual == &binding.1);
            let (position, _) = found.next()
                .ok_or_else(|| freeze("owned-array-read-finished-producer-missing"))?;
            if found.next().is_some() {
                return Err(freeze("owned-array-read-finished-producer-duplicate"));
            }
            Ok((binding.0, position))
        };
        let positions = [locate(&index)?, locate(&invoke)?, locate(&normal)?, locate(&outer)?];
        if positions[3] != call_coordinate {
            return Err(freeze("owned-array-read-finished-call-coordinate"));
        }
        let MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. } = &outer.1 else {
            return Err(freeze("owned-array-read-finished-call-missing"));
        };
        if call.args.get(ordinal as usize) != Some(&value) {
            return Err(freeze("owned-array-read-finished-argument-slot"));
        }
        let MirInstruction::Invoke { normal_landing, .. } = &invoke.1 else {
            return Err(freeze("owned-array-read-finished-get-missing"));
        };
        if !matches!(&normal.1, MirInstruction::InvokeNormalResult { invoke_block, dst }
            if *invoke_block == invoke.0 && *normal_landing == normal.0 && *dst == value)
        {
            return Err(freeze("owned-array-read-finished-normal-link"));
        }
        let dominators = crate::mir::verification::utils::compute_dominators(function);
        for pair in positions.windows(2) {
            let (before, after) = (pair[0], pair[1]);
            if !dominators.is_reachable(before.0)
                || !dominators.is_reachable(after.0)
                || !dominators.dominates(before.0, after.0)
                || (before.0 == after.0 && before.1 >= after.1)
            {
                return Err(freeze("owned-array-read-finished-evaluation-order"));
            }
        }
        Ok(())
    }

    pub(crate) fn check_raw_v1(
        &self, function: &MirFunction, ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<(), String> {
        self.check_physical_v1(function, None, ledger)
    }

    fn check_physical_v1(
        &self,
        function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<(), String> {
        self.value_with_source_v1(&self.call, &self.source, ledger)?;
        for (block, instruction) in self.bindings() {
            if !super::physical_boundary::check_binding(function, projection, *block, instruction)? {
                return Err(freeze("owned-array-read-physical-binding-drift"));
            }
        }
        let invoke = match projection {
            Some(projection) => projection.binding(self.invoke.0, &self.invoke.1)?
                .ok_or_else(|| freeze("owned-array-read-physical-get-removed"))?,
            None => self.invoke.clone(),
        };
        let MirInstruction::Invoke { fault_landing, fault_frame, .. } = invoke.1 else {
            return Err(freeze("owned-array-read-physical-get-missing"));
        };
        super::root_cleanup_graph::ordered_paths::pending_operations(
            function, fault_landing,
            &ledger.owned_array_read_fault_unwind_v1(&self.call, &self.source)?,
            Some(fault_frame),
        )?;
        Ok(())
    }
}
