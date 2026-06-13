//! Runtime-owned executable method-call plans for plugin boxes.

use crate::bid::{BidError, BidResult};
use crate::box_callable::{
    BoxCallableKey, BoxCallableRegistry, BoxCallableRole, BoxCallableTarget, InvokeRoutePlan,
    MethodCallRoutePlan,
};

use super::host_bridge::{BoxInvokeFn, InvokeFn};
use super::loader::PluginLoaderV2;

#[derive(Clone, Copy)]
pub(super) struct PluginMethodExecutionPlan {
    pub type_id: u32,
    pub method_id: u32,
    pub returns_result: bool,
    pub invoke_box_fn: Option<BoxInvokeFn>,
    pub invoke_shim_fn: InvokeFn,
    pub allow_compat_shim: bool,
}

pub(super) fn resolve_method_call_plan(
    loader: &PluginLoaderV2,
    box_type: &str,
    method_name: &str,
    arity: u8,
) -> BidResult<PluginMethodExecutionPlan> {
    let target = resolve_method_target(loader, box_type, method_name, arity)?;
    let BoxCallableTarget::PluginMethod { type_id, .. } = target else {
        return Err(BidError::InvalidMethod);
    };

    let runtime_route = super::runtime_invoke_boundary::resolve(loader, type_id);
    let semantic_route = InvokeRoutePlan::PluginV2 {
        type_id,
        invoke_box_available: runtime_route.invoke_box_fn.is_some(),
        allow_compat_shim: runtime_route.allow_compat_shim,
    };
    let semantic_plan = MethodCallRoutePlan::from_target(&target, Some(semantic_route))
        .ok_or(BidError::InvalidMethod)?;

    let MethodCallRoutePlan::PluginInvoke {
        type_id,
        method_id,
        returns_result,
        ..
    } = semantic_plan
    else {
        return Err(BidError::InvalidMethod);
    };

    Ok(PluginMethodExecutionPlan {
        type_id,
        method_id,
        returns_result,
        invoke_box_fn: runtime_route.invoke_box_fn,
        invoke_shim_fn: runtime_route.invoke_shim_fn,
        allow_compat_shim: runtime_route.allow_compat_shim,
    })
}

pub(super) fn resolve_method_target(
    loader: &PluginLoaderV2,
    box_type: &str,
    method_name: &str,
    arity: u8,
) -> BidResult<BoxCallableTarget> {
    let registry = loader.box_callable_registry_snapshot()?;
    find_method_target(&registry, box_type, method_name, arity)
        .cloned()
        .ok_or(BidError::InvalidMethod)
}

fn find_method_target<'a>(
    registry: &'a BoxCallableRegistry,
    box_type: &str,
    method_name: &str,
    arity: u8,
) -> Option<&'a BoxCallableTarget> {
    let exact = BoxCallableKey::new(box_type, BoxCallableRole::Method, method_name, arity);
    if let Some(target) = registry.get(&exact) {
        return Some(target);
    }

    // Legacy plugin specs do not carry argument declarations, so they seed
    // arity 0 compatibility keys.
    let legacy = BoxCallableKey::new(box_type, BoxCallableRole::Method, method_name, 0);
    registry.get(&legacy)
}

pub(super) fn invoke_plugin_method(
    plan: PluginMethodExecutionPlan,
    instance_id: u32,
    tlv_args: &[u8],
) -> (i32, usize, Vec<u8>) {
    super::host_bridge::invoke_alloc_with_route(
        plan.invoke_box_fn,
        plan.invoke_shim_fn,
        plan.allow_compat_shim,
        plan.type_id,
        plan.method_id,
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
    fn executable_method_plan_rejects_shim_when_disallowed() {
        let plan = PluginMethodExecutionPlan {
            type_id: 42,
            method_id: 7,
            returns_result: false,
            invoke_box_fn: None,
            invoke_shim_fn: shim,
            allow_compat_shim: false,
        };

        let got = invoke_plugin_method(plan, 1, &[]);

        assert_eq!(got.0, -5);
    }
}
