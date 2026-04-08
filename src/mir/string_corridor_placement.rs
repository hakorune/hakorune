/*!
 * String corridor placement/effect scaffold.
 *
 * This module consumes canonical string corridor facts and emits no-op candidate
 * decisions for future placement/effect rewrites. It does not mutate MIR or
 * change runtime behavior in this wave.
 */

use super::{
    string_corridor::{
        StringCorridorFact, StringCorridorOp, StringCorridorRole, StringPlacementFact,
    },
    MirFunction, MirModule,
};

/// Placement/effect decision kinds that later passes may act on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorCandidateKind {
    BorrowCorridorFusion,
    PublicationSink,
    MaterializationSink,
    DirectKernelEntry,
}

impl std::fmt::Display for StringCorridorCandidateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BorrowCorridorFusion => f.write_str("borrowed_corridor_fusion"),
            Self::PublicationSink => f.write_str("publication_sink"),
            Self::MaterializationSink => f.write_str("materialization_sink"),
            Self::DirectKernelEntry => f.write_str("direct_kernel_entry"),
        }
    }
}

/// Whether the candidate is a future transform or already satisfied by current MIR facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorCandidateState {
    Candidate,
    AlreadySatisfied,
}

impl std::fmt::Display for StringCorridorCandidateState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Candidate => f.write_str("candidate"),
            Self::AlreadySatisfied => f.write_str("already_satisfied"),
        }
    }
}

/// Inspection-only candidate record derived from current string corridor facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringCorridorCandidate {
    pub kind: StringCorridorCandidateKind,
    pub state: StringCorridorCandidateState,
    pub reason: &'static str,
}

impl StringCorridorCandidate {
    pub fn summary(&self) -> String {
        format!("{} [{}] {}", self.kind, self.state, self.reason)
    }
}

/// Refresh placement/effect candidates across the module without changing behavior.
pub fn refresh_module_string_corridor_candidates(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_string_corridor_candidates(function);
    }
}

/// Refresh a single function's placement/effect candidates from existing facts.
pub fn refresh_function_string_corridor_candidates(function: &mut MirFunction) {
    function.metadata.string_corridor_candidates.clear();

    for (value, fact) in &function.metadata.string_corridor_facts {
        let candidates = infer_candidates(fact);
        if !candidates.is_empty() {
            function
                .metadata
                .string_corridor_candidates
                .insert(*value, candidates);
        }
    }
}

fn infer_candidates(fact: &StringCorridorFact) -> Vec<StringCorridorCandidate> {
    let mut out = Vec::new();

    if fact.op == StringCorridorOp::StrSlice && fact.role == StringCorridorRole::BorrowProducer {
        out.push(StringCorridorCandidate {
            kind: StringCorridorCandidateKind::BorrowCorridorFusion,
            state: StringCorridorCandidateState::Candidate,
            reason: "borrow-producing slice value can stay inside a borrowed corridor",
        });
    }

    match fact.publish {
        StringPlacementFact::Sink => out.push(StringCorridorCandidate {
            kind: StringCorridorCandidateKind::PublicationSink,
            state: StringCorridorCandidateState::AlreadySatisfied,
            reason: "publish boundary is already sunk at the current corridor exit",
        }),
        StringPlacementFact::Unknown | StringPlacementFact::Deferred => {
            if fact.op == StringCorridorOp::StrSlice {
                out.push(StringCorridorCandidate {
                    kind: StringCorridorCandidateKind::PublicationSink,
                    state: StringCorridorCandidateState::Candidate,
                    reason:
                        "slice result may sink publication until an externally visible boundary",
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
        }),
        StringPlacementFact::Unknown | StringPlacementFact::Deferred => {
            if fact.op == StringCorridorOp::StrSlice {
                out.push(StringCorridorCandidate {
                    kind: StringCorridorCandidateKind::MaterializationSink,
                    state: StringCorridorCandidateState::Candidate,
                    reason: "slice result may defer materialization until a birth sink forces it",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType, ValueId};

    #[test]
    fn slice_fact_emits_borrowed_corridor_and_sink_candidates() {
        let fact = StringCorridorFact::str_slice(crate::mir::StringCorridorCarrier::MethodCall);
        let candidates = infer_candidates(&fact);

        assert!(candidates.iter().any(|candidate| {
            candidate.kind == StringCorridorCandidateKind::BorrowCorridorFusion
        }));
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
        let fact =
            StringCorridorFact::freeze_str(crate::mir::StringCorridorCarrier::CanonicalIntrinsic);
        let candidates = infer_candidates(&fact);

        assert!(candidates.iter().any(|candidate| {
            candidate.kind == StringCorridorCandidateKind::MaterializationSink
                && candidate.state == StringCorridorCandidateState::AlreadySatisfied
        }));
    }

    #[test]
    fn refresh_function_collects_candidates_from_existing_facts() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function.metadata.string_corridor_facts.insert(
            ValueId::new(1),
            StringCorridorFact::str_len(crate::mir::StringCorridorCarrier::MethodCall),
        );

        refresh_function_string_corridor_candidates(&mut function);

        let candidates = function
            .metadata
            .string_corridor_candidates
            .get(&ValueId::new(1))
            .expect("candidates");
        assert!(candidates
            .iter()
            .any(|candidate| { candidate.kind == StringCorridorCandidateKind::DirectKernelEntry }));
    }
}
