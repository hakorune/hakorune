//! The existing New emission keeps exact checked read/local Copy/actual loans.
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use super::*;
impl EmittedNewArgumentV1 {
    pub(super) fn validate_field_array_read(
        &self,
        function: &MirFunction,
        projection: Option<&physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        match (
            self.source.field_array_read(),
            &self.field_array_read,
            self.payload(),
        ) {
            (None, None, _) => Ok(()),
            (Some(source), Some(read), crate::mir::instruction::BirthArgumentV1::Value(value))
                if read.matches(source, *value) =>
            {
                read.validate(function, *value, |original| match projection {
                    Some(projection) => projection.relation_binding(function, original),
                    None => Ok(Some(original.clone())),
                })
            }
            _ => Err(freeze("argument-read-record-drift")),
        }
    }
}

#[cfg(test)]
impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn validate_new_argument_reads_for_test(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.ledger
            .with_finished_projection(owner, |symbol, projection| {
                if symbol != function.signature.name {
                    return Err(freeze("argument-read-function-drift"));
                }
                self.ledger
                    .validate_new_emissions_projected(owner, function, Some(projection))
            })
    }
}

use crate::mir::instruction::{BirthArgumentV1};
use crate::mir::normal_callable_semantic_package::{
    CallValidationContextV1, EmittedLexicalCallProjectionV1, ExpressionCallTransientPrefixV1,
    ExpressionReferenceReadV1,
};
use crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1;
type Binding = (BasicBlockId, MirInstruction);

/// One New argument owns the original evaluation and all of its observations.
/// Clones lend this same affine physical proof; they do not duplicate its pool.
#[derive(Debug)]
pub(in crate::mir) struct EmittedNewExpressionV1 {
    frame: Rc<CheckedExpressionEvaluationV1>,
    values: Vec<Option<EvaluatedExpressionValueV1>>,
    calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
    path_reads: Vec<std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>>,
    reads: std::collections::BTreeMap<usize, ExpressionReferenceReadV1>,
    acquired: Vec<(usize, ValueId)>,
    entry: Binding,
    terminal: Binding,
    continuation: Binding,
    original: Vec<Binding>,
}
impl PartialEq for EmittedNewExpressionV1 {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for EmittedNewExpressionV1 {}
impl EmittedNewExpressionV1 {
    pub(in crate::mir) fn new(
        frame: Rc<CheckedExpressionEvaluationV1>,
        values: Vec<Option<EvaluatedExpressionValueV1>>,
        calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
        path_reads: Vec<std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>>,
        reads: std::collections::BTreeMap<usize, ExpressionReferenceReadV1>,
        acquired: Vec<(usize, ValueId)>,
        entry: Binding,
        terminal: Binding,
        continuation: Binding,
        original: Vec<Binding>,
    ) -> Self {
        Self {
            frame,
            values,
            calls,
            path_reads,
            reads,
            acquired,
            entry,
            terminal,
            continuation,
            original,
        }
    }
    fn projection(&self) -> super::root_home::ExpressionPhysicalProjectionRefV1<'_> {
        super::root_home::ExpressionPhysicalProjectionRefV1::new(
            &self.frame,
            &self.values,
            &self.calls,
            &self.path_reads,
            &self.reads,
            &self.original,
        )
    }
    pub(super) fn calls_v1(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        self.calls.iter().map(Rc::as_ref)
    }
    pub(super) fn with_call_node_v1<R>(&self, site: &OwnedExprSiteV1,
        read: impl FnOnce(&EmittedLexicalCallProjectionV1) -> Result<R, String>,
    ) -> Result<R, String> {
        let mut found = None;
        for root in self.calls_v1() {
            if let Some(packet) = root.retained_node_at_v1(site)? {
                if found.replace(packet).is_some() {
                    return Err(freeze("mixed-result/borrowed-consumed-packet-duplicate"));
                }
            }
        }
        read(found.ok_or_else(|| freeze("mixed-result/borrowed-consumed-packet-missing"))?)
    }
    pub(super) fn visit_calls_v1(
        &self,
        mut visit: impl FnMut(&EmittedLexicalCallProjectionV1,
            &[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1]) -> Result<(), String>,
    ) -> Result<(), String> {
        if self.frame.calls().len() != self.calls.len() {
            return Err(freeze("print-expression/source-coverage"));
        }
        for (source, packet) in self.frame.calls().iter().zip(&self.calls) {
            if source.site() != packet.call_site() {
                return Err(freeze("print-expression/call-source"));
            }
            visit(packet, source.arguments())?;
        }
        Ok(())
    }
    pub(in crate::mir) fn bindings(&self) -> &[Binding] {
        &self.original
    }
    pub(in crate::mir) fn deferred_values_v1(&self) -> Vec<ValueId> {
        self.acquired.iter().map(|(_, value)| *value).collect()
    }
    pub(in crate::mir) fn require_source_v1(
        &self,
        source: &super::super::OrdinaryNewTrivialArgumentV1,
        payload: &BirthArgumentV1,
    ) -> Result<(), String> {
        let super::super::OrdinaryNewTrivialArgumentKindV1::Expression { evaluation } =
            source.kind()
        else {
            return Err(freeze("argument-source-drift"));
        };
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::NewArgument { construction, ordinal, .. } = self.frame.source().root()
            else { return Err(freeze("argument-source-drift")) };
        if !Rc::ptr_eq(evaluation, &self.frame)
            || self.frame.source().owner() != source.owner()
            || construction.site() != source.new_site().site()
            || *ordinal != source.ordinal()
            || construction.arguments().get(*ordinal as usize) != Some(source.site())
            || self.frame.source().nodes().first().map(|node| node.site()) != Some(source.site())
            || !matches!((payload, self.values.first()), (BirthArgumentV1::Text(actual), Some(Some(root))) if root.as_operand().is_ok_and(|root| actual == root && root.is_text()))
        {
            return Err(freeze("argument-source-drift"));
        }
        Ok(())
    }
    pub(in crate::mir) fn validate_emitted_v1(
        &self,
        source: &super::super::OrdinaryNewTrivialArgumentV1,
        payload: &BirthArgumentV1,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<(), String> {
        self.validate_v1(
            source,
            payload,
            CallValidationContextV1::Lowering(ledger),
            function,
            None,
            inherited,
        )
    }
    pub(super) fn validate_v1(
        &self,
        source: &super::super::OrdinaryNewTrivialArgumentV1,
        payload: &BirthArgumentV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&physical_boundary::FinishedBindings>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<(), String> {
        self.require_source_v1(source, payload)?;
        let view = self.projection();
        view.validate_source(context)?;
        view.validate_acquisition_projection_v1(&self.acquired, function, finishing)?;
        let actual = finishing
            .map(|boundary| boundary.bindings(&self.original))
            .transpose()?;
        view.validate_recorded(actual.as_deref().unwrap_or(&self.original), finishing)?;
        let covered = if self.acquired.is_empty() {
            view.validate_local_continuation_v1(
                context,
                function,
                finishing,
                &self.entry,
                &self.terminal,
                &self.continuation,
                None,
                inherited,
            )?
        } else {
            view.validate_deferred_argument_continuation_v1(
                context,
                function,
                finishing,
                &self.entry,
                &self.terminal,
                &self.continuation,
                inherited,
                None,
            )?
        };
        if finishing.is_none() {
            let mut ordered = view.ordered_bindings_in_context(context)?;
            ordered.push(self.terminal.clone());
            view.validate_original_scope(
                context.ledger(),
                function,
                &covered,
                &self.entry,
                &self.terminal,
                &self.continuation,
                &ordered,
            )?;
        }
        Ok(())
    }
}
impl EmittedNewArgumentV1 {
    pub(in crate::mir) fn new_v1(
        source: super::super::OrdinaryNewTrivialArgumentV1,
        payload: BirthArgumentV1,
        field_array_read: Option<
            crate::mir::normal_callable_semantic_package::EmittedFieldArrayReadArgumentV1,
        >,
        expression: Option<Rc<EmittedNewExpressionV1>>,
    ) -> Self {
        Self {
            source,
            payload,
            field_array_read,
            expression,
        }
    }
    pub(in crate::mir) fn expression_v1(&self) -> Option<&Rc<EmittedNewExpressionV1>> {
        self.expression.as_ref()
    }
    pub(in crate::mir) fn deferred_text_values_v1(&self) -> Result<Vec<ValueId>, String> {
        match (&self.expression, self.source.kind()) {
            (Some(packet), super::super::OrdinaryNewTrivialArgumentKindV1::Expression { .. }) => {
                packet.require_source_v1(&self.source, &self.payload)?;
                Ok(packet.deferred_values_v1())
            }
            (None, super::super::OrdinaryNewTrivialArgumentKindV1::Expression { .. })
            | (Some(_), _) => Err(freeze("argument-source-drift")),
            (None, _) => {
                self.value_only_v1()?;
                Ok(Vec::new())
            }
        }
    }
}

/// Recorded New phase boundaries, replayed against source-derived cleanup.
#[derive(Debug)]
pub(in crate::mir) struct NewArgumentCleanupV1 {
    allocation: Binding,
    birth: Option<Binding>,
    continuation: BasicBlockId,
}
impl NewArgumentCleanupV1 {
    pub(in crate::mir) fn new(
        allocation: Binding,
        birth: Option<Binding>,
        continuation: BasicBlockId,
    ) -> Self {
        Self {
            allocation,
            birth,
            continuation,
        }
    }
    pub(super) fn validate_v1(
        &self,
        context: CallValidationContextV1<'_>,
        site: &OwnedExprSiteV1,
        result: ValueId,
        arguments: &[EmittedNewArgumentV1],
        function: &MirFunction,
        finishing: Option<&physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        use super::root_cleanup_graph::ordered_paths as paths;
        let ledger = context.ledger();
        let map = |row: &Binding| -> Result<Binding, String> {
            match finishing {
                Some(boundary) => boundary
                    .binding(row.0, &row.1)?
                    .ok_or_else(|| freeze("emission-binding-drift")),
                None => Ok(row.clone()),
            }
        };
        let mut ordered = Vec::new();
        for argument in arguments {
            if let Some(packet) = argument.expression_v1() {
                ordered.push(packet.entry.clone());
                ordered.extend(packet.projection().ordered_bindings_in_context(context)?);
                ordered.push(packet.terminal.clone());
                ordered.push(packet.continuation.clone());
            }
        }
        ordered.push(self.allocation.clone());
        if let Some(birth) = &self.birth {
            ordered.push(birth.clone());
        }
        let mut mapped = Vec::new();
        for row in ordered {
            let projected = finishing
                .map(|boundary| {
                    if matches!(row.1, MirInstruction::Copy { .. }) {
                        boundary.relation_binding(function, &row)
                    } else {
                        boundary.binding(row.0, &row.1)
                    }
                })
                .transpose()?;
            match projected {
                Some(Some(actual)) => mapped.push(actual),
                Some(None)
                    if matches!(
                        row.1,
                        MirInstruction::Jump {
                            edge_args: None,
                            ..
                        } | MirInstruction::Copy { .. }
                    ) => {}
                Some(None) => return Err(freeze("emission-binding-drift")),
                None => mapped.push(row),
            }
        }
        super::root_home::ExpressionPhysicalProjectionRefV1::validate_normal_order_v1(
            function, &mapped,
        )?;
        let prior = ledger.new_argument_home_unwind_v1(site, false)?;
        let mut ends = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for argument in arguments.iter().rev() {
            for value in argument.deferred_text_values_v1()?.into_iter().rev() {
                if !seen.insert(value) {
                    return Err(freeze("argument-expression/prefix-duplicate-acquisition"));
                }
                ends.push(InvokeOperation::String(
                    crate::mir::instruction::StringInvokeOperation::End { value },
                ));
            }
        }
        let pending: Vec<_> = ends.iter().cloned().chain(prior).collect();
        let (object, _) = ledger.new_argument_object_cleanup_v1(site, result)?;
        let expected_birth = {
            let rows = ledger.local_commits.borrow();
            match rows.get(site) {
                Some(LocalCommitV1::Ordinary(row)) => row.birth_target.clone(),
                Some(LocalCommitV1::Result(row)) => row.birth_target.clone(),
                _ => return Err(freeze("argument-source-drift")),
            }
        };
        let (allocation_block, allocation) = map(&self.allocation)?;
        let MirInstruction::Invoke {
            operation:
                InvokeOperation::NewBox {
                    object: actual_object,
                },
            fault_frame,
            fault_landing,
            normal_landing,
        } = allocation
        else {
            return Err(freeze("emission-binding-drift"));
        };
        if actual_object != object
            || !function.blocks.get(&normal_landing).is_some_and(|block| {
                block.instructions.iter().any(|row| {
                    matches!(row, MirInstruction::InvokeNormalResult { invoke_block, dst }
                if *invoke_block == allocation_block && *dst == result)
                })
            })
        {
            return Err(freeze("emission-binding-drift"));
        }
        let mut covered =
            paths::pending_operations(function, fault_landing, &pending, Some(fault_frame))?;
        let normal = if let Some(birth) = &self.birth {
            let (_, birth) = map(birth)?;
            let MirInstruction::Invoke {
                operation: InvokeOperation::Birth(call),
                fault_frame: same,
                fault_landing,
                normal_landing: after,
            } = birth
            else {
                return Err(freeze("argument-call-drift"));
            };
            if same != fault_frame
                || expected_birth.as_ref() != Some(&call.key)
                || call.receiver != result
                || !call
                    .arguments
                    .iter()
                    .eq(arguments.iter().map(|argument| argument.payload()))
            {
                return Err(freeze("argument-call-drift"));
            }
            let object = ledger.new_argument_object_cleanup_v1(site, result)?.0;
            let reclaim = InvokeOperation::ReclaimUnpublished {
                object,
                value: result,
            };
            let head = paths::skip_until(function, fault_landing, None)?;
            let (normal, fault) = paths::release(function, head, &reclaim, Some(fault_frame))?;
            covered.extend(paths::pending_operations(
                function,
                normal,
                &pending,
                Some(fault_frame),
            )?);
            covered.extend(paths::pending_operations(
                function,
                fault,
                &pending,
                Some(fault_frame),
            )?);
            after
        } else {
            if expected_birth.is_some() {
                return Err(freeze("argument-call-drift"));
            }
            normal_landing
        };
        let stop = match finishing {
            Some(boundary) => boundary
                .destination(self.continuation)
                .ok_or_else(|| freeze("emission-binding-drift"))?,
            None => self.continuation,
        };
        let mut normal = normal;
        let (_, candidate) = ledger.new_argument_object_cleanup_v1(site, result)?;
        let homes = ledger.new_argument_home_unwind_v1(site, true)?;
        for (step, operation) in ends.iter().enumerate() {
            normal = paths::skip_until(function, normal, Some(stop))?;
            let (next, fault) = paths::release(function, normal, operation, Some(fault_frame))?;
            let pending: Vec<_> = candidate
                .iter()
                .cloned()
                .chain(ends[step + 1..].iter().cloned())
                .chain(homes.iter().cloned())
                .collect();
            covered.extend(paths::pending_operations(
                function,
                fault,
                &pending,
                Some(fault_frame),
            )?);
            normal = next;
        }
        if paths::skip_until(function, normal, Some(stop))? != stop {
            return Err(freeze("emission-binding-drift"));
        }
        Ok(covered)
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn new_argument_home_unwind_v1(
        &self,
        site: &OwnedExprSiteV1,
        normal: bool,
    ) -> Result<Vec<InvokeOperation>, String> {
        let homes = {
            let rows = self.local_commits.borrow();
            match rows.get(site) {
                Some(LocalCommitV1::Ordinary(row)) => {
                    let source = row
                        .home_prefix
                        .as_ref()
                        .map_err(|_| freeze("argument-source-unavailable"))?;
                    if normal {
                        source.normal_homes()
                    } else {
                        source.prior_homes()
                    }
                    .to_vec()
                }
                Some(LocalCommitV1::Result(row)) => {
                    let source = row
                        .home_prefix
                        .as_ref()
                        .map_err(|_| freeze("argument-source-unavailable"))?;
                    if normal {
                        source.normal_homes()
                    } else {
                        source.prior_homes()
                    }
                    .to_vec()
                }
                _ => return Err(freeze("argument-source-unavailable")),
            }
        };
        // The New home prefix stores its Homes newest-first; the shared unwind
        // takes oldest-first input and walks it newest-first.
        let homes: Vec<_> = homes.into_iter().rev().collect();
        self.prior_home_unwind_for(&homes)
    }
    pub(in crate::mir) fn new_argument_object_cleanup_v1(
        &self,
        site: &OwnedExprSiteV1,
        result: ValueId,
    ) -> Result<(CanonicalObjectIdV1, Vec<InvokeOperation>), String> {
        let rows = self.local_commits.borrow();
        let (object, destruction, children) = match rows.get(site) {
            Some(LocalCommitV1::Ordinary(row)) => {
                (row.object, row.destruction, row.children.as_deref())
            }
            Some(LocalCommitV1::Result(row)) => {
                (row.object, row.destruction, row.children.as_deref())
            }
            _ => return Err(freeze("argument-source-unavailable")),
        };
        let fields = match destruction {
            super::super::ObjectDestructionDispositionV1::NUMERIC_NO_HOOK => &[][..],
            disposition if disposition.requires_field_end() => {
                children.ok_or_else(|| freeze("fresh-result-cleanup/children"))?
            }
            _ => return Err(freeze("fresh-result-cleanup/destruction")),
        };
        Ok((
            object,
            super::super::object_end::operations_v1(object, false, fields, result)
                .map(|(_, operation)| operation)
                .collect(),
        ))
    }
}

impl FinalizedRootSourceHandoffV1 {
    /// Select only the original finished caller scope. This lends no actual
    /// execution proof and never guesses an owner from local physical IDs.
    pub(in crate::mir) fn birth_actual_belongs_to_function_v1(
        &self, actual: &FinalizedBirthActualsV1, function: &MirFunction,
    ) -> Result<bool, String> {
        self.ledger.with_finished_projection(actual.owner(), |symbol, _|
            Ok(symbol == function.signature.name))
    }

    pub(in crate::mir) fn validate_birth_text_entry_v1(
        &self, abi: &BirthAbiHandoffV1, function: &MirFunction,
    ) -> Result<(), String> {
        use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
        let retained = self.construction.iter().find(|(key, _)| key == abi.target())
            .map(|(_, retained)| retained).ok_or_else(|| freeze("argument-source-unavailable"))?;
        if function.params.len() != abi.abi().physical_arity() { return Err(freeze("argument-source-drift")); }
        let carriers = function.metadata.physical_param_carriers.as_deref().ok_or_else(|| freeze("argument-source-drift"))?;
        if carriers.len() != function.params.len() || carriers.first() != Some(&Carrier::ExistingCallableI64) {
            return Err(freeze("argument-source-drift"));
        }
        for (ordinal, lane) in abi.parameters().iter().enumerate() {
            let expected = if abi.text_formal_source_v1(ordinal as u32)?.is_some() {
                Carrier::BorrowedTaggedValue
            } else { Carrier::ExistingCallableI64 };
            if carriers.get(lane.physical_lane() as usize) != Some(&expected) { return Err(freeze("argument-source-drift")); }
        }
        retained.require_birth_abi_source_v1(abi, function)
    }

    /// Join original callee demands to this exact caller's typed New row. The
    /// two source products are distinct; physical Text bits never issue either.
    pub(in crate::mir) fn validate_birth_text_actuals_v1(
        &self, abi: &BirthAbiHandoffV1, actual: &FinalizedBirthActualsV1, function: &MirFunction,
    ) -> Result<(), String> {
        if actual.target() != abi.target() || actual.arguments().len() != abi.parameters().len() {
            return Err(freeze("argument-source-drift"));
        }
        for (ordinal, argument) in actual.arguments().iter().enumerate() {
            if argument.source().ordinal() as usize != ordinal || argument.source().new_site() != actual.site()
                || argument.source().owner() != actual.owner() { return Err(freeze("argument-source-drift")); }
            let formal = abi.text_formal_source_v1(ordinal as u32)?;
            match (argument.payload(), formal) {
                (BirthArgumentV1::Text(_), Some(_)) => {
                    let expression = argument.expression_v1().ok_or_else(|| freeze("argument-source-drift"))?;
                    use crate::mir::resolved_value_profile::{expression_projection::ExpressionLeafClassV1 as L,
                        expression_class::ExpressionValueClassV1 as V};
                    if expression.frame.root_class() != L::Value(V::String) { return Err(freeze("argument-source-unavailable")); }
                    expression.require_source_v1(argument.source(), argument.payload())?;
                }
                (BirthArgumentV1::Value(_), None) => (),
                _ => return Err(freeze("argument-source-unavailable")),
            }
        }
        self.validate_new_birth_expression_actuals_v1(actual, function)
    }

    pub(in crate::mir) fn validate_new_birth_expression_actuals_v1(
        &self,
        actual: &FinalizedBirthActualsV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        if actual
            .arguments()
            .iter()
            .all(|argument| argument.expression_v1().is_none())
        {
            return Ok(());
        }
        self.ledger.with_finished_projection(actual.owner(), |symbol, projection| {
            if symbol != function.signature.name { return Err(freeze("argument-read-function-drift")); }
            let rows = self.ledger.local_commits.borrow();
            let row = rows.get(actual.site()).ok_or_else(|| freeze("argument-source-drift"))?;
            if !matches!(row.new_emission(), Some(NewEmissionProgress::Emitted { result, arguments, .. })
                if *result == actual.receiver() && arguments.as_ref() == actual.arguments()) {
                return Err(freeze("argument-source-drift"));
            }
            self.ledger.validate_ordinary_new_row_emission_projected(actual.owner(), actual.site(), row, function,
                Some(projection), CallValidationContextV1::Finalized(self))
        })
    }
}

impl FinalizedRootSourceHandoffV1 {
    /// Coordinates may be lent only by the original New argument's own packet.
    pub(super) fn finished_new_argument_call_binding_v1(
        &self, owner: FunctionOwnerIdV1, new_site: &OwnedExprSiteV1, ordinal: u32,
        node_site: &OwnedExprSiteV1, original: &Binding, function: &MirFunction,
        copy: bool,
    ) -> Result<(BasicBlockId, usize), String> {
        if new_site.owner() != owner || node_site.owner() != owner {
            return Err(freeze("finished-local-call/owner"));
        }
        let rows = self.ledger.local_commits.borrow();
        let arguments = match rows.get(new_site).and_then(LocalCommitV1::new_emission) {
            Some(NewEmissionProgress::Emitted { arguments, .. }) => arguments,
            _ => return Err(freeze("argument-source-unavailable")),
        };
        let argument = arguments.get(ordinal as usize)
            .filter(|row| row.source().ordinal() == ordinal && row.source().new_site() == new_site)
            .ok_or_else(|| freeze("argument-source-drift"))?;
        let expression = argument.expression_v1().ok_or_else(|| freeze("argument-source-drift"))?;
        expression.require_source_v1(argument.source(), argument.payload())?;
        let mut found = None;
        for packet in expression.calls_v1() {
            let member = if copy {
                packet.copy_dependencies_in_context(CallValidationContextV1::Finalized(self))?
                    .iter().any(|(site, binding)| site == node_site && binding == original)
            } else { packet.has_producer_at(node_site, original) };
            if member && found.replace(packet).is_some() {
                return Err(freeze("finished-copy/packet-duplicate"));
            }
        }
        let packet = found.ok_or_else(|| freeze("finished-local-call/original-producer"))?;
        if copy {
            self.finished_packet_copy(owner, packet, node_site, original, function)
        } else {
            let (symbol, finished) = self.ledger.finished_binding_for_owner(owner, original)?;
            super::finalized_source_projection::find_finished_producer(&symbol, &finished, function)
        }
    }
}

impl FinalizedRootSourceHandoffV1 {
    /// One original New read and its SAME final Birth occurrence. This lends
    /// source meaning to final use validation and tagged ABI projection alike.
    pub(in crate::mir) fn finite_entry_birth_argument_v1(
        &self, actual: &FinalizedBirthActualsV1, ordinal: u32, function: &MirFunction,
    ) -> Result<Option<(crate::mir::normal_callable_semantic_package::BorrowedFormalDomainV1,
        Binding, (BasicBlockId, usize))>, String> {
        if !self.has_borrowed_ordinary_entry_v1(actual.owner()) { return Ok(None); }
        let entry = self.borrowed_ordinary_entry_source_for_function_v1(actual.owner(), function)?;
        let argument = actual.arguments().get(ordinal as usize)
            .ok_or_else(|| freeze("argument-source-drift"))?;
        if argument.source().ordinal() != ordinal || argument.source().owner() != actual.owner()
            || argument.source().new_site() != actual.site() { return Err(freeze("argument-source-drift")); }
        let Some((binding, domain)) = entry.finite_new_argument_domain_v1(argument.source())? else { return Ok(None); };
        // Read provenance is independent of the recorded Birth payload. Reuse
        // original entry/alias owners; an arbitrary matching MIR id is no loan.
        let expected = if binding == domain.binding() {
            self.borrowed_ordinary_entry_values_v1(actual.owner())?.iter()
                .find(|(index, formal, _)| *index == domain.ordinal() && *formal == domain.binding())
                .map(|(_, _, value)| *value).ok_or_else(|| freeze("argument-binding-unavailable"))?
        } else {
            let mut expected = None;
            self.with_borrowed_ordinary_alias_copies_v1(actual.owner(), function, |_, _, alias, formal, value, _| {
                if alias == binding {
                    if formal != domain.binding() || expected.replace(value).is_some() { return Err(freeze("argument-source-drift")); }
                }
                Ok(())
            })?;
            expected.ok_or_else(|| freeze("argument-binding-unavailable"))?
        };
        if argument.value_only_v1()? != expected { return Err(freeze("argument-source-drift")); }
        self.ledger.with_finished_projection(actual.owner(), |symbol, projection| {
            if symbol != function.signature.name { return Err(freeze("argument-read-function-drift")); }
            let rows = self.ledger.local_commits.borrow();
            let row = rows.get(actual.site()).ok_or_else(|| freeze("argument-source-drift"))?;
            let (target, destination) = match row {
                LocalCommitV1::Ordinary(row) => (row.birth_target.as_ref(), Some(row.binding)),
                LocalCommitV1::Result(row) => (row.birth_target.as_ref(), None),
                _ => return Err(freeze("argument-source-drift")),
            };
            let original = row.new_argument_rows().ok_or_else(|| freeze("argument-source-unavailable"))?
                .as_ref().map_err(|_| freeze("argument-source-unavailable"))?;
            if target != Some(actual.target()) || destination != actual.destination()
                || original.get(ordinal as usize) != Some(argument.source()) { return Err(freeze("argument-source-drift")); }
            let Some(NewEmissionProgress::Emitted { result, arguments, bindings,
                progress: EmittedLocalProgress::Checked { .. }, .. }) = row.new_emission()
                else { return Err(freeze("argument-source-unavailable")); };
            if *result != actual.receiver() || arguments.as_ref() != actual.arguments() { return Err(freeze("argument-source-drift")); }
            let mut births = bindings.iter().filter(|(_, instruction)| matches!(instruction,
                MirInstruction::Invoke { operation: InvokeOperation::Birth(call), .. }
                    if &call.key == actual.target() && call.receiver == actual.receiver()
                        && call.arguments.iter().eq(actual.arguments().iter().map(|argument| argument.payload()))));
            let original = births.next().ok_or_else(|| freeze("argument-source-drift"))?;
            if births.next().is_some() { return Err(freeze("argument-source-drift")); }
            let finished = projection.binding(original.0, &original.1)?
                .ok_or_else(|| freeze("argument-source-drift"))?;
            let coordinate = super::finalized_source_projection::find_finished_producer(symbol, &finished, function)?;
            Ok(Some((domain, finished, coordinate)))
        })
    }
}
