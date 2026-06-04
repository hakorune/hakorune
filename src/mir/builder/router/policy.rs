use crate::mir::definitions::call_unified::TypeCertainty;
use std::sync::OnceLock;

use super::catalog;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Unified,
    BoxCall,
}

/// Decide routing policy for a method call (Unified vs BoxCall) without changing behavior.
/// Rules (behavior-preserving):
/// - UnknownBox -> BoxCall (unified is unstable for unknown receivers)
/// - Core boxes: StringBox/ArrayBox/MapBox -> BoxCall unless a catalog-backed
///   method family has an explicit Unified value-path proof
/// - User boxes: names not ending with "Box" -> BoxCall
/// - Otherwise Unified
pub fn choose_route(box_name: &str, method: &str, certainty: TypeCertainty, arity: usize) -> Route {
    let mut reason = "unified";
    let route = if box_name == "UnknownBox" {
        reason = "unknown_recv";
        Route::BoxCall
    } else if let Some(value_path_reason) =
        catalog::unified_value_path_reason(box_name, method, arity)
    {
        reason = value_path_reason;
        Route::Unified
    } else if catalog::is_core_box(box_name) {
        reason = "core_box";
        Route::BoxCall
    } else if !box_name.ends_with("Box") {
        reason = "user_instance";
        Route::BoxCall
    } else {
        Route::Unified
    };

    if router_trace_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[router] route={:?} reason={} recv={} method={} arity={} certainty={:?}",
            route, reason, box_name, method, arity, certainty
        ));
    }

    route
}

#[inline]
fn router_trace_enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(crate::config::env::builder_router_trace)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn route(box_name: &str, method: &str, arity: usize) -> Route {
        choose_route(box_name, method, TypeCertainty::Known, arity)
    }

    #[test]
    fn unknown_and_user_instance_stay_boxcall() {
        assert_eq!(route("UnknownBox", "length", 0), Route::BoxCall);
        assert_eq!(route("UserThing", "length", 0), Route::BoxCall);
    }

    #[test]
    fn catalog_backed_corebox_methods_use_unified_route() {
        assert_eq!(route("StringBox", "length", 0), Route::Unified);
        assert_eq!(route("ArrayBox", "push", 1), Route::Unified);
        assert_eq!(route("MapBox", "get", 1), Route::Unified);
    }

    #[test]
    fn non_catalog_corebox_methods_stay_boxcall() {
        assert_eq!(route("StringBox", "length", 1), Route::BoxCall);
        assert_eq!(route("ArrayBox", "push", 0), Route::BoxCall);
        assert_eq!(route("MapBox", "get", 2), Route::BoxCall);
    }

    #[test]
    fn non_core_box_names_keep_unified_route() {
        assert_eq!(route("FileBox", "read", 0), Route::Unified);
        assert_eq!(route("ConsoleBox", "log", 1), Route::Unified);
    }
}
