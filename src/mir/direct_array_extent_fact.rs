/*!
 * MIR-owned DirectArray extent facts.
 *
 * This producer keeps unchecked DirectArray planning fact-driven: it links an
 * ArrayBox field receiver and a same-receiver `capacity` field upper bound at
 * the actual loop access site, without naming `.hako` methods.
 */

use crate::mir::function::{DirectArrayExtentFact, DirectArrayExtentProofKind};
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirType, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
struct FieldGetOrigin {
    base_origin: ValueId,
    field: String,
    declared_type: Option<MirType>,
}

pub fn refresh_function_direct_array_extent_facts(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let mut facts = Vec::new();

    for route in &function.metadata.generic_method_routes {
        if route.receiver_origin_box() != Some("ArrayBox") {
            continue;
        }
        if !matches!(
            route.route_kind_tag(),
            "array_slot_load_any" | "array_store_any"
        ) {
            continue;
        }
        let Some(receiver_field) = field_get_origin(function, &def_map, route.receiver_value())
        else {
            continue;
        };
        if !is_arraybox_field(&receiver_field) {
            continue;
        }

        for range in &function.metadata.range_index_facts {
            if range.body_bb != route.block() {
                continue;
            }
            let Some(upper_field) =
                field_get_origin(function, &def_map, range.upper_exclusive_value)
            else {
                continue;
            };
            if upper_field.base_origin != receiver_field.base_origin
                || upper_field.field != "capacity"
                || !is_integer_field(&upper_field)
                || !field_pair_stable_in_block(
                    function,
                    &def_map,
                    range.body_bb,
                    receiver_field.base_origin,
                    receiver_field.field.as_str(),
                    upper_field.field.as_str(),
                )
            {
                continue;
            }

            let fact = DirectArrayExtentFact {
                receiver_value: route.receiver_value(),
                lower_bound_value: range.upper_exclusive_value,
                proof_kind: DirectArrayExtentProofKind::ProducerInvariant,
                stable_in_region: true,
            };
            if !facts.contains(&fact) {
                facts.push(fact);
            }
        }
    }

    function.metadata.direct_array_extent_facts = facts;
}

fn field_get_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<FieldGetOrigin> {
    let origin = resolve_value_origin(function, def_map, value);
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index)? {
        MirInstruction::FieldGet {
            base,
            field,
            declared_type,
            ..
        } => Some(FieldGetOrigin {
            base_origin: resolve_value_origin(function, def_map, *base),
            field: field.clone(),
            declared_type: declared_type.clone(),
        }),
        MirInstruction::Phi { inputs, .. } if !inputs.is_empty() => {
            let mut merged = None;
            for (_, input) in inputs {
                let next = field_get_origin(function, def_map, *input)?;
                merged = match merged {
                    None => Some(next),
                    Some(existing) if existing == next => Some(existing),
                    _ => return None,
                };
            }
            merged
        }
        _ => None,
    }
}

fn is_arraybox_field(field: &FieldGetOrigin) -> bool {
    matches!(
        field.declared_type.as_ref(),
        Some(MirType::Box(name)) if name == "ArrayBox"
    )
}

fn is_integer_field(field: &FieldGetOrigin) -> bool {
    matches!(field.declared_type.as_ref(), Some(MirType::Integer))
}

fn field_pair_stable_in_block(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    base_origin: ValueId,
    array_field: &str,
    extent_field: &str,
) -> bool {
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    block.instructions.iter().all(|inst| {
        let MirInstruction::FieldSet { base, field, .. } = inst else {
            return true;
        };
        if resolve_value_origin(function, def_map, *base) != base_origin {
            return true;
        }
        field != array_field && field != extent_field
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::direct_array_access_plan::{
        refresh_function_direct_array_access_plans, DirectArrayBoundsPolicy, DirectArrayProofKind,
    };
    use crate::mir::function::{CountingLoopFact, RangeIndexFactOriginKind};
    use crate::mir::generic_method_route_plan::refresh_function_generic_method_routes;
    use crate::mir::range_index_fact::refresh_function_range_index_facts;
    use crate::mir::{
        BasicBlock, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction, MirType,
    };

    fn make_function() -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Page.reset/0".to_string(),
                params: vec![MirType::Box("Page".to_string())],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.params = vec![ValueId::new(0)];
        function
    }

    #[test]
    fn refresh_links_array_field_receiver_to_same_receiver_capacity_range() {
        let mut function = make_function();
        let mut body = BasicBlock::new(BasicBlockId::new(1));
        body.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(10),
            base: ValueId::new(0),
            field: "free".to_string(),
            declared_type: Some(MirType::Box("ArrayBox".to_string())),
        });
        body.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(11),
            base: ValueId::new(0),
            field: "capacity".to_string(),
            declared_type: Some(MirType::Integer),
        });
        body.add_instruction(MirInstruction::Const {
            dst: ValueId::new(12),
            value: ConstValue::Integer(0),
        });
        body.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(20)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "set".to_string(),
                receiver: Some(ValueId::new(10)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(12), ValueId::new(12)],
            effects: EffectMask::PURE,
        });
        function.add_block(body);
        function
            .metadata
            .counting_loop_facts
            .push(CountingLoopFact {
                index_name: "i".to_string(),
                lower_value: ValueId::new(12),
                upper_exclusive_value: ValueId::new(11),
                index_value: ValueId::new(12),
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

        refresh_function_generic_method_routes(&mut function);
        refresh_function_range_index_facts(&mut function);
        refresh_function_direct_array_extent_facts(&mut function);

        assert_eq!(function.metadata.range_index_facts.len(), 1);
        assert_eq!(
            function.metadata.range_index_facts[0].origin_kind,
            RangeIndexFactOriginKind::CountingLoop
        );
        assert_eq!(function.metadata.direct_array_extent_facts.len(), 1);
        let fact = &function.metadata.direct_array_extent_facts[0];
        assert_eq!(fact.receiver_value, ValueId::new(10));
        assert_eq!(fact.lower_bound_value, ValueId::new(11));
        assert_eq!(
            fact.proof_kind,
            DirectArrayExtentProofKind::ProducerInvariant
        );
        assert!(fact.stable_in_region);

        refresh_function_direct_array_access_plans(&mut function);
        assert_eq!(function.metadata.direct_array_access_plans.len(), 1);
        let plan = &function.metadata.direct_array_access_plans[0];
        assert_eq!(
            plan.bounds_policy(),
            DirectArrayBoundsPolicy::ProvedUnchecked
        );
        assert_eq!(plan.proof_kind(), DirectArrayProofKind::RangeIndex);
    }

    #[test]
    fn refresh_rejects_extent_when_capacity_is_from_different_receiver() {
        let mut function = make_function();
        let mut body = BasicBlock::new(BasicBlockId::new(1));
        body.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(10),
            base: ValueId::new(0),
            field: "free".to_string(),
            declared_type: Some(MirType::Box("ArrayBox".to_string())),
        });
        body.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(11),
            base: ValueId::new(99),
            field: "capacity".to_string(),
            declared_type: Some(MirType::Integer),
        });
        body.add_instruction(MirInstruction::Const {
            dst: ValueId::new(12),
            value: ConstValue::Integer(0),
        });
        body.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(20)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "set".to_string(),
                receiver: Some(ValueId::new(10)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(12), ValueId::new(12)],
            effects: EffectMask::PURE,
        });
        function.add_block(body);
        function
            .metadata
            .counting_loop_facts
            .push(CountingLoopFact {
                index_name: "i".to_string(),
                lower_value: ValueId::new(12),
                upper_exclusive_value: ValueId::new(11),
                index_value: ValueId::new(12),
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

        refresh_function_generic_method_routes(&mut function);
        refresh_function_range_index_facts(&mut function);
        refresh_function_direct_array_extent_facts(&mut function);

        assert!(function.metadata.direct_array_extent_facts.is_empty());
    }
}
