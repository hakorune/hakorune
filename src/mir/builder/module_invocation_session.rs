//! CUT0-I0-SESSION0: explicit Builder invocation configuration and session.
//!
//! This is a disconnected transaction owner. It snapshots persistent Builder
//! inputs once, installs them into a candidate, and commits only through a
//! single consuming terminal. No compiler ingress uses this box yet.

use std::collections::HashMap;

use super::MirBuilder;
use crate::mir::MirType;
use hakorune_mir_builder::CoreContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct BuilderCoreCursorV1 {
    next_value: u32,
    next_block: u32,
    next_binding: u32,
    temp_slot: u32,
    debug_join: u32,
}

impl BuilderCoreCursorV1 {
    fn from_builder(builder: &MirBuilder) -> Self {
        Self {
            next_value: builder.core_ctx.peek_next_value().as_u32(),
            next_block: builder.core_ctx.peek_next_block().as_u32(),
            next_binding: builder.core_ctx.next_binding_id,
            temp_slot: builder.core_ctx.temp_slot_counter,
            debug_join: builder.core_ctx.debug_join_counter,
        }
    }

    fn install_into(&self, builder: &mut MirBuilder) {
        builder.core_ctx = CoreContext::from_cursors(
            self.next_value,
            self.next_block,
            self.next_binding,
            self.temp_slot,
            self.debug_join,
        );
    }

    pub(in crate::mir::builder) fn next_value(&self) -> u32 {
        self.next_value
    }

    pub(in crate::mir::builder) fn next_block(&self) -> u32 {
        self.next_block
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum BuilderCoreIdSeedV1 {
    ContinueLive(BuilderCoreCursorV1),
    Fresh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum BuilderCoreSeedPolicyV1 {
    ContinueLive,
    Fresh,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct BuilderInvocationConfigV1 {
    repl_mode: bool,
    quiet_internal_logs: bool,
    using_import_boxes: HashMap<String, String>,
    plugin_method_sigs: HashMap<(String, String), MirType>,
    source_file: Option<String>,
    core_id_seed: BuilderCoreIdSeedV1,
}

impl BuilderInvocationConfigV1 {
    pub(in crate::mir::builder) fn snapshot(
        current: &MirBuilder,
        core_id_seed: BuilderCoreIdSeedV1,
    ) -> Self {
        Self {
            repl_mode: current.repl_mode,
            quiet_internal_logs: current.comp_ctx.quiet_internal_logs,
            using_import_boxes: current.comp_ctx.using_import_boxes.clone(),
            plugin_method_sigs: current.comp_ctx.plugin_method_sigs.clone(),
            source_file: current.current_source_file(),
            core_id_seed,
        }
    }

    pub(in crate::mir::builder) fn snapshot_with_policy(
        current: &MirBuilder,
        policy: BuilderCoreSeedPolicyV1,
    ) -> Self {
        let seed = match policy {
            BuilderCoreSeedPolicyV1::ContinueLive => {
                BuilderCoreIdSeedV1::ContinueLive(BuilderCoreCursorV1::from_builder(current))
            }
            BuilderCoreSeedPolicyV1::Fresh => BuilderCoreIdSeedV1::Fresh,
        };
        Self::snapshot(current, seed)
    }

    fn install_into(&self, candidate: &mut MirBuilder) {
        candidate.repl_mode = self.repl_mode;
        candidate.comp_ctx.quiet_internal_logs = self.quiet_internal_logs;
        candidate.comp_ctx.using_import_boxes = self.using_import_boxes.clone();
        candidate.comp_ctx.plugin_method_sigs = self.plugin_method_sigs.clone();
        match &self.source_file {
            Some(source_file) => candidate.set_source_file_hint(source_file.clone()),
            None => candidate.clear_source_file_hint(),
        }
        if let BuilderCoreIdSeedV1::ContinueLive(cursor) = &self.core_id_seed {
            cursor.install_into(candidate);
        }
    }

    pub(in crate::mir::builder) fn core_id_seed(&self) -> &BuilderCoreIdSeedV1 {
        &self.core_id_seed
    }

    pub(in crate::mir::builder) fn repl_mode(&self) -> bool {
        self.repl_mode
    }

    pub(in crate::mir::builder) fn quiet_internal_logs(&self) -> bool {
        self.quiet_internal_logs
    }

    pub(in crate::mir::builder) fn using_import_boxes(&self) -> &HashMap<String, String> {
        &self.using_import_boxes
    }

    pub(in crate::mir::builder) fn plugin_method_sigs(&self) -> &HashMap<(String, String), MirType> {
        &self.plugin_method_sigs
    }

    pub(in crate::mir::builder) fn source_file(&self) -> Option<&str> {
        self.source_file.as_deref()
    }
}

pub(in crate::mir::builder) struct ModuleBuilderInvocationSessionV1 {
    candidate: MirBuilder,
    config: BuilderInvocationConfigV1,
    _seal: ModuleBuilderInvocationSessionSealV1,
}

#[derive(Debug)]
struct ModuleBuilderInvocationSessionSealV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum BuilderCommitReadinessErrorV1 {
    CurrentModuleOpen,
    CurrentFunctionOpen,
    CurrentBlockOpen,
    SlotRegistryOpen,
    CompilationContextOpen,
    RecursionDepthOpen,
    FunctionStateOpen,
}

impl std::fmt::Display for BuilderCommitReadinessErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][invocation_session] {self:?}")
    }
}

impl std::error::Error for BuilderCommitReadinessErrorV1 {}

pub(in crate::mir::builder) struct PreparedBuilderExternalCommitV1 {
    session: ModuleBuilderInvocationSessionV1,
    _seal: PreparedBuilderExternalCommitSealV1,
}

#[derive(Debug)]
struct PreparedBuilderExternalCommitSealV1;

impl ModuleBuilderInvocationSessionV1 {
    pub(in crate::mir::builder) fn open(
        _current: &MirBuilder,
        config: BuilderInvocationConfigV1,
    ) -> Self {
        let mut candidate = MirBuilder::new();
        config.install_into(&mut candidate);
        Self {
            candidate,
            config,
            _seal: ModuleBuilderInvocationSessionSealV1,
        }
    }

    pub(in crate::mir::builder) fn builder_mut(&mut self) -> &mut MirBuilder {
        &mut self.candidate
    }

    pub(in crate::mir::builder) fn config(&self) -> &BuilderInvocationConfigV1 {
        &self.config
    }

    pub(in crate::mir::builder) fn prepare_external_commit(
        self,
    ) -> Result<PreparedBuilderExternalCommitV1, BuilderCommitReadinessErrorV1> {
        if self.candidate.current_module.is_some() {
            return Err(BuilderCommitReadinessErrorV1::CurrentModuleOpen);
        }
        if self.candidate.function_state.current_function.is_some() {
            return Err(BuilderCommitReadinessErrorV1::CurrentFunctionOpen);
        }
        if self.candidate.function_state.current_block.is_some() {
            return Err(BuilderCommitReadinessErrorV1::CurrentBlockOpen);
        }
        if !self.candidate.function_state.is_closed_for_external_commit() {
            return Err(BuilderCommitReadinessErrorV1::FunctionStateOpen);
        }
        if self.candidate.comp_ctx.current_slot_registry.is_some() {
            return Err(BuilderCommitReadinessErrorV1::SlotRegistryOpen);
        }
        if self.candidate.comp_ctx.compilation_context.is_some() {
            return Err(BuilderCommitReadinessErrorV1::CompilationContextOpen);
        }
        if self.candidate.recursion_depth != 0 {
            return Err(BuilderCommitReadinessErrorV1::RecursionDepthOpen);
        }
        Ok(PreparedBuilderExternalCommitV1 {
            session: self,
            _seal: PreparedBuilderExternalCommitSealV1,
        })
    }
}

impl PreparedBuilderExternalCommitV1 {
    pub(in crate::mir::builder) fn commit(self, current: &mut MirBuilder) {
        *current = self.session.candidate;
    }
}
