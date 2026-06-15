/*!
 * MIR-owned proof for get-only missing reads on a freshly local MapBox.
 *
 * This is intentionally narrower than generic MapBox optimization. It only
 * proves that a `MapGet` site on a local, unpublished, never-mutated MapBox can
 * lower to the runtime-data missing value. Generic MapBox storage and visible
 * semantics stay owned by `MapBox`.
 */

use super::core_method_op::CoreMethodOp;
use super::generic_method_route_facts::{
    const_i64_value, GenericMethodKeyRoute, GenericMethodPublicationPolicy,
    GenericMethodReturnShape,
};
use super::generic_method_route_plan::{
    instruction_may_escape_or_mutate_receiver, GenericMethodRoute,
};
use super::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use super::verification::utils::compute_dominators;
use super::{BasicBlockId, MirFunction, MirInstruction, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapMissingEmptyRoute {
    block: BasicBlockId,
    instruction_index: usize,
    receiver_value: ValueId,
    receiver_root: ValueId,
    key_value: ValueId,
    key_const: i64,
    result_value: ValueId,
}

impl MapMissingEmptyRoute {
    pub fn route_id(&self) -> &'static str {
        "map_missing_empty.get"
    }

    pub fn block(&self) -> BasicBlockId {
        self.block
    }

    pub fn instruction_index(&self) -> usize {
        self.instruction_index
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn receiver_root(&self) -> ValueId {
        self.receiver_root
    }

    pub fn key_value(&self) -> ValueId {
        self.key_value
    }

    pub fn key_const(&self) -> i64 {
        self.key_const
    }

    pub fn result_value(&self) -> ValueId {
        self.result_value
    }

    pub fn proof_ids(&self) -> &'static [&'static str] {
        &[
            "receiver_birth_is_new_mapbox",
            "receiver_root_is_same_local_value",
            "receiver_not_published_before_get",
            "receiver_not_escaped_before_get",
            "no_map_set_before_get",
            "no_map_delete_before_get",
            "no_map_clear_before_get",
            "no_unknown_receiver_mutation_before_get",
            "i64_const_key",
            "runtime_data_facade_missing_shape",
        ]
    }
}

pub fn collect_function_map_missing_empty_routes(
    function: &MirFunction,
) -> Vec<MapMissingEmptyRoute> {
    let def_map = build_value_def_map(function);
    let mut routes = function
        .metadata
        .generic_method_routes
        .iter()
        .filter_map(|route| match_missing_empty_route(function, &def_map, route))
        .collect::<Vec<_>>();
    routes.sort_by_key(|route| (route.block().as_u32(), route.instruction_index()));
    routes
}

fn match_missing_empty_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route: &GenericMethodRoute,
) -> Option<MapMissingEmptyRoute> {
    if route.core_method()?.op != CoreMethodOp::MapGet {
        return None;
    }
    if route.receiver_origin_box() != Some("MapBox")
        || route.key_route() != Some(GenericMethodKeyRoute::I64Const)
        || route.publication_policy() != Some(GenericMethodPublicationPolicy::RuntimeDataFacade)
        || route.return_shape() != Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
    {
        return None;
    }

    let receiver_root = resolve_value_origin(function, def_map, route.receiver_value());
    if !receiver_root_is_new_mapbox(function, def_map, receiver_root) {
        return None;
    }
    receiver_birth_dominates_get(function, def_map, receiver_root, route.block())?;

    let key_value = route.key_value()?;
    let key_const = const_i64_value(function, def_map, key_value)?;
    let result_value = route.result_value()?;

    if receiver_is_used_by_publication_or_mutation(
        function,
        def_map,
        receiver_root,
        route.block(),
        route.instruction_index(),
    ) {
        return None;
    }

    Some(MapMissingEmptyRoute {
        block: route.block(),
        instruction_index: route.instruction_index(),
        receiver_value: route.receiver_value(),
        receiver_root,
        key_value,
        key_const,
        result_value,
    })
}

fn receiver_root_is_new_mapbox(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver_root: ValueId,
) -> bool {
    let Some((block_id, instruction_index)) = def_map.get(&receiver_root).copied() else {
        return false;
    };
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    matches!(
        block.instructions.get(instruction_index),
        Some(MirInstruction::NewBox { box_type, .. }) if box_type == "MapBox"
    )
}

fn receiver_birth_dominates_get(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver_root: ValueId,
    get_block: BasicBlockId,
) -> Option<()> {
    let (birth_block, _) = def_map.get(&receiver_root).copied()?;
    compute_dominators(function)
        .dominates(birth_block, get_block)
        .then_some(())
}

fn receiver_is_used_by_publication_or_mutation(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver_root: ValueId,
    get_block: BasicBlockId,
    get_instruction_index: usize,
) -> bool {
    let Some((birth_block, birth_instruction_index)) = def_map.get(&receiver_root).copied() else {
        return true;
    };

    for (block_id, block) in &function.blocks {
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            if *block_id == birth_block && instruction_index == birth_instruction_index {
                continue;
            }
            if *block_id == get_block && instruction_index == get_instruction_index {
                continue;
            }
            if instruction_may_escape_or_mutate_receiver(function, def_map, inst, receiver_root) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::generic_method_route_plan::refresh_function_generic_method_routes;
    use crate::mir::{BasicBlock, Callee, ConstValue, EffectMask, FunctionSignature, MirType};

    fn method_call(
        dst: Option<u32>,
        box_name: &str,
        method: &str,
        receiver: u32,
        args: Vec<u32>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: dst.map(ValueId::new),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(ValueId::new(receiver)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: args.into_iter().map(ValueId::new).collect(),
            effects: EffectMask::PURE,
        }
    }

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
    fn collects_missing_empty_route_for_fresh_unmutated_map_get() {
        let mut function = make_function();
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(0),
        });
        block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));
        function.add_block(block);

        refresh_function_generic_method_routes(&mut function);
        let routes = collect_function_map_missing_empty_routes(&function);

        assert_eq!(routes.len(), 1);
        let route = &routes[0];
        assert_eq!(route.route_id(), "map_missing_empty.get");
        assert_eq!(route.block(), BasicBlockId::new(0));
        assert_eq!(route.instruction_index(), 2);
        assert_eq!(route.receiver_value(), ValueId::new(1));
        assert_eq!(route.receiver_root(), ValueId::new(1));
        assert_eq!(route.key_value(), ValueId::new(2));
        assert_eq!(route.key_const(), 0);
        assert_eq!(route.result_value(), ValueId::new(3));
    }

    #[test]
    fn rejects_missing_empty_route_after_map_set() {
        let mut function = make_function();
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(0),
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(7),
        });
        block.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));
        block.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));
        function.add_block(block);

        refresh_function_generic_method_routes(&mut function);
        let routes = collect_function_map_missing_empty_routes(&function);

        assert!(routes.is_empty());
    }
}
