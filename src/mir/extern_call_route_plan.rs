/*!
 * MIR-owned route plans for extern call policy.
 *
 * Extern calls are not CoreMethodContract rows. This module keeps the narrow
 * extern-call backend contract in MIR metadata so ny-llvmc can consume an
 * explicit plan instead of classifying raw `env.*` strings in the C shim.
 */

mod refresh;
mod route;
mod route_spec;

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
