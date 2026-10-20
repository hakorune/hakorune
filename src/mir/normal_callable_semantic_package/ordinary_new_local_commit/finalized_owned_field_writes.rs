//! Exact LOCAL write/New/ready-read inventory for both independent final scans.
use super::*;
use crate::mir::instruction::InvokeOperation;

pub(in crate::mir) struct OwnedFieldWriteProjectionV1 {
    pub(in crate::mir) invoke: LocatedBinding,
    pub(in crate::mir) receiver: QualifiedFieldWriteReceiverV1,
    pub(in crate::mir) bindings: Vec<LocatedBinding>,
    pub(in crate::mir) before_store: Vec<(BasicBlockId, usize)>,
    pub(in crate::mir) ready_reads: Vec<(BasicBlockId, usize)>,
    pub(in crate::mir) handle_values: BTreeSet<ValueId>,
    pub(in crate::mir) read_dependencies: Vec<((BasicBlockId, usize), (BasicBlockId, usize))>,
}

impl FinalizedRootSourceHandoffV1 {
    /// Select owners from original source frames, including AppMain with no
    /// incoming formal edge. Missing packets are not an empty owner inventory.
    pub(in crate::mir) fn with_owned_field_write_functions_v1(
        &self, module: &crate::mir::MirModule,
        mut visit: impl FnMut(FunctionOwnerIdV1, &MirFunction) -> Result<(), String>,
    ) -> Result<(), String> {
        let owners: BTreeSet<_> = self.ledger.completion_index.values().chain(self.ledger.root_completion.iter())
            .filter_map(|row| row.as_ref().ok())
            .filter(|completion| completion.cleanup().root_flow().is_some_and(|flow|
                flow.expression_plans().iter().filter_map(|plan| plan.field_write_frame()).any(|frame| !frame.is_borrowed())))
            .map(|completion| completion.owner()).collect();
        for owner in owners {
            self.ledger.with_finished_projection(owner, |symbol, _| {
                visit(owner, module.functions.get(symbol).ok_or_else(|| freeze("owned-write/function-missing"))?)
            })?;
        }
        Ok(())
    }

    pub(in crate::mir) fn with_finished_owned_field_writes_v1(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction,
        mut visit: impl FnMut(OwnedFieldWriteProjectionV1) -> Result<(), String>,
    ) -> Result<(), String> {
        let context = CallValidationContextV1::Finalized(self);
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            if function.signature.name != symbol { return Err(freeze("owned-write/function")); }
            let completion = self.ledger.completion_source_for_owner_v1(owner)
                .ok_or_else(|| freeze("owned-write/completion"))?;
            let flow = completion.cleanup().root_flow().ok_or_else(|| freeze("owned-write/source-flow"))?;
            self.ledger.validate_owned_ready_reads_v1(owner, function, context, |row| projection.binding(row.0, &row.1))?;
            self.ledger.validate_checked_field_census_v1(owner, function, context, |row| projection.binding(row.0, &row.1))?;
            for frame in flow.expression_plans().iter().filter_map(|plan| plan.field_write_frame()).filter(|frame| !frame.is_borrowed()) {
                context.with_original_field_write_v1(frame, |packet| {
                    packet.validate_paths(context, function, Some(projection))?;
                    let Some(local) = frame.owned_local() else {
                        let entry = frame.entry().ok_or_else(|| freeze("owned-write/local-origin"))?;
                        let (parameter, value) = entry.checked_entry_value_v1(&self.ledger, function)?;
                        let operands = packet.qualified_operand_bindings_v1(entry.receiver(), value)?;
                        let mut bindings = Vec::new();
                        let mut before_store = Vec::new();
                        for original in packet.bindings().iter().chain(operands.iter()) {
                            let mapped = if original.1.dst_value().is_some() {
                                projection.relation_binding(function, original)?
                            } else if matches!(original.1, MirInstruction::Jump { edge_args: None, .. }) {
                                projection.structural_jump_binding(function, original, true)?
                            } else { Some(projection.required_effect_binding(function, original)?) };
                            let Some(mapped) = mapped else { continue };
                            let position = find_finished_producer(symbol, &mapped, function)?;
                            if !bindings.iter().any(|(binding, _)| binding == &mapped) { bindings.push((mapped, position)); }
                        }
                        let rhs = packet.qualified_rhs_value_v1().ok_or_else(|| freeze("owned-write/producer-removed"))?;
                        let mut producers = bindings.iter().filter(|(row, _)| row.1.dst_value() == Some(rhs));
                        match (producers.next(), producers.next()) {
                            (Some((_, coordinate)), None) => before_store.push(*coordinate),
                            (None, None) if function.params.contains(&rhs) => {},
                            _ => return Err(freeze("owned-write/producer-removed")),
                        }
                        for pin in packet.pins() {
                            let mut originals = Vec::new();
                            pin.append_observed_bindings_v1(&mut originals);
                            for original in originals {
                                if let Some(mapped) = projection.relation_binding(function, &original)? {
                                    before_store.push(find_finished_producer(symbol, &mapped, function)?);
                                }
                            }
                        }
                        let original = packet.invoke_binding();
                        let invoke = projection.required_effect_binding(function, original)?;
                        let coordinate = find_finished_producer(symbol, &invoke, function)?;
                        return visit(OwnedFieldWriteProjectionV1 { invoke: (invoke, coordinate),
                            receiver: QualifiedFieldWriteReceiverV1::Entry { parameter, value },
                            bindings, before_store, ready_reads: Vec::new(), handle_values: BTreeSet::new(), read_dependencies: Vec::new() });
                    };
                    let commits = self.ledger.local_commits.borrow();
                    let row = commits.get(local.acquisition()).ok_or_else(|| freeze("owned-write/new-missing"))?;
                    let LocalCommitV1::Ordinary(new) = row else { return Err(freeze("owned-write/new-origin")); };
                    if new.binding != local.root() || new.object != local.field().object()
                        || new.allocation_execution.as_ref().is_none_or(|lease| !std::rc::Rc::ptr_eq(lease.source(), local.construction()))
                    { return Err(freeze("owned-write/new-source")); }
                    self.ledger.validate_ordinary_new_row_emission_projected(owner, local.acquisition(), row, function, Some(projection))?;
                    let NewEmissionProgress::Emitted { result, bindings: new_bindings,
                        progress: EmittedLocalProgress::Checked { local: installed }, .. } = &new.emission
                        else { return Err(freeze("owned-write/new-unconsumed")); };
                    let mut rows = Vec::new();
                    let mut before_store = Vec::new();
                    let mut add = |original: &(BasicBlockId, MirInstruction), mandatory: bool| -> Result<Option<LocatedBinding>, String> {
                        let mapped = if original.1.dst_value().is_some() { projection.relation_binding(function, original)? }
                            else if mandatory { Some(projection.required_effect_binding(function, original)
                                .map_err(|error| format!("{error} owned-write-original={original:?}"))?) }
                            else { projection.binding(original.0, &original.1)? };
                        let Some(mapped) = mapped else {
                            if mandatory { return Err(freeze("owned-write/producer-removed")); }
                            return Ok(None);
                        };
                        let position = find_finished_producer(symbol, &mapped, function)?;
                        let row = (mapped, position);
                        if !rows.contains(&row) { rows.push(row.clone()); }
                        Ok(Some(row))
                    };
                    let mut new_invoke = None;
                    for original in new_bindings {
                        let projected = add(original, original.1.dst_value().is_none() && !matches!(original.1, MirInstruction::Jump { .. }))?;
                        if matches!(original.1, MirInstruction::Invoke { operation: InvokeOperation::NewBox { object, .. }, .. } if object == new.object) {
                            if new_invoke.replace(projected.ok_or_else(|| freeze("owned-write/new-removed"))?).is_some() {
                                return Err(freeze("owned-write/new-duplicate"));
                            }
                        } else if matches!(original.1, MirInstruction::InvokeNormalResult { dst, .. } if dst == *result) {
                            before_store.push(projected.ok_or_else(|| freeze("owned-write/new-result-removed"))?.1);
                        }
                    }
                    if let Some(copy) = projection.source_local_copy_binding_v1(function, *installed, *result)? {
                        if let Some(row) = add(&copy, false)? { before_store.push(row.1); }
                    }
                    for original in packet.bindings() {
                        add(original, original.1.dst_value().is_none() && !matches!(original.1, MirInstruction::Jump { .. }))?;
                    }
                    let invoke = add(packet.invoke_binding(), true)?.ok_or_else(|| freeze("owned-write/store-removed"))?;
                    // Pin completion is the existing exact receiver/value provenance.
                    for original in packet.qualified_operand_bindings_v1(local.root(), *installed)? {
                        if let Some(row) = add(&original, false)? { before_store.push(row.1); }
                    }
                    let mut ready_reads = Vec::new();
                    let mut read_dependencies = Vec::new();
                    let mut handle_values = packet.owned_handle_values_v1(local.root(), *installed, *result)?;
                    for (original, is_read, target) in self.ledger.owned_ready_bindings_for_frame_v1(frame, *installed, context)? {
                        let Some(row) = add(&original, target.is_none())? else { continue };
                        if is_read { ready_reads.push(row.1); }
                        if let Some(target) = target {
                            if let MirInstruction::Copy { dst, src } = row.0.1 { handle_values.extend([dst, src]); }
                            let target = add(&target, true)?.ok_or_else(|| freeze("owned-write/read-removed"))?;
                            read_dependencies.push((row.1, target.1));
                        }
                    }
                    // The existing boundary's recorded inventory comes ONLY
                    // from source lifecycle_bindings, not its full MIR capture.
                    // Other acquisitions may legitimately unwind this Home.
                    // These rows are already projected; never project twice.
                    for mapped in projection.recorded() {
                        if !mapped.1.used_values().iter().any(|value| handle_values.contains(value)) { continue; }
                        let row = (mapped.clone(), find_finished_producer(symbol, mapped, function)?);
                        if !rows.contains(&row) { rows.push(row); }
                    }
                    visit(OwnedFieldWriteProjectionV1 { invoke, receiver: QualifiedFieldWriteReceiverV1::LocalNew { invoke: new_invoke.ok_or_else(|| freeze("owned-write/new-invoke-missing"))? },
                        bindings: rows, before_store, ready_reads, handle_values, read_dependencies })
                })?;
            }
            Ok(())
        })
    }
}
