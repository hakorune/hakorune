//! Borrowed lifecycle coordinates retained by the one published-view scan.
//!
//! These rows are physical observation only. They do not admit a generic view
//! or issue constructor meaning; the parent pipeline performs the final
//! artifact-only admission after verification and commit preparation.

use hakorune_mir_defs::SameModuleCallableNamespaceV1;

use crate::mir::{Callee, MirInstruction, ValueId};

use super::{
    PublishedMirBackendView, PublishedMirBackendViewErrorV1, PublishedStaticMethodRouteV1,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleInstructionRef<'module> {
    function_name: &'module str,
    block_id: u32,
    instruction_index: u32,
    instruction: &'module MirInstruction,
}

impl<'module> PublishedLifecycleInstructionRef<'module> {
    pub(super) fn return_instruction(
        function_name: &'module str,
        block_id: u32,
        instruction_index: u32,
        instruction: &'module MirInstruction,
    ) -> Option<Self> {
        matches!(instruction, MirInstruction::Return { .. }).then_some(Self {
            function_name,
            block_id,
            instruction_index,
            instruction,
        })
    }

    pub(super) fn from_instruction(
        function_name: &'module str,
        block_id: u32,
        instruction_index: u32,
        instruction: &'module MirInstruction,
    ) -> Option<Self> {
        let selected = matches!(
            instruction,
            MirInstruction::Invoke { .. }
                | MirInstruction::InvokeNormalResult { .. }
                | MirInstruction::ReturnFault { .. }
                | MirInstruction::FaultFrameEnter { .. }
                | MirInstruction::ObjectFieldGet { .. }
        ) || matches!(
            instruction,
            MirInstruction::Call(call) if matches!(call.callee, Callee::BirthConstructor { .. })
        );
        selected.then_some(Self {
            function_name,
            block_id,
            instruction_index,
            instruction,
        })
    }

    pub(crate) fn function_name(self) -> &'module str {
        self.function_name
    }

    pub(crate) const fn block_id(self) -> u32 {
        self.block_id
    }

    pub(crate) const fn instruction_index(self) -> u32 {
        self.instruction_index
    }

    pub(crate) fn instruction(self) -> &'module MirInstruction {
        self.instruction
    }
}

impl<'module> PublishedMirBackendView<'module> {
    /// Identity only, not lifecycle admission. Only the parent pipeline binds
    /// this after final validation, strict verification and commit preparation.
    pub(in crate::mir::compiler) fn bind_retained_root(
        mut self,
        key: Option<&str>,
    ) -> Result<Self, PublishedMirBackendViewErrorV1> {
        self.retained_root = key
            .map(|key| {
                self.module
                    .functions
                    .get(key)
                    .ok_or(PublishedMirBackendViewErrorV1::RetainedRootMissing)
            })
            .transpose()?;
        Ok(self)
    }

    pub(in crate::mir::compiler) fn bind_finalized_root_birth_handoff(
        mut self,
        handoff: Option<crate::mir::normal_callable_semantic_package::FinalizedRootBirthHandoffV1>,
    ) -> Result<Self, String> {
        let Some(handoff) = handoff else {
            return self
                .bind_retained_root(None)
                .map_err(|error| error.to_string());
        };
        let (root_key, root_source, root_result, births) = handoff.into_parts();
        self = self
            .bind_retained_root(Some(&root_key))
            .map_err(|error| error.to_string())?;
        if births.iter().any(|birth| {
            let key = birth.target();
            key.namespace() != SameModuleCallableNamespaceV1::BirthConstructor
                || self
                    .module
                    .canonical_callable_definition_symbol(key)
                    .is_none()
                || self
                    .module
                    .canonical_callable_definition_symbol(key)
                    .and_then(|symbol| self.module.functions.get(symbol))
                    .is_none_or(|function| birth.abi().physical_arity() != function.params.len())
        }) {
            return Err(fault("retained-birth-missing"));
        }
        self.retained_birth_keys =
            Some(births.iter().map(|birth| birth.target().clone()).collect());
        self.retained_birth_abi = Some(births);
        if let Some(source) = root_source.as_ref() {
            let valid = match (source.terminal_i64_add(), source.terminal_unit_return(), root_result) {
                (Some(terminal), None, Some(crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::I64AddReturn { owner })) => terminal.owner() == owner,
                (None, Some(terminal), Some(crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::UnitReturn { owner })) => terminal.owner() == owner,
                _ => false,
            };
            if !valid { return Err(fault("retained-root-source-result-drift")); }
        }
        self.retained_root_source = root_source;
        self.retained_root_result = root_result;
        Ok(self)
    }

    pub(crate) fn retained_root(&self) -> Option<&'module crate::mir::MirFunction> {
        self.retained_root
    }

    pub(crate) fn retained_birth_abi(
        &self,
    ) -> Option<&[crate::mir::normal_callable_semantic_package::BirthAbiHandoffV1]> {
        self.retained_birth_abi.as_deref()
    }

    pub(crate) const fn retained_root_result(
        &self,
    ) -> Option<crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1> {
        self.retained_root_result
    }

    pub(crate) fn retained_root_source(
        &self,
    ) -> Option<&crate::mir::normal_callable_semantic_package::FinalizedRootSourceHandoffV1> {
        self.retained_root_source.as_ref()
    }

    /// Diagnostic/physical borrow only; cloning this module does not carry
    /// lifecycle admission through the generic constructor.
    pub(crate) fn module(&self) -> &'module crate::mir::MirModule {
        self.module
    }

    pub(crate) fn lifecycle_instructions(&self) -> &[PublishedLifecycleInstructionRef<'module>] {
        &self.lifecycle_instructions
    }

    pub(crate) fn lifecycle_storage_profile(
        &self,
    ) -> Option<super::PublishedObjectStorageProfileV1> {
        self.lifecycle_storage_profile
    }

    /// Final-artifact-only admission. Generic view construction deliberately
    /// keeps these rows at `UnsupportedBeforeObject`.
    pub(in crate::mir::compiler) fn activate_lifecycle_for_final_artifact(
        mut self,
        profile: super::PublishedObjectStorageProfileV1,
    ) -> Result<Self, String> {
        if self.route != PublishedStaticMethodRouteV1::UnsupportedBeforeObject
            || self.has_non_lifecycle_unsupported
            || self.lifecycle_instructions.is_empty()
        {
            return Err(fault("candidate-unavailable"));
        }
        let root = self
            .retained_root
            .ok_or_else(|| fault("retained-root-missing"))?;
        let root_name = root.signature.name.as_str();
        if matches!(self.retained_root_result,
            Some(crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::UnitReturn { .. })) {
            return Err(fault("unit-c-role-unavailable"));
        }
        let retained_birth_keys = self
            .retained_birth_keys
            .as_deref()
            .ok_or_else(|| fault("retained-birth-handoff-missing"))?;
        if !matches!(
            self.retained_root_result,
            Some(crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::I64AddReturn { .. })
        ) {
            return Err(fault("retained-root-result-missing"));
        }
        if self.retained_root_source.is_none() {
            return Err(fault("retained-root-source-missing"));
        }
        self.lifecycle_instructions
            .extend(self.return_instructions.iter().copied().filter(|row| {
                row.function_name == root_name
                    || self
                        .module
                        .canonical_callable_definitions
                        .iter()
                        .any(|(key, symbol)| {
                            retained_birth_keys.contains(key)
                                && symbol.as_str() == row.function_name
                        })
            }));
        for row in &self.lifecycle_instructions {
            if row.function_name == root_name {
                continue;
            }
            let Some(function) = self.module.functions.get(row.function_name) else {
                return Err(fault("function-missing"));
            };
            let Some((key, _)) = self
                .module
                .canonical_callable_definitions
                .iter()
                .find(|(_, symbol)| symbol.as_str() == row.function_name)
            else {
                return Err(fault("function-not-cataloged"));
            };
            if key.namespace() != SameModuleCallableNamespaceV1::BirthConstructor
                || function.signature.name != key.mir_symbol_projection()
            {
                return Err(fault("function-not-birth"));
            }
        }
        for row in &self.lifecycle_instructions {
            if let MirInstruction::Call(call) = row.instruction {
                let Callee::BirthConstructor { key, receiver } = &call.callee else {
                    continue;
                };
                if *receiver == ValueId::INVALID
                    || key.namespace() != SameModuleCallableNamespaceV1::BirthConstructor
                    || self
                        .module
                        .canonical_callable_definition_symbol(key)
                        .is_none()
                {
                    return Err(fault("birth-call-drift"));
                }
            }
        }
        self.route = PublishedStaticMethodRouteV1::CanonicalTyped;
        self.lifecycle_storage_profile = Some(profile);
        Ok(self)
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle/admission-{reason}]")
}
