//! Policy-owned route admission for the singleton DirectAccum profile.
//!
//! The admission token is sealed directly from the source-side singleton
//! observation: the canonical family candidate was already selected upstream,
//! so no route schedule, raw cursor, or winner evaluation exists here. The
//! issuer performs no route/family dispatch.

use crate::mir::loop_structural_facts::VerifiedDirectAccumSingletonObservationV1;
use crate::mir::resolved_semantics::LoopExecutionFrameKeyV1;

/// Policy-owned admission brand for the DirectAccum profile.
///
/// The lowerer never inspects a route cursor. This brand is issued only from
/// the sealed source-side singleton observation.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedDirectAccumRouteAdmissionV1 {
    frame_key: LoopExecutionFrameKeyV1,
    _seal: DirectAccumRouteAdmissionSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct DirectAccumRouteAdmissionSealV1;

/// Typed evidence that the DirectAccum admission was issued by the policy
/// owner. The plan retains this receipt after the admission is consumed for
/// Recipe demand, so policy provenance is not silently discarded.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedDirectAccumPolicyReceiptV1 {
    frame_key: LoopExecutionFrameKeyV1,
    _seal: DirectAccumPolicyReceiptSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct DirectAccumPolicyReceiptSealV1;

/// One-shot handoff retaining the source/facts continuation after policy
/// admission. The physicalizer never sees a schedule or raw cursor.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedDirectAccumPolicyHandoffV1 {
    admission: VerifiedDirectAccumRouteAdmissionV1,
    observation: VerifiedDirectAccumSingletonObservationV1,
}

impl VerifiedDirectAccumRouteAdmissionV1 {
    pub(crate) fn frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame_key
    }

    /// Consume the admission; retains the policy receipt as provenance.
    pub(crate) fn into_receipt(self) -> VerifiedDirectAccumPolicyReceiptV1 {
        VerifiedDirectAccumPolicyReceiptV1 {
            frame_key: self.frame_key,
            _seal: DirectAccumPolicyReceiptSealV1,
        }
    }
}

#[cfg(test)]
pub(crate) fn direct_accum_route_admission_for_test(
    frame_key: LoopExecutionFrameKeyV1,
) -> VerifiedDirectAccumRouteAdmissionV1 {
    VerifiedDirectAccumRouteAdmissionV1 {
        frame_key,
        _seal: DirectAccumRouteAdmissionSealV1,
    }
}

impl VerifiedDirectAccumPolicyReceiptV1 {
    pub(crate) fn frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame_key
    }
}

impl VerifiedDirectAccumPolicyHandoffV1 {
    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedDirectAccumRouteAdmissionV1,
        VerifiedDirectAccumSingletonObservationV1,
    ) {
        (self.admission, self.observation)
    }
}

/// Consumes the source-side singleton proof and seals the policy-owned
/// admission for the same frame. No caller can inject a route id or a raw
/// cursor here.
pub(crate) fn issue_direct_accum_route_admission_v1(
    observation: VerifiedDirectAccumSingletonObservationV1,
) -> VerifiedDirectAccumPolicyHandoffV1 {
    VerifiedDirectAccumPolicyHandoffV1 {
        admission: VerifiedDirectAccumRouteAdmissionV1 {
            frame_key: observation.frame_key(),
            _seal: DirectAccumRouteAdmissionSealV1,
        },
        observation,
    }
}
