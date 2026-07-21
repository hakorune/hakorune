//! RAWPORT0-S0: pending child terminal before parent restore.
//!
//! The product remains disconnected. It gives a future invocation-owned port
//! one place to perform `validate -> seal/collect -> restore` without making
//! the existing production `run()` lifecycle observe a new collector.

use crate::mir::function::MirFunction;

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

impl PendingFunctionSessionCloseV1<'_> {
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
}
