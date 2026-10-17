//! Existing owned result law lends its complete original Return/field evidence.
//! Selection belongs to the completed catalog, never an incoming Call guess.
use super::*;
use crate::mir::normal_callable_semantic_package::VerifiedCallableResultContractCohortV1;
use crate::mir::resolved_semantics::home_new_prefix::ObjectReturnAcquisitionV1;

impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn owned_object_return_projections_v1(
        &self,
        module: &MirModule,
        callables: &VerifiedCallableResultContractCohortV1,
        key: &CanonicalSameModuleCallableKeyV1,
        result: InvokeCallResultKind,
        actuals: &[FinalizedBirthActualsV1],
    ) -> Result<Vec<FinalizedObjectReturnOutcomeV1>, String> {
        if !matches!(result, InvokeCallResultKind::Handle | InvokeCallResultKind::NullableHandle) {
            return Err(freeze("owned-return/result"));
        }
        let owner = callables.owner_for_canonical_key(key)
            .ok_or_else(|| freeze("owned-return/owner"))?;
        let contract = callables.completed_result(
            &crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key.clone()))
            .ok_or_else(|| freeze("owned-return/completed-result"))?;
        let completion = self.ledger.completion_for_owner(owner)
            .ok_or_else(|| freeze("owned-return/completion"))?;
        let terminals = self.ledger.terminal_relation_index.get(&owner)
            .ok_or_else(|| freeze("owned-return/terminals"))?;
        if contract.owner() != owner || !std::ptr::eq(contract.completion(), completion)
            || !std::ptr::eq(contract.terminal_relations(), terminals.as_ref())
        { return Err(freeze("owned-return/completed-identity")); }
        // This existing descriptor checks every source alternative, original
        // witness, Completion exit, terminal and retained Normal handoff.
        let descriptor = self.ledger.checked_callable_object_result_v1(key, owner)?
            .ok_or_else(|| freeze("owned-return/original-result"))?;
        if descriptor.owner != owner || descriptor.kind != result {
            return Err(freeze("owned-return/result-agreement"));
        }
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            let function = module.functions.get(symbol)
                .filter(|function| function.signature.name == symbol && symbol == key.mir_symbol_projection())
                .ok_or_else(|| freeze("owned-return/function"))?;
            let exits = self.ledger.root_exits.borrow();
            let mut outcomes = Vec::new();
            for relation in &descriptor.terminals {
                let TerminalRelationV1::Value(terminal) = relation else {
                    return Err(freeze("owned-return/value-terminal"));
                };
                let exit = terminal.return_site();
                let Some(RootHomeExitProgress::Emitted { order, entry, bindings, .. }) =
                    exits.get(&(owner, exit.clone().into())) else {
                        return Err(freeze("owned-return/original-exit"));
                    };
                super::super::super::finalized_root_cleanup::validate_finished_cleanup_entry(
                    &self.ledger, owner, exit, function, projection, order, entry, bindings, Some(projection),
                )?;
                let mut returns = bindings.iter().filter(|(_, row)| matches!(row, MirInstruction::Return { .. }));
                let original = returns.next().ok_or_else(|| freeze("owned-return/return-missing"))?;
                if returns.next().is_some() { return Err(freeze("owned-return/return-duplicate")); }
                let returned = checked_projection(function, projection, original)?;
                let mut field_supports = Vec::new();
                let origin = match terminal.returned() {
                    TerminalReturnedSourceV1::NullLiteral if result == InvokeCallResultKind::NullableHandle => {
                        let producer = checked_projection(function, projection,
                            order.null_return_binding().ok_or_else(|| freeze("owned-return/null-source"))?)?;
                        if !matches!(producer.1, MirInstruction::Const { value: crate::mir::ConstValue::Null, .. }) {
                            return Err(freeze("owned-return/null-producer"));
                        }
                        FinalizedObjectReturnOriginV1::Null { producer }
                    }
                    TerminalReturnedSourceV1::Construction(site) => {
                        let [producer, allocation] = self.ledger.validate_fresh_return_producer_v1(
                            owner, exit, entry, bindings, Some(projection), Some(projection), Some(function),
                        )?.ok_or_else(|| freeze("owned-return/fresh-producer"))?;
                        checked_projection_membership(function, projection, &producer)?;
                        checked_projection_membership(function, projection, &allocation)?;
                        field_supports = self.owned_fresh_field_supports_v1(site, function, actuals)?;
                        FinalizedObjectReturnOriginV1::Fresh { producer, allocation }
                    }
                    TerminalReturnedSourceV1::OwnedCall(source) => {
                        let (packet, invoke, producer) = match source.acquisition() {
                            ObjectReturnAcquisitionV1::Direct { .. } => match entry {
                                RootHomeExitEntry::Call {
                                    row: crate::mir::normal_callable_semantic_package::RootCallDispositionV1::Lexical(packet),
                                    invoke, projection: producer, ..
                                } => (packet.as_ref(), invoke, producer),
                                _ => return Err(freeze("owned-return/direct-packet")),
                            },
                            ObjectReturnAcquisitionV1::Received(received) => {
                                let packet = original_local_packet(entry, received.site())?;
                                let (invoke, producer) = packet.outer_bindings()?;
                                (packet, invoke, producer)
                            }
                        };
                        let invoke = checked_projection(function, projection, invoke)?;
                        let producer = checked_projection(function, projection, producer)?;
                        self.ledger.visit_return_field_support_substitutions_v1(owner, exit, |callee, caller| {
                            let (entry_value, copies) = packet.field_support_projection_v1(
                                caller, callee.ordinal, CallValidationContextV1::Finalized(self))?;
                            let copies = copies.iter().map(|copy| projection.relation_binding(function, copy)?
                                .ok_or_else(|| freeze("owned-return/support-copy")))
                                .collect::<Result<Vec<_>, String>>()?;
                            FinalizedObjectReturnFieldSupportV1::push_group(&mut field_supports, FinalizedObjectReturnFieldSupportV1::new(
                                vec![caller.clone()].into_boxed_slice(), entry_value, copies, [invoke.clone(), producer.clone()],
                                callee.ordinal, None));
                            Ok(())
                        })?;
                        if !matches!(invoke.1, MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call {
                            result: InvokeCallResultKind::Handle | InvokeCallResultKind::NullableHandle, .. }, .. }) {
                            return Err(freeze("owned-return/call-result"));
                        }
                        FinalizedObjectReturnOriginV1::OwnedCall { producer, invoke }
                    }
                    _ => return Err(freeze("owned-return/source-kind")),
                };
                let producer = match &origin {
                    FinalizedObjectReturnOriginV1::Null { producer }
                    | FinalizedObjectReturnOriginV1::Fresh { producer, .. }
                    | FinalizedObjectReturnOriginV1::OwnedCall { producer, .. } => producer,
                    _ => return Err(freeze("owned-return/origin")),
                };
                let expected = producer.1.dst_value().ok_or_else(|| freeze("owned-return/producer-value"))?;
                let transport = original_copy_transport(function, projection, original, expected)?;
                let outcome = FinalizedObjectReturnOutcomeV1 {
                    result, returned, origin, transport, field_supports: field_supports.into_boxed_slice(),
                };
                outcome.validate_physical_function_v1(function, result,
                    key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod)?;
                outcomes.push(outcome);
            }
            outcomes.sort_by_key(|row| row.returned.0);
            let actual: BTreeSet<_> = function.blocks.iter().filter_map(|(id, block)|
                matches!(block.terminator, Some(MirInstruction::Return { .. })).then_some(*id)).collect();
            if outcomes.len() != actual.len() || actual != outcomes.iter().map(|row| row.returned.0).collect() {
                return Err(freeze("owned-return/return-census"));
            }
            Ok(outcomes)
        })
    }

    fn owned_fresh_field_supports_v1(
        &self, site: &OwnedExprSiteV1, function: &MirFunction, actuals: &[FinalizedBirthActualsV1],
    ) -> Result<Vec<FinalizedObjectReturnFieldSupportV1>, String> {
        let required = self.ledger.returned_new_field_supports_v1(site)?;
        if required.is_empty() { return Ok(Vec::new()); }
        let mut matching = actuals.iter().filter(|actual| actual.site() == site && actual.destination().is_none());
        let actual = matching.next().ok_or_else(|| freeze("owned-return/field-actuals"))?;
        if matching.next().is_some() { return Err(freeze("owned-return/field-actuals-duplicate")); }
        let mut projected = Vec::new();
        for argument in actual.arguments() {
            if !required.iter().any(|support| argument.source().child_relations().iter()
                .any(|relation| std::rc::Rc::ptr_eq(relation, &support.relation))) { continue; }
            self.with_received_birth_actual_result_v1(actual, argument.source().ordinal(), function,
                |_, supports| { projected.extend_from_slice(supports); Ok(()) })?
                .ok_or_else(|| freeze("owned-return/field-support-unconnected"))?;
        }
        if projected.iter().flat_map(|support| support.sources()).ne(required.iter()) {
            return Err(freeze("owned-return/field-support-census"));
        }
        Ok(projected)
    }
}
