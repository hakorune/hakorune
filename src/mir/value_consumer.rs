/*!
 * Generic value consumer facts.
 *
 * This module owns function-local consumer capability facts that backend
 * emitters may consume without re-scanning MIR JSON for semantic ownership.
 */

use super::{
    string_corridor_recognizer::match_method_set_call, MirFunction, MirInstruction, MirModule,
    ValueId,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ValueConsumerFacts {
    pub direct_set_consumer: bool,
}

#[derive(Debug, Clone, Copy, Default)]
struct ValueConsumerUseCounts {
    direct_set_uses: usize,
    other_uses: usize,
}

fn record_other_uses(
    counts: &mut BTreeMap<ValueId, ValueConsumerUseCounts>,
    values: impl IntoIterator<Item = ValueId>,
) {
    let mut seen = BTreeSet::new();
    for value in values {
        if seen.insert(value) {
            counts.entry(value).or_default().other_uses += 1;
        }
    }
}

fn record_direct_set_consumer_use(
    counts: &mut BTreeMap<ValueId, ValueConsumerUseCounts>,
    value: ValueId,
) {
    counts.entry(value).or_default().direct_set_uses += 1;
}

fn record_instruction_uses(
    counts: &mut BTreeMap<ValueId, ValueConsumerUseCounts>,
    inst: &MirInstruction,
) {
    if let Some(store) = match_method_set_call(inst) {
        record_direct_set_consumer_use(counts, store.value);
        record_other_uses(
            counts,
            inst.used_values()
                .into_iter()
                .filter(|value| *value != store.value),
        );
        return;
    }

    record_other_uses(counts, inst.used_values());
}

pub fn refresh_function_value_consumer_facts(function: &mut MirFunction) {
    let mut counts = BTreeMap::new();

    for block in function.blocks.values() {
        for inst in &block.instructions {
            record_instruction_uses(&mut counts, inst);
        }
        if let Some(term) = &block.terminator {
            record_instruction_uses(&mut counts, term);
        }
    }

    function.metadata.value_consumer_facts = counts
        .into_iter()
        .filter_map(|(value, counts)| {
            let facts = ValueConsumerFacts {
                direct_set_consumer: counts.direct_set_uses == 1 && counts.other_uses == 0,
            };
            facts.direct_set_consumer.then_some((value, facts))
        })
        .collect();
}

pub fn refresh_module_value_consumer_facts(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_value_consumer_facts(function);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

    fn method_call(
        dst: Option<ValueId>,
        receiver: ValueId,
        method: &str,
        args: Vec<ValueId>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst,
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: method.to_string(),
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::PURE,
        }
    }

    fn make_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![MirType::Box("RuntimeDataBox".to_string())],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn refresh_value_consumer_facts_marks_single_direct_set_value() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.instructions.extend([method_call(
            None,
            ValueId::new(0),
            "set",
            vec![ValueId::new(1), ValueId::new(2)],
        )]);
        block.instruction_spans.extend([Span::unknown()]);

        refresh_function_value_consumer_facts(&mut function);

        assert_eq!(
            function.metadata.value_consumer_facts.get(&ValueId::new(2)),
            Some(&ValueConsumerFacts {
                direct_set_consumer: true
            })
        );
        assert!(!function
            .metadata
            .value_consumer_facts
            .contains_key(&ValueId::new(1)));
    }

    #[test]
    fn refresh_value_consumer_facts_rejects_extra_uses() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.instructions.extend([
            method_call(
                None,
                ValueId::new(0),
                "set",
                vec![ValueId::new(1), ValueId::new(2)],
            ),
            MirInstruction::Return {
                value: Some(ValueId::new(2)),
            },
        ]);
        block.instruction_spans.extend([Span::unknown(); 2]);

        refresh_function_value_consumer_facts(&mut function);

        assert!(!function
            .metadata
            .value_consumer_facts
            .contains_key(&ValueId::new(2)));
    }
}
