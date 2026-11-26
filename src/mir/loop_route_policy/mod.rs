//! Owned, caller-zero route-policy admission plus family observation for Loop
//! rows. See `README.md` for the authority boundary.

mod direct_accum_observation;
#[cfg(test)]
mod direct_accum_observation_tests;
mod loop_cond_break_continue_observation;
#[cfg(test)]
mod loop_cond_break_continue_observation_tests;
mod loop_true_break_continue_observation;
#[cfg(test)]
mod loop_true_break_continue_observation_tests;
mod policy;
mod policy_evidence;
mod schema;

pub(crate) use direct_accum_observation::{
    issue_direct_accum_family_observation_v1, DirectAccumFamilyObservationV1,
    DirectAccumObservationContextV1, DirectAccumObservationDeclineV1, DirectAccumObservationRejectV1,
    DirectAccumObservationUnresolvedV1, VerifiedDirectAccumFamilyCandidateV1,
};
#[cfg(test)]
pub(crate) use policy::direct_accum_route_admission_for_test;
pub(crate) use policy::{
    issue_direct_accum_route_admission_v1, VerifiedDirectAccumPolicyHandoffV1,
    VerifiedDirectAccumPolicyReceiptV1, VerifiedDirectAccumRouteAdmissionV1,
};
pub(crate) use policy_evidence::LoopRoutePolicySourceDeclineReasonV1;
pub(crate) use schema::CANONICAL_LOOP_ROUTE_ORDER_V1;
pub(crate) use loop_cond_break_continue_observation::LoopCondFamilyObservationV1;
pub(crate) use loop_cond_break_continue_observation::{
    issue_loop_cond_family_observation_v1, LoopCondObservationContextV1,
    LoopCondObservationDeclineV1, LoopCondObservationRejectV1, LoopCondObservationUnresolvedV1,
    VerifiedLoopCondFamilyCandidateV1,
};
pub(crate) use loop_true_break_continue_observation::LoopTrueFamilyObservationV1;
pub(crate) use loop_true_break_continue_observation::{
    issue_loop_true_family_observation_v1, LoopTrueObservationContextV1,
    LoopTrueObservationDeclineV1, LoopTrueObservationRejectV1, LoopTrueObservationUnresolvedV1,
    VerifiedLoopTrueFamilyCandidateV1,
};
