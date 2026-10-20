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

    /// Borrow only mapped original Bool producers, after the same group replay
    /// used for Calls. This is an ephemeral verifier image, not a source index.
    pub(in crate::mir) fn visit_finished_map_presence_v1(
        &self, module: &MirModule,
        mut visit: impl FnMut(&MirFunction, (BasicBlockId, MirInstruction)) -> Result<(), String>,
    ) -> Result<(), String> {
        for owner in self.local_calls.keys() {
            self.ledger.with_finished_projection(*owner, |symbol, projection| {
                let function = module.functions.get(symbol).ok_or_else(|| freeze("final-provider/function-missing"))?;
                self.ledger.visit_map_presence_groups_v1(*owner, CallValidationContextV1::Finalized(self), function,
                    projection, module, |row| visit(function, row))
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
