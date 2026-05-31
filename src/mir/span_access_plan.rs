/*!
 * MIR-owned Span access plans.
 *
 * Span v0 is a no-escape view over DirectArrayI64.  This planner intentionally
 * consumes MIR facts only: Span borrow facts, range-index facts, DirectArray
 * extent facts, and region stability facts.  It does not infer legality from
 * source spelling or method names.
 */

use crate::mir::function::{SpanAccessOp, SpanAccessPlan, SpanBorrowFact, SpanBorrowMutability};
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{ConstValue, MirFunction, MirInstruction, ValueId};

pub fn refresh_function_span_access_plans(function: &mut MirFunction) {
    let mut plans = Vec::new();
    let def_map = build_value_def_map(function);

    for route in &function.metadata.generic_method_routes {
        let Some(span) = span_borrow_for_receiver(function, &def_map, route.receiver_value())
        else {
            continue;
        };
        let Some(index_value) = route.key_value() else {
            continue;
        };
        if !span_range_proves_access(function, &def_map, span, index_value) {
            continue;
        }

        match route.route_kind_tag() {
            "array_slot_load_any" => {
                plans.push(SpanAccessPlan {
                    span_id: span.span_id,
                    op: SpanAccessOp::Load,
                    index_value,
                    value_value: None,
                    result_value: route.result_value(),
                    element_type: span.element_type,
                    route: "span_i64_load",
                    bounds_policy: "proved_unchecked",
                    proof_ids: vec!["range_index", "direct_array_extent", "region_stability"],
                    fallback_policy: "fail_fast",
                });
            }
            "array_store_any" if span.mutability == SpanBorrowMutability::Write => {
                let Some(value_value) = array_store_value_arg(
                    function,
                    route.block(),
                    route.instruction_index(),
                    index_value,
                ) else {
                    continue;
                };
                plans.push(SpanAccessPlan {
                    span_id: span.span_id,
                    op: SpanAccessOp::Store,
                    index_value,
                    value_value: Some(value_value),
                    result_value: route.result_value(),
                    element_type: span.element_type,
                    route: "span_i64_store",
                    bounds_policy: "proved_unchecked",
                    proof_ids: vec!["range_index", "direct_array_extent", "region_stability"],
                    fallback_policy: "fail_fast",
                });
            }
            _ => {}
        }
    }

    function.metadata.span_access_plans = plans;
}

fn span_borrow_for_receiver<'a>(
    function: &'a MirFunction,
    def_map: &ValueDefMap,
    receiver_value: ValueId,
) -> Option<&'a SpanBorrowFact> {
    let receiver_origin = resolve_value_origin(function, def_map, receiver_value);
    function.metadata.span_borrow_facts.iter().find(|span| {
        span.no_escape
            && span.owner_stable
            && resolve_value_origin(function, def_map, span.span_value) == receiver_origin
            && region_stability_v0_proves(function, def_map, span)
    })
}

fn span_range_proves_access(
    function: &MirFunction,
    def_map: &ValueDefMap,
    span: &SpanBorrowFact,
    index_value: ValueId,
) -> bool {
    let index_origin = resolve_value_origin(function, def_map, index_value);
    let length_origin = resolve_value_origin(function, def_map, span.length_value);
    function.metadata.range_index_facts.iter().any(|fact| {
        resolve_value_origin(function, def_map, fact.index_value) == index_origin
            && resolve_value_origin(function, def_map, fact.upper_exclusive_value) == length_origin
            && fact.step == 1
            && fact.end_exclusive
            && fact.index_body_read_only
            && !fact.loop_carried_writes_supported
            && value_is_integer_const(function, fact.lower_value, 0)
            && direct_array_extent_v0_proves_span_length(function, def_map, span)
    })
}

fn direct_array_extent_v0_proves_span_length(
    function: &MirFunction,
    def_map: &ValueDefMap,
    span: &SpanBorrowFact,
) -> bool {
    let owner_origin = resolve_value_origin(function, def_map, span.owner_value);
    let length_origin = resolve_value_origin(function, def_map, span.length_value);
    function
        .metadata
        .direct_array_extent_facts
        .iter()
        .any(|fact| {
            resolve_value_origin(function, def_map, fact.receiver_value) == owner_origin
                && resolve_value_origin(function, def_map, fact.lower_bound_value) == length_origin
                && fact.stable_in_region
                && fact.region_stability_fact_id == span.region_stability_fact_id
        })
}

fn region_stability_v0_proves(
    function: &MirFunction,
    def_map: &ValueDefMap,
    span: &SpanBorrowFact,
) -> bool {
    let region_origin = resolve_value_origin(function, def_map, span.region_value);
    function.metadata.region_stability_facts.iter().any(|fact| {
        fact.fact_id == span.region_stability_fact_id
            && fact.stable_in_region
            && resolve_value_origin(function, def_map, fact.region_value) == region_origin
    })
}

fn value_is_integer_const(function: &MirFunction, value_id: ValueId, expected: i64) -> bool {
    let def_map = build_value_def_map(function);
    let origin = resolve_value_origin(function, &def_map, value_id);
    function.blocks.values().any(|block| {
        block.instructions.iter().any(|instruction| {
            matches!(
                instruction,
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(actual),
                } if resolve_value_origin(function, &def_map, *dst) == origin
                    && *actual == expected
            )
        })
    })
}

fn array_store_value_arg(
    function: &MirFunction,
    block: crate::mir::BasicBlockId,
    instruction_index: usize,
    key_value: ValueId,
) -> Option<ValueId> {
    let block = function.blocks.get(&block)?;
    let def_map = build_value_def_map(function);
    let key_origin = resolve_value_origin(function, &def_map, key_value);
    match block.instructions.get(instruction_index)? {
        MirInstruction::Call { args, .. } => args
            .iter()
            .position(|arg| resolve_value_origin(function, &def_map, *arg) == key_origin)
            .and_then(|index| args.get(index + 1).copied()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::function::{
        DirectArrayExtentFact, DirectArrayExtentProofKind, RangeIndexFact,
        RangeIndexFactOriginKind, RegionStabilityFact, RegionStabilityProofKind,
        SpanBorrowMutability, SpanElementType,
    };
    use crate::mir::generic_method_route_facts::GenericMethodValueDemand;
    use crate::mir::generic_method_route_plan::{
        GenericMethodRoute, GenericMethodRouteDecision, GenericMethodRouteEvidence,
        GenericMethodRouteKind, GenericMethodRouteOperands, GenericMethodRouteProof,
        GenericMethodRouteSite, GenericMethodRouteSurface,
    };
    use crate::mir::{
        BasicBlock, BasicBlockId, Callee, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirType,
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

    fn span_get_route(
        block: u32,
        instruction_index: usize,
        receiver: u32,
        key: u32,
        result: u32,
    ) -> GenericMethodRoute {
        GenericMethodRoute::new(
            GenericMethodRouteSite::new(BasicBlockId::new(block), instruction_index),
            GenericMethodRouteSurface::new("SpanI64", "get", 1),
            GenericMethodRouteEvidence::new(Some("SpanI64".to_string()), None),
            GenericMethodRouteOperands::new(
                ValueId::new(receiver),
                Some(ValueId::new(key)),
                Some(ValueId::new(result)),
            ),
            GenericMethodRouteDecision::new(
                GenericMethodRouteKind::ArraySlotLoadAny,
                GenericMethodRouteProof::GetSurfacePolicy,
                None,
                None,
                GenericMethodValueDemand::ScalarI64,
                None,
            ),
        )
    }

    fn span_set_route(
        block: u32,
        instruction_index: usize,
        receiver: u32,
        key: u32,
        result: u32,
    ) -> GenericMethodRoute {
        GenericMethodRoute::new(
            GenericMethodRouteSite::new(BasicBlockId::new(block), instruction_index),
            GenericMethodRouteSurface::new("SpanMutI64", "set", 2),
            GenericMethodRouteEvidence::new(Some("SpanMutI64".to_string()), None),
            GenericMethodRouteOperands::new(
                ValueId::new(receiver),
                Some(ValueId::new(key)),
                Some(ValueId::new(result)),
            ),
            GenericMethodRouteDecision::new(
                GenericMethodRouteKind::ArrayStoreAny,
                GenericMethodRouteProof::SetSurfacePolicy,
                None,
                None,
                GenericMethodValueDemand::WriteAny,
                None,
            ),
        )
    }

    fn add_span_facts(function: &mut MirFunction) {
        function.metadata.span_borrow_facts.push(SpanBorrowFact {
            span_id: 0,
            span_value: ValueId::new(1),
            region_value: ValueId::new(2),
            owner_value: ValueId::new(2),
            mutability: SpanBorrowMutability::Write,
            element_type: SpanElementType::I64,
            start_value: ValueId::new(10),
            length_value: ValueId::new(11),
            scope_bb: BasicBlockId::new(1),
            no_escape: true,
            owner_stable: true,
            region_stability_fact_id: 0,
        });
        function
            .metadata
            .region_stability_facts
            .push(RegionStabilityFact {
                fact_id: 0,
                region_value: ValueId::new(2),
                scope_bb: BasicBlockId::new(1),
                stable_in_region: true,
                proof_kind: RegionStabilityProofKind::ProducerInvariant,
            });
        function
            .metadata
            .direct_array_extent_facts
            .push(DirectArrayExtentFact {
                receiver_value: ValueId::new(2),
                lower_bound_value: ValueId::new(11),
                proof_kind: DirectArrayExtentProofKind::ProducerInvariant,
                region_stability_fact_id: 0,
                stable_in_region: true,
            });
        function.metadata.range_index_facts.push(RangeIndexFact {
            fact_id: 0,
            origin_kind: RangeIndexFactOriginKind::CountingLoop,
            body_bb: BasicBlockId::new(1),
            index_value: ValueId::new(4),
            lower_value: ValueId::new(10),
            upper_exclusive_value: ValueId::new(11),
            step: 1,
            end_exclusive: true,
            index_body_read_only: true,
            loop_carried_writes_supported: false,
        });
    }

    #[test]
    fn refresh_records_span_load_and_store_plans_from_facts() {
        let mut function = make_function();
        function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry")
            .instructions
            .push(MirInstruction::Const {
                dst: ValueId::new(10),
                value: crate::mir::ConstValue::Integer(0),
            });
        let mut body = BasicBlock::new(BasicBlockId::new(1));
        body.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(20)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "SpanI64".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(1), ValueId::new(4)],
            effects: EffectMask::PURE,
        });
        body.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(21)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "SpanMutI64".to_string(),
                method: "set".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(1), ValueId::new(4), ValueId::new(5)],
            effects: EffectMask::PURE,
        });
        function.blocks.insert(BasicBlockId::new(1), body);
        add_span_facts(&mut function);
        function
            .metadata
            .generic_method_routes
            .push(span_get_route(1, 0, 1, 4, 20));
        function
            .metadata
            .generic_method_routes
            .push(span_set_route(1, 1, 1, 4, 21));

        refresh_function_span_access_plans(&mut function);

        assert_eq!(function.metadata.span_access_plans.len(), 2);
        let load = &function.metadata.span_access_plans[0];
        assert_eq!(load.span_id, 0);
        assert_eq!(load.op, SpanAccessOp::Load);
        assert_eq!(load.route, "span_i64_load");
        assert_eq!(load.bounds_policy, "proved_unchecked");
        assert_eq!(load.fallback_policy, "fail_fast");

        let store = &function.metadata.span_access_plans[1];
        assert_eq!(store.span_id, 0);
        assert_eq!(store.op, SpanAccessOp::Store);
        assert_eq!(store.value_value, Some(ValueId::new(5)));
        assert_eq!(store.route, "span_i64_store");
        assert_eq!(
            store.proof_ids,
            vec!["range_index", "direct_array_extent", "region_stability"]
        );
    }

    #[test]
    fn refresh_rejects_span_store_without_mutable_borrow() {
        let mut function = make_function();
        function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry")
            .instructions
            .push(MirInstruction::Const {
                dst: ValueId::new(10),
                value: crate::mir::ConstValue::Integer(0),
            });
        let mut body = BasicBlock::new(BasicBlockId::new(1));
        body.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(21)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "SpanI64".to_string(),
                method: "set".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(1), ValueId::new(4), ValueId::new(5)],
            effects: EffectMask::PURE,
        });
        function.blocks.insert(BasicBlockId::new(1), body);
        add_span_facts(&mut function);
        function.metadata.span_borrow_facts[0].mutability = SpanBorrowMutability::Read;
        function
            .metadata
            .generic_method_routes
            .push(span_set_route(1, 0, 1, 4, 21));

        refresh_function_span_access_plans(&mut function);

        assert!(function.metadata.span_access_plans.is_empty());
    }
}
