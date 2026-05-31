/*!
 * Canonical range-index fact refresh.
 *
 * This module normalizes loop producer facts into a shared consumer view.
 * Fast-path planners should consume `RangeIndexFact` instead of branching on
 * `LoopRangeFact`, counting-loop syntax, or future induction producers.
 */

use crate::mir::function::{RangeIndexFact, RangeIndexFactOriginKind};
use crate::mir::MirFunction;

pub fn refresh_function_range_index_facts(function: &mut MirFunction) {
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
    function.metadata.range_index_facts = facts;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{CountingLoopFact, LoopRangeFact};
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
        MirType, ValueId,
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
}
