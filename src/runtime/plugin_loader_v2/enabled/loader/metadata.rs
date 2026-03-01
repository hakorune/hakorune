use super::super::{
    host_bridge::BoxInvokeFn,
    types::{construct_plugin_box, PluginBoxMetadata},
};
use super::specs;
use super::PluginLoaderV2;
use crate::box_trait::NyashBox;
use crate::runtime::get_global_ring0;

pub(super) fn box_invoke_fn_for_type_id(
    loader: &PluginLoaderV2,
    type_id: u32,
) -> Option<BoxInvokeFn> {
    if let Some((lib_name, box_type)) = super::super::route_resolver::resolve_lib_box_for_type_id(loader, type_id) {
        if let Some(spec) = specs::get_spec(loader, &lib_name, &box_type) {
            if spec.invoke_id.is_none() && super::util::dbg_on() {
                get_global_ring0().log.debug(&format!(
                    "[PluginLoaderV2] WARN: no per-Box invoke for {}.{} (type_id={}). Calls will fail with E_PLUGIN until plugin migrates to v2.",
                    lib_name, box_type, type_id
                ));
            }
            return spec.invoke_id;
        }
    }
    None
}

pub(super) fn metadata_for_type_id(
    loader: &PluginLoaderV2,
    type_id: u32,
) -> Option<PluginBoxMetadata> {
    let (lib_name, box_type) = super::super::route_resolver::resolve_lib_box_for_type_id(loader, type_id)?;
    let plugins = loader.plugins.read().ok()?;
    let _plugin = plugins.get(&lib_name)?.clone();
    let (resolved_type, fini_method) =
        super::super::route_resolver::resolve_type_and_fini_for_lib(
            loader, &lib_name, &box_type, type_id,
        )
        .ok()?;
    Some(PluginBoxMetadata {
        lib_name: lib_name.clone(),
        box_type: box_type.clone(),
        type_id: resolved_type,
        invoke_box_fn: box_invoke_fn_for_type_id(loader, resolved_type),
        fini_method_id: fini_method,
    })
}

pub(super) fn construct_existing_instance(
    loader: &PluginLoaderV2,
    type_id: u32,
    instance_id: u32,
) -> Option<Box<dyn NyashBox>> {
    let (lib_name, box_type) = super::super::route_resolver::resolve_lib_box_for_type_id(loader, type_id)?;
    let plugins = loader.plugins.read().ok()?;
    let _plugin = plugins.get(&lib_name)?.clone();
    let (_resolved_type, fini_method_id) =
        super::super::route_resolver::resolve_type_and_fini_for_lib(
            loader, &lib_name, &box_type, type_id,
        )
        .ok()?;
    let route = super::super::route_resolver::resolve_invoke_route_contract(loader, type_id);
    let bx = construct_plugin_box(
        box_type,
        type_id,
        route.invoke_shim_fn,
        instance_id,
        fini_method_id,
    );
    Some(Box::new(bx))
}
