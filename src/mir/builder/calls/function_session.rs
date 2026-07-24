//! Closure-scoped transaction for one function lowering lifecycle.
//!
//! B0-L2c established this transaction without changing behavior. SA3-B now
//! reuses it through a distinct resolved entry whose successful close requires
//! installed and completed canonical BindingId authority.

use std::fmt;

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::function::{FunctionPublicationErrorV1, MirFunction, MirModule};

use super::context_lifecycle::LoweringContext;

mod terminal;
#[allow(unused_imports)] // RAWPORT0-S0 exposes the later invocation terminal without a caller.
pub(in crate::mir::builder) use terminal::{
    LegacyFunctionPendingSessionV1, PendingFunctionSessionCloseV1, PreparedFunctionSessionCloseV1,
    PreparedFunctionSessionCommitInputV1, RejectedFunctionSessionCloseV1,
};

#[derive(Debug)]
struct FunctionSessionCleanupErrorV1 {
    imbalances: Vec<&'static str>,
    saved_region_depth: usize,
    actual_region_depth: usize,
    region_prefix_matches: bool,
}

impl fmt::Display for FunctionSessionCleanupErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[freeze:contract][canonical_function_session/state_imbalance] fields={} saved_region_depth={} actual_region_depth={} region_prefix_matches={}",
            self.imbalances.join(","),
            self.saved_region_depth,
            self.actual_region_depth,
            self.region_prefix_matches
        )
    }
}

/// Owns the caller snapshot until one Lower result is explicitly closed.
///
/// Legacy and resolved entries share cleanup mechanics, but the captured mode
/// fixes whether resolved authority must be absent or completed at success.
pub(in crate::mir::builder) struct CanonicalFunctionLoweringSessionV1<'builder> {
    builder: &'builder mut MirBuilder,
    context: Option<LoweringContext>,
    requires_resolved_authority: bool,
    closed: bool,
}

enum FunctionBodyCaptureV1 {
    Legacy(Vec<ASTNode>),
    CanonicalClosedFamily,
}

/// Test-only terminal observed while the caller state is still captured.
///
/// FACTSESSION0-P0 uses this to prove a disconnected fact attempt reaches its
/// terminal action before the existing child-session restore. It is not a
/// production lifecycle product and carries neither Builder facts nor a
/// publication capability.
#[cfg(test)]
pub(in crate::mir::builder) enum FunctionSessionP0TerminalV1 {
    Success(MirFunction),
    Primary(String),
    Cleanup(String),
    Panicked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum FunctionDraftPublicationErrorV1 {
    MissingModule { function_name: String },
    Duplicate(FunctionPublicationErrorV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum CanonicalFunctionSessionErrorV1 {
    Primary(String),
    Cleanup(String),
    DuringCleanup { primary: String, cleanup: String },
    Publication(FunctionDraftPublicationErrorV1),
}

impl CanonicalFunctionSessionErrorV1 {
    pub(in crate::mir) fn duplicate_function_name(&self) -> Option<&str> {
        match self {
            Self::Publication(FunctionDraftPublicationErrorV1::Duplicate(error)) => {
                Some(&error.function_name)
            }
            _ => None,
        }
    }
}

impl fmt::Display for CanonicalFunctionSessionErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primary(detail) => formatter.write_str(detail),
            Self::Cleanup(cleanup) => write!(
                formatter,
                "[freeze:contract][canonical_function_session/cleanup_failed] cleanup={cleanup}"
            ),
            Self::DuringCleanup { primary, cleanup } => write!(
                formatter,
                "[freeze:contract][canonical_function_session/during_cleanup] primary={primary} cleanup={cleanup}"
            ),
            Self::Publication(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for CanonicalFunctionSessionErrorV1 {}

impl fmt::Display for FunctionDraftPublicationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingModule { function_name } => write!(
                formatter,
                "[freeze:contract][canonical_function_session/module_missing] function={function_name}"
            ),
            Self::Duplicate(error) => error.fmt(formatter),
        }
    }
}

pub(super) fn publish_function_draft(
    module: Option<&mut MirModule>,
    draft: MirFunction,
    requires_resolved_authority: bool,
) -> Result<(), FunctionDraftPublicationErrorV1> {
    let name = draft.signature.name.clone();
    let module = module.ok_or(FunctionDraftPublicationErrorV1::MissingModule {
        function_name: name,
    })?;
    if requires_resolved_authority {
        module
            .try_add_function(draft)
            .map_err(FunctionDraftPublicationErrorV1::Duplicate)
    } else {
        module.add_function(draft);
        Ok(())
    }
}

impl<'builder> CanonicalFunctionLoweringSessionV1<'builder> {
    fn open(
        builder: &'builder mut MirBuilder,
        function_name: &str,
        body_capture: FunctionBodyCaptureV1,
    ) -> Self {
        let requires_resolved_authority =
            matches!(&body_capture, FunctionBodyCaptureV1::CanonicalClosedFamily);
        let context = builder.prepare_lowering_context(function_name);
        builder.function_state.compilation.fn_body_ast = match body_capture {
            FunctionBodyCaptureV1::Legacy(body) => Some(body),
            FunctionBodyCaptureV1::CanonicalClosedFamily => None,
        };
        Self {
            builder,
            context: Some(context),
            requires_resolved_authority,
            closed: false,
        }
    }

    fn run(
        mut self,
        operation: impl FnOnce(&mut MirBuilder) -> Result<MirFunction, String>,
    ) -> Result<(), CanonicalFunctionSessionErrorV1> {
        let outcome = operation(self.builder);
        let draft = self.close_unpublished(outcome)?;
        publish_function_draft(
            self.builder.current_module.as_mut(),
            draft,
            self.requires_resolved_authority,
        )
        .map_err(CanonicalFunctionSessionErrorV1::Publication)
    }

    fn capture(
        mut self,
        operation: impl FnOnce(&mut MirBuilder) -> Result<MirFunction, String>,
    ) -> Result<MirFunction, CanonicalFunctionSessionErrorV1> {
        let outcome = operation(self.builder);
        self.close_unpublished(outcome)
    }

    fn close_unpublished(
        &mut self,
        outcome: Result<MirFunction, String>,
    ) -> Result<MirFunction, CanonicalFunctionSessionErrorV1> {
        let cleanup = self.cleanup(outcome.is_ok());
        match (outcome, cleanup) {
            (Ok(draft), Ok(())) => Ok(draft),
            (Err(primary), Ok(())) => Err(CanonicalFunctionSessionErrorV1::Primary(primary)),
            (Ok(_draft), Err(cleanup)) => Err(CanonicalFunctionSessionErrorV1::Cleanup(
                cleanup.to_string(),
            )),
            (Err(primary), Err(cleanup)) => Err(CanonicalFunctionSessionErrorV1::DuringCleanup {
                primary,
                cleanup: cleanup.to_string(),
            }),
        }
    }

    fn cleanup(&mut self, operation_succeeded: bool) -> Result<(), FunctionSessionCleanupErrorV1> {
        let validation = self.validate_before_restore(operation_succeeded);
        self.restore_context();
        validation
    }

    fn restore_context(&mut self) {
        if let Some(context) = self.context.take() {
            self.builder.restore_lowering_context(context);
        }
        self.closed = true;
    }

    fn validate_before_restore(
        &self,
        operation_succeeded: bool,
    ) -> Result<(), FunctionSessionCleanupErrorV1> {
        self.validate_session_state(operation_succeeded, false)
    }

    /// Validate the state that must already be closed before a canonical
    /// draft-seal commit takes ownership of the unpublished function.
    ///
    /// Unlike `validate_before_restore(true)`, this intentionally requires
    /// `current_function` to remain installed: the prepared draft-seal
    /// terminal is the sole owner that takes it during its infallible commit.
    pub(super) fn validate_before_draft_seal(&self) -> Result<String, String> {
        self.validate_session_state(true, true)
            .map(|()| {
                self.builder
                    .function_state
                    .current_function
                    .as_ref()
                    .map(|function| function.signature.name.clone())
                    .expect("draft-seal readiness requires one installed function")
            })
            .map_err(|error| error.to_string())
    }

    fn validate_session_state(
        &self,
        operation_succeeded: bool,
        draft_seal_owns_extraction: bool,
    ) -> Result<(), FunctionSessionCleanupErrorV1> {
        let context = self
            .context
            .as_ref()
            .expect("open function session always owns one context");
        let current_regions = self.builder.metadata_ctx.current_region_stack();
        let saved_regions = context.saved_region_stack();
        let region_prefix_matches = current_regions.starts_with(saved_regions);
        let region_depth_is_bounded = current_regions.len() <= saved_regions.len() + 1;

        let mut imbalances = Vec::new();
        if !region_prefix_matches || !region_depth_is_bounded {
            imbalances.push("observer_region_stack");
        }
        if self.builder.recursion_depth != 0 {
            imbalances.push("recursion_depth");
        }
        if self.builder.function_state.in_unified_boxcall_fallback {
            imbalances.push("unified_boxcall_fallback");
        }
        if operation_succeeded {
            if draft_seal_owns_extraction && !self.requires_resolved_authority {
                imbalances.push("draft_seal_requires_resolved_authority");
            }
            if draft_seal_owns_extraction {
                if self.builder.function_state.current_function.is_none() {
                    imbalances.push("draft_not_installed");
                }
            } else if self.builder.function_state.current_function.is_some() {
                imbalances.push("published_draft_still_installed");
            }
            if !self
                .builder
                .function_state
                .scope
                .lexical_scope_stack
                .is_empty()
            {
                imbalances.push("lexical_scope_stack");
            }
            if !self
                .builder
                .function_state
                .scope
                .loop_header_stack
                .is_empty()
            {
                imbalances.push("loop_header_stack");
            }
            if !self.builder.function_state.scope.loop_exit_stack.is_empty() {
                imbalances.push("loop_exit_stack");
            }
            if !self.builder.function_state.scope.if_merge_stack.is_empty() {
                imbalances.push("if_merge_stack");
            }
            if !self.builder.scope_ctx.debug_scope_stack.is_empty() {
                imbalances.push("debug_scope_stack");
            }
            if !self
                .builder
                .function_state
                .scope
                .fastmem_region_stack
                .is_empty()
            {
                imbalances.push("fastmem_region_stack");
            }
            if !self
                .builder
                .function_state
                .resolved_binding_state
                .session_success_is_closed(self.requires_resolved_authority)
            {
                imbalances.push("resolved_binding_authority");
            }
        }

        if imbalances.is_empty() {
            Ok(())
        } else {
            Err(FunctionSessionCleanupErrorV1 {
                imbalances,
                saved_region_depth: saved_regions.len(),
                actual_region_depth: current_regions.len(),
                region_prefix_matches,
            })
        }
    }
}

impl Drop for CanonicalFunctionLoweringSessionV1<'_> {
    fn drop(&mut self) {
        if self.closed {
            return;
        }
        if let Some(context) = self.context.take() {
            self.builder.restore_lowering_context(context);
        }
        if cfg!(debug_assertions) && !std::thread::panicking() {
            panic!("[freeze:contract][canonical_function_session/dropped_without_close]");
        }
    }
}

impl MirBuilder {
    /// Test-only FACTSESSION0-P0 seam: call the supplied terminal observer
    /// while the child session still owns the caller snapshot, then perform
    /// the unchanged restore. Panics are observed, cleaned up, and resumed.
    #[cfg(test)]
    pub(in crate::mir::builder) fn observe_function_terminal_before_restore_for_p0_test(
        &mut self,
        function_name: &str,
        operation: impl FnOnce(&mut MirBuilder) -> Result<MirFunction, String>,
        terminal: impl FnOnce(FunctionSessionP0TerminalV1, Option<String>) -> Result<(), String>,
    ) -> Result<(), String> {
        let mut session = CanonicalFunctionLoweringSessionV1::open(
            self,
            function_name,
            FunctionBodyCaptureV1::Legacy(Vec::new()),
        );
        let outcome =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| operation(session.builder)));
        let current_before_restore = session
            .builder
            .function_state
            .current_function
            .as_ref()
            .map(|function| function.signature.name.clone());

        match outcome {
            Ok(Ok(draft)) => {
                if let Err(cleanup) = session.validate_before_restore(true) {
                    let terminal = terminal(
                        FunctionSessionP0TerminalV1::Cleanup(cleanup.to_string()),
                        current_before_restore,
                    );
                    let restore = session.cleanup(false);
                    return terminal.and(restore.map_err(|error| error.to_string()));
                }
                let terminal = terminal(
                    FunctionSessionP0TerminalV1::Success(draft),
                    current_before_restore,
                );
                let cleanup = session.cleanup(terminal.is_ok());
                terminal.and(cleanup.map_err(|error| error.to_string()))
            }
            Ok(Err(primary)) => {
                let terminal = terminal(
                    FunctionSessionP0TerminalV1::Primary(primary),
                    current_before_restore,
                );
                let cleanup = session.cleanup(false);
                terminal.and(cleanup.map_err(|error| error.to_string()))
            }
            Err(payload) => {
                let _ = terminal(
                    FunctionSessionP0TerminalV1::Panicked,
                    current_before_restore,
                );
                let _ = session.cleanup(false);
                std::panic::resume_unwind(payload);
            }
        }
    }

    pub(super) fn with_function_lowering_session(
        &mut self,
        function_name: &str,
        body_snapshot: Vec<ASTNode>,
        operation: impl FnOnce(&mut MirBuilder) -> Result<MirFunction, String>,
    ) -> Result<(), String> {
        CanonicalFunctionLoweringSessionV1::open(
            self,
            function_name,
            FunctionBodyCaptureV1::Legacy(body_snapshot),
        )
        .run(operation)
        .map_err(|error| error.to_string())
    }

    pub(in crate::mir::builder) fn with_legacy_function_lowering_session_typed(
        &mut self,
        function_name: &str,
        body_snapshot: Vec<ASTNode>,
        operation: impl FnOnce(&mut MirBuilder) -> Result<MirFunction, String>,
    ) -> Result<(), CanonicalFunctionSessionErrorV1> {
        CanonicalFunctionLoweringSessionV1::open(
            self,
            function_name,
            FunctionBodyCaptureV1::Legacy(body_snapshot),
        )
        .run(operation)
    }

    pub(in crate::mir::builder) fn with_resolved_function_lowering_session(
        &mut self,
        function_name: &str,
        operation: impl FnOnce(&mut MirBuilder) -> Result<MirFunction, String>,
    ) -> Result<(), CanonicalFunctionSessionErrorV1> {
        CanonicalFunctionLoweringSessionV1::open(
            self,
            function_name,
            FunctionBodyCaptureV1::CanonicalClosedFamily,
        )
        .run(operation)
    }

    /// Return one restored canonical function draft without module publication.
    pub(in crate::mir::builder) fn with_resolved_function_draft_session(
        &mut self,
        function_name: &str,
        operation: impl FnOnce(&mut MirBuilder) -> Result<MirFunction, String>,
    ) -> Result<MirFunction, CanonicalFunctionSessionErrorV1> {
        CanonicalFunctionLoweringSessionV1::open(
            self,
            function_name,
            FunctionBodyCaptureV1::CanonicalClosedFamily,
        )
        .capture(operation)
    }

    /// Open the canonical session without closing or extracting its
    /// unpublished function. The draft-seal owner consumes this handoff after
    /// all borrow-only plans have been prepared.
    pub(in crate::mir::builder) fn open_resolved_function_draft_seal_session_v1(
        &mut self,
        function_name: &str,
    ) -> CanonicalFunctionLoweringSessionV1<'_> {
        CanonicalFunctionLoweringSessionV1::open(
            self,
            function_name,
            FunctionBodyCaptureV1::CanonicalClosedFamily,
        )
    }
}
