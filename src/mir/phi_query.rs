/*!
 * Generic PHI base-relation queries.
 *
 * This module is the MIR-side SSOT for answering a narrow question:
 * whether a PHI continues the same normalized base value under a caller-
 * supplied anchor predicate. It does not know about string corridors or
 * any placement/effect policy.
 */

use super::{BasicBlockId, MirFunction, MirInstruction, ValueId};
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

pub(crate) fn collect_phi_carry_relations<Normalize, Anchor>(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    normalize_value: Normalize,
    is_anchor: Anchor,
) -> Vec<PhiCarryRelation>
where
    Normalize: Fn(ValueId) -> ValueId + Copy,
    Anchor: Fn(ValueId) -> bool + Copy,
{
    let mut out = Vec::new();
    for block in function.blocks.values() {
        for inst in &block.instructions {
            let MirInstruction::Phi { dst, .. } = inst else {
                continue;
            };
            let query = infer_phi_base_query(function, def_map, *dst, normalize_value, is_anchor);
            out.push(PhiCarryRelation {
                phi_value: *dst,
                relation: query.relation,
                window_safe: query.window_safe,
            });
        }
    }
    out
}

pub(crate) fn infer_phi_base_relation<Normalize, Anchor>(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
    normalize_value: Normalize,
    is_anchor: Anchor,
) -> PhiBaseRelation
where
    Normalize: Fn(ValueId) -> ValueId + Copy,
    Anchor: Fn(ValueId) -> bool + Copy,
{
    infer_phi_base_query(function, def_map, value, normalize_value, is_anchor).relation
}

pub(crate) fn infer_phi_base_query<Normalize, Anchor>(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
    normalize_value: Normalize,
    is_anchor: Anchor,
) -> PhiBaseQueryResult
where
    Normalize: Fn(ValueId) -> ValueId + Copy,
    Anchor: Fn(ValueId) -> bool + Copy,
{
    let visited = BTreeSet::new();
    infer_phi_base_relation_inner(
        function,
        def_map,
        value,
        normalize_value,
        is_anchor,
        visited,
    )
}

fn infer_phi_base_relation_inner<Normalize, Anchor>(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
    normalize_value: Normalize,
    is_anchor: Anchor,
    mut visited: BTreeSet<ValueId>,
) -> PhiBaseQueryResult
where
    Normalize: Fn(ValueId) -> ValueId + Copy,
    Anchor: Fn(ValueId) -> bool + Copy,
{
    let root = normalize_value(value);
    if !visited.insert(root) {
        return PhiBaseQueryResult {
            relation: PhiBaseRelation::Unknown,
            window_safe: false,
        };
    }
    if is_anchor(root) {
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
            let child = infer_phi_base_relation_inner(
                function,
                def_map,
                *carried,
                normalize_value,
                is_anchor,
                visited,
            );
            PhiBaseQueryResult {
                relation: child.relation,
                window_safe: matches!(child.relation, PhiBaseRelation::SameBase(_))
                    && child.window_safe,
            }
        }
        [(_, lhs), (_, rhs)] => {
            let lhs_relation = infer_phi_base_relation_inner(
                function,
                def_map,
                *lhs,
                normalize_value,
                is_anchor,
                visited.clone(),
            );
            let rhs_relation = infer_phi_base_relation_inner(
                function,
                def_map,
                *rhs,
                normalize_value,
                is_anchor,
                visited,
            );
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
    use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType};

    fn build_def_map(function: &MirFunction) -> HashMap<ValueId, (BasicBlockId, usize)> {
        let mut out = HashMap::new();
        for (bbid, block) in &function.blocks {
            for (idx, inst) in block.instructions.iter().enumerate() {
                match inst {
                    MirInstruction::Const { dst, .. }
                    | MirInstruction::Copy { dst, .. }
                    | MirInstruction::UnaryOp { dst, .. }
                    | MirInstruction::BinOp { dst, .. }
                    | MirInstruction::Compare { dst, .. }
                    | MirInstruction::TypeOp { dst, .. }
                    | MirInstruction::FieldGet { dst, .. }
                    | MirInstruction::VariantMake { dst, .. }
                    | MirInstruction::VariantTag { dst, .. }
                    | MirInstruction::VariantProject { dst, .. }
                    | MirInstruction::Load { dst, .. }
                    | MirInstruction::Phi { dst, .. }
                    | MirInstruction::NewBox { dst, .. }
                    | MirInstruction::RefNew { dst, .. }
                    | MirInstruction::WeakRef { dst, .. }
                    | MirInstruction::FutureNew { dst, .. }
                    | MirInstruction::NewClosure { dst, .. }
                    | MirInstruction::Await { dst, .. }
                    | MirInstruction::Select { dst, .. } => {
                        out.insert(*dst, (*bbid, idx));
                    }
                    MirInstruction::Call { dst: Some(dst), .. } => {
                        out.insert(*dst, (*bbid, idx));
                    }
                    _ => {}
                }
            }
        }
        out
    }

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
        let def_map = build_def_map(&function);

        let relation = infer_phi_base_query(
            &function,
            &def_map,
            ValueId(21),
            |value| {
                if value == ValueId(36) {
                    ValueId(10)
                } else {
                    value
                }
            },
            |value| value == ValueId(10),
        );

        assert_eq!(relation.relation, PhiBaseRelation::SameBase(ValueId(10)));
        assert!(!relation.window_safe);
    }

    #[test]
    fn infer_phi_base_relation_reports_mixed_for_different_anchor_bases() {
        let function = build_phi_function();
        let def_map = build_def_map(&function);

        let relation = infer_phi_base_query(
            &function,
            &def_map,
            ValueId(31),
            |value| value,
            |value| value == ValueId(10) || value == ValueId(11),
        );

        assert_eq!(relation.relation, PhiBaseRelation::Mixed);
        assert!(!relation.window_safe);
    }

    #[test]
    fn infer_phi_base_relation_marks_single_input_chain_as_window_safe() {
        let function = build_phi_function();
        let def_map = build_def_map(&function);

        let relation = infer_phi_base_query(
            &function,
            &def_map,
            ValueId(22),
            |value| {
                if value == ValueId(36) {
                    ValueId(10)
                } else {
                    value
                }
            },
            |value| value == ValueId(10),
        );

        assert_eq!(relation.relation, PhiBaseRelation::SameBase(ValueId(10)));
        assert!(relation.window_safe);
    }
}
