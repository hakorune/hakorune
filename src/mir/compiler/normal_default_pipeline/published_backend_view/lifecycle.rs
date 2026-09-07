//! Lifecycle presence observation and borrowed finalized handoff access.
//!
//! Presence is physical observation only. It does not admit a generic view
//! or issue constructor meaning; the parent pipeline performs the final
//! artifact-only admission after verification and commit preparation.

use hakorune_mir_defs::SameModuleCallableNamespaceV1;

use crate::mir::{Callee, MirInstruction};

use super::{PublishedMirBackendView, PublishedMirBackendViewErrorV1};

pub(in crate::mir::compiler::normal_default_pipeline) fn is_lifecycle_instruction(
    instruction: &MirInstruction,
) -> bool {
    matches!(
        instruction,
        MirInstruction::Invoke { .. }
            | MirInstruction::InvokeNormalResult { .. }
            | MirInstruction::ReturnFault { .. }
            | MirInstruction::FaultFrameEnter { .. }
            | MirInstruction::ObjectFieldGet { .. }
    ) || matches!(
        instruction,
        MirInstruction::Call(call) if matches!(call.callee, Callee::BirthConstructor { .. })
    )
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

    pub(in crate::mir::compiler) fn bind_finalized_root_handoff(
        mut self,
        handoff: Option<&'module crate::mir::finalized_root_handoff::FinalizedRootHandoffV1>,
    ) -> Result<Self, String> {
        let Some(handoff) = handoff else {
            return self
                .bind_retained_root(None)
                .map_err(|error| error.to_string());
        };
        self = self
            .bind_retained_root(Some(handoff.root_key()))
            .map_err(|error| error.to_string())?;
        if let Some(array) = handoff.script_array() {
            let root = self
                .retained_root
                .ok_or_else(|| fault("retained-root-missing"))?;
            if root.signature.name != handoff.root_key() {
                return Err(fault("retained-root-key-drift"));
            }
            array.validate_root_binding(root)?;
            self.retained_handoff = Some(handoff);
            return Ok(self);
        }
        let births = handoff
            .births()
            .ok_or_else(|| fault("retained-callable-missing"))?;
        let root_source = handoff.root_source();
        let root_result = handoff.root_result();
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
        if let Some(source) = root_source {
            let valid = match (source.terminal_i64_add(), source.terminal_unit_return(), source.terminal_integer_literal(), source.terminal_i64_field_return(), root_result) {
                (Some(terminal), None, None, None, Some(crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::I64AddReturn { owner })) => terminal.owner() == owner,
                (None, Some(terminal), None, None, Some(crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::UnitReturn { owner })) => terminal.owner() == owner,
                (None, None, Some(terminal), None, Some(crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::IntegerLiteralReturn { owner })) => terminal.owner() == owner,
                (None, None, None, Some(terminal), Some(crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::I64FieldReturn { owner })) => terminal.owner() == owner,
                _ => false,
            };
            if !valid {
                return Err(fault("retained-root-source-result-drift"));
            }
        }
        self.retained_handoff = Some(handoff);
        Ok(self)
    }

    pub(crate) fn retained_root(&self) -> Option<&'module crate::mir::MirFunction> {
        self.retained_root
    }

    pub(crate) fn retained_birth_abi(
        &self,
    ) -> Option<&'module [crate::mir::normal_callable_semantic_package::BirthAbiHandoffV1]> {
        self.retained_handoff.and_then(|handoff| handoff.births())
    }

    pub(crate) fn retained_script_array(
        &self,
    ) -> Option<&'module crate::mir::builder::FinalizedScriptArrayV1> {
        self.retained_handoff
            .and_then(|handoff| handoff.script_array())
    }

    pub(crate) fn retained_root_result(
        &self,
    ) -> Option<crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1> {
        self.retained_handoff
            .and_then(|handoff| handoff.root_result())
    }

    pub(crate) fn retained_root_source(
        &self,
    ) -> Option<&'module crate::mir::normal_callable_semantic_package::FinalizedRootSourceHandoffV1>
    {
        self.retained_handoff
            .and_then(|handoff| handoff.root_source())
    }

    /// Diagnostic/physical borrow only; cloning this module does not carry
    /// lifecycle admission through the generic constructor.
    pub(crate) fn module(&self) -> &'module crate::mir::MirModule {
        self.module
    }

    pub(crate) const fn has_lifecycle_instructions(&self) -> bool {
        self.has_lifecycle_instructions
    }

    pub(crate) fn lifecycle_storage_profile(
        &self,
    ) -> Option<super::PublishedObjectStorageProfileV1> {
        self.lifecycle_storage_profile.copied()
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle/admission-{reason}]")
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedObjectStorageProfileV1 {
    SafeMutex = 1,
    SingleThreadExact = 2,
}

impl PublishedObjectStorageProfileV1 {
    pub(crate) fn from_runtime_name(value: Option<&str>) -> Result<Self, String> {
        match value {
            None | Some("") | Some("safe_mutex") => Ok(Self::SafeMutex),
            Some("single_thread_exact") => Ok(Self::SingleThreadExact),
            Some(other) => Err(format!(
                "[freeze:contract][published-lifecycle/storage-profile] unsupported profile: {other}"
            )),
        }
    }
}

#[cfg(test)]
#[path = "lifecycle_profile_tests.rs"]
mod profile_tests;

#[cfg(test)]
#[path = "script_artifact_tests.rs"]
mod script_artifact_tests;
