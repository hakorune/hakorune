/*!
 * MIR-owned route plans for extern call policy.
 *
 * Extern calls are not CoreMethodContract rows. This module keeps the narrow
 * extern-call backend contract in MIR metadata so ny-llvmc can consume an
 * explicit plan instead of classifying raw `env.*` strings in the C shim.
 */

mod outcome_contract;
mod refresh;
mod route;
mod route_spec;

pub use outcome_contract::{
    extern_outcome_spec, ExternOutcomeSpec, ExternResultPolicy, ExternSuccessOutcome,
    ExternValueUsePolicy, HAKO_MEM_FREE_OUTCOME,
};
pub use route::{ExternCallRoute, ExternCallRouteSite};
pub use route_spec::{
    classify_extern_call_route, extern_call_route_specs, is_hostbridge_extern_invoke_symbol,
    normalize_extern_symbol, ExternCallRouteKind, ExternCallRouteSpec,
};

use super::{MirFunction, MirModule};

pub fn refresh_module_extern_call_routes(module: &mut MirModule) {
    refresh::refresh_module_extern_call_routes(module);
}

pub fn refresh_function_extern_call_routes(function: &mut MirFunction) {
    refresh::refresh_function_extern_call_routes(function);
}

pub fn validate_semantic_outcome_routes(module: &MirModule) -> Result<(), String> {
    for function in module.functions.values() {
        for route in &function.metadata.extern_call_routes {
            if route.route_id() != "extern.hako_mem.free" {
                continue;
            }
            if let Some(result_value) = route.result_value_opt() {
                return Err(format!(
                    "[failure/outcome_unit_result_value_present] route_id={} source_site={} function={} block={} instruction_index={} result_value=%{} use_site=direct_result",
                    route.route_id(),
                    HAKO_MEM_FREE_OUTCOME.source_site,
                    function.signature.name,
                    route.block(),
                    route.instruction_index(),
                    result_value.as_u32(),
                ));
            }
        }
    }
    Ok(())
}
