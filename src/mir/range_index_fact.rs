/*!
 * Canonical range-index fact refresh.
 *
 * This module normalizes loop producer facts into a shared consumer view.
 * Fast-path planners should consume `RangeIndexFact` instead of branching on
 * `LoopRangeFact`, counting-loop syntax, or future induction producers.
 */

use crate::mir::function::{RangeIndexFact, RangeIndexFactOriginKind};
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{
    BasicBlockId, BinaryOp, CompareOp, ConstValue, MirFunction, MirInstruction, ValueId,
};

pub fn refresh_function_range_index_facts(function: &mut MirFunction) {
    let explicit_fastmem_facts = function
        .metadata
        .range_index_facts
        .iter()
        .filter(|fact| fact.origin_kind == RangeIndexFactOriginKind::FastMemAssume)
        .cloned()
        .collect::<Vec<_>>();
    let mut facts = Vec::new();
    for source in &function.metadata.loop_range_facts {
        facts.push(RangeIndexFact {
            fact_id: facts.len() as u32,
            origin_kind: RangeIndexFactOriginKind::RangeLoop,
            index_value: source.index_phi,
            lower_value: source.start_value,
            upper_exclusive_value: source.end_value,
            body_bb: source.body_bb,
            step: source.step,
            end_exclusive: source.end_exclusive,
            index_body_read_only: source.index_read_only,
            loop_carried_writes_supported: source.loop_carried_writes_supported,
        });
    }
    for source in &function.metadata.counting_loop_facts {
        facts.push(RangeIndexFact {
            fact_id: facts.len() as u32,
            origin_kind: RangeIndexFactOriginKind::CountingLoop,
            index_value: source.index_value,
            lower_value: source.lower_value,
            upper_exclusive_value: source.upper_exclusive_value,
            body_bb: source.body_bb,
            step: source.step,
            end_exclusive: source.end_exclusive,
            index_body_read_only: source.index_body_read_only,
            loop_carried_writes_supported: source.loop_carried_writes_supported,
        });
    }
    append_mir_counting_loop_range_index_facts(function, &mut facts);
    append_modulo_range_index_facts(function, &mut facts);
    for mut source in explicit_fastmem_facts {
        source.fact_id = facts.len() as u32;
        if !facts
            .iter()
            .any(|existing| same_range_fact(existing, &source))
        {
            facts.push(source);
        }
    }
    function.metadata.range_index_facts = facts;
}

fn append_modulo_range_index_facts(function: &MirFunction, facts: &mut Vec<RangeIndexFact>) {
    let def_map = build_value_def_map(function);
    let mut derived = Vec::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for inst in &block.instructions {
            let MirInstruction::BinOp {
                dst,
                op: BinaryOp::Mod,
                lhs,
                rhs,
            } = inst
            else {
                continue;
            };
            let lhs_root = resolve_value_origin(function, &def_map, *lhs);
            let rhs_root = resolve_value_origin(function, &def_map, *rhs);
            let Some(modulus) = integer_const_value(function, rhs_root) else {
                continue;
            };
            if modulus <= 0 {
                continue;
            }

            for source in facts.iter() {
                if source.body_bb != block_id
                    || resolve_value_origin(function, &def_map, source.index_value) != lhs_root
                    || !value_is_integer_const(function, source.lower_value, 0)
                    || !source.end_exclusive
                    || !source.index_body_read_only
                    || source.loop_carried_writes_supported
                {
                    continue;
                }
                let fact = RangeIndexFact {
                    fact_id: 0,
                    origin_kind: RangeIndexFactOriginKind::ModuloOfRangeIndex,
                    index_value: *dst,
                    lower_value: source.lower_value,
                    upper_exclusive_value: rhs_root,
                    body_bb: block_id,
                    // Modulo-derived indices are bounded, but not monotonic.
                    step: 0,
                    end_exclusive: true,
                    index_body_read_only: true,
                    loop_carried_writes_supported: false,
                };
                if !facts
                    .iter()
                    .chain(derived.iter())
                    .any(|existing| same_range_fact(existing, &fact))
                {
                    derived.push(fact);
                }
            }
        }
    }

    for mut fact in derived {
        fact.fact_id = facts.len() as u32;
        facts.push(fact);
    }
}

fn append_mir_counting_loop_range_index_facts(
    function: &MirFunction,
    facts: &mut Vec<RangeIndexFact>,
) {
    let def_map = build_value_def_map(function);
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for header_id in block_ids {
        let Some(header) = function.blocks.get(&header_id) else {
            continue;
        };
        let Some(MirInstruction::Branch {
            condition,
            then_bb,
            else_bb: _,
            ..
        }) = header.terminator.as_ref()
        else {
            continue;
        };
        let Some((compare_index, upper_value)) =
            strict_lt_guard_index_and_upper(function, &def_map, *condition)
        else {
            continue;
        };
        let Some((index_phi, lower_value, latch_bb, latch_value)) =
            header_counting_phi(function, &def_map, header_id, compare_index)
        else {
            continue;
        };
        if index_phi != compare_index
            || !latch_increments_index_by_one(
                function,
                &def_map,
                latch_bb,
                header_id,
                index_phi,
                latch_value,
            )
        {
            continue;
        }
        let fact = RangeIndexFact {
            fact_id: facts.len() as u32,
            origin_kind: RangeIndexFactOriginKind::CountingLoop,
            index_value: index_phi,
            lower_value,
            upper_exclusive_value: upper_value,
            body_bb: *then_bb,
            step: 1,
            end_exclusive: true,
            index_body_read_only: true,
            loop_carried_writes_supported: false,
        };
        if !facts
            .iter()
            .any(|existing| same_range_fact(existing, &fact))
        {
            facts.push(fact);
        }
    }
}

fn strict_lt_guard_index_and_upper(
    function: &MirFunction,
    def_map: &ValueDefMap,
    condition: ValueId,
) -> Option<(ValueId, ValueId)> {
    let origin = resolve_value_origin(function, def_map, condition);
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    let MirInstruction::Compare {
        op: CompareOp::Lt,
        lhs,
        rhs,
        ..
    } = block.instructions.get(instruction_index)?
    else {
        return None;
    };
    Some((
        resolve_value_origin(function, def_map, *lhs),
        resolve_value_origin(function, def_map, *rhs),
    ))
}

fn header_counting_phi(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header_id: BasicBlockId,
    index_value: ValueId,
) -> Option<(ValueId, ValueId, BasicBlockId, ValueId)> {
    let header = function.blocks.get(&header_id)?;
    for inst in &header.instructions {
        let MirInstruction::Phi { dst, inputs, .. } = inst else {
            continue;
        };
        if *dst != index_value || inputs.len() != 2 {
            continue;
        }
        let mut lower = None;
        let mut latch = None;
        for (pred, value) in inputs {
            let root = resolve_value_origin(function, def_map, *value);
            if integer_const_value(function, root) == Some(0) {
                lower = Some(root);
            } else {
                latch = Some((*pred, *value));
            }
        }
        let (latch_bb, latch_value) = latch?;
        return Some((*dst, lower?, latch_bb, latch_value));
    }
    None
}

fn latch_increments_index_by_one(
    function: &MirFunction,
    def_map: &ValueDefMap,
    latch_bb: BasicBlockId,
    header_id: BasicBlockId,
    index_value: ValueId,
    latch_value: ValueId,
) -> bool {
    let Some(block) = function.blocks.get(&latch_bb) else {
        return false;
    };
    if !matches!(
        block.terminator.as_ref(),
        Some(MirInstruction::Jump { target, .. }) if *target == header_id
    ) {
        return false;
    }
    block.instructions.iter().any(|inst| {
        let MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } = inst
        else {
            return false;
        };
        if resolve_value_origin(function, def_map, latch_value) != *dst {
            return false;
        }
        let lhs_root = resolve_value_origin(function, def_map, *lhs);
        let rhs_root = resolve_value_origin(function, def_map, *rhs);
        (lhs_root == index_value && integer_const_value(function, rhs_root) == Some(1))
            || (rhs_root == index_value && integer_const_value(function, lhs_root) == Some(1))
    })
}

fn integer_const_value(function: &MirFunction, value_id: ValueId) -> Option<i64> {
    function.blocks.values().find_map(|block| {
        block
            .instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(actual),
                } if *dst == value_id => Some(*actual),
                _ => None,
            })
    })
}

fn value_is_integer_const(function: &MirFunction, value_id: ValueId, expected: i64) -> bool {
    integer_const_value(function, value_id)
        .map(|actual| actual == expected)
        .unwrap_or(false)
}

fn same_range_fact(lhs: &RangeIndexFact, rhs: &RangeIndexFact) -> bool {
    lhs.origin_kind == rhs.origin_kind
        && lhs.index_value == rhs.index_value
        && lhs.lower_value == rhs.lower_value
        && lhs.upper_exclusive_value == rhs.upper_exclusive_value
        && lhs.body_bb == rhs.body_bb
        && lhs.step == rhs.step
        && lhs.end_exclusive == rhs.end_exclusive
        && lhs.index_body_read_only == rhs.index_body_read_only
        && lhs.loop_carried_writes_supported == rhs.loop_carried_writes_supported
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{CountingLoopFact, LoopRangeFact};
    use crate::mir::{
        BasicBlock, BasicBlockId, BinaryOp, CompareOp, ConstValue, EffectMask, FunctionSignature,
        MirFunction, MirInstruction, MirType, ValueId,
    };

    fn make_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn refresh_maps_loop_range_fact_to_range_index_fact() {
        let mut function = make_function();
        let entry = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::Integer(0),
        });
        function.metadata.loop_range_facts.push(LoopRangeFact {
            index_name: "i".to_string(),
            start_value: ValueId::new(10),
            end_value: ValueId::new(11),
            index_phi: ValueId::new(4),
            preheader_bb: BasicBlockId::new(0),
            header_bb: BasicBlockId::new(2),
            body_bb: BasicBlockId::new(1),
            step_bb: BasicBlockId::new(3),
            exit_bb: BasicBlockId::new(4),
            step: 1,
            end_exclusive: true,
            index_read_only: true,
            body_local_writes_supported: true,
            loop_carried_writes_supported: false,
            body_writes_supported: false,
        });

        refresh_function_range_index_facts(&mut function);

        assert_eq!(function.metadata.range_index_facts.len(), 1);
        let fact = &function.metadata.range_index_facts[0];
        assert_eq!(fact.fact_id, 0);
        assert_eq!(fact.origin_kind, RangeIndexFactOriginKind::RangeLoop);
        assert_eq!(fact.index_value, ValueId::new(4));
        assert_eq!(fact.lower_value, ValueId::new(10));
        assert_eq!(fact.upper_exclusive_value, ValueId::new(11));
        assert_eq!(fact.body_bb, BasicBlockId::new(1));
        assert_eq!(fact.step, 1);
        assert!(fact.end_exclusive);
        assert!(fact.index_body_read_only);
        assert!(!fact.loop_carried_writes_supported);
    }

    #[test]
    fn refresh_maps_counting_loop_fact_to_range_index_fact() {
        let mut function = make_function();
        function
            .metadata
            .counting_loop_facts
            .push(CountingLoopFact {
                index_name: "i".to_string(),
                lower_value: ValueId::new(10),
                upper_exclusive_value: ValueId::new(11),
                index_value: ValueId::new(4),
                preheader_bb: BasicBlockId::new(0),
                header_bb: BasicBlockId::new(2),
                body_bb: BasicBlockId::new(1),
                latch_bb: BasicBlockId::new(3),
                exit_bb: BasicBlockId::new(4),
                step: 1,
                end_exclusive: true,
                index_body_read_only: true,
                loop_carried_writes_supported: false,
            });

        refresh_function_range_index_facts(&mut function);

        assert_eq!(function.metadata.range_index_facts.len(), 1);
        let fact = &function.metadata.range_index_facts[0];
        assert_eq!(fact.fact_id, 0);
        assert_eq!(fact.origin_kind, RangeIndexFactOriginKind::CountingLoop);
        assert_eq!(fact.index_value, ValueId::new(4));
        assert_eq!(fact.lower_value, ValueId::new(10));
        assert_eq!(fact.upper_exclusive_value, ValueId::new(11));
        assert_eq!(fact.body_bb, BasicBlockId::new(1));
        assert_eq!(fact.step, 1);
        assert!(fact.end_exclusive);
        assert!(fact.index_body_read_only);
        assert!(!fact.loop_carried_writes_supported);
    }

    #[test]
    fn refresh_derives_modulo_range_index_fact_from_counting_loop_index() {
        let mut function = make_function();
        function
            .metadata
            .counting_loop_facts
            .push(CountingLoopFact {
                index_name: "i".to_string(),
                lower_value: ValueId::new(10),
                upper_exclusive_value: ValueId::new(11),
                index_value: ValueId::new(4),
                preheader_bb: BasicBlockId::new(0),
                header_bb: BasicBlockId::new(2),
                body_bb: BasicBlockId::new(1),
                latch_bb: BasicBlockId::new(3),
                exit_bb: BasicBlockId::new(4),
                step: 1,
                end_exclusive: true,
                index_body_read_only: true,
                loop_carried_writes_supported: false,
            });

        let mut body = BasicBlock::new(BasicBlockId::new(1));
        body.add_instruction(MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::Integer(0),
        });
        body.add_instruction(MirInstruction::Const {
            dst: ValueId::new(12),
            value: ConstValue::Integer(64),
        });
        body.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(13),
            src: ValueId::new(4),
        });
        body.add_instruction(MirInstruction::BinOp {
            dst: ValueId::new(14),
            op: BinaryOp::Mod,
            lhs: ValueId::new(13),
            rhs: ValueId::new(12),
        });
        function.add_block(body);

        refresh_function_range_index_facts(&mut function);

        assert_eq!(function.metadata.range_index_facts.len(), 2);
        let fact = &function.metadata.range_index_facts[1];
        assert_eq!(fact.fact_id, 1);
        assert_eq!(
            fact.origin_kind,
            RangeIndexFactOriginKind::ModuloOfRangeIndex
        );
        assert_eq!(fact.index_value, ValueId::new(14));
        assert_eq!(fact.lower_value, ValueId::new(10));
        assert_eq!(fact.upper_exclusive_value, ValueId::new(12));
        assert_eq!(fact.body_bb, BasicBlockId::new(1));
        assert_eq!(fact.step, 0);
        assert!(fact.end_exclusive);
        assert!(fact.index_body_read_only);
        assert!(!fact.loop_carried_writes_supported);
    }

    #[test]
    fn refresh_derives_range_index_fact_from_mir_phi_counting_loop() {
        let mut function = make_function();
        let entry = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::Integer(0),
        });
        entry.set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(2),
            edge_args: None,
        });

        let mut header = BasicBlock::new(BasicBlockId::new(2));
        header.add_instruction(MirInstruction::Phi {
            dst: ValueId::new(20),
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(10)),
                (BasicBlockId::new(1), ValueId::new(30)),
            ],
            type_hint: Some(MirType::Integer),
        });
        header.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(21),
            src: ValueId::new(20),
        });
        header.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(22),
            src: ValueId::new(11),
        });
        header.add_instruction(MirInstruction::Compare {
            dst: ValueId::new(23),
            op: CompareOp::Lt,
            lhs: ValueId::new(21),
            rhs: ValueId::new(22),
        });
        header.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(23),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(3),
            then_edge_args: None,
            else_edge_args: None,
        });
        function.add_block(header);

        let mut body = BasicBlock::new(BasicBlockId::new(1));
        body.add_instruction(MirInstruction::Const {
            dst: ValueId::new(12),
            value: ConstValue::Integer(1),
        });
        body.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(24),
            src: ValueId::new(20),
        });
        body.add_instruction(MirInstruction::BinOp {
            dst: ValueId::new(30),
            op: BinaryOp::Add,
            lhs: ValueId::new(24),
            rhs: ValueId::new(12),
        });
        body.set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(2),
            edge_args: None,
        });
        function.add_block(body);
        function.add_block(BasicBlock::new(BasicBlockId::new(3)));

        refresh_function_range_index_facts(&mut function);

        assert_eq!(function.metadata.range_index_facts.len(), 1);
        let fact = &function.metadata.range_index_facts[0];
        assert_eq!(fact.origin_kind, RangeIndexFactOriginKind::CountingLoop);
        assert_eq!(fact.index_value, ValueId::new(20));
        assert_eq!(fact.lower_value, ValueId::new(10));
        assert_eq!(fact.upper_exclusive_value, ValueId::new(11));
        assert_eq!(fact.body_bb, BasicBlockId::new(1));
        assert_eq!(fact.step, 1);
        assert!(fact.end_exclusive);
        assert!(fact.index_body_read_only);
        assert!(!fact.loop_carried_writes_supported);
    }
}
