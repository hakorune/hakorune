use std::collections::BTreeMap;

use super::origin_inference::{
    build_user_box_field_return_hints, infer_user_box_field_box_origins,
    infer_user_box_method_param_box_origins,
};
use super::return_shape::UserBoxFieldReturnHints;
use super::value_type_publish::{
    propagate_user_box_box_value_types, publish_generic_route_result_value_types,
    publish_user_box_field_get_value_types, publish_user_box_param_origin_value_types,
    publish_user_box_route_param_value_types, publish_user_box_route_result_value_types,
};
use super::{
    materialization, target_collection, FieldBoxOriginMap, ParamBoxOriginMap, UserBoxMethodRoute,
};
use crate::mir::MirModule;

pub(super) fn refresh_module_user_box_method_routes_fixpoint(module: &mut MirModule) {
    let typed_plan_type_ids = typed_plan_type_ids(module);
    for _ in 0..iteration_budget(module) {
        let before = snapshot_routes(module);
        let empty_field_return_hints = UserBoxFieldReturnHints::new();
        let targets = target_collection::collect_method_targets(
            module,
            &typed_plan_type_ids,
            &empty_field_return_hints,
        );
        let initial_param_box_origins =
            infer_user_box_method_param_box_origins(module, &targets, &BTreeMap::new());
        let field_box_origins =
            infer_user_box_field_box_origins(module, &targets, &initial_param_box_origins);
        let param_box_origins =
            infer_user_box_method_param_box_origins(module, &targets, &field_box_origins);
        let field_box_origins =
            infer_user_box_field_box_origins(module, &targets, &param_box_origins);
        let param_box_origins =
            infer_user_box_method_param_box_origins(module, &targets, &field_box_origins);

        let value_type_changed =
            publish_user_box_param_origin_value_types(module, &param_box_origins);
        let field_get_value_type_changed =
            publish_user_box_field_get_value_types(module, &param_box_origins, &field_box_origins);

        let field_return_hints = build_user_box_field_return_hints(module, &field_box_origins);
        let targets = target_collection::collect_method_targets(
            module,
            &typed_plan_type_ids,
            &field_return_hints,
        );
        materialize_routes(
            module,
            &targets,
            &typed_plan_type_ids,
            &param_box_origins,
            &field_box_origins,
        );

        let route_result_value_type_changed = publish_user_box_route_result_value_types(module);
        let generic_result_value_type_changed = publish_generic_route_result_value_types(module);
        let propagated_value_type_changed = propagate_user_box_box_value_types(module);
        let route_value_type_changed = publish_user_box_route_param_value_types(
            module,
            &param_box_origins,
            &field_box_origins,
        );
        let route_changed = routes_changed(module, &before);

        if !(value_type_changed
            || field_get_value_type_changed
            || route_result_value_type_changed
            || generic_result_value_type_changed
            || propagated_value_type_changed
            || route_value_type_changed
            || route_changed)
        {
            break;
        }
    }
}

fn iteration_budget(module: &MirModule) -> usize {
    module.functions.len().saturating_mul(4).max(8)
}

fn typed_plan_type_ids(module: &MirModule) -> BTreeMap<String, u32> {
    module
        .metadata
        .typed_object_plans
        .iter()
        .map(|plan| (plan.box_name.clone(), plan.type_id))
        .collect()
}

fn snapshot_routes(module: &MirModule) -> BTreeMap<String, Vec<UserBoxMethodRoute>> {
    module
        .functions
        .iter()
        .map(|(name, function)| {
            (
                name.clone(),
                function.metadata.user_box_method_routes.clone(),
            )
        })
        .collect()
}

fn materialize_routes(
    module: &mut MirModule,
    targets: &BTreeMap<String, target_collection::UserBoxMethodTargetFacts>,
    typed_plan_type_ids: &BTreeMap<String, u32>,
    param_box_origins: &ParamBoxOriginMap,
    field_box_origins: &FieldBoxOriginMap,
) {
    for function in module.functions.values_mut() {
        materialization::refresh_function_user_box_method_routes_with_context(
            function,
            targets,
            typed_plan_type_ids,
            param_box_origins,
            field_box_origins,
        );
    }
}

fn routes_changed(module: &MirModule, before: &BTreeMap<String, Vec<UserBoxMethodRoute>>) -> bool {
    module.functions.iter().any(|(name, function)| {
        before.get(name).map_or(true, |routes| {
            routes != &function.metadata.user_box_method_routes
        })
    })
}
