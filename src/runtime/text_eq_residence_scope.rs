//! One-shot runtime scope for the S6C TextEq TextRef pair.
//!
//! This is a caller-zero lifecycle adapter only.  Source/Facts/Recipe and the
//! canonical common-V2 session must establish the V9/ExactText co-seal before
//! constructing it.  The scope owns exactly one V9 End-authorized result and
//! one invocation Text residence; it emits no MIR and does not compare text.

use super::source_bound_v9_runtime::{
    SourceBoundV9RuntimeInputRefV1, SourceBoundV9RuntimeProducerRejectV1,
    SourceBoundV9RuntimeResultV1,
};
use super::text_formal_residence::{
    PinnedTextRootViewRef, TextFormalCallResidenceV1, TextFormalLeaseFinishRejectV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TextEqResidenceScopePrimaryFailureV1 {
    V9Input(SourceBoundV9RuntimeProducerRejectV1),
    Consumer(String),
    ExactTextFinish(TextFormalLeaseFinishRejectV1),
    V9Finish(SourceBoundV9RuntimeProducerRejectV1),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct TextEqResidenceScopeSuppressedCleanupV1 {
    exact_text: Option<TextFormalLeaseFinishRejectV1>,
    v9: Option<SourceBoundV9RuntimeProducerRejectV1>,
}

impl TextEqResidenceScopeSuppressedCleanupV1 {
    pub(crate) const fn exact_text(&self) -> Option<TextFormalLeaseFinishRejectV1> {
        self.exact_text
    }

    pub(crate) fn v9(&self) -> Option<SourceBoundV9RuntimeProducerRejectV1> {
        self.v9.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TextEqResidenceScopeFailureV1 {
    primary: TextEqResidenceScopePrimaryFailureV1,
    suppressed: TextEqResidenceScopeSuppressedCleanupV1,
}

impl TextEqResidenceScopeFailureV1 {
    pub(crate) const fn primary(&self) -> &TextEqResidenceScopePrimaryFailureV1 {
        &self.primary
    }

    pub(crate) const fn suppressed(&self) -> &TextEqResidenceScopeSuppressedCleanupV1 {
        &self.suppressed
    }
}

/// Opaque callback-scoped view of the two already-owned TextRefs.
#[derive(Debug)]
pub(crate) struct TextEqResidenceScopeTextRefs<'scope> {
    v9: SourceBoundV9RuntimeInputRefV1<'scope>,
    exact_text: &'scope TextFormalCallResidenceV1,
}

impl TextEqResidenceScopeTextRefs<'_> {
    pub(crate) fn with_v9_text<R>(
        &self,
        callback: impl FnOnce(&str) -> R,
    ) -> Result<R, super::dynamic_v2_lease::EndAuthorizedTextBorrowRejectV1> {
        self.v9.with_text(callback)
    }

    pub(crate) fn exact_text_root_count(&self) -> usize {
        self.exact_text.root_count()
    }

    pub(crate) fn with_exact_text_root<R>(
        &self,
        index: usize,
        callback: impl for<'root> FnOnce(PinnedTextRootViewRef<'root>) -> R,
    ) -> Option<R> {
        self.exact_text.with_root(index, callback)
    }
}

/// Move-only scope owner.  `with_text_refs` consumes it, so a second consumer
/// or a second cleanup owner cannot be expressed by this API.
#[must_use = "a TextEq residence scope must be consumed exactly once"]
#[derive(Debug)]
pub(crate) struct TextEqResidenceScopeV1 {
    v9: SourceBoundV9RuntimeResultV1,
    exact_text: TextFormalCallResidenceV1,
}

impl TextEqResidenceScopeV1 {
    /// The caller must pass the source-bound V9 result and the exact entry
    /// residence from the same already-verified occurrence/cohort.
    pub(crate) fn new(
        v9: SourceBoundV9RuntimeResultV1,
        exact_text: TextFormalCallResidenceV1,
    ) -> Result<Self, TextEqResidenceScopeFailureV1> {
        if exact_text.root_count() == 0 {
            return Err(TextEqResidenceScopeFailureV1 {
                primary: TextEqResidenceScopePrimaryFailureV1::Consumer(
                    "ExactText residence has no roots".to_owned(),
                ),
                suppressed: TextEqResidenceScopeSuppressedCleanupV1::default(),
            });
        }
        Ok(Self { v9, exact_text })
    }

    /// Lend both opaque refs once, then finish ExactText before canonical V9
    /// End.  The callback error is primary; cleanup errors are suppressed
    /// evidence.  No implicit Drop cleanup, retry, or fallback is provided.
    pub(crate) fn with_text_refs<R>(
        self,
        callback: impl FnOnce(TextEqResidenceScopeTextRefs<'_>) -> Result<R, String>,
    ) -> Result<R, TextEqResidenceScopeFailureV1> {
        let Self { v9, exact_text } = self;
        let callback_result = match v9.with_input(|input| {
            callback(TextEqResidenceScopeTextRefs {
                v9: input,
                exact_text: &exact_text,
            })
        }) {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(error)) => Err(TextEqResidenceScopePrimaryFailureV1::Consumer(error)),
            Err(error) => Err(TextEqResidenceScopePrimaryFailureV1::V9Input(error)),
        };

        let exact_finish = exact_text.finish();
        let v9_finish = v9.finish_at_canonical_end();

        match callback_result {
            Err(primary) => Err(TextEqResidenceScopeFailureV1 {
                primary,
                suppressed: TextEqResidenceScopeSuppressedCleanupV1 {
                    exact_text: exact_finish.err(),
                    v9: v9_finish.err(),
                },
            }),
            Ok(value) => match exact_finish {
                Err(error) => Err(TextEqResidenceScopeFailureV1 {
                    primary: TextEqResidenceScopePrimaryFailureV1::ExactTextFinish(error),
                    suppressed: TextEqResidenceScopeSuppressedCleanupV1 {
                        exact_text: None,
                        v9: v9_finish.err(),
                    },
                }),
                Ok(()) => match v9_finish {
                    Err(error) => Err(TextEqResidenceScopeFailureV1 {
                        primary: TextEqResidenceScopePrimaryFailureV1::V9Finish(error),
                        suppressed: TextEqResidenceScopeSuppressedCleanupV1::default(),
                    }),
                    Ok(()) => Ok(value),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::dynamic_call_slot_wire::{
        DynamicV2CallDispositionV1, DynamicV2CallOutV1, DynamicV2CallStatusV1, DynamicV2WireTagV1,
        DYNAMIC_V2_FORWARDED_NONE_V1,
    };
    use crate::runtime::dynamic_v2_lease::publish_end_authorized_text;
    use crate::runtime::host_handles;
    use crate::runtime::source_bound_v9_runtime::{
        produce_source_bound_v9_runtime_v1, SourceBoundV9RuntimeProducerPlanV1,
    };
    use crate::runtime::text_formal_abi::issue_text_formal_borrow_v1;
    use crate::runtime::text_formal_residence::acquire_text_formal_residence_from_published_wires_v1;
    use std::num::NonZeroU64;

    fn test_lock() -> std::sync::MutexGuard<'static, ()> {
        host_handles::test_host_handle_policy_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn normal_wire(handle: u64, token: NonZeroU64) -> DynamicV2CallOutV1 {
        DynamicV2CallOutV1 {
            status: DynamicV2CallStatusV1::Normal as u32,
            fault_code: 0,
            result_tag: DynamicV2WireTagV1::HostHandle as u32,
            disposition: DynamicV2CallDispositionV1::EndAuthorized as u32,
            forwarded_input: DYNAMIC_V2_FORWARDED_NONE_V1,
            reserved: 0,
            value_payload: handle,
            lease_token: token.get(),
            continuation_token: 0,
        }
    }

    fn scope(v9_text: &str, exact_text: &str) -> (TextEqResidenceScopeV1, u64) {
        let v9 = publish_end_authorized_text(v9_text).expect("v9 publish");
        let v9_wire = normal_wire(v9.handle(), v9.token());
        let v9 = produce_source_bound_v9_runtime_v1(
            SourceBoundV9RuntimeProducerPlanV1::s6c_substring(0, 9, 0).expect("v9 plan"),
            v9_wire,
        )
        .expect("v9 result");

        let exact_handle = host_handles::to_handle_text(exact_text);
        let pair = issue_text_formal_borrow_v1(exact_handle).expect("exact pair");
        let pair = pair.wire_pair();
        let residence = acquire_text_formal_residence_from_published_wires_v1(&[(
            pair.slot(),
            pair.generation(),
        )])
        .expect("exact residence");
        (
            TextEqResidenceScopeV1::new(v9, residence).expect("scope"),
            exact_handle,
        )
    }

    #[test]
    fn scope_lends_both_refs_and_finishes_exact_before_v9_end() {
        let _guard = test_lock();
        let (scope, exact_handle) = scope("substring", "needle");
        let result = scope.with_text_refs(|refs| {
            assert_eq!(refs.with_v9_text(str::to_owned), Ok("substring".to_owned()));
            assert_eq!(refs.exact_text_root_count(), 1);
            assert_eq!(
                refs.with_exact_text_root(0, |root| root.byte_len()),
                Some(6)
            );
            Ok::<_, String>(42_u32)
        });
        assert_eq!(result, Ok(42));
        host_handles::drop_handle(exact_handle);
    }

    #[test]
    fn consumer_error_is_primary_and_still_finishes_both_owners() {
        let _guard = test_lock();
        let (scope, exact_handle) = scope("substring", "needle");
        let error = scope
            .with_text_refs(|_| Err::<(), _>("consumer rejected".to_owned()))
            .expect_err("consumer rejection");
        assert!(matches!(
            error.primary(),
            TextEqResidenceScopePrimaryFailureV1::Consumer(detail) if detail == "consumer rejected"
        ));
        assert_eq!(
            error.suppressed(),
            &TextEqResidenceScopeSuppressedCleanupV1::default()
        );
        host_handles::drop_handle(exact_handle);
    }
}
