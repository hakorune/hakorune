use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{
    boxed_sum_abi_plan, BasicBlockId, Callee, MirFunction, MirInstruction, MirModule, ValueId,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueRepresentationFact {
    BoxedSumHandle { abi_plan_id: u32 },
}

impl ValueRepresentationFact {
    pub fn boxed_sum_plan_id(&self) -> u32 {
        match self {
            Self::BoxedSumHandle { abi_plan_id } => *abi_plan_id,
        }
    }
}

pub fn refresh_module_value_representation_facts(module: &mut MirModule) {
    let plans = module.metadata.boxed_sum_abi_plans.clone();
    let mut return_facts = BTreeMap::<String, ValueRepresentationFact>::new();

    for _ in 0..8 {
        let mut changed = false;
        let mut next_return_facts = BTreeMap::new();
        let function_names = module
            .function_names()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();

        for name in function_names {
            let Some(function) = module.get_function_mut(&name) else {
                continue;
            };
            let facts =
                collect_function_value_representation_facts(function, &plans, &return_facts);
            let return_fact = function_return_fact(function, &facts);
            if return_fact != return_facts.get(&name).cloned() {
                changed = true;
            }
            if let Some(fact) = return_fact {
                next_return_facts.insert(name, fact);
            }
            function.metadata.value_representations = facts;
        }

        return_facts = next_return_facts;
        if !changed {
            break;
        }
    }
}

fn collect_function_value_representation_facts(
    function: &MirFunction,
    plans: &[boxed_sum_abi_plan::BoxedSumAbiPlanV1],
    return_facts: &BTreeMap<String, ValueRepresentationFact>,
) -> BTreeMap<ValueId, ValueRepresentationFact> {
    let site_plans = boxed_sum_abi_plan::build_function_boxed_sum_site_plan_map(function, plans);
    let def_map = build_value_def_map(function);
    let mut facts = BTreeMap::new();
    let mut collection_facts = BTreeMap::new();

    for block_id in function.block_ids() {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, spanned) in block.all_spanned_instructions_enumerated() {
            refresh_instruction_fact(
                &spanned.inst,
                block_id,
                instruction_index,
                &site_plans,
                return_facts,
                function,
                &def_map,
                &mut facts,
                &mut collection_facts,
            );
        }
    }

    facts
}

fn refresh_instruction_fact(
    inst: &MirInstruction,
    block_id: BasicBlockId,
    instruction_index: usize,
    site_plans: &BTreeMap<(BasicBlockId, usize), boxed_sum_abi_plan::BoxedSumSitePlan>,
    return_facts: &BTreeMap<String, ValueRepresentationFact>,
    function: &MirFunction,
    def_map: &ValueDefMap,
    facts: &mut BTreeMap<ValueId, ValueRepresentationFact>,
    collection_facts: &mut BTreeMap<(ValueId, ValueId), ValueRepresentationFact>,
) {
    match inst {
        MirInstruction::VariantMake { dst, .. } => {
            if let Some(site) = site_plans.get(&(block_id, instruction_index)) {
                facts.insert(
                    *dst,
                    ValueRepresentationFact::BoxedSumHandle {
                        abi_plan_id: site.plan_id,
                    },
                );
            }
        }
        MirInstruction::Copy { dst, src } => {
            if let Some(fact) = facts.get(src).cloned() {
                facts.insert(*dst, fact);
            }
        }
        MirInstruction::Phi { dst, inputs, .. } => {
            if let Some(fact) = same_fact_for_inputs(inputs.iter().map(|(_, value)| *value), facts)
            {
                facts.insert(*dst, fact);
            }
        }
        MirInstruction::Select {
            dst,
            then_val,
            else_val,
            ..
        } => {
            if let Some(fact) = same_fact_for_inputs([*then_val, *else_val].into_iter(), facts) {
                facts.insert(*dst, fact);
            }
        }
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Global(name)),
            ..
        } => {
            if let Some(fact) = return_facts.get(name).cloned() {
                facts.insert(*dst, fact);
            }
        }
        MirInstruction::Call {
            dst,
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } => {
            refresh_collection_method_fact(
                function,
                def_map,
                *receiver,
                args,
                *dst,
                box_name,
                method,
                facts,
                collection_facts,
            );
        }
        _ => {}
    }
}

fn refresh_collection_method_fact(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
    args: &[ValueId],
    dst: Option<ValueId>,
    box_name: &str,
    method: &str,
    facts: &mut BTreeMap<ValueId, ValueRepresentationFact>,
    collection_facts: &mut BTreeMap<(ValueId, ValueId), ValueRepresentationFact>,
) {
    if !matches!(box_name, "MapBox" | "RuntimeDataBox" | "OrderedMapBox") {
        return;
    }
    match method {
        "set" => {
            let Some(key) = logical_method_arg(args, receiver, 0) else {
                return;
            };
            let Some(value) = logical_method_arg(args, receiver, 1) else {
                return;
            };
            let value_origin = resolve_value_origin(function, def_map, value);
            let Some(fact) = facts.get(&value_origin).cloned() else {
                return;
            };
            let receiver_origin = resolve_value_origin(function, def_map, receiver);
            let key_origin = resolve_value_origin(function, def_map, key);
            collection_facts.insert((receiver_origin, key_origin), fact);
        }
        "get" => {
            let Some(dst) = dst else {
                return;
            };
            let Some(key) = logical_method_arg(args, receiver, 0) else {
                return;
            };
            let receiver_origin = resolve_value_origin(function, def_map, receiver);
            let key_origin = resolve_value_origin(function, def_map, key);
            if let Some(fact) = collection_facts
                .get(&(receiver_origin, key_origin))
                .cloned()
            {
                facts.insert(dst, fact);
            }
        }
        _ => {}
    }
}

fn logical_method_arg(args: &[ValueId], receiver: ValueId, index: usize) -> Option<ValueId> {
    let start = if args.first().copied() == Some(receiver) {
        1
    } else {
        0
    };
    args.get(start + index).copied()
}

fn same_fact_for_inputs<I>(
    values: I,
    facts: &BTreeMap<ValueId, ValueRepresentationFact>,
) -> Option<ValueRepresentationFact>
where
    I: IntoIterator<Item = ValueId>,
{
    let mut iter = values.into_iter();
    let first = facts.get(&iter.next()?)?.clone();
    for value in iter {
        if facts.get(&value) != Some(&first) {
            return None;
        }
    }
    Some(first)
}

fn function_return_fact(
    function: &MirFunction,
    facts: &BTreeMap<ValueId, ValueRepresentationFact>,
) -> Option<ValueRepresentationFact> {
    let mut result = None;
    for block_id in function.block_ids() {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        let Some(MirInstruction::Return { value: Some(value) }) = &block.terminator else {
            continue;
        };
        let fact = facts.get(value)?.clone();
        if result.as_ref().is_some_and(|current| current != &fact) {
            return None;
        }
        result = Some(fact);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        ConstValue, EffectMask, FunctionSignature, MirEnumDecl, MirEnumVariantDecl, MirType,
    };

    fn option_enum_decl() -> MirEnumDecl {
        MirEnumDecl {
            type_parameters: vec!["T".to_string()],
            variants: vec![
                MirEnumVariantDecl {
                    name: "None".to_string(),
                    payload_type_name: None,
                },
                MirEnumVariantDecl {
                    name: "Some".to_string(),
                    payload_type_name: Some("T".to_string()),
                },
            ],
        }
    }

    fn empty_function(name: &str) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: name.to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn publishes_variant_make_copy_and_same_module_call_facts() {
        let mut module = MirModule::new("value_repr_probe".to_string());
        module
            .metadata
            .enum_decls
            .insert("Option".to_string(), option_enum_decl());

        let mut make = empty_function("make/0");
        let entry = make.get_block_mut(BasicBlockId::new(0)).unwrap();
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(7),
        });
        entry.add_instruction(MirInstruction::VariantMake {
            dst: ValueId::new(2),
            enum_name: "Option".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload: Some(ValueId::new(1)),
            payload_type: Some(MirType::Integer),
        });
        entry.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        module.add_function(make);

        let mut caller = empty_function("main");
        let entry = caller.get_block_mut(BasicBlockId::new(0)).unwrap();
        entry.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("make/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
        entry.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(4),
            src: ValueId::new(3),
        });
        entry.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(4)),
        });
        module.add_function(caller);

        boxed_sum_abi_plan::refresh_module_boxed_sum_abi_plans(&mut module);
        refresh_module_value_representation_facts(&mut module);

        let make = module.get_function("make/0").unwrap();
        let make_fact = make
            .metadata
            .value_representations
            .get(&ValueId::new(2))
            .unwrap();
        let caller = module.get_function("main").unwrap();
        assert_eq!(
            caller.metadata.value_representations.get(&ValueId::new(3)),
            Some(make_fact)
        );
        assert_eq!(
            caller.metadata.value_representations.get(&ValueId::new(4)),
            Some(make_fact)
        );
    }

    #[test]
    fn publishes_collection_get_fact_from_stored_value_fact() {
        let mut module = MirModule::new("value_repr_collection_probe".to_string());
        module
            .metadata
            .enum_decls
            .insert("Option".to_string(), option_enum_decl());

        let mut function = empty_function("main");
        let entry = function.get_block_mut(BasicBlockId::new(0)).unwrap();
        entry.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(7),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(-1),
        });
        entry.add_instruction(MirInstruction::VariantMake {
            dst: ValueId::new(4),
            enum_name: "Option".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload: Some(ValueId::new(3)),
            payload_type: Some(MirType::Integer),
        });
        entry.add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "MapBox".to_string(),
                method: "set".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: hakorune_mir_defs::TypeCertainty::Known,
                box_kind: hakorune_mir_defs::CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(1), ValueId::new(2), ValueId::new(4)],
            effects: EffectMask::PURE,
        });
        entry.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "MapBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: hakorune_mir_defs::TypeCertainty::Known,
                box_kind: hakorune_mir_defs::CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(1), ValueId::new(2)],
            effects: EffectMask::PURE,
        });
        module.add_function(function);

        boxed_sum_abi_plan::refresh_module_boxed_sum_abi_plans(&mut module);
        refresh_module_value_representation_facts(&mut module);

        let function = module.get_function("main").unwrap();
        assert_eq!(
            function
                .metadata
                .value_representations
                .get(&ValueId::new(5)),
            function
                .metadata
                .value_representations
                .get(&ValueId::new(4))
        );
    }

    #[test]
    fn local_variant_without_abi_plan_is_not_a_boxed_sum_fact() {
        let mut function = empty_function("probe");
        let entry = function.get_block_mut(BasicBlockId::new(0)).unwrap();
        entry.add_instruction(MirInstruction::VariantMake {
            dst: ValueId::new(1),
            enum_name: "UnknownEnum".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload: None,
            payload_type: None,
        });

        let facts = collect_function_value_representation_facts(&function, &[], &BTreeMap::new());
        assert!(!facts.contains_key(&ValueId::new(1)));
    }

    #[test]
    fn phi_with_mixed_boxed_sum_plans_fails_closed() {
        let mut module = MirModule::new("mixed_phi_probe".to_string());
        module
            .metadata
            .enum_decls
            .insert("Option".to_string(), option_enum_decl());

        let mut function = empty_function("probe");
        let entry = function.get_block_mut(BasicBlockId::new(0)).unwrap();
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(7),
        });
        entry.add_instruction(MirInstruction::VariantMake {
            dst: ValueId::new(2),
            enum_name: "Option".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload: Some(ValueId::new(1)),
            payload_type: Some(MirType::Integer),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("x".to_string()),
        });
        entry.add_instruction(MirInstruction::VariantMake {
            dst: ValueId::new(4),
            enum_name: "Option".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload: Some(ValueId::new(3)),
            payload_type: Some(MirType::Box("StringBox".to_string())),
        });
        entry.add_instruction(MirInstruction::Phi {
            dst: ValueId::new(5),
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(2)),
                (BasicBlockId::new(0), ValueId::new(4)),
            ],
            type_hint: None,
        });
        module.add_function(function);

        boxed_sum_abi_plan::refresh_module_boxed_sum_abi_plans(&mut module);
        refresh_module_value_representation_facts(&mut module);

        let function = module.get_function("probe").unwrap();
        assert!(function
            .metadata
            .value_representations
            .contains_key(&ValueId::new(2)));
        assert!(function
            .metadata
            .value_representations
            .contains_key(&ValueId::new(4)));
        assert!(!function
            .metadata
            .value_representations
            .contains_key(&ValueId::new(5)));
    }
}
