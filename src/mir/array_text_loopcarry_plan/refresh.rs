use super::super::value_origin::build_value_def_map;
use super::{
    match_array_text_get, match_insert_mid_subrange_trailing_len_route, MirFunction, MirModule,
};

pub fn refresh_module_array_text_loopcarry_len_store_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_text_loopcarry_len_store_routes(function);
    }
}

pub fn refresh_function_array_text_loopcarry_len_store_routes(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let mut routes = Vec::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            let Some((array_value, index_value, source_value)) = match_array_text_get(inst) else {
                continue;
            };
            if let Some(route) = match_insert_mid_subrange_trailing_len_route(
                function,
                &def_map,
                block,
                block_id,
                instruction_index,
                array_value,
                index_value,
                source_value,
            ) {
                routes.push(route);
            }
        }
    }

    routes.sort_by_key(|route| (route.block.as_u32(), route.instruction_index));
    function.metadata.array_text_loopcarry_len_store_routes = routes;
}
