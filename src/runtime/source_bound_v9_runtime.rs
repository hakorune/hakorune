//! Private source-bound V9 runtime bridge.
//!
//! The compiler owns the static CheckedCallOut/NormalResult/End relation.  This
//! child owns only the provider-return handoff: one fixed provider wire is
//! validated and converted into one move-only End-authorized runtime result.
//! No MIR value, raw handle/token tuple, residence, or side table crosses this
//! boundary.

use std::num::NonZeroU64;

use crate::abi::dynamic_call_slot_wire::{
    DynamicV2CallDispositionV1, DynamicV2CallOutV1, DynamicV2CallStatusV1,
    DynamicV2WireSchemaRejectV1, DynamicV2WireTagV1, DYNAMIC_V2_FORWARDED_NONE_V1,
};

use super::dynamic_v2_lease::{
    EndAuthorizedTextBorrowRejectV1, EndAuthorizedTextV1, LeaseConsumeRejectV1,
};

/// The only source-bound provider admitted by this caller-zero bridge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceBoundV9RuntimeProviderV1 {
    S6CSubstringV1,
}

/// Static, backend-private evidence for one exact provider-return site.
///
/// These fields are physical binding evidence, not a source semantic issuer.
/// The source/Facts/Recipe/co-seal owner must validate them before constructing
/// this plan; the runtime bridge only checks that the fixed provider shape is
/// internally consistent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SourceBoundV9RuntimeProducerPlanV1 {
    provider: SourceBoundV9RuntimeProviderV1,
    site_id: u32,
    occurrence: u32,
    lease_slot: u32,
}

impl SourceBoundV9RuntimeProducerPlanV1 {
    /// Issue the fixed S6C Substring plan after the caller has checked the
    /// source/cohort/site relation.  No raw handle or generation is accepted.
    #[inline(always)]
    pub(crate) const fn s6c_substring(
        site_id: u32,
        occurrence: u32,
        lease_slot: u32,
    ) -> Result<Self, SourceBoundV9RuntimeProducerRejectV1> {
        if occurrence == 0 || lease_slot != 0 {
            return Err(SourceBoundV9RuntimeProducerRejectV1::PlanShape);
        }
        Ok(Self {
            provider: SourceBoundV9RuntimeProviderV1::S6CSubstringV1,
            site_id,
            occurrence,
            lease_slot,
        })
    }

    #[inline(always)]
    fn validate(self) -> Result<Self, SourceBoundV9RuntimeProducerRejectV1> {
        if self.occurrence == 0 || self.lease_slot != 0 {
            return Err(SourceBoundV9RuntimeProducerRejectV1::PlanShape);
        }
        Ok(self)
    }
}

/// Rejection at the provider-return boundary.  No result owner is exposed on
/// any error path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceBoundV9RuntimeProducerRejectV1 {
    PlanShape,
    Wire(DynamicV2WireSchemaRejectV1),
    NonNormalStatus(DynamicV2CallStatusV1),
    WrongNormalShape,
    ZeroHandle,
    ZeroLease,
    Lease(EndAuthorizedTextBorrowRejectV1),
}

/// Opaque borrow passed to the future TextRef scope.  It cannot expose the
/// handle/token or reconstruct a residence; the only operation is a closure
/// over validated live Text.
#[derive(Debug)]
pub(crate) struct SourceBoundV9RuntimeInputRefV1<'result> {
    result: &'result EndAuthorizedTextV1,
}

impl SourceBoundV9RuntimeInputRefV1<'_> {
    #[inline(always)]
    pub(crate) fn with_text<R>(
        &self,
        callback: impl FnOnce(&str) -> R,
    ) -> Result<R, EndAuthorizedTextBorrowRejectV1> {
        self.result.with_text(callback)
    }
}

/// Move-only runtime owner for one exact source-bound V9 occurrence.
///
/// There is deliberately no `Drop` cleanup or retry path.  The selected
/// consumer must call one of the two terminal methods exactly once.
#[must_use = "a source-bound V9 result must be finished or aborted"]
#[derive(Debug)]
pub(crate) struct SourceBoundV9RuntimeResultV1 {
    plan: SourceBoundV9RuntimeProducerPlanV1,
    result: Option<EndAuthorizedTextV1>,
}

impl SourceBoundV9RuntimeResultV1 {
    #[inline(always)]
    pub(crate) fn with_input<R>(
        &self,
        callback: impl FnOnce(SourceBoundV9RuntimeInputRefV1<'_>) -> R,
    ) -> Result<R, SourceBoundV9RuntimeProducerRejectV1> {
        let _plan = self.plan;
        let result = self
            .result
            .as_ref()
            .ok_or(SourceBoundV9RuntimeProducerRejectV1::Lease(
                EndAuthorizedTextBorrowRejectV1::UnknownOrAlreadyConsumed,
            ))?;
        Ok(callback(SourceBoundV9RuntimeInputRefV1 { result }))
    }

    /// Consume the End-authorized owner at the canonical End cutpoint.
    #[inline(always)]
    pub(crate) fn finish_at_canonical_end(
        mut self,
    ) -> Result<(), SourceBoundV9RuntimeProducerRejectV1> {
        let _plan = self.plan;
        let result = self
            .result
            .take()
            .ok_or(SourceBoundV9RuntimeProducerRejectV1::Lease(
                EndAuthorizedTextBorrowRejectV1::UnknownOrAlreadyConsumed,
            ))?;
        result
            .finish()
            .map_err(SourceBoundV9RuntimeProducerRejectV1::from)
    }

    /// Terminal cleanup for a consumer/provider failure.  This is explicit
    /// and move-only; it is not an implicit Drop or retry mechanism.
    #[inline(always)]
    pub(crate) fn abort_on_terminal_failure(
        self,
    ) -> Result<(), SourceBoundV9RuntimeProducerRejectV1> {
        self.finish_at_canonical_end()
    }

    #[cfg(test)]
    fn plan(&self) -> SourceBoundV9RuntimeProducerPlanV1 {
        self.plan
    }
}

impl From<LeaseConsumeRejectV1> for SourceBoundV9RuntimeProducerRejectV1 {
    #[inline(always)]
    fn from(value: LeaseConsumeRejectV1) -> Self {
        Self::Lease(match value {
            LeaseConsumeRejectV1::UnknownOrAlreadyConsumed => {
                EndAuthorizedTextBorrowRejectV1::UnknownOrAlreadyConsumed
            }
            LeaseConsumeRejectV1::TokenHandleMismatch => {
                EndAuthorizedTextBorrowRejectV1::TokenHandleMismatch
            }
            LeaseConsumeRejectV1::StaleHandleIdentity => {
                EndAuthorizedTextBorrowRejectV1::StaleHandleIdentity
            }
        })
    }
}

/// The only runtime producer boundary.  The provider call itself is outside
/// this function; its complete out-struct is passed immediately and is never
/// stored for deferred pairing.
#[inline(always)]
pub(crate) fn produce_source_bound_v9_runtime_v1(
    plan: SourceBoundV9RuntimeProducerPlanV1,
    out: DynamicV2CallOutV1,
) -> Result<SourceBoundV9RuntimeResultV1, SourceBoundV9RuntimeProducerRejectV1> {
    let plan = plan.validate()?;
    let status = out
        .validate_transport()
        .map_err(SourceBoundV9RuntimeProducerRejectV1::Wire)?;
    if status != DynamicV2CallStatusV1::Normal {
        return Err(SourceBoundV9RuntimeProducerRejectV1::NonNormalStatus(
            status,
        ));
    }
    if out.result_tag != DynamicV2WireTagV1::HostHandle as u32
        || out.disposition != DynamicV2CallDispositionV1::EndAuthorized as u32
        || out.forwarded_input != DYNAMIC_V2_FORWARDED_NONE_V1
        || out.continuation_token != 0
    {
        return Err(SourceBoundV9RuntimeProducerRejectV1::WrongNormalShape);
    }
    if out.value_payload == 0 {
        return Err(SourceBoundV9RuntimeProducerRejectV1::ZeroHandle);
    }
    if out.lease_token == 0 {
        return Err(SourceBoundV9RuntimeProducerRejectV1::ZeroLease);
    }
    let token =
        NonZeroU64::new(out.lease_token).ok_or(SourceBoundV9RuntimeProducerRejectV1::ZeroLease)?;
    let result = EndAuthorizedTextV1::adopt(out.value_payload, token)
        .map_err(SourceBoundV9RuntimeProducerRejectV1::Lease)?;
    Ok(SourceBoundV9RuntimeResultV1 {
        plan,
        result: Some(result),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{dynamic_v2_lease::publish_end_authorized_text, host_handles};

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

    fn plan() -> SourceBoundV9RuntimeProducerPlanV1 {
        SourceBoundV9RuntimeProducerPlanV1::s6c_substring(0, 9, 0).expect("plan")
    }

    #[test]
    fn producer_lends_text_and_finishes_once() {
        let published = publish_end_authorized_text("v9").expect("publish");
        let wire = normal_wire(published.handle(), published.token());
        let result = produce_source_bound_v9_runtime_v1(plan(), wire).expect("produce");
        assert_eq!(result.plan().site_id, 0);
        assert_eq!(result.plan().occurrence, 9);
        assert_eq!(
            result
                .with_input(|input| input.with_text(str::to_owned))
                .expect("borrow")
                .expect("text"),
            "v9"
        );
        assert_eq!(result.finish_at_canonical_end(), Ok(()));
    }

    #[test]
    fn producer_rejects_fault_without_owner() {
        let wire = DynamicV2CallOutV1 {
            status: DynamicV2CallStatusV1::Fault as u32,
            fault_code: 1,
            result_tag: DynamicV2WireTagV1::Invalid as u32,
            disposition: DynamicV2CallDispositionV1::None as u32,
            forwarded_input: DYNAMIC_V2_FORWARDED_NONE_V1,
            reserved: 0,
            value_payload: 0,
            lease_token: 0,
            continuation_token: 0,
        };
        assert!(matches!(
            produce_source_bound_v9_runtime_v1(plan(), wire),
            Err(SourceBoundV9RuntimeProducerRejectV1::NonNormalStatus(
                DynamicV2CallStatusV1::Fault
            ))
        ));
    }

    #[test]
    fn producer_rejects_wrong_shape_before_adoption() {
        let published = publish_end_authorized_text("wrong-shape").expect("publish");
        let mut wire = normal_wire(published.handle(), published.token());
        wire.result_tag = DynamicV2WireTagV1::ImmediateI64 as u32;
        assert!(matches!(
            produce_source_bound_v9_runtime_v1(plan(), wire),
            Err(SourceBoundV9RuntimeProducerRejectV1::Wire(_))
                | Err(SourceBoundV9RuntimeProducerRejectV1::WrongNormalShape)
        ));
        published.finish().expect("test cleanup");
    }

    #[test]
    fn producer_rejects_zero_handle_before_adoption() {
        let published = publish_end_authorized_text("zero-handle").expect("publish");
        assert!(matches!(
            produce_source_bound_v9_runtime_v1(plan(), normal_wire(0, published.token())),
            Err(SourceBoundV9RuntimeProducerRejectV1::ZeroHandle)
        ));
        published.finish().expect("test cleanup");
    }

    #[test]
    fn producer_rejects_foreign_and_stale_identity() {
        let first = publish_end_authorized_text("first").expect("first");
        let second = publish_end_authorized_text("second").expect("second");
        assert!(matches!(
            produce_source_bound_v9_runtime_v1(plan(), normal_wire(second.handle(), first.token()),),
            Err(SourceBoundV9RuntimeProducerRejectV1::Lease(
                EndAuthorizedTextBorrowRejectV1::TokenHandleMismatch
            ))
        ));
        first.finish().expect("first cleanup");
        second.finish().expect("second cleanup");

        let published = publish_end_authorized_text("stale").expect("stale");
        let old_handle = published.handle();
        let token = published.token();
        host_handles::drop_handle(old_handle);
        let replacement = host_handles::to_handle_text("replacement");
        assert!(matches!(
            produce_source_bound_v9_runtime_v1(plan(), normal_wire(old_handle, token)),
            Err(SourceBoundV9RuntimeProducerRejectV1::Lease(
                EndAuthorizedTextBorrowRejectV1::StaleHandleIdentity
            ))
        ));
        assert_eq!(
            published.finish(),
            Err(LeaseConsumeRejectV1::StaleHandleIdentity)
        );
        host_handles::drop_handle(replacement);
    }

    #[test]
    fn producer_adoption_is_one_shot_before_end() {
        let published = publish_end_authorized_text("one-shot").expect("publish");
        let handle = published.handle();
        let token = published.token();
        let first = produce_source_bound_v9_runtime_v1(plan(), normal_wire(handle, token))
            .expect("first adoption");
        assert!(matches!(
            produce_source_bound_v9_runtime_v1(plan(), normal_wire(handle, token)),
            Err(SourceBoundV9RuntimeProducerRejectV1::Lease(
                EndAuthorizedTextBorrowRejectV1::UnknownOrAlreadyConsumed
            ))
        ));
        first
            .abort_on_terminal_failure()
            .expect("abort first owner");
    }

    #[test]
    fn plan_rejects_non_s6c_lease_slot_shape() {
        assert_eq!(
            SourceBoundV9RuntimeProducerPlanV1::s6c_substring(0, 9, 1),
            Err(SourceBoundV9RuntimeProducerRejectV1::PlanShape)
        );
    }
}
