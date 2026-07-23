//! CUT0-I0-SESSION0: explicit Builder invocation configuration and session.
//!
//! This is a disconnected transaction owner. It snapshots persistent Builder
//! inputs once, installs them into a candidate, and commits only through a
//! single consuming terminal. No compiler ingress uses this box yet.

use std::collections::HashMap;

use super::module_invocation_identity::{
    ModuleInvocationBrandV1, ModuleInvocationFamilyV1, ModuleInvocationTokenV1,
};
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
pub(in crate::mir) struct BuilderInvocationConfigV1 {
    repl_mode: bool,
    quiet_internal_logs: bool,
    using_import_boxes: HashMap<String, String>,
    plugin_method_sigs: HashMap<(String, String), MirType>,
    source_file: Option<String>,
    core_id_seed: BuilderCoreIdSeedV1,
}

impl BuilderInvocationConfigV1 {
    pub(in crate::mir) fn snapshot_for_canonical(
        current: &MirBuilder,
        source_file: Option<&str>,
    ) -> Self {
        let mut config = Self::snapshot_with_policy(current, BuilderCoreSeedPolicyV1::Fresh);
        config.source_file = source_file.map(str::to_owned);
        config
    }

    /// RAW-SOURCE0-BIND0: capture the legacy candidate inputs without
    /// mutating the live Builder. Raw preserves the existing CoreContext
    /// continuation policy; public ingress wiring remains disconnected.
    pub(in crate::mir) fn snapshot_for_raw(
        current: &MirBuilder,
        source_file: Option<&str>,
    ) -> Self {
        let mut config = Self::snapshot_with_policy(current, BuilderCoreSeedPolicyV1::ContinueLive);
        config.source_file = source_file.map(str::to_owned);
        config
    }

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

    pub(in crate::mir::builder) fn plugin_method_sigs(
        &self,
    ) -> &HashMap<(String, String), MirType> {
        &self.plugin_method_sigs
    }

    pub(in crate::mir) fn source_file(&self) -> Option<&str> {
        self.source_file.as_deref()
    }
}

pub(in crate::mir) struct ModuleBuilderInvocationSessionV1 {
    brand: ModuleInvocationBrandV1,
    family: ModuleInvocationFamilyV1,
    candidate: MirBuilder,
    config: BuilderInvocationConfigV1,
    _seal: ModuleBuilderInvocationSessionSealV1,
}

impl std::fmt::Debug for ModuleBuilderInvocationSessionV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ModuleBuilderInvocationSessionV1")
            .field("brand", &self.brand)
            .field("family", &self.family)
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
struct ModuleBuilderInvocationSessionSealV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum BuilderCommitReadinessErrorV1 {
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

#[derive(Debug)]
pub(in crate::mir) struct PreparedBuilderExternalCommitV1 {
    brand: ModuleInvocationBrandV1,
    family: ModuleInvocationFamilyV1,
    session: ModuleBuilderInvocationSessionV1,
    _seal: PreparedBuilderExternalCommitSealV1,
}

#[derive(Debug)]
struct PreparedBuilderExternalCommitSealV1;

/// A closed Builder candidate ready to enter module finalization.
///
/// This is deliberately distinct from the external-commit capability.  The
/// finalizer may still pair this owner with route-specific module evidence,
/// but no mutable Builder access is available after this product exists.
#[derive(Debug)]
pub(in crate::mir) struct PreparedBuilderModuleSessionV1 {
    brand: ModuleInvocationBrandV1,
    family: ModuleInvocationFamilyV1,
    session: ModuleBuilderInvocationSessionV1,
    _seal: PreparedBuilderModuleSessionSealV1,
}

#[derive(Debug)]
struct PreparedBuilderModuleSessionSealV1;

/// Readiness failure retaining the unpublished session for the caller's
/// rejected-owner chain.  No retry or recovery terminal is provided here.
#[derive(Debug)]
pub(in crate::mir) struct RejectedPreparedBuilderModuleSessionV1 {
    session: ModuleBuilderInvocationSessionV1,
    error: BuilderCommitReadinessErrorV1,
    _seal: RejectedPreparedBuilderModuleSessionSealV1,
}

#[derive(Debug)]
struct RejectedPreparedBuilderModuleSessionSealV1;

impl ModuleBuilderInvocationSessionV1 {
    pub(in crate::mir) fn open_for_token(
        token: &ModuleInvocationTokenV1,
        _current: &MirBuilder,
        config: BuilderInvocationConfigV1,
    ) -> Self {
        Self::open_with_identity(token.brand(), token.family(), config)
    }

    fn open_with_identity(
        brand: ModuleInvocationBrandV1,
        family: ModuleInvocationFamilyV1,
        config: BuilderInvocationConfigV1,
    ) -> Self {
        let mut candidate = MirBuilder::new();
        config.install_into(&mut candidate);
        Self {
            brand,
            family,
            candidate,
            config,
            _seal: ModuleBuilderInvocationSessionSealV1,
        }
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn open(
        _current: &MirBuilder,
        config: BuilderInvocationConfigV1,
    ) -> Self {
        Self::open_with_identity(
            ModuleInvocationBrandV1::legacy_test(),
            ModuleInvocationFamilyV1::Raw,
            config,
        )
    }

    pub(in crate::mir) fn builder_mut(&mut self) -> &mut MirBuilder {
        &mut self.candidate
    }

    pub(in crate::mir::builder) fn config(&self) -> &BuilderInvocationConfigV1 {
        &self.config
    }

    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir) const fn family(&self) -> ModuleInvocationFamilyV1 {
        self.family
    }

    fn readiness_error(&self) -> Result<(), BuilderCommitReadinessErrorV1> {
        if self.candidate.current_module.is_some() {
            return Err(BuilderCommitReadinessErrorV1::CurrentModuleOpen);
        }
        if self.candidate.function_state.current_function.is_some() {
            return Err(BuilderCommitReadinessErrorV1::CurrentFunctionOpen);
        }
        if self.candidate.function_state.current_block.is_some() {
            return Err(BuilderCommitReadinessErrorV1::CurrentBlockOpen);
        }
        if !self
            .candidate
            .function_state
            .is_closed_for_external_commit()
        {
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
        Ok(())
    }

    /// Consume a candidate after all Builder-owned state is closed.
    ///
    /// A rejected result retains the original session, so an outer
    /// finalization owner can discard the complete unpublished chain without
    /// reconstructing Builder state.
    pub(in crate::mir) fn prepare_module_session(
        self,
    ) -> Result<PreparedBuilderModuleSessionV1, RejectedPreparedBuilderModuleSessionV1> {
        if let Err(error) = self.readiness_error() {
            return Err(RejectedPreparedBuilderModuleSessionV1 {
                session: self,
                error,
                _seal: RejectedPreparedBuilderModuleSessionSealV1,
            });
        }
        let brand = self.brand;
        let family = self.family;
        Ok(PreparedBuilderModuleSessionV1 {
            brand,
            family,
            session: self,
            _seal: PreparedBuilderModuleSessionSealV1,
        })
    }

    pub(in crate::mir::builder) fn prepare_external_commit(
        self,
    ) -> Result<PreparedBuilderExternalCommitV1, BuilderCommitReadinessErrorV1> {
        let prepared = self
            .prepare_module_session()
            .map_err(|rejected| rejected.into_parts().1)?;
        let (brand, family, session) = prepared.into_parts();
        Ok(PreparedBuilderExternalCommitV1 {
            brand,
            family,
            session,
            _seal: PreparedBuilderExternalCommitSealV1,
        })
    }
}

impl PreparedBuilderModuleSessionV1 {
    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir) const fn family(&self) -> ModuleInvocationFamilyV1 {
        self.family
    }

    /// Consume the readiness product without exposing a mutable accessor on
    /// the prepared type.  A later phase may choose how to consume the closed
    /// session (finalization or external commit) exactly once.
    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        ModuleInvocationBrandV1,
        ModuleInvocationFamilyV1,
        ModuleBuilderInvocationSessionV1,
    ) {
        (self.brand, self.family, self.session)
    }

    pub(in crate::mir) fn into_external_commit(self) -> PreparedBuilderExternalCommitV1 {
        let (brand, family, session) = self.into_parts();
        PreparedBuilderExternalCommitV1 {
            brand,
            family,
            session,
            _seal: PreparedBuilderExternalCommitSealV1,
        }
    }
}

impl RejectedPreparedBuilderModuleSessionV1 {
    pub(in crate::mir) fn error(&self) -> &BuilderCommitReadinessErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        ModuleBuilderInvocationSessionV1,
        BuilderCommitReadinessErrorV1,
    ) {
        (self.session, self.error)
    }
}

impl PreparedBuilderExternalCommitV1 {
    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir) const fn family(&self) -> ModuleInvocationFamilyV1 {
        self.family
    }

    pub(in crate::mir) fn commit(self, current: &mut MirBuilder) {
        *current = self.session.candidate;
    }
}
