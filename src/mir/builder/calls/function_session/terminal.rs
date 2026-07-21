//! RAWPORT0-M0-T0: pending child terminal before parent restore.
//!
//! The product remains disconnected. It gives a future invocation-owned port
//! one place to perform `validate -> seal/collect -> restore` without making
//! the existing production `run()` lifecycle observe a new collector.

use crate::ast::ASTNode;
use crate::mir::function::MirFunction;
use crate::mir::MirBuilder;

use super::{CanonicalFunctionLoweringSessionV1, CanonicalFunctionSessionErrorV1};

/// One successful child draft whose parent context is still captured.
///
/// It cannot be cloned. Dropping it aborts the draft and restores the parent,
/// which preserves unwind safety while this disconnected product has no
/// production caller.
pub(in crate::mir::builder) struct PendingFunctionSessionCloseV1<'builder> {
    session: CanonicalFunctionLoweringSessionV1<'builder>,
    draft: Option<MirFunction>,
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

impl<'builder> CanonicalFunctionLoweringSessionV1<'builder> {
    /// Run one child operation and retain its successful draft before restore.
    ///
    /// Existing production facades continue through `run()` and therefore do
    /// not consume this transition until the later atomic cutover.
    pub(in crate::mir::builder) fn capture_pending(
        mut self,
        operation: impl FnOnce(&mut crate::mir::MirBuilder) -> Result<MirFunction, String>,
    ) -> Result<PendingFunctionSessionCloseV1<'builder>, CanonicalFunctionSessionErrorV1> {
        match operation(self.builder) {
            Ok(draft) => match self.validate_before_restore(true) {
                Ok(()) => Ok(PendingFunctionSessionCloseV1 {
                    session: self,
                    draft: Some(draft),
                }),
                Err(cleanup) => {
                    self.restore_context();
                    Err(CanonicalFunctionSessionErrorV1::Cleanup(
                        cleanup.to_string(),
                    ))
                }
            },
            Err(primary) => match self.cleanup(false) {
                Ok(()) => Err(CanonicalFunctionSessionErrorV1::Primary(primary)),
                Err(cleanup) => Err(CanonicalFunctionSessionErrorV1::DuringCleanup {
                    primary,
                    cleanup: cleanup.to_string(),
                }),
            },
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
        mut self,
        complete: impl FnOnce(MirFunction) -> Result<R, E>,
    ) -> Result<R, E> {
        let draft = self
            .draft
            .take()
            .expect("pending terminal owns exactly one successful draft");
        let result = complete(draft);
        self.restore_parent();
        result
    }

    /// Discard the pending draft and restore after a later pre-collect error.
    #[allow(dead_code)] // RAWPORT0-S0 exposes the later error path without a production caller.
    pub(in crate::mir::builder) fn abort_and_restore(mut self) {
        self.draft.take();
        self.restore_parent();
    }

    fn restore_parent(&mut self) {
        if !self.session.closed {
            self.session.restore_context();
        }
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

impl Drop for PendingFunctionSessionCloseV1<'_> {
    fn drop(&mut self) {
        self.draft.take();
        self.restore_parent();
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::ASTNode;
    use crate::mir::builder::module_draft_collector::{
        CompletedDraftSignatureViewV1, ModuleDraftCollectorV1,
    };
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirType,
    };

    use super::super::FunctionBodyCaptureV1;
    use super::CanonicalFunctionLoweringSessionV1;

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
        assert!(pending.session.context.is_some());

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
}
