//! One source Append owns its candidate packet and physical transfer/fault paths.
use super::super::physical_boundary::FinishedBindings;
use super::*;
use crate::mir::builder::ExactLexicalReadV1;
use crate::mir::instruction::OwnedArrayInvokeOperation as Array;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::normal_callable_semantic_package::OwnedArrayAppendSourceLoanV1;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(crate) enum OwnedArrayAppendCandidateV1 {
    Null(Binding),
    Call(Rc<EmittedLexicalCallProjectionV1>),
    DynamicExpression(
        crate::mir::normal_callable_semantic_package::LexicalCallArgumentProjectionV1,
    ),
}
#[derive(Debug)]
pub(crate) struct EmittedOwnedArrayAppendV1 {
    source: OwnedArrayAppendSourceLoanV1,
    receiver: ExactLexicalReadV1,
    candidate: OwnedArrayAppendCandidateV1,
    invoke: Binding,
    continuation: Binding,
    original: Vec<Binding>,
}
impl EmittedOwnedArrayAppendV1 {
    pub(crate) fn source(&self) -> &OwnedArrayAppendSourceLoanV1 {
        &self.source
    }
    pub(crate) fn invoke_binding(&self) -> &Binding {
        &self.invoke
    }
    pub(crate) fn normal_continuation_binding(&self) -> &Binding {
        &self.continuation
    }
    pub(crate) fn check_raw_v1(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.check_raw_in_context_v1(CallValidationContextV1::Lowering(ledger), function)
    }
    pub(super) fn check_raw_in_context_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_paths_in_context_mode(
            context,
            function,
            None,
            true,
        )
    }
    pub(crate) fn candidate_packet(&self) -> Option<&EmittedLexicalCallProjectionV1> {
        match &self.candidate {
            OwnedArrayAppendCandidateV1::Call(packet) => Some(packet),
            _ => None,
        }
    }
    pub(crate) fn candidate_entry_binding(&self) -> &Binding {
        match &self.candidate {
            OwnedArrayAppendCandidateV1::Null(binding) => binding,
            OwnedArrayAppendCandidateV1::Call(packet) => packet.invoke_binding(),
            OwnedArrayAppendCandidateV1::DynamicExpression(packet) => {
                packet.dynamic_carrier_append_entry_binding_v1().expect("validated Dynamic carrier append packet")
            }
        }
    }
    pub(crate) fn new(
        source: OwnedArrayAppendSourceLoanV1,
        receiver: ExactLexicalReadV1,
        candidate: OwnedArrayAppendCandidateV1,
        invoke: Binding,
        continuation: Binding,
        original: Vec<Binding>,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<Self, String> {
        let packet = Self {
            source,
            receiver,
            candidate,
            invoke,
            continuation,
            original,
        };
        packet.validate_recorded(&packet.original, None)?;
        packet.check_raw_v1(ledger, function)?;
        Ok(packet)
    }
    pub(super) fn validate_recorded(
        &self,
        bindings: &[Binding],
        projection: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        for original in &self.original {
            let mapped = match projection {
                Some(projection) => projection.binding(original.0, &original.1)?,
                None => Some(original.clone()),
            };
            if let Some(mapped) = mapped {
                if bindings.iter().filter(|row| **row == mapped).count() != 1 {
                    return Err(freeze("array-append/recorded-binding-drift"));
                }
            }
        }
        Ok(())
    }
    pub(super) fn validate_paths(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
        projection: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_paths_in_context(
            CallValidationContextV1::Lowering(ledger),
            function,
            projection,
        )
    }

    pub(super) fn validate_paths_in_context(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        projection: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_paths_in_context_mode(context, function, projection, false)
    }

    fn validate_paths_in_context_mode(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        projection: Option<&FinishedBindings>,
        allow_unreachable_source_order: bool,
    ) -> Result<(), String> {
        let ledger = context.ledger();
        let source = &self.source;
        let owner = source.site().owner();
        let array = ledger.live_owned_array_append_value_v1(source)?;
        if self
            .receiver
            .value_for(owner, source.receiver().node(), source.holder())
            .map_err(|issue| issue.to_string())?
            != array
        {
            return Err(freeze("array-append/receiver-drift"));
        }
        for (block, instruction) in &self.original {
            if !super::super::physical_boundary::check_binding(
                function,
                projection,
                *block,
                instruction,
            )? {
                return Err(freeze("array-append/physical-binding-drift"));
            }
        }
        let map = |original: &Binding| -> Result<Binding, String> {
            match projection {
                Some(projection) => projection
                    .binding(original.0, &original.1)?
                    .ok_or_else(|| freeze("array-append/producer-removed")),
                None => Ok(original.clone()),
            }
        };
        let invoke = map(&self.invoke)?;
        let continuation = match projection {
            Some(projection) => projection.binding(self.continuation.0, &self.continuation.1)?,
            None => Some(self.continuation.clone()),
        };
        let MirInstruction::Invoke {
            operation,
            fault_frame,
            normal_landing,
            fault_landing,
        } = &invoke.1
        else {
            return Err(freeze("array-append/invoke-missing"));
        };
        let dynamic_carrier = source.dynamic_carrier_source().is_some();
        let (observed, candidate) = match (operation, dynamic_carrier) {
            (
                InvokeOperation::OwnedArray(Array::Append {
                    array,
                    descriptor,
                    candidate,
                }),
                false,
            ) if source.descriptor().is_some_and(|expected| *descriptor == expected) => {
                (*array, *candidate)
            }
            (
                InvokeOperation::DynamicCarrierArray(
                    crate::mir::instruction::DynamicCarrierArrayInvokeOperation::Append {
                        array,
                        carrier,
                    },
                ),
                true,
            ) => (*array, *carrier),
            _ => return Err(freeze("array-append/profile-drift")),
        };
        let continuation_block = projection.map_or(Some(self.continuation.0), |p| {
            p.destination(self.continuation.0)
        });
        if observed != array
            || Some(*normal_landing) != continuation_block
            || normal_landing == fault_landing
            || !matches!(
                self.continuation.1,
                MirInstruction::Jump {
                    edge_args: None,
                    ..
                }
            )
        {
            return Err(freeze("array-append/transfer-drift"));
        }
        let homes = ledger.owned_array_append_unwind_v1(source)?;
        let cancellation = match &self.candidate {
            OwnedArrayAppendCandidateV1::Null(original) => {
                let null = map(original)?;
                if dynamic_carrier
                    || !source.null_candidate()
                    || !matches!(null.1, MirInstruction::Const {
                    dst, value: crate::mir::ConstValue::Null } if dst == candidate)
                {
                    return Err(freeze("array-append/null-source-drift"));
                }
                require_before(
                    function,
                    &null,
                    &invoke,
                    allow_unreachable_source_order.then_some(self.original.as_slice()),
                )?;
                Vec::new()
            }
            OwnedArrayAppendCandidateV1::Call(packet) => {
                if dynamic_carrier {
                    return Err(freeze("array-append/typed-candidate-in-carrier-array"));
                }
                let row = packet.original_row()?;
                source.check_candidate_v1(row)?;
                let original = ledger.borrowed_object_call_packet_v1(row)?;
                if packet.value_in_context_v1(owner, original.arguments(), context)? != candidate {
                    return Err(freeze("array-append/candidate-normal-value"));
                }
                let inventory = match projection {
                    Some(projection) => projection.bindings(&self.original)?,
                    None => self.original.clone(),
                };
                packet.validate_recorded_projected(&inventory, projection)?;
                let argument_source = crate::mir::normal_callable_semantic_package::ExpressionCallArgumentSourceV1::for_owned_array_candidate_v1(
                    ledger, source, row)?;
                packet.validate_expression_arguments_at_source_v1(
                    context,
                    function,
                    projection,
                    &inventory,
                    projection,
                    None,
                    Some(argument_source),
                    &original.cancellation_operations_v1(candidate),
                )?;
                let call = map(packet.invoke_binding())?;
                let normal = map(packet.outer_bindings()?.1)?;
                let MirInstruction::Invoke {
                    operation: InvokeOperation::Call { result, .. },
                    fault_frame: call_frame,
                    normal_landing: call_normal,
                    fault_landing: call_fault,
                } = call.1
                else {
                    return Err(freeze("array-append/candidate-call-missing"));
                };
                if call_frame != *fault_frame
                    || result != original.result()
                    || !matches!(normal.1, MirInstruction::InvokeNormalResult { invoke_block, dst }
                        if invoke_block == call.0 && normal.0 == call_normal && dst == candidate)
                {
                    return Err(freeze("array-append/candidate-normal-link"));
                }
                let fault_operations: Vec<_> = packet
                    .deferred_argument_ends_v1(context)?
                    .into_iter()
                    .chain(homes.iter().cloned())
                    .collect();
                super::super::root_cleanup_graph::ordered_paths::pending_operations(
                    function,
                    call_fault,
                    &fault_operations,
                    Some(call_frame),
                )?;
                require_before(
                    function,
                    &map(packet.argument_completion_binding_v1()?)?,
                    &invoke,
                    allow_unreachable_source_order.then_some(self.original.as_slice()),
                )?;
                original.cancellation_operations_v1(candidate)
            }
            OwnedArrayAppendCandidateV1::DynamicExpression(packet) => {
                if !dynamic_carrier || source.null_candidate() {
                    return Err(freeze("array-append/dynamic-candidate-profile"));
                }
                let (value, entry) = packet.dynamic_carrier_append_value_v1(source, context)?;
                if value != candidate {
                    return Err(freeze("array-append/dynamic-candidate-value"));
                }
                let inventory = match projection {
                    Some(projection) => projection.bindings(&self.original)?,
                    None => self.original.clone(),
                };
                packet.validate_expression_recorded_v1(&inventory, projection)?;
                packet.validate_expression_physical_v1(
                    context,
                    function,
                    projection,
                    &inventory,
                    projection,
                )?;
                let entry = map(&entry)?;
                require_before(
                    function,
                    &entry,
                    &invoke,
                    allow_unreachable_source_order.then_some(self.original.as_slice()),
                )?;
                vec![InvokeOperation::DynamicCarrierEnd { carrier: value }]
            }
        };
        let unwind: Vec<_> = cancellation.into_iter().chain(homes).collect();
        super::super::root_cleanup_graph::ordered_paths::pending_operations(
            function,
            *fault_landing,
            &unwind,
            Some(*fault_frame),
        )?;
        if let Some(continuation) = continuation {
            require_before(
                function,
                &invoke,
                &continuation,
                allow_unreachable_source_order.then_some(self.original.as_slice()),
            )?;
        }
        Ok(())
    }
}
fn require_before(
    function: &MirFunction,
    producer: &Binding,
    consumer: &Binding,
    unreachable_source_order: Option<&[Binding]>,
) -> Result<(), String> {
    let locate = |binding: &Binding| -> Result<usize, String> {
        let block = function
            .blocks
            .get(&binding.0)
            .ok_or_else(|| freeze("array-append/block-missing"))?;
        let mut rows = block
            .all_instructions()
            .enumerate()
            .filter(|(_, row)| *row == &binding.1);
        let (position, _) = rows
            .next()
            .ok_or_else(|| freeze("array-append/producer-missing"))?;
        if rows.next().is_some() {
            return Err(freeze("array-append/producer-duplicate"));
        }
        Ok(position)
    };
    let (before, after) = (locate(producer)?, locate(consumer)?);
    let dom = crate::mir::verification::utils::compute_dominators(function);
    let producer_reachable = dom.is_reachable(producer.0);
    let consumer_reachable = dom.is_reachable(consumer.0);
    if producer_reachable && consumer_reachable {
        if !dom.dominates(producer.0, consumer.0)
            || (producer.0 == consumer.0 && before >= after)
        {
            return Err(freeze("array-append/evaluation-order"));
        }
        return Ok(());
    }
    if !producer_reachable && !consumer_reachable {
        if let Some(inventory) = unreachable_source_order {
            let source_position = |needle: &Binding| -> Result<usize, String> {
                let mut rows = inventory
                    .iter()
                    .enumerate()
                    .filter(|(_, row)| *row == needle);
                let (position, _) = rows
                    .next()
                    .ok_or_else(|| freeze("array-append/evaluation-order"))?;
                if rows.next().is_some() {
                    return Err(freeze("array-append/evaluation-order"));
                }
                Ok(position)
            };
            if source_position(producer)? < source_position(consumer)? {
                return Ok(());
            }
        }
    }
    Err(freeze("array-append/evaluation-order"))
}
impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn record_owned_array_append_bindings_v1(
        &self,
        packet: Rc<EmittedOwnedArrayAppendV1>,
        bindings: Vec<Binding>,
    ) -> Result<(), String> {
        self.prepare_local_call_binding_group(
            packet.source().site().owner(),
            RootLifecyclePrefixBindingGroupV1::new_append(packet, bindings)?,
        )?
        .commit()
    }
}
