//! Same returned-New and received-call proofs lent to the final Birth ABI.
//! The generic BoundValue kind itself grants no ownership or non-null claim.
use super::*;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1;
use crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreRhsV1;

impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn received_birth_actual_result_v1(
        &self, actual: &FinalizedBirthActualsV1, ordinal: u32, function: &MirFunction,
    ) -> Result<Option<InvokeCallResultKind>, String> {
        self.with_received_birth_actual_result_v1(actual, ordinal, function, |kind, _| Ok(kind))
    }

    /// The result and every original field-support loan are projected by the
    /// same validation pass. A consumer may retain these immutable loans; no
    /// source classification or acquisition is repeated by the callback.
    pub(super) fn with_received_birth_actual_result_v1<T>(
        &self, actual: &FinalizedBirthActualsV1, ordinal: u32, function: &MirFunction,
        read: impl FnOnce(InvokeCallResultKind, &[FinalizedObjectReturnFieldSupportV1]) -> Result<T, String>,
    ) -> Result<Option<T>, String> {
        let argument = actual.arguments().iter().find(|arg| arg.source().ordinal() == ordinal)
            .ok_or_else(|| freeze("received-birth/argument-missing"))?;
        let OrdinaryNewTrivialArgumentKindV1::BoundValue { binding } = argument.source().kind()
            else { return Ok(None) };
        let binding = *binding;
        let argument_value = argument.value_only_v1()?;
        if actual.destination().is_some() || actual.owner() != binding.owner() {
            return Err(freeze("received-birth/result-source-required"));
        }
        let completion = self.ledger.completion_for_owner(binding.owner())
            .ok_or_else(|| freeze("received-birth/completion"))?;
        let mut calls = completion.cleanup().root_flow()
            .ok_or_else(|| freeze("received-birth/source-flow"))?.local_calls().iter()
            .filter(|call| call.local_binding().is_some_and(|(_, local)| local == binding));
        let original = calls.next().ok_or_else(|| freeze("received-birth/acquisition"))?;
        if calls.next().is_some() || original.owner() != binding.owner()
            || !matches!(original.local_binding(), Some((SourceBindingSiteV1::Local { .. }, local)) if local == binding) {
            return Err(freeze("received-birth/acquisition-identity"));
        }
        // Each original source owner keeps its own result law. A Mixed
        // obligation may be Fresh, Forward or Null; removing it from Normal
        // Homes does not make every outcome owned.
        let (result_kind, release) = match original.result() {
            LocalCallResultClassV1::Handle => {
                let call = self.ledger.handle_call_source(original.site())
                    .ok_or_else(|| freeze("received-birth/acquisition"))?;
                if call.owner() != binding.owner()
                    || call.site() != original.site()
                    || call.local_binding() != original.local_binding()
                    || original.local_binding().is_none()
                {
                    return Err(freeze("received-birth/acquisition-identity"));
                }
                (InvokeCallResultKind::Handle, CallReceivedReleaseV1::Handle)
            }
            LocalCallResultClassV1::Nullable => {
                self.ledger.borrowed_nullable_actual_class_v1(binding)?
                    .ok_or_else(|| freeze("received-birth/nullable-source-required"))?;
                (InvokeCallResultKind::NullableHandle, CallReceivedReleaseV1::Nullable)
            }
            LocalCallResultClassV1::MixedObject => {
                let loan = original.mixed().ok_or_else(|| freeze("received-birth/mixed-source-required"))?;
                if loan.call() != original.site() || loan.destination() != binding
                    || original.local_binding() != Some((loan.declaration(), binding)) {
                    return Err(freeze("received-birth/mixed-source-identity"));
                }
                // The existing received-packet validator below rechecks the
                // SAME Mixed roots, result seal, support and teardown.
                (InvokeCallResultKind::MixedHandle { formal_ordinal: loan.callee_ordinal() },
                    CallReceivedReleaseV1::Mixed { formal_ordinal: loan.callee_ordinal() })
            }
            _ => return Err(freeze("received-birth/result-source-unavailable")),
        };
        self.ledger.with_finished_projection(binding.owner(), |symbol, finishing| {
            if function.signature.name != symbol { return Err(freeze("received-birth/function")); }
            let commits = self.ledger.local_commits.borrow();
            let Some(LocalCommitV1::Result(outer)) = commits.get(actual.site()) else {
                return Err(freeze("received-birth/outer-source"));
            };
            let prefix = outer.home_prefix.as_ref().map_err(|_| freeze("received-birth/prefix"))?;
            let rows = outer.argument_rows.as_ref().map_err(|_| freeze("received-birth/source-arguments"))?;
            let source = rows.iter().find(|row| row.ordinal() == ordinal)
                .ok_or_else(|| freeze("received-birth/source-argument"))?;
            self.ledger.check_return_argument_relations_v1(actual.site(), rows)?;
            let plan = outer.construction.as_ref().map_err(|_| freeze("received-birth/construction"))?;
            if outer.site != *actual.site() || outer.birth_target.as_ref() != Some(actual.target())
                || source != argument.source()
                || prefix.prior_homes().iter().filter(|home| home.binding() == binding).count() != 1
                || prefix.normal_homes().contains(&crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Stable(binding))
                || !matches!(&outer.emission, NewEmissionProgress::Emitted {
                    result, arguments, progress: EmittedLocalProgress::Checked { .. }, ..
                } if *result == actual.receiver() && arguments.as_ref() == actual.arguments())
            { return Err(freeze("received-birth/original-move")); }
            let Some(LocalCommitV1::CallReceived(received)) = commits.get(original.site()) else {
                return Err(freeze("received-birth/received-commit"));
            };
            if received.owner != binding.owner() || received.binding != binding
                || received.release != release
                || original.local_binding() != Some((&received.declaration, binding))
            { return Err(freeze("received-birth/received-identity")); }
            let [relation] = source.child_relations() else {
                return Err(freeze("received-birth/child-relation"));
            };
            let (constructor, owner) = plan.constructor()
                .ok_or_else(|| freeze("received-birth/constructor"))?;
            if relation.outer() != actual.site() || relation.actual().owner() != binding.owner()
                || relation.actual().site() != source.site() || relation.formal_ordinal() != ordinal
                || relation.field().object() != outer.object || relation.child() != received.object
                || relation.constructor_owner() != *owner || !relation.constructor_source().same_as(constructor)
                || plan.stores().iter().filter(|store| store.field() == relation.field()
                    && matches!(store.rhs(), ConstructionStoreRhsV1::Parameter { binding, provided: Some(child), .. }
                        if *binding == relation.formal_binding() && *child == received.object)).count() != 1
            { return Err(freeze("received-birth/source-relation")); }
            let CallReceivedProgress::Emitted { result, bindings, packet: Some(packet), .. } = &received.progress else {
                return Err(freeze("received-birth/received-packet"));
            };
            received.checked_bindings()?;
            if packet.call_site() != original.site() { return Err(freeze("received-birth/packet-site")); }
            packet.validate_recorded(bindings)?;
            packet.validate_recorded_projected(&finishing.bindings(bindings)?, Some(finishing))?;
            let producer = packet.outer_bindings()?.1;
            if !matches!(&producer.1, MirInstruction::InvokeNormalResult { dst, .. } if dst == result)
                || !bindings.contains(producer) { return Err(freeze("received-birth/original-producer")); }
            let final_producer = finishing.binding(producer.0, &producer.1)?
                .ok_or_else(|| freeze("received-birth/producer-removed"))?;
            if !matches!(&final_producer.1, MirInstruction::InvokeNormalResult { dst, .. } if *dst == argument_value)
                || function.blocks.get(&final_producer.0).into_iter().flat_map(|block| block.all_instructions())
                    .filter(|instruction| **instruction == final_producer.1).count() != 1
            { return Err(freeze("received-birth/finished-producer")); }
            let mut field_supports = Vec::new();
            if let Some(loan) = original.mixed() {
                let supports = self.ledger.returned_new_field_supports_v1(actual.site())?;
                let selected: Vec<_> = supports.iter().filter(|support|
                    std::rc::Rc::ptr_eq(&support.relation, relation)).collect();
                let required = relation.witnesses().iter().filter(|witness| matches!(witness.origin(),
                    super::super::result_class_claim::ResultValueOriginV1::ForwardFormal { .. })).count();
                if selected.is_empty() || selected.len() != required {
                    return Err(freeze("received-birth/field-support-census"));
                }
                let (formal, ordinal) = loan.formal_support_v1()
                    .ok_or_else(|| freeze("received-birth/acquired-support-escape"))?;
                if selected.iter().any(|support|
                    support.formal != formal || support.ordinal != ordinal) {
                    return Err(freeze("received-birth/field-support-source"));
                }
                // Alternative Forward witnesses share one original argument
                // evaluation. Check every source witness, then borrow that
                // same physical read once rather than once per alternative.
                let support = selected[0];
                let has_receiver = self.ledger.borrowed_ordinary_entry_receiver_required_v1(binding.owner())?;
                let (entry, copies) = packet.field_support_projection_v1(support, loan.callee_ordinal(),
                    CallValidationContextV1::Finalized(self))?;
                let lane = support.ordinal as usize + usize::from(has_receiver);
                if function.params.get(lane) != Some(&entry)
                    || function.metadata.physical_param_carriers.as_ref().and_then(|rows| rows.get(lane))
                        != Some(&crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue)
                { return Err(freeze("received-birth/field-support-parameter")); }
                let copies = copies.iter().map(|copy| finishing.relation_binding(function, copy)?
                    .ok_or_else(|| freeze("received-birth/field-support-copy")))
                    .collect::<Result<Vec<_>, String>>()?;
                let NewEmissionProgress::Emitted { bindings: construction_bindings, .. } = &outer.emission
                    else { return Err(freeze("received-birth/construction-bindings")); };
                let mut birth = construction_bindings.iter().filter(|(_, row)| matches!(row,
                    MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Birth(call), .. }
                    if &call.key == actual.target() && call.receiver == actual.receiver()
                        && call.arguments.iter().eq(actual.arguments().iter().map(|arg| arg.payload()))));
                let original_birth = birth.next().ok_or_else(|| freeze("received-birth/original-birth"))?;
                if birth.next().is_some() { return Err(freeze("received-birth/duplicate-birth")); }
                let birth = finishing.binding(original_birth.0, &original_birth.1)?
                    .ok_or_else(|| freeze("received-birth/mandatory-birth"))?;
                let original_invoke = packet.outer_bindings()?.0;
                let invoke = finishing.binding(original_invoke.0, &original_invoke.1)?
                    .ok_or_else(|| freeze("received-birth/mandatory-support-invoke"))?;
                let projected = FinalizedObjectReturnFieldSupportV1::new(
                    selected.into_iter().cloned().collect(), entry, copies,
                    [invoke, final_producer.clone()], loan.callee_ordinal(), Some(birth));
                projected.validate_function(function, has_receiver)?;
                field_supports.push(projected);
            }
            // Cheap source/actual and final-producer correspondence refuses
            // drift before recursively checking the unchanged callee seals.
            self.ledger.validate_call_received_emission_in_context_v1(original.site(), function, Some(finishing), None,
                CallValidationContextV1::Finalized(self))?;
            read(result_kind, &field_supports).map(Some)
        })
    }
}
