/*!
 * Backend-consumable string kernel plan seam.
 *
 * This module owns the thin derived view that MIR refresh now materializes
 * first-class. It is downstream of string corridor candidates and upstream of
 * JSON/shim transport. Placement remains the owner of candidate metadata
 * itself.
 */

use std::collections::BTreeMap;

use super::{
    build_value_def_map, resolve_value_origin,
    string_corridor_placement::{
        StringCorridorCandidate, StringCorridorCandidateKind, StringCorridorCandidateProof,
        StringCorridorCandidateState,
    },
    CompareOp, ConstValue, MirFunction, MirInstruction, MirModule, ValueDefMap, ValueId,
};

/// Backend-consumable family names derived from string corridor candidate plans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanFamily {
    BorrowedSliceWindow,
    ConcatTripletWindow,
}

impl std::fmt::Display for StringKernelPlanFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BorrowedSliceWindow => f.write_str("borrowed_slice_window"),
            Self::ConcatTripletWindow => f.write_str("concat_triplet_window"),
        }
    }
}

/// Current retained-form names exported to backend consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanRetainedForm {
    BorrowedText,
}

impl std::fmt::Display for StringKernelPlanRetainedForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BorrowedText => f.write_str("borrowed_text"),
        }
    }
}

/// Backend consumer role selected from current candidate families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanConsumer {
    DirectKernelEntry,
}

impl std::fmt::Display for StringKernelPlanConsumer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirectKernelEntry => f.write_str("direct_kernel_entry"),
        }
    }
}

/// Backend-consumable publication boundary for a string kernel plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanPublicationBoundary {
    FirstExternalBoundary,
}

impl std::fmt::Display for StringKernelPlanPublicationBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FirstExternalBoundary => f.write_str("first_external_boundary"),
        }
    }
}

/// Backend-consumable string kernel plan part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringKernelPlanPart {
    Slice {
        value: Option<ValueId>,
        source: ValueId,
        start: ValueId,
        end: ValueId,
    },
    Const {
        value: ValueId,
        known_length: Option<i64>,
        literal: Option<String>,
    },
}

/// Narrow scalar payload for the current substring-concat exact loop route.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringKernelPlanLoopPayload {
    pub seed_value: ValueId,
    pub seed_literal: String,
    pub seed_length: i64,
    pub loop_bound: i64,
    pub split_length: i64,
}

/// Thin legality facts that backend consumers may check before emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringKernelPlanLegality {
    pub byte_exact: bool,
    pub no_publish_inside: bool,
}

/// Thin backend-consumable kernel plan derived from the current candidate set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringKernelPlan {
    pub version: u32,
    pub family: StringKernelPlanFamily,
    pub corridor_root: ValueId,
    pub source_root: Option<ValueId>,
    pub known_length: Option<i64>,
    pub retained_form: StringKernelPlanRetainedForm,
    pub publication_boundary: Option<StringKernelPlanPublicationBoundary>,
    pub publication: Option<StringCorridorCandidateState>,
    pub materialization: Option<StringCorridorCandidateState>,
    pub direct_kernel_entry: Option<StringCorridorCandidateState>,
    pub consumer: Option<StringKernelPlanConsumer>,
    pub proof: StringCorridorCandidateProof,
    pub middle_literal: Option<String>,
    pub loop_payload: Option<StringKernelPlanLoopPayload>,
}

impl StringKernelPlan {
    pub fn parts(&self) -> Vec<StringKernelPlanPart> {
        match self.proof {
            StringCorridorCandidateProof::BorrowedSlice { source, start, end } => {
                vec![StringKernelPlanPart::Slice {
                    value: None,
                    source,
                    start,
                    end,
                }]
            }
            StringCorridorCandidateProof::ConcatTriplet {
                left_value,
                left_source,
                left_start,
                left_end,
                middle,
                right_value,
                right_source,
                right_start,
                right_end,
                shared_source: _,
            } => vec![
                StringKernelPlanPart::Slice {
                    value: left_value,
                    source: left_source,
                    start: left_start,
                    end: left_end,
                },
                StringKernelPlanPart::Const {
                    value: middle,
                    known_length: self.known_length,
                    literal: self.middle_literal.clone(),
                },
                StringKernelPlanPart::Slice {
                    value: right_value,
                    source: right_source,
                    start: right_start,
                    end: right_end,
                },
            ],
        }
    }

    pub fn legality(&self) -> StringKernelPlanLegality {
        StringKernelPlanLegality {
            byte_exact: true,
            no_publish_inside: self.publication.is_some(),
        }
    }
}

fn candidate_priority(kind: StringCorridorCandidateKind) -> u8 {
    match kind {
        StringCorridorCandidateKind::DirectKernelEntry => 0,
        StringCorridorCandidateKind::PublicationSink => 1,
        StringCorridorCandidateKind::MaterializationSink => 2,
        StringCorridorCandidateKind::BorrowCorridorFusion => 3,
    }
}

fn const_string_literal(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<(ValueId, String)> {
    let root = resolve_value_origin(function, def_map, value);
    let (bbid, idx) = def_map.get(&root).copied()?;
    match function.blocks.get(&bbid)?.instructions.get(idx)? {
        MirInstruction::Const {
            value: ConstValue::String(text),
            ..
        } => Some((root, text.clone())),
        _ => None,
    }
}

fn const_integer_literal(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<i64> {
    let root = resolve_value_origin(function, def_map, value);
    let (bbid, idx) = def_map.get(&root).copied()?;
    match function.blocks.get(&bbid)?.instructions.get(idx)? {
        MirInstruction::Const {
            value: ConstValue::Integer(actual),
            ..
        } => Some(*actual),
        _ => None,
    }
}

fn find_loop_bound_for_corridor(function: &MirFunction, corridor_root: ValueId) -> Option<i64> {
    let def_map = build_value_def_map(function);
    let root = resolve_value_origin(function, &def_map, corridor_root);
    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    if !matches!(block.instructions.get(idx)?, MirInstruction::Phi { .. }) {
        return None;
    }
    let branch_condition = match block.terminator.as_ref() {
        Some(MirInstruction::Branch { condition, .. }) => Some(*condition),
        _ => block
            .instructions
            .iter()
            .find_map(|candidate| match candidate {
                MirInstruction::Branch { condition, .. } => Some(*condition),
                _ => None,
            }),
    };
    block.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Compare {
            dst,
            op: CompareOp::Lt,
            lhs,
            rhs,
        } if branch_condition == Some(*dst) => const_integer_literal(function, &def_map, *lhs)
            .or_else(|| const_integer_literal(function, &def_map, *rhs)),
        _ => None,
    })
}

fn find_seed_input_for_corridor(
    function: &MirFunction,
    def_map: &ValueDefMap,
    corridor_root: ValueId,
) -> Option<(ValueId, String)> {
    let root = resolve_value_origin(function, def_map, corridor_root);
    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let inputs = match block.instructions.get(idx)? {
        MirInstruction::Phi { inputs, .. } => inputs,
        _ => return None,
    };
    inputs
        .iter()
        .find_map(|(_, value)| const_string_literal(function, def_map, *value))
}

fn derive_concat_triplet_loop_payload(
    function: &MirFunction,
    proof: &StringCorridorCandidateProof,
    corridor_root: ValueId,
) -> Option<StringKernelPlanLoopPayload> {
    let def_map = build_value_def_map(function);
    let (seed_value, seed_literal) =
        find_seed_input_for_corridor(function, &def_map, corridor_root)?;
    let seed_length = seed_literal.len() as i64;
    let loop_bound = find_loop_bound_for_corridor(function, corridor_root)?;
    let split_value = match proof {
        StringCorridorCandidateProof::ConcatTriplet {
            left_end,
            right_start,
            ..
        } if left_end == right_start => *left_end,
        _ => return None,
    };
    let split_root = resolve_value_origin(function, &def_map, split_value);
    let (bbid, idx) = def_map.get(&split_root).copied()?;
    let divisor = match function.blocks.get(&bbid)?.instructions.get(idx)? {
        MirInstruction::BinOp { lhs, rhs, .. } => const_integer_literal(function, &def_map, *rhs)
            .or_else(|| const_integer_literal(function, &def_map, *lhs)),
        _ => None,
    }?;
    if divisor <= 0 {
        return None;
    }
    let split_length = seed_length / divisor;
    if split_length <= 0 {
        return None;
    }
    Some(StringKernelPlanLoopPayload {
        seed_value,
        seed_literal,
        seed_length,
        loop_bound,
        split_length,
    })
}

/// Derive a backend-consumable string kernel plan from current candidate metadata.
pub fn derive_string_kernel_plan(
    function: &MirFunction,
    candidates: &[StringCorridorCandidate],
) -> Option<StringKernelPlan> {
    let mut representative: Option<StringCorridorCandidate> = None;
    let mut publication = None;
    let mut materialization = None;
    let mut direct_kernel_entry = None;
    let mut publication_boundary = None;

    for candidate in candidates {
        match candidate.kind {
            StringCorridorCandidateKind::PublicationSink => {
                publication = Some(candidate.state);
                if matches!(
                    candidate.publication_boundary,
                    Some(crate::mir::StringCorridorPublicationBoundary::FirstExternalBoundary)
                ) {
                    publication_boundary =
                        Some(StringKernelPlanPublicationBoundary::FirstExternalBoundary);
                }
            }
            StringCorridorCandidateKind::MaterializationSink => {
                materialization = Some(candidate.state)
            }
            StringCorridorCandidateKind::DirectKernelEntry => {
                direct_kernel_entry = Some(candidate.state)
            }
            StringCorridorCandidateKind::BorrowCorridorFusion => {}
        }

        let Some(plan) = candidate.plan else {
            continue;
        };
        representative = match representative {
            Some(current)
                if current.plan.is_some()
                    && candidate_priority(current.kind) <= candidate_priority(candidate.kind) =>
            {
                Some(current)
            }
            _ => Some(StringCorridorCandidate {
                kind: candidate.kind,
                state: candidate.state,
                reason: candidate.reason,
                plan: Some(plan),
                publication_boundary: candidate.publication_boundary,
            }),
        };
    }

    let representative = representative?;
    let plan = representative.plan?;
    let family = match plan.proof {
        StringCorridorCandidateProof::BorrowedSlice { .. } => {
            StringKernelPlanFamily::BorrowedSliceWindow
        }
        StringCorridorCandidateProof::ConcatTriplet { .. } => {
            StringKernelPlanFamily::ConcatTripletWindow
        }
    };

    let def_map = build_value_def_map(function);
    let middle_literal = match plan.proof {
        StringCorridorCandidateProof::ConcatTriplet { middle, .. } => {
            const_string_literal(function, &def_map, middle).map(|(_, text)| text)
        }
        _ => None,
    };
    let loop_payload = match plan.proof {
        StringCorridorCandidateProof::ConcatTriplet { .. } => derive_concat_triplet_loop_payload(
            function,
            &plan.proof,
            plan.source_root.unwrap_or(plan.corridor_root),
        ),
        _ => None,
    };

    Some(StringKernelPlan {
        version: 1,
        family,
        corridor_root: plan.corridor_root,
        source_root: plan.source_root,
        known_length: plan.known_length,
        retained_form: StringKernelPlanRetainedForm::BorrowedText,
        publication_boundary,
        publication,
        materialization,
        direct_kernel_entry,
        consumer: direct_kernel_entry.map(|_| StringKernelPlanConsumer::DirectKernelEntry),
        proof: plan.proof,
        middle_literal,
        loop_payload,
    })
}

pub fn refresh_module_string_kernel_plans(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_string_kernel_plans(function);
    }
}

pub fn refresh_function_string_kernel_plans(function: &mut MirFunction) {
    let mut plans = BTreeMap::new();
    for (corridor_root, candidates) in &function.metadata.string_corridor_candidates {
        if let Some(plan) = derive_string_kernel_plan(function, candidates) {
            plans.insert(*corridor_root, plan);
        }
    }
    function.metadata.string_kernel_plans = plans;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlock, BasicBlockId, BinaryOp, EffectMask, FunctionSignature, MirType,
        StringCorridorPublicationBoundary,
    };

    fn make_loop_function() -> MirFunction {
        let entry = BasicBlockId::new(0);
        let header = BasicBlockId::new(18);
        let body = BasicBlockId::new(19);
        let exit = BasicBlockId::new(21);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: Vec::new(),
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            entry,
        );

        function
            .blocks
            .get_mut(&entry)
            .unwrap()
            .instructions
            .extend([
                MirInstruction::Const {
                    dst: ValueId::new(3),
                    value: ConstValue::String("line-seed-abcdef".to_string()),
                },
                MirInstruction::Copy {
                    dst: ValueId::new(4),
                    src: ValueId::new(3),
                },
                MirInstruction::Const {
                    dst: ValueId::new(5),
                    value: ConstValue::Integer(16),
                },
            ]);

        let mut header_block = BasicBlock::new(header);
        header_block.instructions.extend([
            MirInstruction::Phi {
                dst: ValueId::new(15),
                inputs: vec![(entry, ValueId::new(12)), (body, ValueId::new(16))],
                type_hint: Some(MirType::Integer),
            },
            MirInstruction::Phi {
                dst: ValueId::new(21),
                inputs: vec![(entry, ValueId::new(4)), (body, ValueId::new(36))],
                type_hint: Some(MirType::String),
            },
            MirInstruction::Const {
                dst: ValueId::new(41),
                value: ConstValue::Integer(300000),
            },
            MirInstruction::Compare {
                dst: ValueId::new(37),
                op: CompareOp::Lt,
                lhs: ValueId::new(15),
                rhs: ValueId::new(41),
            },
            MirInstruction::Branch {
                condition: ValueId::new(37),
                then_bb: body,
                else_bb: exit,
                then_edge_args: None,
                else_edge_args: None,
            },
        ]);
        function.blocks.insert(header, header_block);

        let mut body_block = BasicBlock::new(body);
        body_block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(50),
                value: ConstValue::Integer(2),
            },
            MirInstruction::BinOp {
                dst: ValueId::new(47),
                op: BinaryOp::Div,
                lhs: ValueId::new(5),
                rhs: ValueId::new(50),
            },
            MirInstruction::Const {
                dst: ValueId::new(66),
                value: ConstValue::String("xx".to_string()),
            },
            MirInstruction::Copy {
                dst: ValueId::new(36),
                src: ValueId::new(21),
            },
        ]);
        function.blocks.insert(body, body_block);
        function.blocks.insert(exit, BasicBlock::new(exit));
        function
    }

    #[test]
    fn derive_string_kernel_plan_prefers_direct_entry_and_collects_barriers() {
        let function = make_loop_function();
        let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(7),
            source_root: Some(ValueId::new(1)),
            start: Some(ValueId::new(2)),
            end: Some(ValueId::new(3)),
            known_length: Some(2),
            proof: StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(4)),
                left_source: ValueId::new(1),
                left_start: ValueId::new(4),
                left_end: ValueId::new(5),
                middle: ValueId::new(6),
                right_value: Some(ValueId::new(8)),
                right_source: ValueId::new(1),
                right_start: ValueId::new(5),
                right_end: ValueId::new(9),
                shared_source: true,
            },
        };
        let candidates = vec![
            StringCorridorCandidate {
                kind: StringCorridorCandidateKind::PublicationSink,
                state: StringCorridorCandidateState::AlreadySatisfied,
                reason: "publish boundary is already sunk at the current corridor exit",
                plan: Some(plan),
                publication_boundary: Some(
                    StringCorridorPublicationBoundary::FirstExternalBoundary,
                ),
            },
            StringCorridorCandidate {
                kind: StringCorridorCandidateKind::MaterializationSink,
                state: StringCorridorCandidateState::Candidate,
                reason: "slice result may stay borrowed until a later boundary",
                plan: Some(plan),
                publication_boundary: None,
            },
            StringCorridorCandidate {
                kind: StringCorridorCandidateKind::DirectKernelEntry,
                state: StringCorridorCandidateState::Candidate,
                reason:
                    "borrowed slice corridor can target a direct kernel entry before publication",
                plan: Some(plan),
                publication_boundary: Some(
                    StringCorridorPublicationBoundary::FirstExternalBoundary,
                ),
            },
        ];

        let kernel_plan = derive_string_kernel_plan(&function, &candidates).expect("kernel plan");

        assert_eq!(kernel_plan.version, 1);
        assert_eq!(
            kernel_plan.family,
            StringKernelPlanFamily::ConcatTripletWindow
        );
        assert_eq!(kernel_plan.corridor_root, ValueId::new(7));
        assert_eq!(kernel_plan.source_root, Some(ValueId::new(1)));
        assert_eq!(kernel_plan.known_length, Some(2));
        assert_eq!(
            kernel_plan.retained_form,
            StringKernelPlanRetainedForm::BorrowedText
        );
        assert_eq!(
            kernel_plan.publication_boundary,
            Some(StringKernelPlanPublicationBoundary::FirstExternalBoundary)
        );
        assert_eq!(
            kernel_plan.publication,
            Some(StringCorridorCandidateState::AlreadySatisfied)
        );
        assert_eq!(
            kernel_plan.materialization,
            Some(StringCorridorCandidateState::Candidate)
        );
        assert_eq!(
            kernel_plan.direct_kernel_entry,
            Some(StringCorridorCandidateState::Candidate)
        );
        assert_eq!(
            kernel_plan.consumer,
            Some(StringKernelPlanConsumer::DirectKernelEntry)
        );
        let parts = kernel_plan.parts();
        assert_eq!(parts.len(), 3);
        assert_eq!(
            kernel_plan.legality(),
            StringKernelPlanLegality {
                byte_exact: true,
                no_publish_inside: true,
            }
        );
    }

    #[test]
    fn derive_string_kernel_plan_collects_concat_loop_payload() {
        let function = make_loop_function();
        let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(21),
            source_root: Some(ValueId::new(21)),
            start: Some(ValueId::new(71)),
            end: Some(ValueId::new(72)),
            known_length: Some(2),
            proof: StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(26)),
                left_source: ValueId::new(21),
                left_start: ValueId::new(46),
                left_end: ValueId::new(47),
                middle: ValueId::new(66),
                right_value: Some(ValueId::new(27)),
                right_source: ValueId::new(21),
                right_start: ValueId::new(47),
                right_end: ValueId::new(42),
                shared_source: true,
            },
        };
        let candidates = vec![StringCorridorCandidate {
            kind: StringCorridorCandidateKind::DirectKernelEntry,
            state: StringCorridorCandidateState::Candidate,
            reason: "direct kernel entry candidate",
            plan: Some(plan),
            publication_boundary: None,
        }];

        let kernel_plan = derive_string_kernel_plan(&function, &candidates).expect("kernel plan");
        let payload = kernel_plan.loop_payload.expect("loop payload");

        assert_eq!(payload.seed_value, ValueId::new(3));
        assert_eq!(payload.seed_literal, "line-seed-abcdef");
        assert_eq!(payload.seed_length, 16);
        assert_eq!(payload.loop_bound, 300000);
        assert_eq!(payload.split_length, 8);
        assert_eq!(kernel_plan.middle_literal.as_deref(), Some("xx"));
    }

    #[test]
    fn refresh_function_collects_string_kernel_plans() {
        let mut function = make_loop_function();
        let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(7),
            source_root: Some(ValueId::new(1)),
            start: Some(ValueId::new(2)),
            end: Some(ValueId::new(3)),
            known_length: Some(2),
            proof: StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(4)),
                left_source: ValueId::new(1),
                left_start: ValueId::new(4),
                left_end: ValueId::new(5),
                middle: ValueId::new(6),
                right_value: Some(ValueId::new(8)),
                right_source: ValueId::new(1),
                right_start: ValueId::new(5),
                right_end: ValueId::new(9),
                shared_source: true,
            },
        };
        function.metadata.string_corridor_candidates.insert(
            ValueId::new(8),
            vec![StringCorridorCandidate {
                kind: StringCorridorCandidateKind::DirectKernelEntry,
                state: StringCorridorCandidateState::Candidate,
                reason:
                    "borrowed slice corridor can target a direct kernel entry before publication",
                plan: Some(plan),
                publication_boundary: None,
            }],
        );

        refresh_function_string_kernel_plans(&mut function);

        let kernel_plans = &function.metadata.string_kernel_plans;
        let kernel_plan = kernel_plans.get(&ValueId::new(8)).expect("kernel plan");
        assert_eq!(kernel_plan.version, 1);
        assert_eq!(
            kernel_plan.family,
            StringKernelPlanFamily::ConcatTripletWindow
        );
        assert_eq!(
            kernel_plan.consumer,
            Some(StringKernelPlanConsumer::DirectKernelEntry)
        );
    }
}
