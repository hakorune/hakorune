//! RAWPORT0-M0-T0: pending child terminal before parent restore.
//!
//! The product remains disconnected. It gives a future invocation-owned port
//! one place to perform `validate -> seal/collect -> restore` without making
//! the existing production `run()` lifecycle observe a new collector.

use crate::ast::ASTNode;
use crate::mir::builder::type_context::TypeContext;
use crate::mir::function::MirFunction;
use crate::mir::MirBuilder;

use super::payload_terminal::PendingFunctionPayloadSessionCloseV1;
use super::{
    CanonicalFunctionLoweringSessionV1, CanonicalFunctionSessionErrorV1,
    LegacyFunctionPayloadSessionErrorV1,
};

/// Receipt issued only after an unpublished canonical child has restored its
/// captured caller context. It carries no retry, Builder, or draft access.
#[derive(Debug)]
pub(in crate::mir::builder) struct CanonicalFunctionSessionRestorationReceiptV1 {
    _seal: CanonicalFunctionSessionRestorationReceiptSealV1,
}

#[derive(Debug)]
struct CanonicalFunctionSessionRestorationReceiptSealV1;

/// One successful child draft whose parent context is still captured.
///
/// It cannot be cloned. Dropping it aborts the draft and restores the parent,
/// which preserves unwind safety while this disconnected product has no
/// production caller.
pub(in crate::mir::builder) struct PendingFunctionSessionCloseV1<'builder> {
    pending: PendingFunctionPayloadSessionCloseV1<'builder, ()>,
}

/// One successful legacy child whose parent context is still captured.
///
/// This newtype deliberately prevents a raw legacy child from entering the
/// resolved-child terminal by accident.  Its only future consumer is the
/// invocation-owned legacy terminal; it carries no collector, identity, or
/// publication policy.
pub(in crate::mir::builder) struct LegacyFunctionPendingSessionV1<'builder> {
    pending: PendingFunctionSessionCloseV1<'builder>,
    _seal: LegacyFunctionPendingSessionSealV1,
}

struct LegacyFunctionPendingSessionSealV1;

/// A canonical-only close that has passed every session invariant while the
/// child function is still installed.  The draft-seal owner consumes this
/// product exactly once; no fallible session work remains after `commit`.
pub(in crate::mir::builder) struct PreparedFunctionSessionCloseV1<'builder> {
    session: Option<CanonicalFunctionLoweringSessionV1<'builder>>,
    function_name: String,
}

/// Typed payload for the canonical draft-seal commit.  The session terminal
/// applies the projected function and final type facts before it extracts the
/// function; no caller mutation closure or bare Builder access is needed.
pub(in crate::mir::builder) struct PreparedFunctionSessionCommitInputV1 {
    function: MirFunction,
    type_ctx: TypeContext,
}

impl PreparedFunctionSessionCommitInputV1 {
    pub(in crate::mir::builder) fn new(function: MirFunction, type_ctx: TypeContext) -> Self {
        Self { function, type_ctx }
    }
}

/// Rejection from the canonical prepared close.  The original session stays
/// owned so its unpublished function and caller snapshot can be discarded
/// together, without a retry or partial restore path.
pub(in crate::mir::builder) struct RejectedFunctionSessionCloseV1<'builder> {
    owner: Option<CanonicalFunctionLoweringSessionV1<'builder>>,
    error: CanonicalFunctionSessionErrorV1,
}

impl<'builder> CanonicalFunctionLoweringSessionV1<'builder> {
    /// Borrow the live Builder only for draft-seal preparation. No ownership
    /// or mutable capability crosses this view.
    pub(in crate::mir::builder) fn builder_view(&self) -> &MirBuilder {
        self.builder
    }

    /// Mutable lowering view used only before the draft-seal prepare begins.
    /// The prepared owner never exposes this capability.
    pub(in crate::mir::builder) fn builder_view_mut_for_lowering(&mut self) -> &mut MirBuilder {
        self.builder
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn builder_view_mut_for_test(&mut self) -> &mut MirBuilder {
        self.builder_view_mut_for_lowering()
    }

    /// Discard the unpublished function and restore the captured caller
    /// context exactly once. This is the explicit rejection terminal for an
    /// owner-preserving draft-seal prepare; callers cannot retry the session.
    pub(in crate::mir::builder) fn discard_unpublished(
        mut self,
    ) -> CanonicalFunctionSessionRestorationReceiptV1 {
        self.restore_context();
        CanonicalFunctionSessionRestorationReceiptV1 {
            _seal: CanonicalFunctionSessionRestorationReceiptSealV1,
        }
    }

    /// Borrow-only readiness used by the owner-preserving draft-seal path.
    /// The session is not consumed, so a caller can return it unchanged when
    /// a later planner rejects.
    pub(in crate::mir::builder) fn draft_seal_readiness(
        &self,
    ) -> Result<String, CanonicalFunctionSessionErrorV1> {
        self.validate_before_draft_seal()
            .map_err(|error| CanonicalFunctionSessionErrorV1::Cleanup(error.to_string()))
    }

    /// Run one child operation and retain its successful draft before restore.
    ///
    /// Existing production facades continue through `run()` and therefore do
    /// not consume this transition until the later atomic cutover.
    pub(in crate::mir::builder) fn capture_pending(
        self,
        operation: impl FnOnce(&mut crate::mir::MirBuilder) -> Result<MirFunction, String>,
    ) -> Result<PendingFunctionSessionCloseV1<'builder>, CanonicalFunctionSessionErrorV1> {
        match self.capture_pending_payload(|builder| operation(builder).map(|draft| (draft, ()))) {
            Ok(pending) => Ok(PendingFunctionSessionCloseV1 { pending }),
            Err(LegacyFunctionPayloadSessionErrorV1::Primary(primary)) => {
                Err(CanonicalFunctionSessionErrorV1::Primary(primary))
            }
            Err(LegacyFunctionPayloadSessionErrorV1::CleanupAfterSuccess {
                payload: (),
                detail,
            }) => Err(CanonicalFunctionSessionErrorV1::Cleanup(detail.into())),
            Err(LegacyFunctionPayloadSessionErrorV1::DuringCleanup { primary, detail }) => {
                Err(CanonicalFunctionSessionErrorV1::DuringCleanup {
                    primary,
                    cleanup: detail.into(),
                })
            }
        }
    }

    /// Prepare the canonical draft-seal session close without extracting or
    /// mutating the unpublished function.  The sole commit terminal below
    /// takes `current_function` and restores the caller context infallibly.
    pub(in crate::mir::builder) fn prepare_draft_seal_close(
        self,
    ) -> Result<PreparedFunctionSessionCloseV1<'builder>, RejectedFunctionSessionCloseV1<'builder>>
    {
        match self.validate_before_draft_seal() {
            Ok(function_name) => Ok(PreparedFunctionSessionCloseV1 {
                session: Some(self),
                function_name,
            }),
            Err(error) => Err(RejectedFunctionSessionCloseV1 {
                owner: Some(self),
                error: CanonicalFunctionSessionErrorV1::Cleanup(error),
            }),
        }
    }

    /// Consume a session only after the caller has already completed the
    /// borrow-only readiness check. This is the no-failure handoff used by
    /// `OpenFunctionDraftSealV1::prepare`.
    pub(in crate::mir::builder) fn prepare_draft_seal_close_after_readiness(
        self,
        function_name: String,
    ) -> PreparedFunctionSessionCloseV1<'builder> {
        PreparedFunctionSessionCloseV1 {
            session: Some(self),
            function_name,
        }
    }
}

impl MirBuilder {
    /// Capture one resolved child draft without restoring its parent yet.
    ///
    /// Only the invocation-owned module port consumes this M0 seam.  The
    /// existing resolved production entries keep their V1 close-and-publish
    /// path until HEADERPORT0-I0.
    pub(in crate::mir::builder) fn capture_resolved_function_pending_session_v1(
        &mut self,
        function_name: &str,
        operation: impl FnOnce(&mut MirBuilder) -> Result<MirFunction, String>,
    ) -> Result<PendingFunctionSessionCloseV1<'_>, CanonicalFunctionSessionErrorV1> {
        CanonicalFunctionLoweringSessionV1::open(
            self,
            function_name,
            super::FunctionBodyCaptureV1::CanonicalClosedFamily,
        )
        .capture_pending(operation)
    }

    /// Capture one raw legacy child without publishing it after restoration.
    ///
    /// This is disconnected S0 vocabulary.  The legacy body snapshot retains
    /// the existing function-session capture mode, while the returned newtype
    /// cannot be paired with a resolved owner admission.
    pub(in crate::mir::builder) fn capture_legacy_function_pending_session_v1(
        &mut self,
        function_name: &str,
        body_snapshot: Vec<ASTNode>,
        operation: impl FnOnce(&mut MirBuilder) -> Result<MirFunction, String>,
    ) -> Result<LegacyFunctionPendingSessionV1<'_>, CanonicalFunctionSessionErrorV1> {
        CanonicalFunctionLoweringSessionV1::open(
            self,
            function_name,
            super::FunctionBodyCaptureV1::Legacy(body_snapshot),
        )
        .capture_pending(operation)
        .map(|pending| LegacyFunctionPendingSessionV1 {
            pending,
            _seal: LegacyFunctionPendingSessionSealV1,
        })
    }
}

impl PendingFunctionSessionCloseV1<'_> {
    /// Run one port-owned completion while the parent context remains captured.
    ///
    /// The closure must close every admission failure before it returns.  Its
    /// success path may then collect infallibly; both success and failure
    /// restore the parent exactly once.  Unwind relies on `Drop` for the same
    /// restoration guarantee.
    pub(in crate::mir::builder) fn complete_before_restore<R, E>(
        self,
        complete: impl FnOnce(MirFunction) -> Result<R, E>,
    ) -> Result<R, E> {
        self.pending
            .complete_before_restore(|draft, ()| complete(draft))
    }

    /// Discard the pending draft and restore after a later pre-collect error.
    #[allow(dead_code)] // RAWPORT0-S0 exposes the later error path without a production caller.
    pub(in crate::mir::builder) fn abort_and_restore(self) {
        self.pending.abort_and_restore();
    }
}

impl LegacyFunctionPendingSessionV1<'_> {
    /// Complete one legacy child while its parent remains captured.
    ///
    /// The caller must close all collector admission failures before this
    /// closure returns.  Both outcomes restore the parent exactly once.
    pub(in crate::mir::builder) fn complete_before_restore<R, E>(
        self,
        complete: impl FnOnce(MirFunction) -> Result<R, E>,
    ) -> Result<R, E> {
        self.pending.complete_before_restore(complete)
    }

    #[allow(dead_code)] // The disconnected S0 vocabulary exposes explicit abort semantics.
    pub(in crate::mir::builder) fn abort_and_restore(self) {
        self.pending.abort_and_restore();
    }
}

impl PreparedFunctionSessionCloseV1<'_> {
    /// Consume the prepared close and return the unpublished function.  All
    /// checks have already happened, so extraction and caller restoration are
    /// deliberately infallible ownership transitions.
    pub(in crate::mir::builder) fn commit(self) -> MirFunction {
        self.commit_with_input(None)
    }

    /// Apply one prepared projected function/type payload and then perform the
    /// same infallible extraction/restore terminal.  This is the only session
    /// path that may install a draft-seal projection into the live function
    /// slot before `current_function.take()`.
    pub(in crate::mir::builder) fn commit_projected(
        self,
        input: PreparedFunctionSessionCommitInputV1,
    ) -> MirFunction {
        self.commit_with_input(Some(input))
    }

    fn commit_with_input(
        mut self,
        input: Option<PreparedFunctionSessionCommitInputV1>,
    ) -> MirFunction {
        let mut session = self
            .session
            .take()
            .expect("prepared function session close commits once");
        if let Some(input) = input {
            session.builder.function_state.current_function = Some(input.function);
            session.builder.function_state.type_ctx = input.type_ctx;
        }
        let draft = session
            .builder
            .function_state
            .current_function
            .take()
            .expect("prepared close validated one installed function");
        debug_assert_eq!(draft.signature.name, self.function_name);
        session.restore_context();
        draft
    }

    pub(in crate::mir::builder) fn function_name(&self) -> &str {
        &self.function_name
    }
}

impl RejectedFunctionSessionCloseV1<'_> {
    pub(in crate::mir::builder) fn error(&self) -> &CanonicalFunctionSessionErrorV1 {
        &self.error
    }

    /// Discard the unpublished function and restore the captured caller.
    pub(in crate::mir::builder) fn discard(mut self) {
        if let Some(mut owner) = self.owner.take() {
            owner.restore_context();
        }
    }
}

impl Drop for PreparedFunctionSessionCloseV1<'_> {
    fn drop(&mut self) {
        if let Some(session) = self.session.as_mut() {
            session.restore_context();
        }
    }
}

impl Drop for RejectedFunctionSessionCloseV1<'_> {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.as_mut() {
            owner.restore_context();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::ast::{ASTNode, DeclarationAttrs, Span};
    use crate::mir::builder::module_draft_collector::{
        CompletedDraftSignatureViewV1, ModuleDraftCollectorV1,
    };
    use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1};
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirType,
    };

    use super::super::FunctionBodyCaptureV1;
    use super::{CanonicalFunctionLoweringSessionV1, PreparedFunctionSessionCommitInputV1};

    fn resolved_product() -> Arc<crate::mir::resolved_semantics::VerifiedResolvedFunctionV1> {
        let function = ASTNode::FunctionDeclaration {
            name: "draft_seal/0".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        };
        let view = FunctionSyntaxViewV1::from_ast(&function).unwrap();
        FunctionSemanticResolverSessionV1::new(0)
            .unwrap()
            .resolve(view)
            .unwrap()
    }

    fn draft(symbol: &str, arity: usize) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.into(),
                params: vec![MirType::Integer; arity],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn pending_success_keeps_parent_captured_until_explicit_abort() {
        let mut builder = MirBuilder::new();
        let pending = CanonicalFunctionLoweringSessionV1::open(
            &mut builder,
            "pending/0",
            FunctionBodyCaptureV1::Legacy(Vec::<ASTNode>::new()),
        )
        .capture_pending(|_| Ok(draft("pending/0", 0)))
        .unwrap();
        assert!(pending.pending.parent_is_captured());

        pending.abort_and_restore();
        assert_eq!(builder.next_value_id().0, 0);
    }

    #[test]
    fn explicit_abort_discards_the_draft_and_releases_the_builder() {
        let mut builder = MirBuilder::new();
        let pending = CanonicalFunctionLoweringSessionV1::open(
            &mut builder,
            "pending/0",
            FunctionBodyCaptureV1::Legacy(Vec::<ASTNode>::new()),
        )
        .capture_pending(|_| Ok(draft("pending/0", 0)))
        .unwrap();

        pending.abort_and_restore();
        assert_eq!(builder.next_value_id().0, 0);
    }

    #[test]
    fn dropping_pending_restores_without_a_collector_side_effect() {
        let mut builder = MirBuilder::new();
        let collector = ModuleDraftCollectorV1::default();
        {
            let _pending = CanonicalFunctionLoweringSessionV1::open(
                &mut builder,
                "pending/0",
                FunctionBodyCaptureV1::Legacy(Vec::<ASTNode>::new()),
            )
            .capture_pending(|_| Ok(draft("pending/0", 0)))
            .unwrap();
        }
        assert_eq!(builder.next_value_id().0, 0);
        assert_eq!(collector.symbol_count(), 0);
    }

    #[test]
    fn legacy_pending_capture_keeps_legacy_authority_distinct_from_resolved() {
        let mut builder = MirBuilder::new();
        let pending = builder
            .capture_legacy_function_pending_session_v1("Legacy.f/0", Vec::new(), |_| {
                Ok(draft("Legacy.f/0", 0))
            })
            .unwrap();

        pending
            .complete_before_restore(|draft| {
                assert_eq!(draft.signature.name, "Legacy.f/0");
                Ok::<(), ()>(())
            })
            .unwrap();
        assert_eq!(builder.next_value_id().0, 0);
    }

    #[test]
    fn draft_seal_close_extracts_once_then_restores_parent_context() {
        let mut builder = MirBuilder::new();
        let pending = builder.open_resolved_function_draft_seal_session_v1("draft_seal/0");
        let product = resolved_product();
        let owner = product.owner();
        pending
            .builder
            .function_state
            .resolved_binding_state
            .install(&product)
            .unwrap();
        pending
            .builder
            .function_state
            .resolved_binding_state
            .finish(owner)
            .unwrap();
        pending
            .builder
            .enter_function_for_test("draft_seal/0".into());

        let prepared = match pending.prepare_draft_seal_close() {
            Ok(prepared) => prepared,
            Err(_) => panic!("resolved canonical session should be seal-ready"),
        };
        assert_eq!(prepared.function_name(), "draft_seal/0");
        assert!(prepared
            .session
            .as_ref()
            .unwrap()
            .builder
            .function_state
            .current_function
            .is_some());

        let mut projected = draft("draft_seal/0", 0);
        projected.signature.return_type = MirType::Bool;
        let mut projected_types = crate::mir::builder::type_context::TypeContext::new();
        projected_types.set_type(crate::mir::ValueId::new(7), MirType::Bool);
        let draft = prepared.commit_projected(PreparedFunctionSessionCommitInputV1::new(
            projected,
            projected_types,
        ));
        assert_eq!(draft.signature.name, "draft_seal/0");
        assert_eq!(draft.signature.return_type, MirType::Bool);
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    }

    #[test]
    fn draft_seal_close_rejects_legacy_session_before_extracting() {
        let mut builder = MirBuilder::new();
        let pending = CanonicalFunctionLoweringSessionV1::open(
            &mut builder,
            "legacy/0",
            FunctionBodyCaptureV1::Legacy(Vec::new()),
        );
        pending.builder.enter_function_for_test("legacy/0".into());

        let rejected = match pending.prepare_draft_seal_close() {
            Ok(_) => panic!("legacy session must not enter the canonical draft seal"),
            Err(rejected) => rejected,
        };
        assert!(rejected
            .error()
            .to_string()
            .contains("draft_seal_requires_resolved_authority"));
        rejected.discard();
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    }

    #[test]
    fn draft_seal_session_discard_restores_parent_context_once() {
        let mut builder = MirBuilder::new();
        let pending = builder.open_resolved_function_draft_seal_session_v1("discard/0");
        pending.discard_unpublished();
        assert_eq!(builder.next_value_id().0, 0);
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    }
}
