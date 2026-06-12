use super::common::*;

#[test]
fn slice_fact_emits_borrowed_corridor_and_sink_candidates() {
    let fact = StringCorridorFact::str_slice(StringCorridorCarrier::MethodCall);
    let function = make_function(MirType::Void);
    let def_map = build_value_def_map(&function);
    let candidates = infer_candidates(&function, ValueId::new(1), &fact, &def_map);

    assert!(candidates
        .iter()
        .any(|candidate| { candidate.kind == StringCorridorCandidateKind::BorrowCorridorFusion }));
    assert!(candidates.iter().any(|candidate| {
        candidate.kind == StringCorridorCandidateKind::PublicationSink
            && candidate.state == StringCorridorCandidateState::Candidate
    }));
    assert!(candidates.iter().any(|candidate| {
        candidate.kind == StringCorridorCandidateKind::MaterializationSink
            && candidate.state == StringCorridorCandidateState::Candidate
    }));
}

#[test]
fn freeze_fact_marks_materialization_sink_as_already_satisfied() {
    let fact = StringCorridorFact::freeze_str(StringCorridorCarrier::CanonicalIntrinsic);
    let function = make_function(MirType::Void);
    let def_map = build_value_def_map(&function);
    let candidates = infer_candidates(&function, ValueId::new(1), &fact, &def_map);

    assert!(candidates.iter().any(|candidate| {
        candidate.kind == StringCorridorCandidateKind::MaterializationSink
            && candidate.state == StringCorridorCandidateState::AlreadySatisfied
    }));
}
