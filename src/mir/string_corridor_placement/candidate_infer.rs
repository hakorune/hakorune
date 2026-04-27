use super::plan_infer::infer_plan;
use super::types::{
    StringCorridorCandidate, StringCorridorCandidateKind, StringCorridorCandidatePlan,
    StringCorridorCandidateState, StringCorridorPublicationBoundary,
};
use crate::mir::string_corridor::{
    StringCorridorFact, StringCorridorOp, StringCorridorRole, StringPlacementFact,
    StringPublishReason, StringPublishReprPolicy,
};
use crate::mir::{BasicBlockId, MirFunction, ValueId};
use std::collections::HashMap;

fn annotate_publication_plan(
    plan: Option<StringCorridorCandidatePlan>,
) -> Option<StringCorridorCandidatePlan> {
    plan.map(|mut plan| {
        plan.publish_reason = Some(StringPublishReason::StableObjectDemand);
        plan.publish_repr_policy = Some(StringPublishReprPolicy::StableOwned);
        plan
    })
}

pub(super) fn infer_candidates(
    function: &MirFunction,
    value: ValueId,
    fact: &StringCorridorFact,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
) -> Vec<StringCorridorCandidate> {
    let mut out = Vec::new();
    let plan = infer_plan(function, value, fact, def_map);

    if fact.op == StringCorridorOp::StrSlice && fact.role == StringCorridorRole::BorrowProducer {
        out.push(StringCorridorCandidate {
            kind: StringCorridorCandidateKind::BorrowCorridorFusion,
            state: StringCorridorCandidateState::Candidate,
            reason: "borrow-producing slice value can stay inside a borrowed corridor",
            plan,
            publication_boundary: None,
        });
    }

    match fact.publish {
        StringPlacementFact::Sink => out.push(StringCorridorCandidate {
            kind: StringCorridorCandidateKind::PublicationSink,
            state: StringCorridorCandidateState::AlreadySatisfied,
            reason: "publish boundary is already sunk at the current corridor exit",
            plan: annotate_publication_plan(plan),
            publication_boundary: Some(StringCorridorPublicationBoundary::FirstExternalBoundary),
        }),
        StringPlacementFact::Unknown | StringPlacementFact::Deferred => {
            if fact.op == StringCorridorOp::StrSlice {
                out.push(StringCorridorCandidate {
                    kind: StringCorridorCandidateKind::PublicationSink,
                    state: StringCorridorCandidateState::Candidate,
                    reason:
                        "slice result may sink publication until an externally visible boundary",
                    plan: annotate_publication_plan(plan),
                    publication_boundary: Some(
                        StringCorridorPublicationBoundary::FirstExternalBoundary,
                    ),
                });
            }
        }
        StringPlacementFact::None => {}
    }

    match fact.materialize {
        StringPlacementFact::Sink => out.push(StringCorridorCandidate {
            kind: StringCorridorCandidateKind::MaterializationSink,
            state: StringCorridorCandidateState::AlreadySatisfied,
            reason: "materialization boundary is already a sink in the current facts",
            plan,
            publication_boundary: None,
        }),
        StringPlacementFact::Unknown | StringPlacementFact::Deferred => {
            if fact.op == StringCorridorOp::StrSlice {
                out.push(StringCorridorCandidate {
                    kind: StringCorridorCandidateKind::MaterializationSink,
                    state: StringCorridorCandidateState::Candidate,
                    reason: "slice result may defer materialization until a birth sink forces it",
                    plan,
                    publication_boundary: None,
                });
            }
        }
        StringPlacementFact::None => {}
    }

    if matches!(
        fact.op,
        StringCorridorOp::StrSlice | StringCorridorOp::StrLen
    ) {
        out.push(StringCorridorCandidate {
            kind: StringCorridorCandidateKind::DirectKernelEntry,
            state: StringCorridorCandidateState::Candidate,
            reason: direct_kernel_reason(fact),
            plan,
            publication_boundary: match fact.publish {
                StringPlacementFact::Sink
                | StringPlacementFact::Unknown
                | StringPlacementFact::Deferred => {
                    Some(StringCorridorPublicationBoundary::FirstExternalBoundary)
                }
                StringPlacementFact::None => None,
            },
        });
    }

    out
}

fn direct_kernel_reason(fact: &StringCorridorFact) -> &'static str {
    match fact.op {
        StringCorridorOp::StrLen => {
            "scalar string consumer can bypass ABI facade on the AOT-internal path"
        }
        StringCorridorOp::StrSlice => {
            "borrowed slice corridor can target a direct kernel entry before publication"
        }
        StringCorridorOp::FreezeStr => {
            "freeze sink is not part of the current direct-kernel-entry pilot"
        }
    }
}
