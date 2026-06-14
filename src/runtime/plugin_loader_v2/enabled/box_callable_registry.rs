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
    use crate::box_callable::{
        BoxCallableKey, BoxCallableRole, BoxCallableSource, BoxCallableTarget,
    };
    use crate::config::nyash_toml_v2::NyashConfigV2;

    fn seed_loader_with_spec() -> PluginLoaderV2 {
        let mut loader = PluginLoaderV2::new();
        let toml_str = r#"
[libraries]
[libraries.demo]
boxes = ["DemoBox"]
path = "./libdemo.so"

[libraries.demo.DemoBox]
type_id = 42

[libraries.demo.DemoBox.methods]
birth = { method_id = 1 }
fini = { method_id = 999 }
run = { method_id = 7, returns_result = true }
"#;
        loader.config = Some(NyashConfigV2::from_str(toml_str).expect("parse config"));
        loader.config_toml = Some(toml::from_str::<toml::Value>(toml_str).expect("parse raw toml"));
        loader
    }

    #[test]
    fn empty_loader_snapshot_is_empty_registry() {
        let loader = PluginLoaderV2::new();

        let registry = build_snapshot(&loader).expect("snapshot");

        assert!(registry.is_empty());
    }

    #[test]
    fn plugin_loader_snapshot_seeds_registry_provider_rows() {
        let loader = seed_loader_with_spec();

        let registry = build_snapshot(&loader).expect("snapshot");

        let method = BoxCallableKey::new("DemoBox", BoxCallableRole::Method, "run", 0);
        let birth = BoxCallableKey::new("DemoBox", BoxCallableRole::Birth, "birth", 0);
        assert_eq!(
            registry.get(&method),
            Some(&BoxCallableTarget::PluginMethod {
                type_id: 42,
                method_id: 7,
                returns_result: true,
            })
        );
        assert_eq!(
            registry.get(&birth),
            Some(&BoxCallableTarget::PluginLifecycle {
                type_id: 42,
                birth_id: Some(1),
                fini_id: Some(999),
            })
        );
        assert_eq!(
            registry.get_source(&method),
            Some(BoxCallableSource::PluginLoaderProvider)
        );
        assert_eq!(
            registry.get_source(&birth),
            Some(BoxCallableSource::PluginLoaderProvider)
        );
    }

    #[test]
    fn lifecycle_snapshot_filters_to_requested_library_and_box() {
        let loader = seed_loader_with_spec();

        let registry = build_lifecycle_snapshot_for_lib(&loader, "demo", "DemoBox")
            .expect("lifecycle snapshot");

        let birth = BoxCallableKey::new("DemoBox", BoxCallableRole::Birth, "birth", 0);
        let method = BoxCallableKey::new("DemoBox", BoxCallableRole::Method, "run", 0);
        assert!(registry.get(&birth).is_some());
        assert!(registry.get(&method).is_none());
        assert!(registry
            .iter_entries()
            .all(|(_key, entry)| entry.source == BoxCallableSource::PluginLoaderProvider));
    }
}
