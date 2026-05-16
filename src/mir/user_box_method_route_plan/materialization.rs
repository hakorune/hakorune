use std::collections::{BTreeMap, BTreeSet};

use super::origin_inference::user_box_route_receiver_box_name;
use super::{
    method_target_symbol, FieldBoxOriginMap, ParamBoxOriginMap, UserBoxMethodRoute,
    UserBoxMethodRouteSite, UserBoxMethodTargetFacts,
};
use crate::mir::value_origin::build_value_def_map;
use crate::mir::{Callee, MirFunction, MirInstruction};

pub(super) fn refresh_function_user_box_method_routes_with_context(
    function: &mut MirFunction,
    targets: &BTreeMap<String, UserBoxMethodTargetFacts>,
    typed_plan_type_ids: &BTreeMap<String, u32>,
    param_box_origins: &ParamBoxOriginMap,
    field_box_origins: &FieldBoxOriginMap,
) {
    let mut routes = Vec::new();
    let mut user_box_names = targets
        .values()
        .map(|target| target.box_name.clone())
        .collect::<BTreeSet<_>>();
    user_box_names.extend(typed_plan_type_ids.keys().cloned());
    let def_map = build_value_def_map(function);
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, instruction) in block.instructions.iter().enumerate() {
            let MirInstruction::Call {
                dst,
                callee:
                    Some(Callee::Method {
                        box_name,
                        method,
                        receiver: Some(receiver),
                        certainty,
                        box_kind: _,
                    }),
                args,
                ..
            } = instruction
            else {
                continue;
            };
            let Some(route_box_name) = user_box_route_receiver_box_name(
                function,
                &def_map,
                &user_box_names,
                box_name,
                *certainty,
                *receiver,
                param_box_origins,
                field_box_origins,
            ) else {
                continue;
            };
            let target_symbol = method_target_symbol(&route_box_name, method, args.len());
            let target = targets.get(&target_symbol);
            let type_id = typed_plan_type_ids.get(&route_box_name).copied();
            routes.push(UserBoxMethodRoute {
                site: UserBoxMethodRouteSite::new(block_id, instruction_index),
                box_name: route_box_name,
                method: method.clone(),
                receiver_value: *receiver,
                arity: args.len(),
                result_value: *dst,
                target_symbol,
                target_exists: target.is_some(),
                target_arity: target.map(|target| target.arity),
                target_return_type: target.map(|target| target.return_type.clone()),
                target_inferred_return: target.and_then(|target| target.inferred_return),
                target_result_box_name: target.and_then(|target| target.result_box_name.clone()),
                target_body_supported: target.map(|target| target.body_supported).unwrap_or(false),
                type_id,
            });
        }
    }

    function.metadata.user_box_method_routes = routes;
}
