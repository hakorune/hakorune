//! Borrow original mixed Return evidence through the existing finishing owner.
//! This projection grants no ABI or runtime ownership by itself.
use super::super::super::completion_index::{MixedReturnSourceV1, VerifiedMixedCallResultV1};
use super::*;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::MirModule;
use std::collections::BTreeSet;
#[path = "finalized_object_return_outcomes/map_slot.rs"]
mod map_slot;
#[path = "finalized_object_return_outcomes/declared_read.rs"]
mod declared_read;
use declared_read::FinalizedDeclaredReadTransportV1;
pub(in crate::mir) use map_slot::FinalizedMapSlotReturnV1;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir) enum FinalizedObjectReturnOriginV1 {
    Null {
        producer: Binding,
    },
    Fresh {
        producer: Binding,
        allocation: Binding,
    },
    OwnedCall {
        producer: Binding,
        invoke: Binding,
    },
    BorrowedCall {
        producer: Binding,
        invoke: Binding,
        support: crate::mir::instruction::BorrowedResultSupportV1,
    },
    MapSlot(FinalizedMapSlotReturnV1),
    BorrowedReceiver { value: ValueId },
    BorrowedReceiverField {
        receiver: ValueId,
        source: Rc<crate::mir::normal_callable_semantic_package::StoredChildFieldSourceV1>,
        invoke: Binding, producer: Binding,
    },
    /// The original nullable formal carries Null/Forward at runtime. It is
    /// not an instruction producer and never acquires a Home here.
    BorrowedFormal {
        value: ValueId,
        formal_ordinal: u32,
        /// Optional original qualified constraint. NullableForwarded itself
        /// promises identity with a formal, never a concrete object class.
        object: Option<hakorune_mir_defs::CanonicalObjectIdV1>,
    },
}

#[derive(Debug, Clone, PartialEq)]
enum FinalizedReturnReadV1 {
    Local(FinalizedDeclaredReadTransportV1),
    Direct(Box<crate::mir::normal_callable_semantic_package::BorrowedCallSourceLoanV1>),
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FinalizedObjectReturnOutcomeV1 {
    // Original OUTER support is independent of the inner Call's support.
    // The retained Invoke establishes actual substitution without inventing
    // a logical argument for the receiver.
    result: InvokeCallResultKind,
    returned: Binding,
    origin: FinalizedObjectReturnOriginV1,
    transport: Vec<Binding>,
    declared_read: Option<FinalizedReturnReadV1>,
    field_supports: Box<[FinalizedObjectReturnFieldSupportV1]>,
}
impl FinalizedObjectReturnOutcomeV1 {
    pub(in crate::mir) fn declared_read_transport_v1(&self) -> Option<&FinalizedDeclaredReadTransportV1> {
        match &self.declared_read { Some(FinalizedReturnReadV1::Local(read)) => Some(read), _ => None }
    }
    pub(in crate::mir) fn has_declared_support_read_v1(&self) -> bool {
        match &self.declared_read {
            Some(FinalizedReturnReadV1::Local(_)) => true,
            Some(FinalizedReturnReadV1::Direct(source)) => source.is_declared_read_v1()
                && source.return_statement().is_some() && source.reborrowed_read_v1().is_some(),
            None => false,
        }
    }
    pub(in crate::mir) fn returned(&self) -> &Binding { &self.returned }
    pub(in crate::mir) fn origin(&self) -> &FinalizedObjectReturnOriginV1 { &self.origin }
    pub(in crate::mir) fn has_field_supports(&self) -> bool { !self.field_supports.is_empty() }
    pub(in crate::mir) fn field_supports(&self) -> &[FinalizedObjectReturnFieldSupportV1] { &self.field_supports }

    #[cfg(test)]
    pub(in crate::mir) fn field_support_witness_counts(&self) -> Vec<usize> {
        self.field_supports.iter().map(|group| group.sources().len()).collect()
    }


    /// Recheck the retained projection against the consumer's exact function.
    /// This validates correspondence only; it never reclassifies an origin.
    pub(in crate::mir) fn validate_physical_function_v1(
        &self, function: &MirFunction, result: InvokeCallResultKind, has_receiver: bool,
    ) -> Result<(), String> {
        if result != self.result { return Err(freeze("object-return/physical-result")); }
        if let InvokeCallResultKind::BorrowedNullableHandle {
            access: crate::mir::instruction::BorrowedResultAccessV1::LegacyUnqualified,
            support: support @ crate::mir::instruction::BorrowedResultSupportV1::ReceiverFieldSlot { .. }
        } = result {
            let FinalizedObjectReturnOriginV1::MapSlot(source) = &self.origin else {
                return Err(freeze("object-return/physical-origin-role"));
            };
            if !has_receiver { return Err(freeze("object-return/physical-carrier")); }
            source.validate(function, support)?;
            source.validate_uses(function, &self.returned, &self.transport)?;
            return self.validate_owned_physical_function_v1(function, has_receiver);
        }
        if let InvokeCallResultKind::BorrowedNullableHandle {
            access: crate::mir::instruction::BorrowedResultAccessV1::ReadOnly,
            support: crate::mir::instruction::BorrowedResultSupportV1::Receiver,
        } = result {
            if !has_receiver || !matches!(self.origin,
                FinalizedObjectReturnOriginV1::Null { .. } | FinalizedObjectReturnOriginV1::BorrowedReceiver { .. } | FinalizedObjectReturnOriginV1::BorrowedReceiverField { .. }
                | FinalizedObjectReturnOriginV1::BorrowedCall { support: crate::mir::instruction::BorrowedResultSupportV1::Receiver, .. }) {
                return Err(freeze("object-return/physical-origin-role"));
            }
            if let FinalizedObjectReturnOriginV1::BorrowedReceiver { value }
                | FinalizedObjectReturnOriginV1::BorrowedReceiverField { receiver: value, .. } = &self.origin {
                if function.params.first() != Some(value)
                    || function.metadata.physical_param_carriers.as_deref().and_then(|rows| rows.first())
                        != Some(&crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::ExistingCallableI64) {
                    return Err(freeze("object-return/physical-carrier"));
                }
            }
            if let FinalizedObjectReturnOriginV1::BorrowedCall { invoke, producer, .. } = &self.origin {
                if function.metadata.physical_param_carriers.as_deref().and_then(|rows| rows.first())
                    != Some(&crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::ExistingCallableI64) {
                    return Err(freeze("object-return/physical-carrier"));
                }
                if !matches!((&invoke.1, &producer.1),
                    (MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { call,
                        result: InvokeCallResultKind::BorrowedNullableHandle { access: crate::mir::instruction::BorrowedResultAccessV1::ReadOnly,
                            support: crate::mir::instruction::BorrowedResultSupportV1::Receiver } }, normal_landing, fault_landing, .. },
                    MirInstruction::InvokeNormalResult { invoke_block, .. })
                    if *invoke_block == invoke.0 && *normal_landing == producer.0 && normal_landing != fault_landing
                        && call.callee.receiver().is_some() && call.callee.receiver().as_ref() == function.params.first()) {
                    return Err(freeze("object-return/physical-origin-role"));
                }
            }
            return self.validate_owned_physical_function_v1(function, has_receiver);
        }
        let ordinal = match result {
            InvokeCallResultKind::MixedHandle { formal_ordinal }
            | InvokeCallResultKind::BorrowedNullableHandle { access: _, support: crate::mir::instruction::BorrowedResultSupportV1::Formal(formal_ordinal) } => formal_ordinal,
            InvokeCallResultKind::Handle | InvokeCallResultKind::NullableHandle => {
                if !matches!(self.origin, FinalizedObjectReturnOriginV1::Fresh { .. }
                    | FinalizedObjectReturnOriginV1::OwnedCall { .. })
                    && !(result == InvokeCallResultKind::NullableHandle
                        && matches!(self.origin, FinalizedObjectReturnOriginV1::Null { .. }))
                { return Err(freeze("object-return/physical-origin-role")); }
                return self.validate_owned_physical_function_v1(function, has_receiver);
            }
            _ => return Err(freeze("object-return/physical-result")),
        };
        let lane = ordinal as usize + usize::from(has_receiver);
        let value = function.params.get(lane).ok_or_else(|| freeze("object-return/physical-ordinal"))?;
        if function.metadata.physical_param_carriers.as_ref().and_then(|rows| rows.get(lane))
            != Some(&crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue)
        { return Err(freeze("object-return/physical-carrier")); }
        if matches!(result, InvokeCallResultKind::BorrowedNullableHandle { .. })
            && !matches!(self.origin, FinalizedObjectReturnOriginV1::Null { .. } | FinalizedObjectReturnOriginV1::BorrowedFormal { .. })
            && !(matches!(result, InvokeCallResultKind::BorrowedNullableHandle { access: crate::mir::instruction::BorrowedResultAccessV1::ReadOnly, .. })
                && matches!(self.origin, FinalizedObjectReturnOriginV1::BorrowedCall { support: crate::mir::instruction::BorrowedResultSupportV1::Formal(index), .. } if index == ordinal))
            || matches!(result, InvokeCallResultKind::MixedHandle { .. })
                && matches!(self.origin, FinalizedObjectReturnOriginV1::BorrowedFormal { .. })
        { return Err(freeze("object-return/physical-origin-role")); }
        if let (InvokeCallResultKind::BorrowedNullableHandle { access: crate::mir::instruction::BorrowedResultAccessV1::ReadOnly, .. },
            FinalizedObjectReturnOriginV1::BorrowedCall { invoke, producer, .. }) = (result, &self.origin) {
            if !matches!((&invoke.1, &producer.1),
                (MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { call,
                    result: InvokeCallResultKind::BorrowedNullableHandle { access: crate::mir::instruction::BorrowedResultAccessV1::ReadOnly,
                        support: crate::mir::instruction::BorrowedResultSupportV1::Formal(inner) } }, normal_landing, fault_landing, .. },
                 MirInstruction::InvokeNormalResult { invoke_block, .. })
                    if *invoke_block == invoke.0 && *normal_landing == producer.0 && normal_landing != fault_landing
                        && call.args.get(*inner as usize).is_some()) {
                return Err(freeze("object-return/physical-origin-role"));
            }
        }
        if let FinalizedObjectReturnOriginV1::BorrowedFormal { value: original, formal_ordinal, .. } = &self.origin {
            if *formal_ordinal != ordinal || original != value {
                return Err(freeze("object-return/physical-formal"));
            }
        }
        self.validate_owned_physical_function_v1(function, has_receiver)
    }

    fn validate_owned_physical_function_v1(&self, function: &MirFunction, has_receiver: bool) -> Result<(), String> {
        for support in &self.field_supports { support.validate_function(function, has_receiver)?; }
        let mut required = vec![&self.returned];
        required.extend(self.transport.iter());
        if let Some(read) = self.declared_read_transport_v1() {
            if !matches!((self.result, &self.origin),
                (InvokeCallResultKind::BorrowedNullableHandle { access: crate::mir::instruction::BorrowedResultAccessV1::ReadOnly, .. },
                 FinalizedObjectReturnOriginV1::BorrowedCall { .. })) { return Err(freeze("object-return/physical-origin-role")); }
            required.extend(read.bindings());
        }
        match &self.origin {
            FinalizedObjectReturnOriginV1::Null { producer } => required.push(producer),
            FinalizedObjectReturnOriginV1::Fresh { producer, allocation } => required.extend([producer, allocation]),
            FinalizedObjectReturnOriginV1::OwnedCall { producer, invoke }
            | FinalizedObjectReturnOriginV1::BorrowedCall { producer, invoke, .. } => required.extend([producer, invoke]),
            FinalizedObjectReturnOriginV1::MapSlot(source) => required.extend(source.bindings()),
            FinalizedObjectReturnOriginV1::BorrowedReceiverField { receiver, source, invoke, producer } => {
                required.extend([invoke, producer]);
                if !has_receiver || function.params.first() != Some(receiver)
                    || !matches!((&invoke.1, &producer.1),
                        (MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::ObjectFieldBorrow {
                            field, base, child, nullable }, normal_landing, fault_landing, .. },
                         MirInstruction::InvokeNormalResult { invoke_block, .. })
                            if *field == source.field() && *base == *receiver && *child == source.child()
                                && *nullable == source.nullable() && *invoke_block == invoke.0
                                && *normal_landing == producer.0 && normal_landing != fault_landing) {
                    return Err(freeze("object-return/physical-origin-role"));
                }
            }
            FinalizedObjectReturnOriginV1::BorrowedReceiver { value }
            | FinalizedObjectReturnOriginV1::BorrowedFormal { value, .. } => {
                if function.params.iter().filter(|parameter| *parameter == value).count() != 1 {
                    return Err(freeze("object-return/physical-formal"));
                }
            }
        }
        for binding in required {
            if !super::super::physical_boundary::check_binding(function, None, binding.0, &binding.1)?
                || function.blocks.get(&binding.0).map(|block| block.all_instructions()
                    .filter(|row| *row == &binding.1).count()) != Some(1)
            {
                return Err(freeze("object-return/physical-binding"));
            }
        }
        if function.blocks.get(&self.returned.0).and_then(|block| block.terminator.as_ref()) != Some(&self.returned.1) {
            return Err(freeze("object-return/physical-terminator"));
        }
        Ok(())
    }

    pub(in crate::mir) fn transport_bindings(&self) -> &[Binding] {
        &self.transport
    }

    pub(in crate::mir) fn origin_name(&self) -> &'static str {
        match self.origin {
            FinalizedObjectReturnOriginV1::Null { .. } => "null",
            FinalizedObjectReturnOriginV1::Fresh { .. } => "fresh",
            FinalizedObjectReturnOriginV1::OwnedCall { .. } => "owned_call",
            FinalizedObjectReturnOriginV1::BorrowedCall { .. } => "borrowed_call",
            FinalizedObjectReturnOriginV1::BorrowedFormal { .. } => "borrowed_formal",
            FinalizedObjectReturnOriginV1::BorrowedReceiver { .. } => "borrowed_receiver",
            FinalizedObjectReturnOriginV1::BorrowedReceiverField { .. } => "borrowed_receiver_field",
            FinalizedObjectReturnOriginV1::MapSlot(_) => "borrowed_map_slot",
        }
    }
}

impl FinalizedRootSourceHandoffV1 {
    /// Borrowed nullable callees use the same Return correspondence owner.
    /// Every original incoming packet must still corroborate its own seal.
    pub(in crate::mir) fn borrowed_nullable_return_projections_v1(
        &self,
        module: &MirModule,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Result<(Vec<FinalizedObjectReturnOutcomeV1>, Option<std::sync::Arc<crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>>), String> {
        let mut outcomes: Option<(Vec<FinalizedObjectReturnOutcomeV1>, Option<std::sync::Arc<crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>>)> = None;
        self.visit_finalized_borrowed_call_nodes_v1(module, |packet, _, _, _, _| {
            if packet.target() != key {
                return Ok(());
            }
            let super::super::finalized_borrowed_call_visit::FinalizedBorrowedCallPacketV1::Lexical(
                packet,
            ) = packet
            else {
                return Err(freeze("borrowed-return/original-packet-kind"));
            };
            let row = packet.original_row()?;
            self.ledger.corroborate_borrowed_nullable_call_v1(row)?;
            let source = row.source_target();
            let loan = source
                .borrowed_nullable_call_source()
                .ok_or_else(|| freeze("borrowed-return/original-loan"))?;
            if source.target() != key
                || source.callee_owner() != loan.callee_owner()
                || row.result()
                    != Some(loan.result_kind_v1())
            {
                return Err(freeze("borrowed-return/original-contract"));
            }
            let current = self.project_borrowed_nullable_returns(module, source)?;
            if outcomes.as_ref().is_some_and(|(prior, declaration)| prior != &current ||
                !loan.declared_result_matches_v1(declaration.as_ref())) {
                return Err(freeze("borrowed-return/incoming-disagreement"));
            }
            outcomes = Some((current, loan.declared_read_result_v1().cloned()));
            Ok(())
        })?;
        outcomes.ok_or_else(|| freeze("borrowed-return/original-seal-missing"))
    }

    /// Same source packet and entry-support correspondence for inferred Mixed
    /// and explicit read forwarding. Access remains the original caller law.
    fn project_borrowed_call_return_v1(
        &self, borrowed: &crate::mir::normal_callable_semantic_package::BorrowedCallResultSourceLoanV1,
        destination: Option<&SourceStmtSiteV1>, outer_support: crate::mir::instruction::BorrowedResultSupportV1, owner: FunctionOwnerIdV1,
        namespace: hakorune_mir_defs::SameModuleCallableNamespaceV1,
        function: &MirFunction, projection: &super::super::physical_boundary::FinishedBindings,
        entry: &RootHomeExitEntry, access: crate::mir::instruction::BorrowedResultAccessV1,
    ) -> Result<FinalizedObjectReturnOriginV1, String> {
        let packet = if let Some(statement) = destination {
            let RootHomeExitEntry::Call { row: crate::mir::normal_callable_semantic_package::RootCallDispositionV1::Lexical(packet), invoke, projection, .. } = entry
                else { return Err(freeze("mixed-return/borrowed-source")); };
            let bindings = packet.outer_bindings()?;
            if packet.call_site() != borrowed.call() || bindings != (invoke, projection)
                || packet.original_row()?.source_target().borrowed_nullable_call_source()
                    .and_then(|source| source.return_statement()) != Some(statement) {
                return Err(freeze("mixed-return/borrowed-source"));
            }
            packet
        } else { original_local_packet(entry, borrowed.call())? };
        let original = packet.original_row()?;
        if original.source_target().borrowed_nullable_call_source().is_none_or(|source|
            &**source != borrowed || (destination.is_none() && source.local().is_none())) {
            return Err(freeze("mixed-return/borrowed-source"));
        }
        self.ledger.corroborate_borrowed_nullable_call_v1(original)?;
        let (invoke, producer) = packet.outer_bindings()?;
        let invoke = checked_projection(function, projection, invoke)?;
        let producer = checked_projection(function, projection, producer)?;
        if !matches!(invoke.1, MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call {
            result: InvokeCallResultKind::BorrowedNullableHandle { access: inner_access, support: inner_support }, .. }, .. }
            if inner_support == borrowed.result_support_v1() && inner_access == access) {
            return Err(freeze("mixed-return/borrowed-call-contract"));
        }
        // The original entry binding, not signature storage or
        // the final Return operand, establishes the support.
        let Some(crate::mir::normal_callable_semantic_package::BorrowedLifetimeAnchorV1::Entry { root, caller_support }) = borrowed.lifetime_anchor_v1() else { return Err(freeze("mixed-return/borrowed-support")); };
        if caller_support != outer_support { return Err(freeze("mixed-return/borrowed-support")); }
        if outer_support == crate::mir::instruction::BorrowedResultSupportV1::Receiver {
            let mut contracts = self.ledger.original_parameter_contracts.as_deref().ok_or_else(|| freeze("mixed-return/borrowed-support"))?
                .iter().filter(|row| row.owner == owner);
            let contract = contracts.next().ok_or_else(|| freeze("mixed-return/borrowed-support"))?;
            if contracts.next().is_some() { return Err(freeze("mixed-return/borrowed-support")); }
            let (_, receiver) = self.ledger.checked_instance_receiver_value_v1(root, contract.batch_slot, owner, function)?;
            if !matches!(&invoke.1, MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { call, .. }, .. }
                if call.callee.receiver() == Some(receiver)) { return Err(freeze("mixed-return/borrowed-support")); }
            return Ok(FinalizedObjectReturnOriginV1::BorrowedCall { producer, invoke, support: outer_support });
        }
        let crate::mir::instruction::BorrowedResultSupportV1::Formal(outer_ordinal) = outer_support
            else { return Err(freeze("mixed-return/borrowed-support")); };
        let support = self.borrowed_ordinary_entry_values_v1(owner)?;
        let source_entry = self.borrowed_ordinary_entry_source_for_function_v1(owner, function)?;
        let mut values = support.iter().filter(|(_, binding, _)| Some(*binding) == borrowed.root());
        let (ordinal, _, value) = values.next().ok_or_else(|| freeze("mixed-return/borrowed-support"))?;
        let offset = match namespace {
            hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod => 1,
            hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod => 0,
            _ => return Err(freeze("mixed-return/borrowed-entry-namespace")),
        };
        let slot = *ordinal as usize + offset;
        if values.next().is_some() || *ordinal != outer_ordinal
            || !borrowed.root().is_some_and(|root| source_entry.formals().contains(&(*ordinal, root)))
            || function.params.get(slot) != Some(value)
            || function.metadata.physical_param_carriers.as_deref().and_then(|rows| rows.get(slot))
                != Some(&crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue) {
            return Err(freeze("mixed-return/borrowed-support"));
        }
        Ok(FinalizedObjectReturnOriginV1::BorrowedCall { producer, invoke, support: outer_support })
    }

    fn project_borrowed_nullable_returns(
        &self,
        module: &MirModule,
        source: &super::super::super::lexical_instance_call::LexicalInstanceCallSourceTargetV1,
    ) -> Result<Vec<FinalizedObjectReturnOutcomeV1>, String> {
        use super::super::super::result_class_claim::{ResultValueOriginV1, ResultWitnessStepV1};
        let owner = source.callee_owner();
        let loan = source
            .borrowed_nullable_call_source()
            .ok_or_else(|| freeze("borrowed-return/original-loan"))?;
        // This joins the SAME Completion, explicit exits, source witnesses and
        // original terminal relations; neither MIR shape nor names selects exits.
        let original_exits = self
            .ledger
            .checked_completed_object_callee_target_exits_v1(source)?
            .ok_or_else(|| freeze("borrowed-return/original-exits"))?;
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            let function = module.functions.get(symbol)
                .filter(|function| function.signature.name == symbol && symbol == source.target().mir_symbol_projection())
                .ok_or_else(|| freeze("borrowed-return/function"))?;
            let exits = self.ledger.root_exits.borrow();
            let mut outcomes = Vec::new();
            for (root, terminal) in original_exits.iter() {
                let exit = terminal.return_site();
                if terminal.owner() != owner || self.ledger.normal_exit_projection_v1(owner, exit)?.is_none() {
                    return Err(freeze("borrowed-return/normal-source"));
                }
                let Some(RootHomeExitProgress::Emitted { order, entry, bindings, .. }) =
                    exits.get(&(owner, exit.clone().into())) else {
                        return Err(freeze("borrowed-return/original-exit"));
                    };
                super::super::finalized_root_cleanup::validate_finished_cleanup_entry(
                    CallValidationContextV1::Finalized(self), owner, exit, function, projection, order, entry, bindings, Some(projection),
                )?;
                let mut returns = bindings.iter().filter(|(_, row)| matches!(row, MirInstruction::Return { .. }));
                let original = returns.next().ok_or_else(|| freeze("borrowed-return/return-missing"))?;
                if returns.next().is_some() { return Err(freeze("borrowed-return/return-duplicate")); }
                let returned = checked_projection(function, projection, original)?;
                let mut declared_read = None;
                let (origin, expected) = match (root.step(), terminal.returned()) {
                    (ResultWitnessStepV1::NullLiteral, TerminalReturnedSourceV1::NullLiteral)
                        if root.origin() == &ResultValueOriginV1::Null => {
                        let producer = checked_projection(function, projection,
                            order.null_return_binding().ok_or_else(|| freeze("borrowed-return/null-source"))?)?;
                        let MirInstruction::Const { dst, value: crate::mir::ConstValue::Null } = producer.1 else {
                            return Err(freeze("borrowed-return/null-producer"));
                        };
                        (FinalizedObjectReturnOriginV1::Null { producer }, dst)
                    }
                    (ResultWitnessStepV1::Receiver { binding, projection: super::super::super::result_class_claim::ReceiverResultProjectionV1::Identity }, TerminalReturnedSourceV1::Handle(receiver))
                        if binding == receiver && loan.result_support_v1() == crate::mir::instruction::BorrowedResultSupportV1::Receiver => {
                        let (_, value) = self.ledger.checked_instance_receiver_value_v1(
                            *binding, source.target_batch_slot(), owner, function)?;
                        (FinalizedObjectReturnOriginV1::BorrowedReceiver { value }, value)
                    }
                    (ResultWitnessStepV1::Receiver { binding, projection: super::super::super::result_class_claim::ReceiverResultProjectionV1::Field { read, source: field_source } },
                     TerminalReturnedSourceV1::ReceiverField { binding: actual, receiver: receiver_site, field, source: retained })
                        if binding == actual && read.receiver.site() == receiver_site && read.field == *field
                            && Rc::ptr_eq(field_source, retained) && loan.result_support_v1() == crate::mir::instruction::BorrowedResultSupportV1::Receiver => {
                        let (_, receiver) = self.ledger.checked_instance_receiver_value_v1(*binding, source.target_batch_slot(), owner, function)?;
                        let [invoke, producer] = self.ledger.checked_field_pair_v1(root.site(), CallValidationContextV1::Finalized(self))?
                            .ok_or_else(|| freeze("borrowed-return/source-kind"))?;
                        let invoke = checked_projection(function, projection, &invoke)?;
                        let producer = checked_projection(function, projection, &producer)?;
                        let MirInstruction::InvokeNormalResult { dst, .. } = producer.1 else { return Err(freeze("borrowed-return/source-kind")); };
                        (FinalizedObjectReturnOriginV1::BorrowedReceiverField { receiver, source: Rc::clone(field_source), invoke, producer }, dst)
                    }
                    (ResultWitnessStepV1::Formal { binding, ordinal }, TerminalReturnedSourceV1::Handle(formal))
                        if binding == formal && Some(*ordinal) == loan.callee_ordinal() => {
                        let entry_source = self.borrowed_ordinary_entry_source_for_function_v1(owner, function)?;
                        let formal_returns = entry_source.borrowed_formal_return_uses()?;
                        let entry_values = self.borrowed_ordinary_entry_values_v1(owner)?;
                        let mut uses = formal_returns.iter().filter(|(_, root, at, _)| root == formal && *at == exit);
                        let (_, _, _, retained) = uses.next().ok_or_else(|| freeze("borrowed-return/formal-source"))?;
                        if uses.next().is_some() || retained.key() != source.target()
                            || retained.ordinal() != *ordinal || retained.formal() != *formal
                            || !retained.roots().iter().any(|witness| Rc::ptr_eq(witness, root)) {
                            return Err(freeze("borrowed-return/formal-identity"));
                        }
                        self.ledger.callable_result_classes.recheck_borrowed_formal_return_source_v1(retained)
                            .map_err(freeze)?;
                        let mut values = entry_values.iter().filter(|(index, root, _)| index == ordinal && root == formal);
                        let (_, _, value) = values.next().ok_or_else(|| freeze("borrowed-return/formal-value"))?;
                        let offset = usize::from(self.ledger.borrowed_ordinary_entry_receiver_required_v1(owner)?);
                        let slot = *ordinal as usize + offset;
                        if values.next().is_some() || function.params.get(slot) != Some(value)
                            || function.metadata.physical_param_carriers.as_deref().and_then(|rows| rows.get(slot))
                                != Some(&crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue) {
                            return Err(freeze("borrowed-return/formal-carrier"));
                        }
                        let object = entry_source.formal_object_view(*formal)?.map(|view| view.object());
                        (FinalizedObjectReturnOriginV1::BorrowedFormal {
                            value: *value, formal_ordinal: *ordinal, object,
                        }, *value)
                    }
                    (ResultWitnessStepV1::Call { .. }, TerminalReturnedSourceV1::BorrowedNullableLocal(borrowed))
                        if loan.is_declared_read_v1() && borrowed.is_declared_read_v1()
                            && self.ledger.callable_result_classes.retains_declared_borrowed_return_v1(terminal, borrowed)
                            && borrowed.corroborates_call_witness_v1(root) => {
                        let origin = self.project_borrowed_call_return_v1(borrowed,
                            borrowed.direct_v1().and_then(|source| source.return_statement()), loan.result_support_v1(), owner,
                            source.target().namespace(), function, projection, entry, loan.access_v1())?;
                        let FinalizedObjectReturnOriginV1::BorrowedCall { producer, .. } = &origin else { unreachable!() };
                        let expected = if let Some(view) = borrowed.read_v1() {
                            let read = order.return_read.as_ref()
                                .ok_or_else(|| freeze("borrowed-return/source-kind"))?;
                            let retained = FinalizedDeclaredReadTransportV1::issue(CallValidationContextV1::Finalized(self),
                                view, read, function, projection)?;
                            let value = retained.value();
                            declared_read = Some(FinalizedReturnReadV1::Local(retained));
                            value
                        } else {
                            if let Some(source) = borrowed.direct_v1() {
                                declared_read = Some(FinalizedReturnReadV1::Direct(Box::new(source.clone())));
                            }
                            producer.1.dst_value().ok_or_else(|| freeze("borrowed-return/source-kind"))?
                        };
                        (origin, expected)
                    }
                    (ResultWitnessStepV1::MapStored { source: stored },
                        TerminalReturnedSourceV1::StoredMapGet { .. } | TerminalReturnedSourceV1::StoredMapInstalled(_))
                        if loan.is_map_slot() && stored.result_support_v1() == loan.result_support_v1() => {
                        let provenance = FinalizedMapSlotReturnV1::issue(stored, terminal, entry, order,
                            CallValidationContextV1::Finalized(self), function, projection)?;
                        let value = provenance.value()?;
                        (FinalizedObjectReturnOriginV1::MapSlot(provenance), value)
                    }
                    _ => return Err(freeze("borrowed-return/source-kind")),
                };
                let transport = original_copy_transport(function, projection, original, expected)?;
                let outcome = FinalizedObjectReturnOutcomeV1 {
                    result: loan.result_kind_v1(),
                    returned, origin, transport, declared_read, field_supports: Box::new([]),
                };
                if let Some(prior) = outcomes.iter().find(|prior: &&FinalizedObjectReturnOutcomeV1| prior.returned.0 == outcome.returned.0) {
                    if prior != &outcome { return Err(freeze("borrowed-return/exit-disagreement")); }
                } else { outcomes.push(outcome); }
            }
            outcomes.sort_by_key(|row| row.returned.0);
            let actual: BTreeSet<_> = function.blocks.iter().filter_map(|(id, block)|
                matches!(block.terminator, Some(MirInstruction::Return { .. })).then_some(*id)).collect();
            if actual != outcomes.iter().map(|row| row.returned.0).collect() {
                return Err(freeze("borrowed-return/return-census"));
            }
            Ok(outcomes)
        })
    }

    /// All real incoming packets must lend the SAME callee Return projections.
    /// This does not select an unused function out of the participant closure.
    pub(in crate::mir) fn mixed_return_projections_v1(
        &self,
        module: &MirModule,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Result<Vec<FinalizedObjectReturnOutcomeV1>, String> {
        let mut outcomes = None;
        self.visit_finalized_borrowed_call_nodes_v1(module, |packet, _, _, _, _| {
            if packet.target() != key {
                return Ok(());
            }
            let super::super::finalized_borrowed_call_visit::FinalizedBorrowedCallPacketV1::Lexical(
                packet,
            ) = packet
            else {
                return Err(freeze("mixed-return/original-packet-kind"));
            };
            let row = packet.original_row()?;
            self.ledger
                .corroborate_mixed_call_using_v1(row, CallValidationContextV1::Finalized(self))?;
            let proof = row
                .mixed_result_source_v1()
                .ok_or_else(|| freeze("mixed-return/original-seal-missing"))?;
            let current = self.project_mixed_returns(module, proof)?;
            if outcomes.as_ref().is_some_and(|prior| prior != &current) {
                return Err(freeze("mixed-return/incoming-disagreement"));
            }
            outcomes = Some(current);
            Ok(())
        })?;
        outcomes.ok_or_else(|| freeze("mixed-return/original-seal-missing"))
    }

    fn project_mixed_returns(
        &self,
        module: &MirModule,
        proof: &VerifiedMixedCallResultV1,
    ) -> Result<Vec<FinalizedObjectReturnOutcomeV1>, String> {
        let source = proof.source();
        let owner = source.callee_owner();
        let loan = source
            .mixed_local_source()
            .ok_or_else(|| freeze("mixed-return/source"))?;
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            let function = module.functions.get(symbol)
                .filter(|function| function.signature.name == symbol && symbol == source.target().mir_symbol_projection())
                .ok_or_else(|| freeze("mixed-return/function"))?;
            let exits = self.ledger.root_exits.borrow();
            let mut outcomes = Vec::new();
            for (_, terminal, evidence) in proof.returns() {
                let exit = terminal.return_site();
                if terminal.owner() != owner || self.ledger.normal_exit_projection_v1(owner, exit)?.is_none() {
                    return Err(freeze("mixed-return/normal-source"));
                }
                let Some(RootHomeExitProgress::Emitted { order, entry, bindings, .. }) =
                    exits.get(&(owner, exit.clone().into())) else {
                        return Err(freeze("mixed-return/original-exit"));
                    };
                super::super::finalized_root_cleanup::validate_finished_cleanup_entry(
                    CallValidationContextV1::Finalized(self), owner, exit, function, projection, order, entry, bindings, Some(projection),
                )?;
                let mut returns = bindings.iter().filter(|(_, instruction)| matches!(instruction, MirInstruction::Return { .. }));
                let original = returns.next().ok_or_else(|| freeze("mixed-return/return-missing"))?;
                if returns.next().is_some() { return Err(freeze("mixed-return/return-duplicate")); }
                let returned = checked_projection(function, projection, original)?;
                let origin = match evidence {
                    MixedReturnSourceV1::Leaf(super::super::super::completion_index::VerifiedObjectReturnLeafV1::Null) => {
                        let original = order.null_return_binding().ok_or_else(|| freeze("mixed-return/null-source"))?;
                        let producer = checked_projection(function, projection, original)?;
                        if !matches!(producer.1, MirInstruction::Const { value: crate::mir::ConstValue::Null, .. }) {
                            return Err(freeze("mixed-return/null-producer"));
                        }
                        FinalizedObjectReturnOriginV1::Null { producer }
                    }
                    MixedReturnSourceV1::Leaf(super::super::super::completion_index::VerifiedObjectReturnLeafV1::Fresh { .. }) => {
                        let [producer, allocation] = self.ledger.validate_fresh_return_producer_v1(
                            owner, exit, entry, bindings, Some(projection), Some(projection), Some(function),
                        )?.ok_or_else(|| freeze("mixed-return/fresh-producer"))?;
                        checked_projection_membership(function, projection, &producer)?;
                        checked_projection_membership(function, projection, &allocation)?;
                        FinalizedObjectReturnOriginV1::Fresh { producer, allocation }
                    }
                    MixedReturnSourceV1::OwnedCall => {
                        use crate::mir::resolved_semantics::home_new_prefix::ObjectReturnAcquisitionV1;
                        let TerminalReturnedSourceV1::OwnedCall(original) = terminal.returned()
                            else { return Err(freeze("mixed-return/owned-source")); };
                        let (invoke, producer) = match original.acquisition() {
                            ObjectReturnAcquisitionV1::Direct { .. } => match entry {
                                RootHomeExitEntry::Call { invoke, projection: producer, .. } => (invoke, producer),
                                _ => return Err(freeze("mixed-return/owned-entry")),
                            },
                            ObjectReturnAcquisitionV1::Received(received) => {
                                let packet = original_local_packet(entry, received.site())?;
                                packet.outer_bindings()?
                            }
                        };
                        let invoke = checked_projection(function, projection, invoke)?;
                        let producer = checked_projection(function, projection, producer)?;
                        if !matches!(invoke.1, MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call {
                            result: InvokeCallResultKind::Handle | InvokeCallResultKind::NullableHandle, .. }, .. }) {
                            return Err(freeze("mixed-return/owned-call-contract"));
                        }
                        FinalizedObjectReturnOriginV1::OwnedCall { producer, invoke }
                    }
                    MixedReturnSourceV1::BorrowedLocal(borrowed) => {
                        self.project_borrowed_call_return_v1(borrowed, None, crate::mir::instruction::BorrowedResultSupportV1::Formal(loan.callee_ordinal()), owner,
                            source.target().namespace(), function, projection, entry,
                            crate::mir::instruction::BorrowedResultAccessV1::LegacyUnqualified)?
                    }
                };
                let producer = match &origin {
                    FinalizedObjectReturnOriginV1::Null { producer }
                    | FinalizedObjectReturnOriginV1::Fresh { producer, .. }
                    | FinalizedObjectReturnOriginV1::OwnedCall { producer, .. }
                    | FinalizedObjectReturnOriginV1::BorrowedCall { producer, .. } => producer,
                    FinalizedObjectReturnOriginV1::BorrowedFormal { .. } | FinalizedObjectReturnOriginV1::BorrowedReceiver { .. } | FinalizedObjectReturnOriginV1::BorrowedReceiverField { .. } | FinalizedObjectReturnOriginV1::MapSlot(_) => unreachable!("Mixed original seal does not issue borrowed leaves"),
                };
                let expected = producer.1.dst_value().ok_or_else(|| freeze("mixed-return/producer-value"))?;
                let transport = original_copy_transport(function, projection, original, expected)?;
                let outcome = FinalizedObjectReturnOutcomeV1 {
                    result: InvokeCallResultKind::MixedHandle { formal_ordinal: loan.callee_ordinal() },
                    returned, origin, transport, declared_read: None, field_supports: Box::new([]),
                };
                if let Some(prior) = outcomes.iter().find(|prior: &&FinalizedObjectReturnOutcomeV1| prior.returned.0 == outcome.returned.0) {
                    if prior != &outcome { return Err(freeze("mixed-return/exit-disagreement")); }
                } else { outcomes.push(outcome); }
            }
            outcomes.sort_by_key(|row| row.returned.0);
            let actual: BTreeSet<_> = function.blocks.iter().filter_map(|(id, block)|
                matches!(block.terminator, Some(MirInstruction::Return { .. })).then_some(*id)).collect();
            if actual != outcomes.iter().map(|row| row.returned.0).collect() {
                return Err(freeze("mixed-return/return-census"));
            }
            Ok(outcomes)
        })
    }
}

fn original_local_packet<'a>(
    entry: &'a RootHomeExitEntry,
    site: &OwnedExprSiteV1,
) -> Result<&'a EmittedLexicalCallProjectionV1, String> {
    let mut groups = entry
        .local_call_groups()
        .iter()
        .filter(|group| group.site() == site);
    let packet = groups
        .next()
        .and_then(|group| group.lexical())
        .ok_or_else(|| freeze("mixed-return/child-packet"))?;
    if groups.next().is_some() || packet.call_site() != site {
        return Err(freeze("mixed-return/child-packet-identity"));
    }
    Ok(packet)
}
fn checked_projection(
    function: &MirFunction,
    projection: &super::super::physical_boundary::FinishedBindings,
    original: &Binding,
) -> Result<Binding, String> {
    let finished = projection
        .binding(original.0, &original.1)?
        .ok_or_else(|| freeze("mixed-return/mandatory-producer-removed"))?;
    checked_projection_membership(function, projection, &finished)?;
    Ok(finished)
}
fn checked_projection_membership(
    function: &MirFunction,
    projection: &super::super::physical_boundary::FinishedBindings,
    finished: &Binding,
) -> Result<(), String> {
    if !projection.recorded().contains(finished)
        || !super::super::physical_boundary::check_binding(function, None, finished.0, &finished.1)?
    {
        return Err(freeze("mixed-return/finished-producer"));
    }
    Ok(())
}

/// This is value correspondence, not result eligibility. Both ends were
/// selected by the SAME original Return source and checked Call/acquisition.
/// Only captured original Copies can bridge local publication; a final-shape
/// origin walk, Phi inference, or a new source receipt cannot replace them.
fn original_copy_transport(
    function: &MirFunction,
    projection: &super::super::physical_boundary::FinishedBindings,
    returned: &Binding,
    expected: ValueId,
) -> Result<Vec<Binding>, String> {
    let MirInstruction::Return {
        value: Some(mut value),
    } = returned.1
    else {
        return Err(freeze("mixed-return/return-value"));
    };
    let mut seen = BTreeSet::new();
    let mut transport = Vec::new();
    while value != expected {
        if !seen.insert(value) {
            return Err(freeze("mixed-return/copy-cycle"));
        }
        let original = projection.captured_definition(value)?;
        let MirInstruction::Copy { dst, src } = original.1 else {
            return Err(format!(
                "{} returned={returned:?} expected={expected:?} original={original:?}",
                freeze("mixed-return/original-copy-unavailable")
            ));
        };
        if dst != value {
            return Err(freeze("mixed-return/copy-destination"));
        }
        // A source-local Copy is an auxiliary definition, not a lifecycle
        // capability. Use the captured-definition relation owner; omission is
        // forbidden because this original Return still consumes its value.
        let finished = projection
            .relation_binding(function, original)?
            .ok_or_else(|| freeze("mixed-return/mandatory-copy-removed"))?;
        transport.push(finished);
        value = src;
    }
    transport.reverse();
    Ok(transport)
}

#[path = "finalized_object_return_outcomes/owned.rs"]
mod owned;
