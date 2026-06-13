//! Runtime invoke boundary for plugin boxes.
//!
//! This is not callable route truth. It closes over runtime function pointers
//! and compat-shim policy after a route plan has selected plugin execution.

use super::host_bridge::{BoxInvokeFn, InvokeFn};
use super::loader::PluginLoaderV2;

#[derive(Clone, Copy)]
pub(super) struct RuntimeInvokeBoundary {
    pub invoke_box_fn: Option<BoxInvokeFn>,
    pub invoke_shim_fn: InvokeFn,
    pub allow_compat_shim: bool,
}

pub(super) fn resolve(loader: &PluginLoaderV2, type_id: u32) -> RuntimeInvokeBoundary {
    RuntimeInvokeBoundary {
        invoke_box_fn: loader.box_invoke_fn_for_type_id(type_id),
        invoke_shim_fn: super::super::nyash_plugin_invoke_v2_shim,
        allow_compat_shim: compat_route_fallback_enabled(),
    }
}

#[inline]
fn compat_fallback_allowed() -> bool {
    crate::config::env::vm_compat_fallback_allowed()
}

#[inline]
fn compat_route_fallback_enabled() -> bool {
    !crate::config::env::fail_fast() && compat_fallback_allowed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_invoke_boundary_returns_shim_when_invoke_box_missing() {
        let loader = PluginLoaderV2::new();

        let got = resolve(&loader, 42);

        assert!(got.invoke_box_fn.is_none());
        assert_eq!(
            got.invoke_shim_fn as usize,
            super::super::super::nyash_plugin_invoke_v2_shim as usize
        );
    }
}
