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
                    let requirement = &array.requirement;
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
                } else if let LocalFieldReadResultV1::ArrayField(array) = source.result() {
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
        let owners: std::collections::BTreeSet<_> = self.ledger.completion_index.values()
            .chain(self.ledger.root_completion.iter()).filter_map(|row| row.as_ref().ok())
            .map(|row| row.owner()).filter(|owner| self.ledger.has_checked_field_reads_v1(*owner)).collect();
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
                matches!(row, MirInstruction::Invoke { operation: InvokeOperation::FieldGet { .. }, .. })) {
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
