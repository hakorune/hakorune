//! BoxCallableRegistry projection for PluginLoaderV2.
//!
//! This module is the provider projection boundary. PluginLoader reads plugin
//! config/spec state, then publishes a registry snapshot for planners.

use crate::bid::BidResult;
use crate::box_callable::providers::plugin_loader::{seed_plugin_exports, seed_plugin_loader};
use crate::box_callable::BoxCallableRegistry;

use super::loader::PluginLoaderV2;
use super::PluginCallableExport;

pub(super) fn build_snapshot(loader: &PluginLoaderV2) -> BidResult<BoxCallableRegistry> {
    let mut registry = BoxCallableRegistry::new();
    seed_plugin_loader(&mut registry, loader)?;
    Ok(registry)
}

pub(super) fn build_lifecycle_snapshot_for_lib(
    loader: &PluginLoaderV2,
    lib_name: &str,
    box_type: &str,
) -> BidResult<BoxCallableRegistry> {
    let exports = loader.export_box_callables()?;
    let selected: Vec<_> = exports
        .iter()
        .filter(|export| match export {
            PluginCallableExport::Lifecycle {
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
    Ok(registry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_loader_snapshot_is_empty_registry() {
        let loader = PluginLoaderV2::new();

        let registry = build_snapshot(&loader).expect("snapshot");

        assert!(registry.is_empty());
    }
}
