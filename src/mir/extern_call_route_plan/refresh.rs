use super::super::{MirFunction, MirModule};

pub(super) fn refresh_module_extern_call_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        super::route::refresh_function_extern_call_routes(function);
    }
}

pub(super) fn refresh_function_extern_call_routes(function: &mut MirFunction) {
    super::route::refresh_function_extern_call_routes(function);
}
