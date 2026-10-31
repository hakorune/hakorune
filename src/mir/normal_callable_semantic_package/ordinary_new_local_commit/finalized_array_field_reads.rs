//! Original field-resident Array reads through the sole finished-binding owner.
use super::*;
use crate::mir::normal_callable_semantic_package::EmittedNamedArrayRequirementV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;
use hakorune_mir_defs::CanonicalFieldRefV1;

impl FinalizedRootSourceHandoffV1 {
    /// Lend the same source-issued reads to final module verification. Owners
    /// come from completed source rows, never from an ObjectFieldGet census.
    pub(in crate::mir) fn visit_finished_array_field_reads_v1(
        &self, module: &crate::mir::MirModule,
        arrays: &[EmittedNamedArrayRequirementV1],
        mut visit: impl FnMut(&MirFunction, Binding, CanonicalFieldRefV1) -> Result<(), String>,
    ) -> Result<(), String> {
        let owners: std::collections::BTreeSet<_> = self.ledger.completion_index.values()
            .chain(self.ledger.root_completion.iter()).filter_map(|row| row.as_ref().ok())
            .map(|row| row.owner()).collect();
        for owner in owners {
            // No finished function is demanded for a source-only declaration
            // with no retained Array read.
            let mut selected = false;
            self.ledger.visit_array_field_read_originals_v1(owner, |_, _, _| {
                selected = true;
                Ok(())
            })?;
            if !selected { continue; }
            self.ledger.with_finished_projection(owner, |symbol, _| {
                let function = module.functions.get(symbol)
                    .ok_or_else(|| freeze("finished-array-field/function-membership"))?;
                for (binding, field) in self.finished_array_field_reads_v1(owner, module, function, arrays)? {
                    visit(function, binding, field)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    }

    /// The Count and Release of a `keys()` snapshot, as finished bindings in `function`.
    pub(in crate::mir) fn finished_keys_count_reads_v1(
        &self, module: &crate::mir::MirModule, function: &MirFunction,
    ) -> Result<Vec<Binding>, String> {
        let owners: Vec<_> = self.ledger.map_read_bindings.borrow().keys().copied().collect();
        let mut result = Vec::new();
        for owner in owners {
            self.ledger.with_finished_projection(owner, |symbol, projection| {
                if module.functions.get(symbol).is_none_or(|current| !std::ptr::eq(current, function)) {
                    return Ok(());
                }
                for (_, rows) in self.ledger.map_read_bindings.borrow().get(&owner).into_iter().flatten() {
                    if !rows.iter().any(|(_, row)| matches!(row, MirInstruction::Invoke {
                        operation: crate::mir::instruction::InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::FieldKeys { .. }), .. })) {
                        continue;
                    }
                    // The Count and the End of the acquired snapshot it counts.
                    for (block, row) in rows.iter().filter(|(_, row)| matches!(row, MirInstruction::ArrayRead {
                        operation: crate::mir::instruction::ArrayReadOperationV1::Count, .. }
                        | MirInstruction::ArrayResidenceRelease { .. })) {
                        let finished = projection.binding(*block, row)?
                            .ok_or_else(|| freeze("keys-count/read-removed"))?;
                        if !projection.recorded().contains(&finished) { return Err(freeze("keys-count/read-unrecorded")); }
                        result.push(finished);
                    }
                }
                Ok(())
            })?;
        }
        Ok(result)
    }

    /// The caller lends already-validated named-array emissions from this view.
    /// A passive declaration or an unrelated tagged formal grants no read.
    pub(in crate::mir) fn finished_array_field_reads_v1(
        &self, owner: FunctionOwnerIdV1, module: &crate::mir::MirModule,
        function: &MirFunction, arrays: &[EmittedNamedArrayRequirementV1],
    ) -> Result<Vec<(Binding, CanonicalFieldRefV1)>, String> {
        let mut originals = Vec::new();
        self.ledger.visit_array_field_read_originals_v1(owner, |site, source, original| {
            originals.push((site.clone(), Rc::clone(source), original.clone()));
            Ok(())
        })?;
        if originals.is_empty() { return Ok(Vec::new()); }
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            if module.functions.get(symbol).is_none_or(|current| !std::ptr::eq(current, function)) {
                return Err(freeze("finished-array-field/function-membership"));
            }
            let mut result = Vec::new();
            for (site, source, original) in &originals {
                let mut provider_key = None;
                if let LocalFieldReadResultV1::BorrowedArray(array) = source.result() {
                    if let Some(requirement) = array.integer_requirement_v1() {
                        if requirement.owner() != owner || requirement.field_site() != site.site()
                            || requirement.field() != source.field()
                        { return Err(freeze("finished-array-field/source-membership")); }
                        let mut emissions = arrays.iter().filter(|row| matches!(row,
                            EmittedNamedArrayRequirementV1::Write(write)
                                if write.borrows_field_residence_v1(requirement)));
                        let emitted = emissions.next().ok_or_else(|| freeze("finished-array-field/emission-missing"))?;
                        if emissions.next().is_some() || emitted.owner() != owner || emitted.validate(module)? != symbol {
                            return Err(freeze("finished-array-field/emission-membership"));
                        }
                        provider_key = Some(requirement.provider_caller());
                    } else {
                        let (field, _) = array.fixed_source_v1().ok_or_else(|| freeze("finished-array-field/source-membership"))?;
                        if field.site() != site || field.field() != source.field() || source.array_operation_sources_v1().is_empty() {
                            return Err(freeze("finished-array-field/source-membership"));
                        }
                        for operation in source.array_operation_sources_v1() {
                            operation.recheck_census(owner, &self.ledger.array_i64_fields)?;
                            if operation.receiver_alias_v1() != Some(array)
                                || operation.field() != source.field()
                                || provider_key.is_some_and(|key| key != operation.provider()) {
                                return Err(freeze("finished-array-field/source-membership"));
                            }
                            self.validate_array_operation_field_v1(owner, module, function, arrays, projection, operation, original)?;
                            provider_key = Some(operation.provider());
                        }
                    }
                } else if let LocalFieldReadResultV1::ArrayField(array) = source.result() {
                    for operation in source.array_operation_sources_v1() {
                        operation.recheck_census(owner, &self.ledger.array_i64_fields)?;
                        if !operation.borrows_direct_receiver_v1(array) || operation.receiver() != site {
                            return Err(freeze("finished-array-field/source-membership"));
                        }
                        self.validate_array_operation_field_v1(owner, module, function, arrays, projection, operation, original)?;
                        if provider_key.is_some_and(|key| key != operation.provider()) {
                            return Err(freeze("finished-array-field/source-membership"));
                        }
                        provider_key = Some(operation.provider());
                    }
                    if array.site() != site || array.field() != source.field() || !module.canonical_array_field(source.field()) {
                        return Err(freeze("finished-array-field/source-membership"));
                    }
                    if let Some(frame) = array.text_append_origin_v1() {
                        let loan = self.ledger.field_array_text_append_source_v1(frame.source().call())?;
                        if !loan.matches_text_frame(frame) || frame.source().receiver() != site
                            || frame.source().field() != source.field() {
                            return Err(freeze("finished-array-field/source-membership"));
                        }
                        // Borrow source/provider membership without recursively
                        // validating the append packet that consumes this read.
                        provider_key = Some(frame.source().provider());
                    }
                } else { unreachable!("original visitor selects only source Array reads"); }
                if let Some(provider_key) = provider_key {
                    let mut providers = self.construction.iter().filter(|(key, _)| key == provider_key);
                    let (_, provider) = providers.next().ok_or_else(|| freeze("finished-array-field/provider-missing"))?;
                    if providers.next().is_some() { return Err(freeze("finished-array-field/provider-duplicate")); }
                    let provider_symbol = module.canonical_callable_definition_symbol(provider_key)
                        .ok_or_else(|| freeze("finished-array-field/provider-definition"))?;
                    provider.validate_artifact_after_compiler_finishing(module.functions.get(provider_symbol)
                        .ok_or_else(|| freeze("finished-array-field/provider-body"))?)?;
                }
                let finished = projection.relation_binding(function, original)?
                    .ok_or_else(|| freeze("finished-array-field/read-removed"))?;
                if !projection.recorded().contains(&finished)
                    || result.iter().any(|(binding, _)| binding == &finished)
                { return Err(freeze("finished-array-field/read-membership")); }
                result.push((finished, source.field()));
            }
            Ok(result)
        })
    }
    /// Each operation must be lent by one existing fully validated purpose.
    fn validate_array_operation_field_v1(
        &self, owner: FunctionOwnerIdV1, module: &crate::mir::MirModule, function: &MirFunction,
        arrays: &[EmittedNamedArrayRequirementV1], projection: &super::super::physical_boundary::FinishedBindings,
        operation: &Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>, original: &Binding,
    ) -> Result<(), String> {
        let check = || -> Result<(), String> {
            let context = CallValidationContextV1::Finalized(self);
            let mut selected = None;
            let mut observe = |pair: (Binding, Binding)| -> Result<(), String> {
                if selected.is_some() || pair.0 != *original {
                    return Err(freeze("finished-array-field/source-membership"));
                }
                selected = Some(pair.1);
                Ok(())
            };
            {
                let exits = self.ledger.root_exits.borrow();
                for ((exit_owner, key), progress) in exits.iter().filter(|((exit_owner, _), _)| *exit_owner == owner) {
                    let order = match progress {
                        RootHomeExitProgress::Emitted { order, .. } | RootHomeExitProgress::Finalized(order) => order,
                        _ => continue,
                    };
                    let Some(expression) = order.return_expression_v1() else { continue; };
                    expression.require_source_v1(*exit_owner, key.explicit_site()
                        .ok_or_else(|| freeze("finished-array-field/source-membership"))?)?;
                    if let Some(pair) = expression.array_operation_bindings_v1(operation, context, function, projection)? {
                        observe(pair)?;
                    }
                }
            }
            context.with_local_call_group_tree_v2(owner, |groups| {
                for group in groups {
                    if let Some(pair) = group.array_operation_bindings_v1(operation, context, function, projection)? {
                        observe(pair)?;
                    }
                    for call in group.lexical_packets() {
                        if let Some(pair) = call.array_argument_operation_bindings_v1(operation, context)? {
                            group.validate_recorded_in_context(context, function, Some(projection))?;
                            observe(pair)?;
                        }
                    }
                }
                Ok(())
            })?;
            let binding = selected.ok_or_else(|| format!("{} consumer unavailable", freeze("finished-array-field/source-membership")))?;
            if operation.is_count() {
                let mut emissions = arrays.iter().filter_map(|row| match row {
                    EmittedNamedArrayRequirementV1::Read(read) if read.recipe().source().as_ref() == operation.as_ref() => Some(read),
                    _ => None,
                });
                let emitted = emissions.next().ok_or_else(|| freeze("finished-array-field/emission-missing"))?;
                let finished = projection.relation_binding(function, &binding)?
                    .ok_or_else(|| freeze("finished-array-field/read-removed"))?;
                if emissions.next().is_some() || emitted.count_binding_v1(module, function)?.as_ref() != Some(&finished) {
                    return Err(freeze("finished-array-field/emission-membership"));
                }
            } else if operation.is_dynamic_get() {
                projection.required_effect_binding(function, &binding)?;
            } else { return Err(freeze("finished-array-field/source-membership")); }
            Ok(())
        };
        check().map_err(|issue| format!("{issue} operation={:?} field={:?}", operation.call(), operation.field()))
    }
}

impl FinalizedRootSourceHandoffV1 {
    /// Borrow original field-Push packets through the same finalized group and
    /// field residence emissions. Neither opcode shape nor a symbol selects it.
    pub(in crate::mir) fn finished_field_array_appends_v1(
        &self, module: &crate::mir::MirModule, function: &MirFunction,
        arrays: &[EmittedNamedArrayRequirementV1],
    ) -> Result<Vec<Binding>, String> {
        if !module.functions.get(&function.signature.name).is_some_and(|stored| std::ptr::eq(stored, function)) {
            return Err(freeze("finished-field-append/function-membership"));
        }
        let mut result = Vec::new();
        for (owner, groups) in &self.local_calls {
            if !groups.iter().any(|group| group.field_array_append().is_some()) { continue; }
            self.ledger.with_finished_projection(*owner, |symbol, projection| {
                if symbol != function.signature.name { return Ok(()); }
                for group in groups {
                    let Some(packet) = group.field_array_append() else { continue };
                    group.require_sealed_condition_v1()?;
                    if packet.source().site().owner() != *owner || group.site() != packet.source().site() {
                        return Err(freeze("finished-field-append/source-owner"));
                    }
                    group.validate_recorded_in_context(CallValidationContextV1::Finalized(self), function, Some(projection))?;
                    let mut emissions = arrays.iter().filter(|row| matches!(row,
                        EmittedNamedArrayRequirementV1::Write(write)
                            if write.borrows_append_source_v1(packet.source())
                                && write.borrows_checked_field_append_v1(packet)));
                    let emitted = emissions.next().ok_or_else(|| freeze("finished-field-append/residence-missing"))?;
                    if emissions.next().is_some() || emitted.owner() != *owner || emitted.validate(module)? != symbol {
                        return Err(freeze("finished-field-append/residence-membership"));
                    }
                    // Invoke has no dst: use its canonical block/landing
                    // projection, then require exact retained census and one
                    // final coordinate. Scalar relations use relation_binding.
                    let original = packet.invoke_binding();
                    let finished = projection.binding(original.0, &original.1)?
                        .ok_or_else(|| freeze("finished-field-append/invoke-removed"))?;
                    let count = function.blocks.iter().flat_map(|(block, body)|
                        body.all_instructions().map(move |row| (*block, row)))
                        .filter(|(block, row)| *block == finished.0 && **row == finished.1).count();
                    if count != 1 || projection.recorded().iter().filter(|row| **row == finished).count() != 1
                        || result.contains(&finished)
                    { return Err(freeze("finished-field-append/invoke-membership")); }
                    result.push(finished);
                }
                Ok(())
            })?;
        }
        Ok(result)
    }
}

impl FinalizedRootSourceHandoffV1 {
    /// Enumerate the original checked-read owners, not opcode-shaped functions.
    pub(in crate::mir) fn with_checked_field_read_functions_v1(
        &self, module: &crate::mir::MirModule,
        mut visit: impl FnMut(FunctionOwnerIdV1, &MirFunction) -> Result<(), String>,
    ) -> Result<(), String> {
        let mut owners: std::collections::BTreeSet<_> = self.ledger.completion_index.values()
            .chain(self.ledger.root_completion.iter()).filter_map(|row| row.as_ref().ok())
            .map(|row| row.owner()).filter(|owner| self.ledger.has_checked_field_reads_v1(*owner)).collect();
        owners.extend(self.ledger.checked_field_read_owners_v1());
        for owner in owners {
            self.ledger.with_finished_projection(owner, |symbol, _| {
                visit(owner, module.functions.get(symbol).ok_or_else(|| freeze("finished-received-field/function-membership"))?)
            })?;
        }
        Ok(())
    }

    /// The same field rows now borrow their Call packets from the finalized
    /// owner. Both read Invoke and Normal must survive the sole projection.
    pub(in crate::mir) fn finished_received_field_reads_v1(
        &self, owner: FunctionOwnerIdV1, module: &crate::mir::MirModule, function: &MirFunction,
    ) -> Result<Vec<Binding>, String> {
        use crate::mir::instruction::InvokeOperation;
        if !self.ledger.has_checked_field_reads_v1(owner) {
            if function.blocks.values().flat_map(|body| body.all_instructions()).any(|row|
                matches!(row, MirInstruction::Invoke { operation: InvokeOperation::FieldGet { .. } | InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::FieldMaterialize { .. }) | InvokeOperation::Shared(crate::mir::instruction::SharedInvokeOperation::Borrow { .. }), .. })) {
                return Err(freeze("finished-received-field/source-missing"));
            }
            return Ok(Vec::new());
        }
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            if module.functions.get(symbol).is_none_or(|current| !std::ptr::eq(current, function)) {
                return Err(freeze("finished-received-field/function-membership"));
            }
            for group in self.local_calls.get(&owner).into_iter().flatten() {
                group.validate_recorded_in_context(CallValidationContextV1::Finalized(self), function, Some(projection))?;
            }
            self.ledger.validate_checked_field_census_v1(owner, function, CallValidationContextV1::Finalized(self), |original| {
                let mapped = projection.binding(original.0, &original.1)?
                    .ok_or_else(|| freeze("finished-received-field/binding-removed"))?;
                if !projection.recorded().contains(&mapped) { return Err(freeze("finished-received-field/binding-unrecorded")); }
                Ok(Some(mapped))
            })
        })
    }
}
