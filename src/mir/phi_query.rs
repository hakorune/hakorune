/*!
 * Generic PHI base-relation queries.
 *
 * This module is the MIR-side SSOT for answering a narrow question:
 * whether a PHI continues the same normalized base value under a generic
 * anchor set. It does not know about string corridors or any
 * placement/effect policy.
 */

use super::value_origin::{build_value_def_map, resolve_value_origin, ParentMap, ValueDefMap};
use super::{MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PhiBaseRelation {
    SameBase(ValueId),
    Mixed,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PhiCarryRelation {
    pub phi_value: ValueId,
    pub relation: PhiBaseRelation,
    pub window_safe: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PhiBaseQueryResult {
    pub relation: PhiBaseRelation,
    pub window_safe: bool,
}

pub(crate) fn collect_phi_carry_relations(
    function: &MirFunction,
    anchors: &BTreeSet<ValueId>,
) -> Vec<PhiCarryRelation> {
    if anchors.is_empty() {
        return Vec::new();
    }
    let def_map = build_value_def_map(function);
    let mut out = Vec::new();
    for block in function.blocks.values() {
        for inst in &block.instructions {
            let MirInstruction::Phi { dst, .. } = inst else {
                continue;
            };
            let query = infer_phi_base_query_with_anchors(function, &def_map, *dst, anchors);
            out.push(PhiCarryRelation {
                phi_value: *dst,
                relation: query.relation,
                window_safe: query.window_safe,
            });
        }
    }
    out
}

#[allow(dead_code)] // Phase 291x-126: narrow query facade retained for tests / future callers.
pub(crate) fn infer_phi_base_relation(
    function: &MirFunction,
    value: ValueId,
    anchors: &BTreeSet<ValueId>,
) -> PhiBaseRelation {
    infer_phi_base_query(function, value, anchors).relation
}

#[allow(dead_code)] // Phase 291x-126: narrow query facade retained for tests / future callers.
pub(crate) fn infer_phi_base_query(
    function: &MirFunction,
    value: ValueId,
    anchors: &BTreeSet<ValueId>,
) -> PhiBaseQueryResult {
    let def_map = build_value_def_map(function);
    infer_phi_base_query_with_anchors(function, &def_map, value, anchors)
}

pub(crate) fn collect_passthrough_phi_parents(function: &MirFunction) -> ParentMap {
    let mut parents: ParentMap = HashMap::new();
    for block in function.blocks.values() {
        for inst in &block.instructions {
            let MirInstruction::Phi { dst, inputs, .. } = inst else {
                continue;
            };
            if let [(_, carried)] = inputs.as_slice() {
                parents.insert(*dst, *carried);
            }
        }
    }
    parents
}

pub(crate) fn infer_phi_base_query_with_anchors(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    anchors: &BTreeSet<ValueId>,
) -> PhiBaseQueryResult {
    let visited = BTreeSet::new();
    infer_phi_base_relation_inner(function, def_map, value, anchors, visited)
}

fn infer_phi_base_relation_inner(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    anchors: &BTreeSet<ValueId>,
    mut visited: BTreeSet<ValueId>,
) -> PhiBaseQueryResult {
    let root = resolve_value_origin(function, def_map, value);
    if !visited.insert(root) {
        return PhiBaseQueryResult {
            relation: PhiBaseRelation::Unknown,
            window_safe: false,
        };
    }
    if anchors.contains(&root) {
        return PhiBaseQueryResult {
            relation: PhiBaseRelation::SameBase(root),
            window_safe: true,
        };
    }

    let Some((bbid, idx)) = def_map.get(&root).copied() else {
        return PhiBaseQueryResult {
            relation: PhiBaseRelation::Unknown,
            window_safe: false,
        };
    };
    let Some(block) = function.blocks.get(&bbid) else {
        return PhiBaseQueryResult {
            relation: PhiBaseRelation::Unknown,
            window_safe: false,
        };
    };
    let Some(MirInstruction::Phi { inputs, .. }) = block.instructions.get(idx) else {
        return PhiBaseQueryResult {
            relation: PhiBaseRelation::Unknown,
            window_safe: false,
        };
    };

    match inputs.as_slice() {
        [(_, carried)] => {
            let child =
                infer_phi_base_relation_inner(function, def_map, *carried, anchors, visited);
            PhiBaseQueryResult {
                relation: child.relation,
                window_safe: matches!(child.relation, PhiBaseRelation::SameBase(_))
                    && child.window_safe,
            }
        }
        [(_, lhs), (_, rhs)] => {
            let lhs_relation =
                infer_phi_base_relation_inner(function, def_map, *lhs, anchors, visited.clone());
            let rhs_relation =
                infer_phi_base_relation_inner(function, def_map, *rhs, anchors, visited);
            let relation = merge_phi_base_relations(lhs_relation.relation, rhs_relation.relation);
            PhiBaseQueryResult {
                relation,
                window_safe: false,
            }
        }
        _ => PhiBaseQueryResult {
            relation: PhiBaseRelation::Unknown,
            window_safe: false,
        },
    }
}

fn merge_phi_base_relations(lhs: PhiBaseRelation, rhs: PhiBaseRelation) -> PhiBaseRelation {
    use PhiBaseRelation::{Mixed, SameBase, Unknown};

    match (lhs, rhs) {
        (SameBase(lhs), SameBase(rhs)) if lhs == rhs => SameBase(lhs),
        (SameBase(_), SameBase(_)) => Mixed,
        (Mixed, _) | (_, Mixed) => Mixed,
        (SameBase(base), Unknown) | (Unknown, SameBase(base)) => SameBase(base),
        (Unknown, Unknown) => Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::{
        BasicBlock, BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirType,
    };

    fn build_phi_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        function.add_block(BasicBlock::new(BasicBlockId(1)));
        function.add_block(BasicBlock::new(BasicBlockId(2)));
        function.add_block(BasicBlock::new(BasicBlockId(3)));

        let entry = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(10),
            value: ConstValue::Integer(1),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(11),
            value: ConstValue::Integer(2),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });

        let header = function.blocks.get_mut(&BasicBlockId(1)).expect("header");
        header.instructions.push(MirInstruction::Phi {
            dst: ValueId(21),
            inputs: vec![
                (BasicBlockId(0), ValueId(4)),
                (BasicBlockId(3), ValueId(22)),
            ],
            type_hint: Some(MirType::Integer),
        });
        header.instruction_spans.push(Span::unknown());
        header.instructions.push(MirInstruction::Phi {
            dst: ValueId(31),
            inputs: vec![
                (BasicBlockId(0), ValueId(10)),
                (BasicBlockId(3), ValueId(11)),
            ],
            type_hint: Some(MirType::Integer),
        });
        header.instruction_spans.push(Span::unknown());
        header.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(2),
            edge_args: None,
        });

        let body = function.blocks.get_mut(&BasicBlockId(2)).expect("body");
        body.instructions.push(MirInstruction::Copy {
            dst: ValueId(36),
            src: ValueId(10),
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
            type_hint: Some(MirType::Integer),
        });
        latch.instruction_spans.push(Span::unknown());
        latch.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });

        function
    }

    #[test]
    fn infer_phi_base_relation_detects_same_base_through_single_input_phi() {
        let function = build_phi_function();
        let anchors = BTreeSet::from([ValueId(10)]);

        let relation = infer_phi_base_query(&function, ValueId(21), &anchors);

        assert_eq!(relation.relation, PhiBaseRelation::SameBase(ValueId(10)));
        assert!(!relation.window_safe);
    }

    #[test]
    fn collect_passthrough_phi_parents_records_single_input_phi_only() {
        let function = build_phi_function();

        let parents = collect_passthrough_phi_parents(&function);

        assert_eq!(parents.get(&ValueId(22)), Some(&ValueId(36)));
        assert!(!parents.contains_key(&ValueId(21)));
        assert!(!parents.contains_key(&ValueId(31)));
    }

    #[test]
    fn infer_phi_base_relation_reports_mixed_for_different_anchor_bases() {
        let function = build_phi_function();
        let anchors = BTreeSet::from([ValueId(10), ValueId(11)]);

        let relation = infer_phi_base_query(&function, ValueId(31), &anchors);

        assert_eq!(relation.relation, PhiBaseRelation::Mixed);
        assert!(!relation.window_safe);
    }

    #[test]
    fn infer_phi_base_relation_marks_single_input_chain_as_window_safe() {
        let function = build_phi_function();
        let anchors = BTreeSet::from([ValueId(10)]);

        let relation = infer_phi_base_query(&function, ValueId(22), &anchors);

        assert_eq!(relation.relation, PhiBaseRelation::SameBase(ValueId(10)));
        assert!(relation.window_safe);
    }
}
