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
            value_consumer_used_values(inst)
                .into_iter()
                .filter(|value| *value != store.value),
        );
        return;
    }

    record_other_uses(counts, value_consumer_used_values(inst));
}

fn value_consumer_used_values(inst: &MirInstruction) -> Vec<ValueId> {
    match inst {
        MirInstruction::Invoke { .. }
        | MirInstruction::InvokeNormalResult { .. }
        | MirInstruction::FaultFrameEnter { .. }
        | MirInstruction::ReturnFault { .. } => inst.used_values(),
        MirInstruction::Const { .. } | MirInstruction::Safepoint => Vec::new(),
        MirInstruction::ArrayElementWrite {
            receiver,
            index,
            value,
            ..
        } => {
            let mut values = vec![*receiver];
            values.extend(index.iter().copied());
            values.push(*value);
            values
        }
        MirInstruction::ArrayStateContractClaim { array, .. } => vec![*array],
        MirInstruction::UnaryOp { operand, .. }
        | MirInstruction::Load { ptr: operand, .. }
        | MirInstruction::StaticDataLoad { index: operand, .. }
        | MirInstruction::TypeOp { value: operand, .. }
        | MirInstruction::Copy { src: operand, .. }
        | MirInstruction::CopyOwned { src: operand, .. }
        | MirInstruction::LocalContractWrite { src: operand, .. }
        | MirInstruction::RecordFieldContractCheck { value: operand, .. }
        | MirInstruction::Debug { value: operand, .. }
        | MirInstruction::VariantTag { value: operand, .. }
        | MirInstruction::VariantProject { value: operand, .. } => vec![*operand],
        MirInstruction::BinOp { lhs, rhs, .. }
        | MirInstruction::Compare { lhs, rhs, .. }
        | MirInstruction::Store {
            value: lhs,
            ptr: rhs,
            ..
        } => vec![*lhs, *rhs],
        MirInstruction::MemOp { operands, .. } => operands.clone(),
        MirInstruction::PinnedTextOp { kind, .. } => kind.used_values(),
        MirInstruction::PinnedTextResidenceFinish { .. }
        | MirInstruction::PinnedTextResidenceEnter { .. }
        | MirInstruction::PinnedTextResidenceTrap { .. } => Vec::new(),
        MirInstruction::FieldGet { base, .. } => vec![*base],
        MirInstruction::FieldSet { base, value, .. } => vec![*base, *value],
        MirInstruction::WeakFieldWrite { base, value, .. } => vec![*base, *value],
        MirInstruction::VariantMake { payload, .. } => payload.iter().copied().collect(),
        MirInstruction::Call(_) | MirInstruction::LegacyCallV0 { .. } => inst.used_values(),
        MirInstruction::Phi { inputs, .. } => inputs.iter().map(|(_, value)| *value).collect(),
        MirInstruction::Branch {
            condition,
            then_edge_args,
            else_edge_args,
            ..
        } => {
            let mut used = vec![*condition];
            if let Some(args) = then_edge_args {
                used.extend(args.values.iter().copied());
            }
            if let Some(args) = else_edge_args {
                used.extend(args.values.iter().copied());
            }
            used
        }
        MirInstruction::Jump { edge_args, .. } => edge_args
            .as_ref()
            .map(|args| args.values.clone())
            .unwrap_or_default(),
        MirInstruction::Return { value, .. } => value.iter().copied().collect(),
        MirInstruction::CheckedCallOut {
            receiver,
            arguments,
            ..
        } => {
            let mut values = vec![*receiver];
            values.extend(arguments.iter().copied());
            values
        }
        MirInstruction::CheckedCallOutNormalResult { .. } => Vec::new(),
        MirInstruction::CheckedCallOutEnd { .. } | MirInstruction::CheckedCallOutFault { .. } => {
            Vec::new()
        }
        MirInstruction::NewBox { args, .. } => args.clone(),
        MirInstruction::RecordValuePublish { base, fields, .. } => {
            let mut used = base.iter().copied().collect::<Vec<_>>();
            used.extend(fields.iter().copied());
            used
        }
        MirInstruction::KeepAlive { values } => values.clone(),
        MirInstruction::DestroyOwned { value } => vec![*value],
        MirInstruction::ReleaseStrong { values } => values.clone(),
        MirInstruction::Throw { exception, .. } => vec![*exception],
        MirInstruction::Catch {
            exception_value, ..
        } => vec![*exception_value],
        MirInstruction::RefNew { box_val, .. } => vec![*box_val],
        MirInstruction::WeakRef { value, .. } => vec![*value],
        MirInstruction::Barrier { ptr, .. } => vec![*ptr],
        MirInstruction::FutureNew { value, .. } => vec![*value],
        MirInstruction::FutureSet { future, value } => vec![*future, *value],
        MirInstruction::Await { future, .. } => vec![*future],
        MirInstruction::NewClosure { captures, me, .. } => {
            let mut used = Vec::with_capacity(captures.len() + usize::from(me.is_some()));
            used.extend(captures.iter().map(|(_, value)| *value));
            if let Some(me) = me {
                used.push(*me);
            }
            used
        }
        MirInstruction::Select {
            cond,
            then_val,
            else_val,
            ..
        } => vec![*cond, *then_val, *else_val],
    }
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
        MirInstruction::LegacyCallV0 {
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

    #[test]
    fn refresh_value_consumer_facts_counts_typed_callee_targets_as_other_uses() {
        let mut function = make_function();
        let captured = ValueId::new(7);
        let me = ValueId::new(8);
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.instructions.extend([
            MirInstruction::LegacyCallV0 {
                dst: Some(ValueId::new(20)),
                func: ValueId::new(99),
                callee: Some(crate::mir::Callee::Value(captured)),
                args: vec![],
                effects: EffectMask::PURE,
            },
            MirInstruction::LegacyCallV0 {
                dst: Some(ValueId::new(21)),
                func: ValueId::new(98),
                callee: Some(crate::mir::Callee::Closure {
                    params: vec!["x".to_string()],
                    captures: vec![("captured".to_string(), captured)],
                    me_capture: Some(me),
                }),
                args: vec![],
                effects: EffectMask::PURE,
            },
            method_call(
                None,
                ValueId::new(0),
                "set",
                vec![ValueId::new(1), captured],
            ),
            method_call(None, ValueId::new(0), "set", vec![ValueId::new(2), me]),
        ]);
        block.instruction_spans.extend([Span::unknown(); 4]);

        refresh_function_value_consumer_facts(&mut function);

        assert!(!function
            .metadata
            .value_consumer_facts
            .contains_key(&captured));
        assert!(!function.metadata.value_consumer_facts.contains_key(&me));
    }

    #[test]
    fn refresh_value_consumer_facts_ignores_typed_func_and_dst_decoration() {
        let mut function = make_function();
        let stale_func = ValueId::new(55);
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.instructions.extend([
            MirInstruction::LegacyCallV0 {
                dst: Some(ValueId::new(56)),
                func: stale_func,
                callee: Some(crate::mir::Callee::Global(crate::mir::test_global_target(
                    "global".to_string(),
                ))),
                args: vec![],
                effects: EffectMask::PURE,
            },
            method_call(
                None,
                ValueId::new(0),
                "set",
                vec![ValueId::new(1), stale_func],
            ),
        ]);
        block.instruction_spans.extend([Span::unknown(); 2]);

        refresh_function_value_consumer_facts(&mut function);

        assert_eq!(
            function.metadata.value_consumer_facts.get(&stale_func),
            Some(&ValueConsumerFacts {
                direct_set_consumer: true
            })
        );
        assert!(!function
            .metadata
            .value_consumer_facts
            .contains_key(&ValueId::new(56)));
    }

    #[test]
    fn refresh_value_consumer_facts_preserves_legacy_func_use() {
        let mut function = make_function();
        let legacy_func = ValueId::new(66);
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.instructions.extend([
            MirInstruction::LegacyCallV0 {
                dst: None,
                func: legacy_func,
                callee: None,
                args: vec![],
                effects: EffectMask::PURE,
            },
            method_call(
                None,
                ValueId::new(0),
                "set",
                vec![ValueId::new(1), legacy_func],
            ),
        ]);
        block.instruction_spans.extend([Span::unknown(); 2]);

        refresh_function_value_consumer_facts(&mut function);

        assert!(!function
            .metadata
            .value_consumer_facts
            .contains_key(&legacy_func));
    }
}
