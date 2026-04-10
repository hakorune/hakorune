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
    string_corridor_recognizer::{
        build_def_map, const_string_length, match_concat_triplet, match_len_call,
        match_substring_call, match_substring_call_shape, match_substring_concat3_helper_call,
        resolve_copy_chain_source, string_source_identity, ConcatTripletShape,
        StringSourceIdentity,
    },
    string_corridor_relation::{StringCorridorRelation, StringCorridorRelationKind},
    BasicBlockId, MirFunction, MirModule, ValueId,
};
use std::collections::HashMap;

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

/// Proof-bearing plan metadata for broader string corridor routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringCorridorCandidatePlan {
    /// The borrowed carrier value that this plan is about.
    pub corridor_root: ValueId,
    /// Shared source root when the corridor proves a single underlying source.
    pub source_root: Option<ValueId>,
    /// Outer consumer window when the candidate is itself a substring consumer.
    pub start: Option<ValueId>,
    pub end: Option<ValueId>,
    /// Known constant length contribution already proven in the corridor.
    pub known_length: Option<i64>,
    /// Shape proof that explains why this value is on the corridor.
    pub proof: StringCorridorCandidateProof,
}

impl StringCorridorCandidatePlan {
    pub fn summary(&self) -> String {
        let source = self
            .source_root
            .map(|value| format!("%{}", value.0))
            .unwrap_or_else(|| "-".to_string());
        let outer_window = match (self.start, self.end) {
            (Some(start), Some(end)) => format!("[%{}, %{}]", start.0, end.0),
            _ => "-".to_string(),
        };
        let known_len = self
            .known_length
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string());
        format!(
            "plan(root=%{} source={} outer={} known_len={} proof={})",
            self.corridor_root.0,
            source,
            outer_window,
            known_len,
            self.proof.summary()
        )
    }
}

/// Proof payload attached to a string corridor candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorCandidateProof {
    BorrowedSlice {
        source: ValueId,
        start: ValueId,
        end: ValueId,
    },
    ConcatTriplet {
        left_source: ValueId,
        left_start: ValueId,
        left_end: ValueId,
        middle: ValueId,
        right_source: ValueId,
        right_start: ValueId,
        right_end: ValueId,
        shared_source: bool,
    },
}

impl StringCorridorCandidateProof {
    pub fn summary(&self) -> String {
        match self {
            Self::BorrowedSlice { source, start, end } => format!(
                "borrowed_slice(src=%{} start=%{} end=%{})",
                source.0, start.0, end.0
            ),
            Self::ConcatTriplet {
                left_source,
                left_start,
                left_end,
                middle,
                right_source,
                right_start,
                right_end,
                shared_source,
            } => format!(
                "concat_triplet(shared_source={} left=%{}[%{},%{}] middle=%{} right=%{}[%{},%{}])",
                shared_source,
                left_source.0,
                left_start.0,
                left_end.0,
                middle.0,
                right_source.0,
                right_start.0,
                right_end.0
            ),
        }
    }
}

/// Inspection-only candidate record derived from current string corridor facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringCorridorCandidate {
    pub kind: StringCorridorCandidateKind,
    pub state: StringCorridorCandidateState,
    pub reason: &'static str,
    pub plan: Option<StringCorridorCandidatePlan>,
}

impl StringCorridorCandidate {
    pub fn summary(&self) -> String {
        match self.plan {
            Some(plan) => format!(
                "{} [{}] {} | {}",
                self.kind,
                self.state,
                self.reason,
                plan.summary()
            ),
            None => format!("{} [{}] {}", self.kind, self.state, self.reason),
        }
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
    let def_map = build_def_map(function);

    for (value, fact) in &function.metadata.string_corridor_facts {
        let candidates = infer_candidates(function, *value, fact, &def_map);
        if !candidates.is_empty() {
            function
                .metadata
                .string_corridor_candidates
                .insert(*value, candidates);
        }
    }

    refresh_function_string_corridor_relation_candidates(function, &def_map);
}

fn refresh_function_string_corridor_relation_candidates(
    function: &mut MirFunction,
    _def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
) {
    let mut phi_updates: Vec<(ValueId, Vec<StringCorridorCandidate>)> = Vec::new();

    for (value, relations) in &function.metadata.string_corridor_relations {
        let carried_candidates = carried_candidates_from_relations(function, relations);
        if !carried_candidates.is_empty() {
            phi_updates.push((*value, carried_candidates));
        }
    }

    for (value, candidates) in phi_updates {
        function
            .metadata
            .string_corridor_candidates
            .entry(value)
            .or_default()
            .extend(candidates);
    }
}

fn carried_candidates_from_relations(
    function: &MirFunction,
    relations: &[StringCorridorRelation],
) -> Vec<StringCorridorCandidate> {
    let mut out = Vec::new();
    for relation in relations {
        if relation.kind != StringCorridorRelationKind::PhiCarryBase {
            continue;
        }
        let Some(base_candidates) = base_phi_carry_candidates(function, relation.base_value) else {
            continue;
        };
        out.extend(
            base_candidates
                .into_iter()
                .map(|candidate| StringCorridorCandidate {
                    kind: candidate.kind,
                    state: candidate.state,
                    reason: relation.reason,
                    plan: if relation.carries_plan_window {
                        candidate.plan
                    } else {
                        None
                    },
                }),
        );
    }
    out
}

fn base_phi_carry_candidates(
    function: &MirFunction,
    value: ValueId,
) -> Option<Vec<StringCorridorCandidate>> {
    let candidates = function.metadata.string_corridor_candidates.get(&value)?;
    let carried: Vec<_> = candidates
        .iter()
        .filter(|candidate| {
            matches!(
                candidate.kind,
                StringCorridorCandidateKind::PublicationSink
                    | StringCorridorCandidateKind::MaterializationSink
                    | StringCorridorCandidateKind::DirectKernelEntry
            ) && matches!(
                candidate.plan.map(|plan| plan.proof),
                Some(StringCorridorCandidateProof::ConcatTriplet { .. })
            )
        })
        .copied()
        .collect();
    if carried.is_empty() {
        None
    } else {
        Some(carried)
    }
}

fn infer_borrowed_slice_plan(
    function: &MirFunction,
    value: ValueId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
) -> Option<StringCorridorCandidatePlan> {
    let root = resolve_copy_chain_source(function, def_map, value);
    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let (_, receiver, start, end, _) = match_substring_call(block.instructions.get(idx)?)?;
    let source = resolve_copy_chain_source(function, def_map, receiver);
    let start = resolve_copy_chain_source(function, def_map, start);
    let end = resolve_copy_chain_source(function, def_map, end);
    Some(StringCorridorCandidatePlan {
        corridor_root: root,
        source_root: Some(source),
        start: Some(start),
        end: Some(end),
        known_length: None,
        proof: StringCorridorCandidateProof::BorrowedSlice { source, start, end },
    })
}

fn shared_source_root(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    lhs_source: ValueId,
    rhs_source: ValueId,
) -> (bool, Option<ValueId>) {
    let lhs_identity = string_source_identity(function, def_map, lhs_source);
    let rhs_identity = string_source_identity(function, def_map, rhs_source);
    match (lhs_identity, rhs_identity) {
        (Some(StringSourceIdentity::Value(lhs)), Some(StringSourceIdentity::Value(rhs)))
            if lhs == rhs =>
        {
            (true, Some(lhs))
        }
        (
            Some(StringSourceIdentity::ConstString(lhs)),
            Some(StringSourceIdentity::ConstString(rhs)),
        ) if lhs == rhs => (true, None),
        _ => (false, None),
    }
}

fn infer_concat_triplet_plan(
    function: &MirFunction,
    bbid: BasicBlockId,
    receiver: ValueId,
    outer_start: Option<ValueId>,
    outer_end: Option<ValueId>,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    require_shared_source: bool,
) -> Option<StringCorridorCandidatePlan> {
    let receiver_root = resolve_copy_chain_source(function, def_map, receiver);
    let ConcatTripletShape {
        left,
        middle,
        right,
    } = match_concat_triplet(function, bbid, def_map, receiver_root)?;
    let Some(StringSourceIdentity::ConstString(text)) =
        string_source_identity(function, def_map, middle)
    else {
        return None;
    };
    let left = match_substring_call_shape(function, def_map, left)?;
    let right = match_substring_call_shape(function, def_map, right)?;
    let (shared_source, source_root) =
        shared_source_root(function, def_map, left.source, right.source);
    if require_shared_source && !shared_source {
        return None;
    }

    Some(StringCorridorCandidatePlan {
        corridor_root: receiver_root,
        source_root,
        start: outer_start.map(|value| resolve_copy_chain_source(function, def_map, value)),
        end: outer_end.map(|value| resolve_copy_chain_source(function, def_map, value)),
        known_length: Some(const_string_length(&text)),
        proof: StringCorridorCandidateProof::ConcatTriplet {
            left_source: left.source,
            left_start: left.start,
            left_end: left.end,
            middle,
            right_source: right.source,
            right_start: right.start,
            right_end: right.end,
            shared_source,
        },
    })
}

fn infer_concat_triplet_result_plan(
    function: &MirFunction,
    value: ValueId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
) -> Option<StringCorridorCandidatePlan> {
    let root = resolve_copy_chain_source(function, def_map, value);
    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let helper = match_substring_concat3_helper_call(block.instructions.get(idx)?)?;
    let (left, middle, right, start, end) = (
        helper.left,
        helper.middle,
        helper.right,
        helper.start,
        helper.end,
    );
    let Some(StringSourceIdentity::ConstString(text)) =
        string_source_identity(function, def_map, middle)
    else {
        return None;
    };
    let left = match_substring_call_shape(function, def_map, left)?;
    let right = match_substring_call_shape(function, def_map, right)?;
    let (shared_source, source_root) =
        shared_source_root(function, def_map, left.source, right.source);

    Some(StringCorridorCandidatePlan {
        corridor_root: root,
        source_root,
        start: Some(resolve_copy_chain_source(function, def_map, start)),
        end: Some(resolve_copy_chain_source(function, def_map, end)),
        known_length: Some(const_string_length(&text)),
        proof: StringCorridorCandidateProof::ConcatTriplet {
            left_source: left.source,
            left_start: left.start,
            left_end: left.end,
            middle: resolve_copy_chain_source(function, def_map, middle),
            right_source: right.source,
            right_start: right.start,
            right_end: right.end,
            shared_source,
        },
    })
}

fn infer_plan(
    function: &MirFunction,
    value: ValueId,
    fact: &StringCorridorFact,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
) -> Option<StringCorridorCandidatePlan> {
    let root = resolve_copy_chain_source(function, def_map, value);
    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let inst = block.instructions.get(idx)?;

    match fact.op {
        StringCorridorOp::StrSlice => {
            if let Some((_, receiver, start, end, _)) = match_substring_call(inst) {
                infer_concat_triplet_plan(
                    function,
                    bbid,
                    receiver,
                    Some(start),
                    Some(end),
                    def_map,
                    true,
                )
                .or_else(|| infer_borrowed_slice_plan(function, value, def_map))
            } else {
                infer_concat_triplet_result_plan(function, value, def_map)
            }
        }
        StringCorridorOp::StrLen => {
            let (_, receiver, _) = match_len_call(inst)?;
            infer_concat_triplet_plan(function, bbid, receiver, None, None, def_map, false).or_else(
                || {
                    infer_borrowed_slice_plan(function, receiver, def_map).map(|plan| {
                        StringCorridorCandidatePlan {
                            corridor_root: plan.corridor_root,
                            source_root: plan.source_root,
                            start: plan.start,
                            end: plan.end,
                            known_length: plan.known_length,
                            proof: plan.proof,
                        }
                    })
                },
            )
        }
        StringCorridorOp::FreezeStr => None,
    }
}

fn infer_candidates(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlock, BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, FunctionSignature,
        MirInstruction, MirType, ValueId,
    };

    #[test]
    fn slice_fact_emits_borrowed_corridor_and_sink_candidates() {
        let fact = StringCorridorFact::str_slice(crate::mir::StringCorridorCarrier::MethodCall);
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let function = MirFunction::new(signature, BasicBlockId::new(0));
        let def_map = build_def_map(&function);
        let candidates = infer_candidates(&function, ValueId::new(1), &fact, &def_map);

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
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let function = MirFunction::new(signature, BasicBlockId::new(0));
        let def_map = build_def_map(&function);
        let candidates = infer_candidates(&function, ValueId::new(1), &fact, &def_map);

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

        crate::mir::refresh_function_string_corridor_relations(&mut function);
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

    #[test]
    fn refresh_function_attaches_plan_metadata_for_concat_corridor_candidates() {
        use crate::ast::Span;

        fn method_call(
            dst: ValueId,
            receiver: ValueId,
            box_name: &str,
            method: &str,
            args: Vec<ValueId>,
        ) -> MirInstruction {
            MirInstruction::Call {
                dst: Some(dst),
                func: ValueId::INVALID,
                callee: Some(Callee::Method {
                    box_name: box_name.to_string(),
                    method: method.to_string(),
                    receiver: Some(receiver),
                    certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                    box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
                }),
                args,
                effects: EffectMask::PURE,
            }
        }

        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(1),
            ValueId(0),
            "StringBox",
            "length",
            vec![],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(3),
            op: BinaryOp::Div,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(4),
            value: ConstValue::Integer(0),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(5),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(4), ValueId(3)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(6),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(3), ValueId(1)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(7),
            value: ConstValue::String("xx".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(8),
            op: BinaryOp::Add,
            lhs: ValueId(5),
            rhs: ValueId(7),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(9),
            op: BinaryOp::Add,
            lhs: ValueId(8),
            rhs: ValueId(6),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(10),
            ValueId(9),
            "RuntimeDataBox",
            "length",
            vec![],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(11),
            value: ConstValue::Integer(1),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(12),
            op: BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(11),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(13),
            ValueId(9),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(11), ValueId(12)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(10)),
        });

        crate::mir::refresh_function_string_corridor_facts(&mut function);
        crate::mir::refresh_function_string_corridor_relations(&mut function);
        refresh_function_string_corridor_candidates(&mut function);

        let len_candidates = function
            .metadata
            .string_corridor_candidates
            .get(&ValueId(10))
            .expect("len candidates");
        let len_direct = len_candidates
            .iter()
            .find(|candidate| candidate.kind == StringCorridorCandidateKind::DirectKernelEntry)
            .expect("direct kernel candidate");
        let len_plan = len_direct.plan.expect("plan metadata on len candidate");
        assert_eq!(len_plan.corridor_root, ValueId(9));
        assert_eq!(len_plan.source_root, Some(ValueId(0)));
        assert_eq!(len_plan.known_length, Some(2));
        assert_eq!(len_plan.start, None);
        assert_eq!(len_plan.end, None);
        assert!(matches!(
            len_plan.proof,
            StringCorridorCandidateProof::ConcatTriplet {
                left_source: ValueId(0),
                left_start: ValueId(4),
                left_end: ValueId(3),
                middle: ValueId(7),
                right_source: ValueId(0),
                right_start: ValueId(3),
                right_end: ValueId(1),
                shared_source: true,
            }
        ));

        let substring_candidates = function
            .metadata
            .string_corridor_candidates
            .get(&ValueId(13))
            .expect("substring candidates");
        let publication = substring_candidates
            .iter()
            .find(|candidate| candidate.kind == StringCorridorCandidateKind::PublicationSink)
            .expect("publication candidate");
        let substring_plan = publication
            .plan
            .expect("plan metadata on substring candidate");
        assert_eq!(substring_plan.corridor_root, ValueId(9));
        assert_eq!(substring_plan.source_root, Some(ValueId(0)));
        assert_eq!(substring_plan.start, Some(ValueId(11)));
        assert_eq!(substring_plan.end, Some(ValueId(12)));
        assert_eq!(substring_plan.known_length, Some(2));
    }

    #[test]
    fn runtime_export_substring_concat_keeps_publication_sink_candidate() {
        use crate::ast::Span;

        fn method_call(
            dst: ValueId,
            receiver: ValueId,
            box_name: &str,
            method: &str,
            args: Vec<ValueId>,
        ) -> MirInstruction {
            MirInstruction::Call {
                dst: Some(dst),
                func: ValueId::INVALID,
                callee: Some(Callee::Method {
                    box_name: box_name.to_string(),
                    method: method.to_string(),
                    receiver: Some(receiver),
                    certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                    box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
                }),
                args,
                effects: EffectMask::PURE,
            }
        }

        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Box("RuntimeDataBox".to_string()),
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(1),
            ValueId(0),
            "RuntimeDataBox",
            "length",
            vec![],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(3),
            op: BinaryOp::Div,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(4),
            value: ConstValue::Integer(0),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(5),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(4), ValueId(3)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(6),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(3), ValueId(1)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(7),
            value: ConstValue::String("xx".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(8),
            value: ConstValue::Integer(1),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(9),
            op: BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(8),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern(
                "nyash.string.substring_concat3_hhhii".to_string(),
            )),
            args: vec![ValueId(5), ValueId(7), ValueId(6), ValueId(8), ValueId(9)],
            effects: EffectMask::PURE,
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(10)),
        });

        crate::mir::refresh_function_string_corridor_facts(&mut function);
        crate::mir::refresh_function_string_corridor_relations(&mut function);
        refresh_function_string_corridor_candidates(&mut function);

        let candidates = function
            .metadata
            .string_corridor_candidates
            .get(&ValueId(10))
            .expect("substring concat result candidates");
        let publication = candidates
            .iter()
            .find(|candidate| candidate.kind == StringCorridorCandidateKind::PublicationSink)
            .expect("publication sink candidate");
        let plan = publication.plan.expect("plan metadata on helper result");
        assert_eq!(plan.corridor_root, ValueId(10));
        assert_eq!(plan.source_root, Some(ValueId(0)));
        assert_eq!(plan.start, Some(ValueId(8)));
        assert_eq!(plan.end, Some(ValueId(9)));
        assert_eq!(plan.known_length, Some(2));
        assert!(matches!(
            plan.proof,
            StringCorridorCandidateProof::ConcatTriplet {
                left_source: ValueId(0),
                left_start: ValueId(4),
                left_end: ValueId(3),
                middle: ValueId(7),
                right_source: ValueId(0),
                right_start: ValueId(3),
                right_end: ValueId(1),
                shared_source: true,
            }
        ));
    }

    #[test]
    fn refresh_function_carries_corridor_candidates_across_narrow_phi_route() {
        use crate::ast::Span;

        fn method_call(
            dst: ValueId,
            receiver: ValueId,
            box_name: &str,
            method: &str,
            args: Vec<ValueId>,
        ) -> MirInstruction {
            MirInstruction::Call {
                dst: Some(dst),
                func: ValueId::INVALID,
                callee: Some(Callee::Method {
                    box_name: box_name.to_string(),
                    method: method.to_string(),
                    receiver: Some(receiver),
                    certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                    box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
                }),
                args,
                effects: EffectMask::PURE,
            }
        }

        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        function.add_block(BasicBlock::new(BasicBlockId(1)));
        function.add_block(BasicBlock::new(BasicBlockId(2)));
        function.add_block(BasicBlock::new(BasicBlockId(3)));
        function.add_block(BasicBlock::new(BasicBlockId(4)));

        let entry = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
        entry.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });

        let header = function.blocks.get_mut(&BasicBlockId(1)).expect("header");
        header.instructions.push(MirInstruction::Phi {
            dst: ValueId(21),
            inputs: vec![
                (BasicBlockId(0), ValueId(0)),
                (BasicBlockId(3), ValueId(22)),
            ],
            type_hint: Some(MirType::Box("RuntimeDataBox".to_string())),
        });
        header.instruction_spans.push(Span::unknown());
        header.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(2),
            edge_args: None,
        });

        let body = function.blocks.get_mut(&BasicBlockId(2)).expect("body");
        body.instructions.push(MirInstruction::Const {
            dst: ValueId(46),
            value: ConstValue::Integer(0),
        });
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(MirInstruction::Const {
            dst: ValueId(47),
            value: ConstValue::Integer(1),
        });
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(MirInstruction::Const {
            dst: ValueId(48),
            value: ConstValue::Integer(2),
        });
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(method_call(
            ValueId(26),
            ValueId(21),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(46), ValueId(47)],
        ));
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(method_call(
            ValueId(27),
            ValueId(21),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(47), ValueId(48)],
        ));
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(MirInstruction::Const {
            dst: ValueId(66),
            value: ConstValue::String("xx".to_string()),
        });
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(MirInstruction::Const {
            dst: ValueId(71),
            value: ConstValue::Integer(1),
        });
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(MirInstruction::Const {
            dst: ValueId(72),
            value: ConstValue::Integer(3),
        });
        body.instruction_spans.push(Span::unknown());
        body.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(36)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern(
                "nyash.string.substring_concat3_hhhii".to_string(),
            )),
            args: vec![
                ValueId(26),
                ValueId(66),
                ValueId(27),
                ValueId(71),
                ValueId(72),
            ],
            effects: EffectMask::PURE,
        });
        body.instruction_spans.push(Span::unknown());
        body.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(3),
            edge_args: None,
        });

        let latch = function.blocks.get_mut(&BasicBlockId(3)).expect("latch");
        latch.instructions.push(MirInstruction::Phi {
            dst: ValueId(22),
            inputs: vec![(BasicBlockId(2), ValueId(36))],
            type_hint: Some(MirType::Box("RuntimeDataBox".to_string())),
        });
        latch.instruction_spans.push(Span::unknown());
        latch.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });

        let exit = function.blocks.get_mut(&BasicBlockId(4)).expect("exit");
        exit.set_terminator(MirInstruction::Return { value: None });

        crate::mir::refresh_function_string_corridor_facts(&mut function);
        crate::mir::refresh_function_string_corridor_relations(&mut function);
        refresh_function_string_corridor_candidates(&mut function);

        let helper = function
            .metadata
            .string_corridor_candidates
            .get(&ValueId(36))
            .expect("helper candidates");
        assert!(helper.iter().any(|candidate| {
            candidate.kind == StringCorridorCandidateKind::DirectKernelEntry
                && candidate.plan.is_some()
        }));

        let latch_candidates = function
            .metadata
            .string_corridor_candidates
            .get(&ValueId(22))
            .expect("phi %22 candidates");
        assert!(latch_candidates.iter().any(|candidate| {
            candidate.kind == StringCorridorCandidateKind::DirectKernelEntry
                && candidate.plan.is_some()
        }));
        assert!(latch_candidates.iter().any(|candidate| {
            candidate.kind == StringCorridorCandidateKind::PublicationSink
                && candidate.plan.is_some()
        }));
        assert!(latch_candidates.iter().any(|candidate| {
            candidate.kind == StringCorridorCandidateKind::MaterializationSink
                && candidate.plan.is_some()
        }));
        assert!(!latch_candidates.iter().any(|candidate| {
            candidate.kind == StringCorridorCandidateKind::BorrowCorridorFusion
        }));

        let header_candidates = function
            .metadata
            .string_corridor_candidates
            .get(&ValueId(21))
            .expect("phi %21 candidates");
        assert!(header_candidates
            .iter()
            .all(|candidate| candidate.plan.is_none()));
        assert!(header_candidates
            .iter()
            .any(|candidate| { candidate.kind == StringCorridorCandidateKind::PublicationSink }));
        assert!(header_candidates.iter().any(|candidate| {
            candidate.kind == StringCorridorCandidateKind::MaterializationSink
        }));
        assert!(header_candidates
            .iter()
            .any(|candidate| { candidate.kind == StringCorridorCandidateKind::DirectKernelEntry }));
        assert!(!header_candidates.iter().any(|candidate| {
            candidate.kind == StringCorridorCandidateKind::BorrowCorridorFusion
        }));
    }
}
