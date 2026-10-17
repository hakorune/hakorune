//! Exact source/publication/operand correspondence for provider argument Calls.
use super::*;
use crate::mir::{Callee, ConstValue, MirInstruction};
use crate::mir::normal_callable_semantic_package::{
    OrdinaryNewTrivialArgumentKindV1 as Kind, QualifiedStaticCallArgumentKindV1 as Literal,
};

impl ProviderCallArgEmission {
    pub(super) fn validate_source_v1(
        &self,
        function: &MirFunction,
        store: &SelectedConstructionStore,
    ) -> Result<(), String> {
        let ConstructionStoreRhsV1::ProviderConstruction {
            site, caller, arguments, static_sources, ..
        } = &store.rhs else { return Err(fault("provider-argument-source-kind")); };
        let argument = arguments.get(self.ordinal as usize)
            .ok_or_else(|| fault("provider-argument-ordinal"))?;
        let Kind::QualifiedStaticCall { target, arguments: literals } = argument.kind()
            else { return Err(fault("provider-argument-source-kind")); };
        if static_sources.iter().filter(|source| std::rc::Rc::ptr_eq(source, &self.source)).count() != 1
            || !self.source.matches_store_v1(store.field, site, caller, arguments)
            || !self.source.matches_argument_v1(argument)
            || !self.source.corroborates_publication_v1(&self.publication)
            || self.arguments.len() != literals.len() {
            return Err(fault("provider-argument-source-drift"));
        }
        if let Some(packet) = &self.packet {
            if !std::rc::Rc::ptr_eq(packet.original_source(), &self.source)
                || packet.incoming().call_site().site() != argument.site()
                || packet.incoming().target() != target || packet.incoming().caller() != caller {
                return Err(fault("provider-argument-packet-drift"));
            }
        }
        let block = function.blocks.get(&self.invoke_block)
            .ok_or_else(|| fault("provider-argument-block"))?;
        if !matches!(block.terminator.as_ref(), Some(MirInstruction::Invoke {
            operation: crate::mir::instruction::InvokeOperation::Call { call, .. }, ..
        }) if matches!(&call.callee, Callee::Global(global)
                if target.canonical_global_target_v1().as_ref() == Ok(global))
            && call.args.as_slice() == self.arguments.as_ref()) {
            return Err(fault("provider-argument-call-tuple"));
        }
        for (value, literal) in self.arguments.iter().zip(literals.iter()) {
            let expected = match literal {
                Literal::Integer(value) => ConstValue::Integer(*value),
                Literal::Bool(value) => ConstValue::Bool(*value),
            };
            let definitions: Vec<_> = function.blocks.values().flat_map(|block|
                block.instructions.iter()).filter(|instruction|
                    instruction.dst_value() == Some(*value)).collect();
            if definitions.len() != 1 || !matches!(definitions[0], MirInstruction::Const {
                dst, value: actual,
            } if dst == value && *actual == expected)
                || !block.instructions.iter().any(|instruction| std::ptr::eq(instruction, definitions[0])) {
                return Err(fault("provider-argument-constant-drift"));
            }
        }
        if function.blocks.get(&self.landing).is_none_or(|block|
            block.instructions.iter().filter(|instruction|
                matches!(instruction, MirInstruction::InvokeNormalResult { dst, invoke_block }
                    if *dst == self.value && *invoke_block == self.invoke_block)).count() != 1) {
            return Err(fault("provider-argument-normal-result"));
        }
        Ok(())
    }
}

impl RetainedConstructionValidation {
    pub(in crate::mir) fn has_provider_calls_v1(&self) -> bool {
        matches!(&self.construction, ConstructionState::Selected { stores, .. }
            if stores.values().any(|store| matches!(&store.progress,
                StoreProgress::Emitted { provider_child: Some(ProviderChildEmissionV1::Birth(birth)), .. }
                    if !birth.call_args.is_empty())))
    }

    pub(in crate::mir) fn corroborates_birth_owner_v1(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        function: &MirFunction,
    ) -> Result<bool, String> {
        if owner != self.owner { return Ok(false); }
        if key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor
            || function.signature.name != key.mir_symbol_projection() {
            return Err(fault("provider-argument-birth-owner"));
        }
        // The sole constructor capture paired this original owner with the
        // canonical key. Caller participation does not depend on provider count.
        self.validate_artifact_after_compiler_finishing(function)?;
        Ok(true)
    }

    pub(in crate::mir) fn visit_provider_packets_v1(
        &self,
        key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        function: &MirFunction,
        mut visit: impl FnMut(&ProviderCallArgEmission,
            &crate::mir::normal_callable_semantic_package::PreparedProviderStaticArgumentV1) -> Result<(), String>,
    ) -> Result<(), String> {
        if key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor
            || function.signature.name != key.mir_symbol_projection() {
            return Err(fault("provider-argument-birth-owner"));
        }
        self.validate_after_compiler_finishing(function)?;
        let ConstructionState::Selected { stores, .. } = &self.construction else { return Ok(()); };
        for store in stores.values() {
            let StoreProgress::Emitted {
                provider_child: Some(ProviderChildEmissionV1::Birth(birth)), ..
            } = &store.progress else { continue; };
            for emission in &birth.call_args {
                emission.validate_source_v1(function, store)?;
                if let Some(packet) = &emission.packet {
                    if packet.incoming().call_site().owner() != self.owner || emission.publication.caller() != key {
                        return Err(fault("provider-argument-birth-source"));
                    }
                    visit(emission, packet)?;
                }
            }
        }
        Ok(())
    }
}

impl ProviderCallArgEmission {
    pub(in crate::mir) fn coordinate_v1(&self, function: &MirFunction) -> Result<(BasicBlockId, usize), String> {
        let block = function.blocks.get(&self.invoke_block)
            .ok_or_else(|| fault("provider-argument-block"))?;
        Ok((self.invoke_block, block.instructions.len()))
    }
}
