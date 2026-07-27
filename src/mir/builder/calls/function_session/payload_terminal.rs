//! Payload-bearing legacy function session terminal.
//!
//! This module adds no second restoration algorithm. The generic capture
//! transition is owned by `CanonicalFunctionLoweringSessionV1`; this file only
//! carries its successful draft/payload pair until one explicit completion.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::function::MirFunction;

use super::{CanonicalFunctionLoweringSessionV1, FunctionBodyCaptureV1};

/// Typed failure from one payload-bearing legacy child capture.
///
/// Cleanup after a successful operation retains the owned payload. Cleanup
/// after a primary failure retains the original typed primary error.
#[derive(Debug)]
pub(in crate::mir::builder) enum LegacyFunctionPayloadSessionErrorV1<E, P> {
    Primary(E),
    CleanupAfterSuccess { payload: P, detail: Box<str> },
    DuringCleanup { primary: E, detail: Box<str> },
}

/// Generic draft/payload core while the parent context remains captured.
///
/// Only the legacy newtype below is exported. Keeping this core private
/// prevents a payload from being paired with a different session family.
pub(super) struct PendingFunctionPayloadSessionCloseV1<'builder, P> {
    session: Option<CanonicalFunctionLoweringSessionV1<'builder>>,
    draft: Option<MirFunction>,
    payload: Option<P>,
}

impl<'builder, P> PendingFunctionPayloadSessionCloseV1<'builder, P> {
    pub(super) fn new(
        session: CanonicalFunctionLoweringSessionV1<'builder>,
        draft: MirFunction,
        payload: P,
    ) -> Self {
        Self {
            session: Some(session),
            draft: Some(draft),
            payload: Some(payload),
        }
    }

    pub(super) fn complete_before_restore<R, E>(
        mut self,
        complete: impl FnOnce(MirFunction, P) -> Result<R, E>,
    ) -> Result<R, E> {
        let draft = self
            .draft
            .take()
            .expect("payload terminal owns exactly one successful draft");
        let payload = self
            .payload
            .take()
            .expect("payload terminal owns exactly one successful payload");
        let result = complete(draft, payload);
        self.restore_parent();
        result
    }

    pub(super) fn abort_and_restore(mut self) {
        self.draft.take();
        self.payload.take();
        self.restore_parent();
    }

    #[cfg(test)]
    pub(super) fn parent_is_captured(&self) -> bool {
        self.session
            .as_ref()
            .is_some_and(|session| session.context.is_some())
    }

    fn restore_parent(&mut self) {
        if let Some(mut session) = self.session.take() {
            if !session.closed {
                session.restore_context();
            }
        }
    }
}

impl<P> Drop for PendingFunctionPayloadSessionCloseV1<'_, P> {
    fn drop(&mut self) {
        self.draft.take();
        self.payload.take();
        self.restore_parent();
    }
}

/// One successful legacy child draft plus one owned completion payload.
///
/// The only terminal consumes both values and restores the captured parent
/// exactly once. No draft accessor, retry, resume, or rearm surface exists.
pub(in crate::mir::builder) struct LegacyFunctionPayloadPendingSessionV1<'builder, P> {
    pending: PendingFunctionPayloadSessionCloseV1<'builder, P>,
    _seal: LegacyFunctionPayloadPendingSessionSealV1,
}

struct LegacyFunctionPayloadPendingSessionSealV1;

impl<P> LegacyFunctionPayloadPendingSessionV1<'_, P> {
    pub(in crate::mir::builder) fn complete_before_restore<R, E>(
        self,
        complete: impl FnOnce(MirFunction, P) -> Result<R, E>,
    ) -> Result<R, E> {
        self.pending.complete_before_restore(complete)
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn parent_is_captured_for_test(&self) -> bool {
        self.pending.parent_is_captured()
    }
}

impl MirBuilder {
    /// Capture one legacy child together with an owned completion payload.
    pub(in crate::mir::builder) fn capture_legacy_function_payload_pending_session_v1<P, E>(
        &mut self,
        function_name: &str,
        body_snapshot: Vec<ASTNode>,
        operation: impl FnOnce(&mut MirBuilder) -> Result<(MirFunction, P), E>,
    ) -> Result<
        LegacyFunctionPayloadPendingSessionV1<'_, P>,
        LegacyFunctionPayloadSessionErrorV1<E, P>,
    > {
        CanonicalFunctionLoweringSessionV1::open(
            self,
            function_name,
            FunctionBodyCaptureV1::Legacy(body_snapshot),
        )
        .capture_pending_payload(operation)
        .map(|pending| LegacyFunctionPayloadPendingSessionV1 {
            pending,
            _seal: LegacyFunctionPayloadPendingSessionSealV1,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirType,
    };

    use super::LegacyFunctionPayloadSessionErrorV1;

    #[derive(Debug, PartialEq, Eq)]
    struct NonClonePayload<'source> {
        source: &'source str,
        ordinal: u32,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct TypedPrimary {
        code: u32,
    }

    fn draft(symbol: &str) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.to_owned(),
                params: Vec::new(),
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn seeded_builder() -> MirBuilder {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("outer/0".to_owned());
        builder
    }

    fn assert_outer_restored(builder: &MirBuilder) {
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .expect("outer function must be restored")
                .signature
                .name,
            "outer/0"
        );
        assert_eq!(builder.recursion_depth, 0);
    }

    #[test]
    fn payload_success_is_retained_until_completion_then_restores_parent() {
        let source = String::from("exact-source-owner");
        let mut builder = seeded_builder();
        let pending = builder
            .capture_legacy_function_payload_pending_session_v1("child/0", Vec::new(), |_| {
                Ok::<_, TypedPrimary>((
                    draft("child/0"),
                    NonClonePayload {
                        source: &source,
                        ordinal: 7,
                    },
                ))
            })
            .unwrap();

        let completed = pending
            .complete_before_restore(|draft, payload| {
                assert_eq!(draft.signature.name, "child/0");
                assert_eq!(
                    payload,
                    NonClonePayload {
                        source: "exact-source-owner",
                        ordinal: 7,
                    }
                );
                Ok::<_, TypedPrimary>(payload.ordinal)
            })
            .unwrap();
        assert_eq!(completed, 7);
        assert_outer_restored(&builder);
    }

    #[test]
    fn typed_primary_is_retained_and_a_fresh_session_succeeds() {
        let mut builder = seeded_builder();
        let rejected = match builder.capture_legacy_function_payload_pending_session_v1::<(), _>(
            "primary/0",
            Vec::new(),
            |_| Err(TypedPrimary { code: 11 }),
        ) {
            Ok(_) => panic!("typed primary must reject"),
            Err(rejected) => rejected,
        };
        assert!(matches!(
            rejected,
            LegacyFunctionPayloadSessionErrorV1::Primary(TypedPrimary { code: 11 })
        ));
        assert_outer_restored(&builder);

        builder
            .capture_legacy_function_payload_pending_session_v1("fresh/0", Vec::new(), |_| {
                Ok::<_, TypedPrimary>((draft("fresh/0"), 23_u32))
            })
            .unwrap()
            .complete_before_restore(|_, payload| Ok::<_, TypedPrimary>(payload))
            .unwrap();
        assert_outer_restored(&builder);
    }

    #[test]
    fn cleanup_after_success_retains_payload_without_draft_escape() {
        let mut builder = seeded_builder();
        let rejected = match builder.capture_legacy_function_payload_pending_session_v1(
            "cleanup/0",
            Vec::new(),
            |builder| {
                builder.recursion_depth = 1;
                Ok::<_, TypedPrimary>((
                    draft("cleanup/0"),
                    NonClonePayload {
                        source: "retained",
                        ordinal: 29,
                    },
                ))
            },
        ) {
            Ok(_) => panic!("cleanup imbalance must reject"),
            Err(rejected) => rejected,
        };
        match rejected {
            LegacyFunctionPayloadSessionErrorV1::CleanupAfterSuccess { payload, detail } => {
                assert_eq!(payload.source, "retained");
                assert_eq!(payload.ordinal, 29);
                assert!(detail.contains("recursion_depth"));
            }
            _ => panic!("expected cleanup-after-success rejection"),
        }
        assert_outer_restored(&builder);
    }

    #[test]
    fn during_cleanup_retains_typed_primary_and_allows_fresh_success() {
        let mut builder = seeded_builder();
        let rejected = match builder.capture_legacy_function_payload_pending_session_v1::<(), _>(
            "during/0",
            Vec::new(),
            |builder| {
                builder.recursion_depth = 1;
                Err(TypedPrimary { code: 31 })
            },
        ) {
            Ok(_) => panic!("primary plus cleanup imbalance must reject"),
            Err(rejected) => rejected,
        };
        match rejected {
            LegacyFunctionPayloadSessionErrorV1::DuringCleanup { primary, detail } => {
                assert_eq!(primary, TypedPrimary { code: 31 });
                assert!(detail.contains("recursion_depth"));
            }
            _ => panic!("expected during-cleanup rejection"),
        }
        assert_outer_restored(&builder);

        builder
            .capture_legacy_function_payload_pending_session_v1("fresh/0", Vec::new(), |_| {
                Ok::<_, TypedPrimary>((draft("fresh/0"), ()))
            })
            .unwrap()
            .complete_before_restore(|_, ()| Ok::<_, TypedPrimary>(()))
            .unwrap();
        assert_outer_restored(&builder);
    }

    #[test]
    fn dropping_pending_restores_parent_once() {
        let mut builder = seeded_builder();
        {
            let _pending = builder
                .capture_legacy_function_payload_pending_session_v1("drop/0", Vec::new(), |_| {
                    Ok::<_, TypedPrimary>((draft("drop/0"), 37_u32))
                })
                .unwrap();
        }
        assert_outer_restored(&builder);

        builder.recursion_depth = 41;
        assert_eq!(builder.recursion_depth, 41);
    }

    #[test]
    fn completion_rejection_restores_parent_without_rearming_payload() {
        let mut builder = seeded_builder();
        let rejected = builder
            .capture_legacy_function_payload_pending_session_v1(
                "completion-reject/0",
                Vec::new(),
                |_| Ok::<_, TypedPrimary>((draft("completion-reject/0"), 43_u32)),
            )
            .unwrap()
            .complete_before_restore(|_, payload| {
                assert_eq!(payload, 43);
                Err::<(), _>(TypedPrimary { code: 47 })
            });
        assert_eq!(rejected, Err(TypedPrimary { code: 47 }));
        assert_outer_restored(&builder);
    }
}
