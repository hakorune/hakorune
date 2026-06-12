//! Runtime-owned executable lifecycle plans for plugin boxes.
//!
//! BoxCallable route plans remain semantic. This module closes over plugin
//! function pointers at the runtime boundary.

use crate::bid::{BidError, BidResult};
use crate::box_callable::providers::plugin_loader::{seed_plugin_exports, seed_plugin_loader};
use crate::box_callable::{
    BoxCallableKey, BoxCallableRegistry, BoxCallableRole, BoxCallableTarget, InvokeRoutePlan,
    NewBoxRoutePlan,
};

use super::host_bridge::{BoxInvokeFn, InvokeFn};
use super::loader::PluginLoaderV2;

#[derive(Clone, Copy)]
pub(super) struct PluginInvokeExecutionPlan {
    pub invoke_box_fn: Option<BoxInvokeFn>,
    pub invoke_shim_fn: InvokeFn,
    pub allow_compat_shim: bool,
}

#[derive(Clone, Copy)]
pub(super) struct PluginNewBoxExecutionPlan {
    pub type_id: u32,
    pub birth_id: u32,
    pub fini_id: Option<u32>,
    pub invoke_route: PluginInvokeExecutionPlan,
}

#[derive(Clone, Copy)]
pub(super) struct PluginDropBoxExecutionPlan {
    pub type_id: u32,
    pub fini_id: u32,
    pub invoke_route: PluginInvokeExecutionPlan,
}

pub(super) fn resolve_newbox_lifecycle_plan(
    loader: &PluginLoaderV2,
    box_type: &str,
) -> BidResult<PluginNewBoxExecutionPlan> {
    let mut registry = BoxCallableRegistry::new();
    seed_plugin_loader(&mut registry, loader)?;
    resolve_newbox_lifecycle_plan_from_registry(loader, &registry, box_type)
}

pub(super) fn resolve_newbox_lifecycle_plan_for_lib(
    loader: &PluginLoaderV2,
    lib_name: &str,
    box_type: &str,
) -> BidResult<PluginNewBoxExecutionPlan> {
    let exports = loader.export_box_callables()?;
    let selected: Vec<_> = exports
        .iter()
        .filter(|export| match export {
            crate::runtime::plugin_loader_v2::PluginCallableExport::Lifecycle {
                lib_name: export_lib,
                box_type: export_box,
                ..
            } => export_lib == lib_name && export_box == box_type,
            _ => false,
        })
        .cloned()
        .collect();
    let mut registry = BoxCallableRegistry::new();
    seed_plugin_exports(&mut registry, selected.iter());
    resolve_newbox_lifecycle_plan_from_registry(loader, &registry, box_type)
}

fn resolve_newbox_lifecycle_plan_from_registry(
    loader: &PluginLoaderV2,
    registry: &BoxCallableRegistry,
    box_type: &str,
) -> BidResult<PluginNewBoxExecutionPlan> {
    let key = BoxCallableKey::new(box_type, BoxCallableRole::Birth, "birth", 0);
    let target = registry.get(&key).ok_or(BidError::InvalidMethod)?;
    let BoxCallableTarget::PluginLifecycle { type_id, .. } = target else {
        return Err(BidError::InvalidMethod);
    };

    let runtime_route = super::route_resolver::resolve_invoke_route_contract(loader, *type_id);
    let semantic_route = InvokeRoutePlan::PluginV2 {
        type_id: *type_id,
        invoke_box_available: runtime_route.invoke_box_fn.is_some(),
        allow_compat_shim: runtime_route.allow_compat_shim,
    };
    let semantic_plan = NewBoxRoutePlan::plugin_birth_from_target(target, semantic_route)
        .ok_or(BidError::InvalidMethod)?;

    let NewBoxRoutePlan::PluginBirth {
        type_id,
        birth_id,
        fini_id,
        ..
    } = semantic_plan
    else {
        return Err(BidError::InvalidMethod);
    };

    Ok(PluginNewBoxExecutionPlan {
        type_id,
        birth_id,
        fini_id,
        invoke_route: PluginInvokeExecutionPlan {
            invoke_box_fn: runtime_route.invoke_box_fn,
            invoke_shim_fn: runtime_route.invoke_shim_fn,
            allow_compat_shim: runtime_route.allow_compat_shim,
        },
    })
}

pub(super) fn invoke_plugin_lifecycle(
    plan: PluginInvokeExecutionPlan,
    type_id: u32,
    method_id: u32,
    instance_id: u32,
    tlv_args: &[u8],
) -> (i32, usize, Vec<u8>) {
    super::host_bridge::invoke_alloc_with_route(
        plan.invoke_box_fn,
        plan.invoke_shim_fn,
        plan.allow_compat_shim,
        type_id,
        method_id,
        instance_id,
        tlv_args,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe extern "C" fn shim(
        _type_id: u32,
        _method_id: u32,
        _instance_id: u32,
        _args: *const u8,
        _args_len: usize,
        _result: *mut u8,
        _result_len: *mut usize,
    ) -> i32 {
        17
    }

    #[test]
    fn executable_invoke_plan_rejects_shim_when_disallowed() {
        let plan = PluginInvokeExecutionPlan {
            invoke_box_fn: None,
            invoke_shim_fn: shim,
            allow_compat_shim: false,
        };

        let got = invoke_plugin_lifecycle(plan, 42, 1, 0, &[]);

        assert_eq!(got.0, -5);
    }
}
