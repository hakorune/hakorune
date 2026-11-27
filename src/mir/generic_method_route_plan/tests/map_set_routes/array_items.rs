use super::*;

#[test]
fn mir_json_array_item_runtime_data_routes_preserve_schema() {
    for (case_name, function_name, expected_proof, expected_return_shape, expected_demand) in [
        (
            "proves_mir_json_vid_array_item_runtime_data_get",
            "MirJsonEmitBox._emit_vid_array_rec/3",
            GenericMethodRouteProof::MirJsonVidArrayItem,
            GenericMethodReturnShape::ScalarI64OrMissingZero,
            GenericMethodValueDemand::ScalarI64,
        ),
        (
            "proves_mir_json_effects_array_item_runtime_data_get",
            "MirJsonEmitBox._emit_effects_rec/3",
            GenericMethodRouteProof::MirJsonEffectsArrayItem,
            GenericMethodReturnShape::MixedRuntimeI64OrHandle,
            GenericMethodValueDemand::RuntimeI64OrHandle,
        ),
        (
            "proves_mir_json_block_inst_array_item_runtime_data_get",
            "MirJsonEmitBox._emit_block_rec/3",
            GenericMethodRouteProof::MirJsonBlockInstArrayItem,
            GenericMethodReturnShape::MixedRuntimeI64OrHandle,
            GenericMethodValueDemand::RuntimeI64OrHandle,
        ),
        (
            "proves_mir_json_function_block_array_item_runtime_data_get",
            "MirJsonEmitBox._emit_function_rec/3",
            GenericMethodRouteProof::MirJsonFunctionBlockArrayItem,
            GenericMethodReturnShape::MixedRuntimeI64OrHandle,
            GenericMethodValueDemand::RuntimeI64OrHandle,
        ),
        (
            "proves_mir_json_module_function_array_item_runtime_data_get",
            "MirJsonEmitBox._emit_module_rec/3",
            GenericMethodRouteProof::MirJsonModuleFunctionArrayItem,
            GenericMethodReturnShape::MixedRuntimeI64OrHandle,
            GenericMethodValueDemand::RuntimeI64OrHandle,
        ),
        (
            "proves_mir_json_params_array_item_runtime_data_get",
            "MirJsonEmitBox._emit_params_rec/3",
            GenericMethodRouteProof::MirJsonParamsArrayItem,
            GenericMethodReturnShape::MixedRuntimeI64OrHandle,
            GenericMethodValueDemand::RuntimeI64OrHandle,
        ),
    ] {
        println!("case: {case_name}");
        let mut function = make_function();
        function.signature.name = function_name.to_string();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = route_for(&function, "RuntimeDataBox", "get", Some(3));
        assert_eq!(route.proof(), expected_proof);
        assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
        assert_eq!(route.return_shape(), Some(expected_return_shape));
        assert_eq!(route.value_demand(), expected_demand);
        assert_eq!(
            route.publication_policy(),
            Some(GenericMethodPublicationPolicy::NoPublication)
        );
    }
}
