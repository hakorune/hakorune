/*!
 * Semantic metadata refresh orchestration.
 *
 * This module owns the refresh entry points for MIR-side semantic metadata.
 * It does not invent new facts by itself; it only defines which existing
 * refresh helpers run together and in what order.
 */

use super::{
    agg_local_scalarization::refresh_function_agg_local_scalarization_routes,
    array_rmw_add1_leaf_seed_plan::refresh_function_array_rmw_add1_leaf_seed_route,
    array_rmw_window_plan::refresh_function_array_rmw_window_routes,
    array_string_len_window_plan::refresh_function_array_string_len_window_routes,
    array_string_store_micro_seed_plan::refresh_function_array_string_store_micro_seed_route,
    array_text_combined_region_plan::refresh_function_array_text_combined_region_routes,
    array_text_edit_plan::refresh_function_array_text_edit_routes,
    array_text_loopcarry_plan::refresh_function_array_text_loopcarry_len_store_routes,
    array_text_observer_plan::refresh_function_array_text_observer_routes,
    array_text_residence_session_plan::refresh_function_array_text_residence_session_routes,
    array_text_state_residence_plan::refresh_function_array_text_state_residence_route,
    concat_const_suffix_micro_seed_plan::refresh_function_concat_const_suffix_micro_seed_route,
    exact_seed_backend_route::refresh_function_exact_seed_backend_route, function::ModuleMetadata,
    generic_method_route_plan::refresh_function_generic_method_routes,
    placement_effect::refresh_function_placement_effect_routes,
    refresh_function_storage_class_facts, refresh_function_string_corridor_candidates,
    refresh_function_string_corridor_facts, refresh_function_string_corridor_relations,
    refresh_function_string_direct_set_window_routes, refresh_function_string_kernel_plans,
    refresh_function_sum_placement_facts, refresh_function_sum_placement_layouts,
    refresh_function_sum_placement_selections, refresh_function_sum_variant_project_seed_route,
    refresh_function_sum_variant_tag_seed_route, refresh_function_thin_entry_candidates,
    refresh_function_thin_entry_selections, refresh_function_userbox_local_scalar_seed_route,
    refresh_function_userbox_loop_micro_seed_route, refresh_function_value_consumer_facts,
    substring_views_micro_seed_plan::refresh_function_substring_views_micro_seed_route,
    MirFunction, MirModule,
};

/// Refresh the current string-corridor metadata stack for one function.
///
/// This is the narrow function-local entry point used by string-corridor
/// transforms after they mutate a function in-place.
pub fn refresh_function_string_corridor_metadata(function: &mut MirFunction) {
    refresh_function_string_corridor_facts(function);
    refresh_function_string_corridor_relations(function);
    refresh_function_string_corridor_candidates(function);
}

/// Refresh MIR semantic metadata for one function using the current module
/// metadata as the shared context owner.
///
/// The first cut keeps the existing refresh order behavior-preserving while
/// moving the owner behind a single entry point. Demand facts are refreshed
/// beside placement decisions here, but they remain inspection-only metadata.
pub fn refresh_function_semantic_metadata(
    function: &mut MirFunction,
    module_metadata: &ModuleMetadata,
) {
    refresh_function_string_corridor_metadata(function);
    refresh_function_storage_class_facts(function);
    refresh_function_thin_entry_candidates(function, module_metadata);
    refresh_function_thin_entry_selections(function);
    refresh_function_sum_placement_facts(function);
    refresh_function_sum_placement_selections(function);
    refresh_function_sum_placement_layouts(function);
    refresh_function_agg_local_scalarization_routes(function);
    refresh_function_placement_effect_routes(function);
    refresh_function_value_consumer_facts(function);
    refresh_function_string_kernel_plans(function);
    refresh_function_string_direct_set_window_routes(function);
    refresh_function_generic_method_routes(function);
    refresh_function_array_rmw_window_routes(function);
    refresh_function_array_string_len_window_routes(function);
    refresh_function_array_text_loopcarry_len_store_routes(function);
    refresh_function_array_text_edit_routes(function);
    refresh_function_array_text_residence_session_routes(function);
    refresh_function_array_text_observer_routes(function);
    refresh_function_array_text_combined_region_routes(function);
    refresh_function_array_string_store_micro_seed_route(function);
    refresh_function_array_rmw_add1_leaf_seed_route(function);
    refresh_function_concat_const_suffix_micro_seed_route(function);
    refresh_function_substring_views_micro_seed_route(function);
    refresh_function_sum_variant_tag_seed_route(function);
    refresh_function_sum_variant_project_seed_route(function);
    refresh_function_userbox_local_scalar_seed_route(function);
    refresh_function_userbox_loop_micro_seed_route(function);
    refresh_function_exact_seed_backend_route(function);
    refresh_function_array_text_state_residence_route(function);
}

/// Refresh MIR semantic metadata for the whole module.
pub fn refresh_module_semantic_metadata(module: &mut MirModule) {
    let module_metadata = module.metadata.clone();
    for function in module.functions.values_mut() {
        refresh_function_semantic_metadata(function, &module_metadata);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{
        BasicBlockId, Callee, EffectMask, FunctionSignature, MirInstruction, MirType, ValueId,
    };

    #[test]
    fn refresh_module_semantic_metadata_populates_string_and_storage_metadata() {
        let mut module = MirModule::new("semantic_refresh_test".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function
            .metadata
            .value_types
            .insert(ValueId::new(1), MirType::Float);

        let entry = function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block");
        entry.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "StringBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId::new(0)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        entry.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });

        module.add_function(function);

        refresh_module_semantic_metadata(&mut module);

        let function = module.get_function("main").expect("refreshed function");
        assert!(function
            .metadata
            .string_corridor_facts
            .contains_key(&ValueId::new(2)));
        assert!(function
            .metadata
            .value_storage_classes
            .contains_key(&ValueId::new(1)));
    }
}
