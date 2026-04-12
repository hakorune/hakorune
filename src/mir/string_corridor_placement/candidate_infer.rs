use super::plan_infer::infer_plan;
use super::*;

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
        });
    }

    match fact.publish {
        StringPlacementFact::Sink => out.push(StringCorridorCandidate {
            kind: StringCorridorCandidateKind::PublicationSink,
            state: StringCorridorCandidateState::AlreadySatisfied,
            reason: "publish boundary is already sunk at the current corridor exit",
            plan,
        }),
        StringPlacementFact::Unknown | StringPlacementFact::Deferred => {
            if fact.op == StringCorridorOp::StrSlice {
                out.push(StringCorridorCandidate {
                    kind: StringCorridorCandidateKind::PublicationSink,
                    state: StringCorridorCandidateState::Candidate,
                    reason:
                        "slice result may sink publication until an externally visible boundary",
                    plan,
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
        }),
        StringPlacementFact::Unknown | StringPlacementFact::Deferred => {
            if fact.op == StringCorridorOp::StrSlice {
                out.push(StringCorridorCandidate {
                    kind: StringCorridorCandidateKind::MaterializationSink,
                    state: StringCorridorCandidateState::Candidate,
                    reason: "slice result may defer materialization until a birth sink forces it",
                    plan,
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
