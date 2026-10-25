//! Existing retained call packets, lent without changing their source authority.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    CallValidationContextV1, PreparedBorrowedFormalActualV1, PreparedLoopStaticBodyDetachedPacketV1,
};
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
use crate::mir::MirModule;
use std::collections::BTreeSet;

/// Transport view only: a detached LoopBody is never a Home lexical packet.
#[derive(Clone, Copy)]
pub(in crate::mir) enum FinalizedBorrowedCallPacketV1<'source> {
    Lexical(&'source EmittedLexicalCallProjectionV1),
    LoopBody(&'source PreparedLoopStaticBodyDetachedPacketV1),
    ProviderArgument(&'source crate::mir::normal_callable_semantic_package::PreparedProviderStaticArgumentV1),
}

impl<'source> FinalizedBorrowedCallPacketV1<'source> {
    pub(in crate::mir) fn call_site(self) -> &'source OwnedExprSiteV1 {
        match self {
            Self::Lexical(packet) => packet.call_site(),
            Self::LoopBody(packet) => packet.source_site(),
            Self::ProviderArgument(packet) => packet.incoming().call_site(),
        }
    }

    pub(in crate::mir) fn target(self) -> &'source CanonicalSameModuleCallableKeyV1 {
        match self {
            Self::Lexical(packet) => packet.original_source().target(),
            Self::LoopBody(packet) => packet.original_source().target(),
            Self::ProviderArgument(packet) => packet.incoming().target(),
        }
    }

    pub(in crate::mir) fn callee_owner(self) -> FunctionOwnerIdV1 {
        match self {
            Self::Lexical(packet) => packet.original_source().callee_owner(),
            Self::LoopBody(packet) => packet.original_source().callee_owner(),
            Self::ProviderArgument(packet) => packet.incoming().callee_owner(),
        }
    }

    /// Result authority remains with the original checked call packet.
    pub(in crate::mir) fn result(self) -> Option<crate::mir::instruction::InvokeCallResultKind> {
        match self {
            Self::Lexical(packet) => packet.original_source().result(),
            Self::LoopBody(packet) if packet.corroborates_retained_scalar() => {
                Some(crate::mir::instruction::InvokeCallResultKind::I64)
            }
            Self::LoopBody(_) => None,
            Self::ProviderArgument(_) => Some(crate::mir::instruction::InvokeCallResultKind::I64),
        }
    }

    pub(in crate::mir) fn borrowed_computed_argument_operand_v1(
        self, ordinal: u32, context: CallValidationContextV1<'_>,
    ) -> Result<Option<crate::mir::instruction::StringInvokeOperand>, String> {
        match self {
            Self::Lexical(packet) => packet.borrowed_computed_argument_operand_v1(ordinal, context),
            Self::LoopBody(_) | Self::ProviderArgument(_) => Ok(None),
        }
    }

    pub(in crate::mir) fn selects_borrowed_source(self, arguments: &[LocalCallArgumentV1]) -> bool {
        match self {
            Self::Lexical(_) => arguments
                .iter()
                .any(|argument| matches!(argument, LocalCallArgumentV1::BorrowedActual { .. })),
            Self::LoopBody(_) | Self::ProviderArgument(_) => true,
        }
    }
}

impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn require_finished_map_birth_v1(
        &self, module: &MirModule, key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        owner: FunctionOwnerIdV1, store: &crate::mir::normal_callable_semantic_package::ConstructionStoreV1,
    ) -> Result<(), String> {
        let mut rows = self.construction.iter().filter(|(actual, _)| actual == key);
        let (_, validation) = rows.next().ok_or_else(|| freeze("final-provider/definition-missing"))?;
        if rows.next().is_some() { return Err(freeze("final-provider/definition-missing")); }
        let function = module.canonical_callable_definition_symbol(key).and_then(|symbol| module.functions.get(symbol))
            .ok_or_else(|| freeze("final-provider/function-missing"))?;
        validation.require_map_field_source_v1(owner, store, function)
    }

    /// Borrow mapped original presence/get producers, after the same group replay
    /// used for Calls. This is an ephemeral verifier image, not a source index.
    pub(in crate::mir) fn visit_finished_map_field_reads_v1(
        &self, module: &MirModule,
        mut visit: impl FnMut(&MirFunction, (BasicBlockId, MirInstruction)) -> Result<(), String>,
    ) -> Result<(), String> {
        for owner in self.local_calls.keys() {
            self.ledger.with_finished_projection(*owner, |symbol, projection| {
                let function = module.functions.get(symbol).ok_or_else(|| freeze("final-provider/function-missing"))?;
                self.ledger.visit_map_presence_groups_v1(*owner, CallValidationContextV1::Finalized(self), function,
                    projection, module, |row| visit(function, row))?;
                self.ledger.visit_stored_map_get_groups_v1(*owner, CallValidationContextV1::Finalized(self), function,
                    projection, module, |row| visit(function, row))
            })?;
        }
        Ok(())
    }

    /// Exact installed-field writes retain their own source and finished graph.
    pub(in crate::mir) fn visit_finished_map_field_installs_v1(
        &self, module: &MirModule,
        mut visit: impl FnMut(&MirFunction, (BasicBlockId, MirInstruction)) -> Result<(), String>,
    ) -> Result<(), String> {
        for owner in self.local_calls.keys() {
            self.ledger.with_finished_projection(*owner, |symbol, projection| {
                let function = module.functions.get(symbol).ok_or_else(|| freeze("final-provider/function-missing"))?;
                let context = CallValidationContextV1::Finalized(self);
                context.with_local_call_group_tree_v2(*owner, |groups| {
                    for group in groups {
                        let Some(packet) = group.map_field_install() else { continue; };
                        group.validate_recorded_in_context(context, function, Some(projection))?;
                        packet.frame().source().receiver_v1().require_finished_birth_v1(self, module)?;
                        let row = packet.invoke();
                        visit(function, projection.binding(row.0, &row.1)?.ok_or_else(|| freeze("map-get-binding-missing"))?)?;
                    }
                    Ok(())
                })
            })?;
        }
        Ok(())
    }

    /// Finished condition correspondence for one original entry owner. No new
    /// source classification or executable operand is issued by this visitor.
    pub(in crate::mir) fn with_finished_bool_bindings_v1(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction,
        mut visit: impl FnMut((BasicBlockId, usize), MirInstruction) -> Result<(), String>,
    ) -> Result<(), String> {
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            if symbol != function.signature.name { return Err(freeze("finished-local-call/function")); }
            self.ledger.visit_finished_bool_bindings_v1(owner, CallValidationContextV1::Finalized(self),
                function, projection, |binding| {
                    let coordinate = super::finalized_source_projection::find_finished_producer(symbol, &binding, function)?;
                    visit(coordinate, binding.1)
                })
        })
    }

    /// Exact finite-entry Local copies and SAME source-Loop edge transports.
    /// This lends SSA definitions only, not a mutable binding's formal origin.
    pub(in crate::mir) fn with_finished_scalar_bindings_v1(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction,
        mut visit: impl FnMut(&(BasicBlockId, MirInstruction), (BasicBlockId, usize)) -> Result<(), String>,
    ) -> Result<(), String> {
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            if symbol != function.signature.name { return Err(freeze("finished-copy/owner")); }
            let context = CallValidationContextV1::Finalized(self);
            context.with_local_call_group_tree_v2(owner, |groups| {
                for group in groups {
                    if let Some(packet) = group.source_loop_rc() {
                        group.validate_recorded_in_context(context, function, Some(projection))?;
                        for original in packet.incoming_copy_bindings_v2(function, projection)? {
                            if let Some(finished) = projection.relation_binding(function, &original)? {
                                let coordinate = super::finalized_source_projection::find_finished_producer(symbol, &finished, function)?;
                                visit(&finished, coordinate)?;
                            }
                        }
                    }
                    let Some(packet) = group.initializer().filter(|packet| packet.frame().is_i64()) else { continue; };
                    group.validate_recorded_in_context(context, function, Some(projection))?;
                    for original in packet.scalar_source_bindings_v1(context)? {
                        if let Some(finished) = projection.relation_binding(function, &original)? {
                            let coordinate = super::finalized_source_projection::find_finished_producer(symbol, &finished, function)?;
                            visit(&finished, coordinate)?;
                        }
                    }
                }
                Ok(())
            })
        })
    }

    /// Original tagged String operands remain tied to their operation and side.
    pub(in crate::mir) fn with_finished_tagged_text_operands_v1(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction,
        mut visit: impl FnMut(BindingRefV1, ValueId, usize, (BasicBlockId, usize)) -> Result<(), String>,
    ) -> Result<(), String> {
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            if symbol != function.signature.name { return Err(freeze("finished-copy/owner")); }
            let context = CallValidationContextV1::Finalized(self);
            context.with_local_call_group_tree_v2(owner, |groups| {
                for group in groups {
                    let Some(packet) = group.initializer() else { continue; };
                    group.validate_recorded_in_context(context, function, Some(projection))?;
                    packet.visit_tagged_text_operands_v1(context, function, projection, |original, side, formal, value| {
                        let finished = projection.required_effect_binding(function, original)?;
                        let coordinate = super::finalized_source_projection::find_finished_producer(symbol, &finished, function)?;
                        visit(formal, value, side, coordinate)
                    })?;
                }
                Ok(())
            })
        })
    }

    pub(in crate::mir) fn visit_finished_construction_writes_v1(
        &self, module: &MirModule,
        mut visit: impl FnMut(&MirFunction, (BasicBlockId, MirInstruction)) -> Result<(), String>,
    ) -> Result<(), String> {
        for (key, validation) in &self.construction {
            let symbol = module.canonical_callable_definition_symbol(key)
                .ok_or_else(|| freeze("final-provider/definition-missing"))?;
            let function = module.functions.get(symbol)
                .ok_or_else(|| freeze("final-provider/function-missing"))?;
            validation.visit_finished_field_writes_v1(key, function, |row| visit(function, row))?;
        }
        Ok(())
    }

    /// Return loans tied to the retained pool, not the visitor's callback borrow.
    pub(in crate::mir) fn borrowed_finalized_call_actuals_v1(
        &self,
        packet: FinalizedBorrowedCallPacketV1<'_>,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        match packet {
            FinalizedBorrowedCallPacketV1::Lexical(packet) => {
                self.borrowed_packet_actuals_v1(packet)
            }
            FinalizedBorrowedCallPacketV1::ProviderArgument(packet) => {
                self.ledger.borrowed_provider_packet_actuals_v1(packet).map(Some)
            }
            FinalizedBorrowedCallPacketV1::LoopBody(packet) => {
                let retained = self
                    .loop_body_calls
                    .get(packet.source_site())
                    .ok_or_else(|| freeze("finished-loop-body/retained-source-missing"))?;
                if !std::ptr::eq(retained, packet) || !retained.corroborates_retained_scalar() {
                    return Err(freeze("finished-loop-body/retained-source-drift"));
                }
                Ok(Some(std::slice::from_ref(retained.scalar_actual())))
            }
        }
    }

    /// Both coverage consumers inspect the same original pools and final producers.
    /// Project the SAME checked stored receiver already corroborated by the
    /// original node visitor; no opcode scan can manufacture this source loan.
    pub(in crate::mir) fn finished_stored_receiver_read_v1(
        &self, packet: FinalizedBorrowedCallPacketV1<'_>, function: &MirFunction,
    ) -> Result<Option<(BasicBlockId, MirInstruction)>, String> {
        let FinalizedBorrowedCallPacketV1::Lexical(packet) = packet else { return Ok(None); };
        let Some(read) = packet.stored_receiver_read_v1() else { return Ok(None); };
        if !matches!(read.1, MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::ObjectFieldBorrow { .. }, .. }) {
            return Ok(None);
        }
        let (symbol, read) = self.ledger.finished_binding_for_owner(packet.call_site().owner(), read)?;
        if symbol != function.signature.name || !super::physical_boundary::check_binding(function, None, read.0, &read.1)? {
            return Err(freeze("final-call-visit/function-missing"));
        }
        Ok(Some(read))
    }

    pub(in crate::mir) fn visit_finalized_borrowed_call_nodes_v1(
        &self,
        module: &MirModule,
        mut visit: impl FnMut(
            FinalizedBorrowedCallPacketV1<'_>,
            &[LocalCallArgumentV1],
            &MirFunction,
            (BasicBlockId, usize),
            &[((BasicBlockId, MirInstruction), (BasicBlockId, usize))],
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        let mut sites = BTreeSet::new();
        let mut coordinates = BTreeSet::new();
        self.visit_finalized_lexical_call_nodes_v1(
            module,
            |_, _, packet, arguments, caller, coordinate, copies| {
                if !sites.insert(packet.call_site().clone())
                    || !coordinates.insert((caller.signature.name.clone(), coordinate))
                {
                    return Err(freeze("final-borrowed-call/duplicate"));
                }
                visit(
                    FinalizedBorrowedCallPacketV1::Lexical(packet),
                    arguments,
                    caller,
                    coordinate,
                    copies,
                )
            },
        )?;
        for (site, packet) in &self.loop_body_calls {
            let (caller, coordinate) = self
                .ledger
                .finished_loop_body_call_producer_v1(site, packet, module)?;
            if !sites.insert(site.clone())
                || !coordinates.insert((caller.signature.name.clone(), coordinate))
            {
                return Err(freeze("final-borrowed-call/duplicate"));
            }
            visit(
                FinalizedBorrowedCallPacketV1::LoopBody(packet),
                &[],
                caller,
                coordinate,
                &[],
            )?;
        }
        for (key, validation) in &self.construction {
            let symbol = module.canonical_callable_definition_symbol(key)
                .ok_or_else(|| freeze("final-provider/definition-missing"))?;
            let caller = module.functions.get(symbol)
                .ok_or_else(|| freeze("final-provider/function-missing"))?;
            validation.visit_constructor_unit_packets_v1(key, caller, |packet, coordinate| {
                self.validate_constructor_unit_packet_source_v1(packet)?;
                if !sites.insert(packet.call_site().clone())
                    || !coordinates.insert((caller.signature.name.clone(), coordinate)) {
                    return Err(freeze("final-borrowed-call/duplicate"));
                }
                visit(FinalizedBorrowedCallPacketV1::Lexical(packet), &[], caller, coordinate, &[])
            })?;
            validation.visit_provider_packets_v1(key, caller, |emission, packet| {
                self.ledger.borrowed_provider_packet_actuals_v1(packet)?;
                let coordinate = emission.coordinate_v1(caller)?;
                if !sites.insert(packet.incoming().call_site().clone())
                    || !coordinates.insert((caller.signature.name.clone(), coordinate)) {
                    return Err(freeze("final-borrowed-call/duplicate"));
                }
                visit(FinalizedBorrowedCallPacketV1::ProviderArgument(packet),
                    packet.arguments(), caller, coordinate, &[])
            })?;
        }
        Ok(())
    }

    fn validate_constructor_unit_packet_source_v1(
        &self, packet: &EmittedLexicalCallProjectionV1,
    ) -> Result<(), String> {
        let row = packet.original_row()?;
        let loan = self.ledger.constructor_unit_call_source_v1(row)?
            .ok_or_else(|| freeze("final-constructor-call/source-missing"))?;
        if loan.call().site() != packet.call_site()
            || row.result() != Some(crate::mir::instruction::InvokeCallResultKind::Unit)
            || !row.argument_sites().is_empty() {
            return Err(freeze("final-constructor-call/source-drift"));
        }
        packet.call_in_context(packet.call_site().owner(), &[], CallValidationContextV1::Finalized(self))?;
        Ok(())
    }

    /// Exact source-backed Birth caller result witness, separate from generic
    /// borrowed-argument coverage. Only the original constructor pool selects it.
    pub(in crate::mir) fn corroborate_constructor_unit_call_v1(
        &self, module: &MirModule, key: &CanonicalSameModuleCallableKeyV1,
        caller: &MirFunction, coordinate: (BasicBlockId, usize), instruction: &MirInstruction,
    ) -> Result<(), String> {
        let symbol = module.canonical_callable_definition_symbol(key)
            .ok_or_else(|| freeze("final-constructor-call/birth-definition"))?;
        if module.functions.get(symbol).is_none_or(|original| !std::ptr::eq(original, caller)) {
            return Err(freeze("final-constructor-call/caller-identity"));
        }
        let mut originals = self.construction.iter().filter(|(original, _)| original == key);
        let (_, retained) = originals.next()
            .ok_or_else(|| freeze("final-constructor-call/birth-source-missing"))?;
        if originals.next().is_some() { return Err(freeze("final-constructor-call/birth-source-duplicate")); }
        let mut matching = 0;
        retained.visit_constructor_unit_packets_v1(key, caller, |packet, actual| {
            if actual != coordinate { return Ok(()); }
            self.validate_constructor_unit_packet_source_v1(packet)?;
            if &packet.invoke_binding().1 != instruction {
                return Err(freeze("final-constructor-call/instruction"));
            }
            matching += 1;
            Ok(())
        })?;
        if matching != 1 { return Err(freeze("final-constructor-call/coordinate-membership")); }
        Ok(())
    }
}

/// Ephemeral final view of one source carrier. Construction stays with the
/// original Loop owner and its existing finished projection, never MIR search.
pub(in crate::mir) struct FinalizedTextLoopViewV2 {
    pub(in crate::mir) slot: ValueId,
    pub(in crate::mir) edges: Vec<((BasicBlockId, MirInstruction), BasicBlockId, ValueId)>,
    pub(in crate::mir) dependencies: Vec<(BasicBlockId, MirInstruction)>,
    pub(in crate::mir) operations: Vec<(BasicBlockId, MirInstruction)>,
    pub(in crate::mir) normal_ends: Vec<((BasicBlockId, MirInstruction), ValueId)>,
}
impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn visit_finished_text_loops_v2(
        &self, module: &MirModule,
        mut visit: impl FnMut(&MirFunction, &FinalizedTextLoopViewV2) -> Result<(), String>,
    ) -> Result<(), String> {
        self.validate_finalized_root_cleanup_v1(module)?;
        for ((owner, _), role) in self.ledger.loop_cond_after_roles.borrow().iter() {
            let Some(proof) = role.source_proof_v2() else { continue; };
            if proof.product().source.source.owner() != *owner {
                return Err(freeze("source-loop/aggregate-correspondence"));
            }
            self.ledger.with_finished_projection(*owner, |symbol, projection| {
                let function = module.functions.get(symbol)
                    .filter(|function| function.signature.name == symbol)
                    .ok_or_else(|| freeze("finished-local-call/child-not-finished"))?;
                let mut views = std::collections::BTreeMap::<ValueId, FinalizedTextLoopViewV2>::new();
                proof.visit_text_edges_v2(|slot, original_phi, predecessor, incoming, originals, normal_ends| {
                    let phi = projection.relation_binding(function, original_phi)?
                        .ok_or_else(|| freeze("source-loop/phi-removed"))?;
                    let predecessor = projection.destination(predecessor)
                        .ok_or_else(|| freeze("source-loop/aggregate-correspondence"))?;
                    if !matches!(&phi.1, MirInstruction::Phi { inputs, .. }
                        if inputs.iter().filter(|(pred, value)| *pred == predecessor && *value == incoming).count() == 1) {
                        return Err(freeze("source-loop/phi-kind"));
                    }
                    let view = views.entry(slot).or_insert_with(|| FinalizedTextLoopViewV2 {
                        slot, edges: Vec::new(), dependencies: Vec::new(), operations: Vec::new(), normal_ends: Vec::new(),
                    });
                    view.edges.push((phi, predecessor, incoming));
                    // A port's normal suffix may End a local while retaining
                    // the current carrier. Fault has no such retention grant.
                    for original in normal_ends {
                        let finished = projection.binding(original.0, &original.1)?
                            .ok_or_else(|| freeze("finished-local-call/producer-removed"))?;
                        if !projection.recorded().contains(&finished) {
                            return Err(freeze("finished-local-call/producer-unrecorded"));
                        }
                        super::finalized_source_projection::find_finished_producer(symbol, &finished, function)?;
                        if !view.normal_ends.contains(&(finished.clone(), incoming)) {
                            view.normal_ends.push((finished, incoming));
                        }
                    }
                    for original in originals {
                        if let Some(finished) = projection.relation_binding(function, original)? {
                            if !view.dependencies.contains(&finished) { view.dependencies.push(finished); }
                        }
                    }
                    Ok(())
                })?;
                if views.is_empty() { return Ok(()); }
                let context = CallValidationContextV1::Finalized(self);
                let mut originals = proof.bindings().to_vec();
                context.with_local_call_groups_v1(*owner, |groups| {
                    for group in groups { originals.extend_from_slice(group.bindings()); }
                    Ok(())
                })?;
                // Return-expression packets keep their original scope even
                // when direct terminal entries have moved into this handoff.
                for ((exit_owner, _), progress) in self.ledger.root_exits.borrow().iter() {
                    if exit_owner != owner { continue; }
                    let order = match progress {
                        RootHomeExitProgress::Emitted { order, bindings, .. } => {
                            originals.extend_from_slice(bindings); order
                        }
                        RootHomeExitProgress::Finalized(order) => order,
                        _ => continue,
                    };
                    originals.extend(order.return_expression_bindings_v1(context)?);
                }
                let mut operations = Vec::new();
                for original in originals {
                    let selected = matches!(original.1,
                        MirInstruction::StringResidenceInstall { .. } | MirInstruction::Invoke { .. });
                    if !selected { continue; }
                    let finished = projection.binding(original.0, &original.1)?
                        .ok_or_else(|| freeze("finished-local-call/producer-removed"))?;
                    if !projection.recorded().contains(&finished) {
                        return Err(freeze("finished-local-call/producer-unrecorded"));
                    }
                    super::finalized_source_projection::find_finished_producer(symbol, &finished, function)?;
                    if !operations.contains(&finished) { operations.push(finished); }
                }
                for mut view in views.into_values() {
                    view.operations.clone_from(&operations);
                    visit(function, &view)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    }
}
