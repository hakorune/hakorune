//! Plugin Box callable provider backed by PluginLoader callable exports.

use crate::bid::BidResult;
use crate::runtime::plugin_loader_v2::{PluginCallableExport, PluginLoaderV2};

use super::super::{BoxCallableKey, BoxCallableRegistry, BoxCallableRole, BoxCallableTarget};

pub const TRUTH_SOURCE_PLUGIN_LOADER_PROVIDER: &str = "plugin_loader_provider";

pub fn seed_plugin_loader(
    registry: &mut BoxCallableRegistry,
    loader: &PluginLoaderV2,
) -> BidResult<usize> {
    let exports = loader.export_box_callables()?;
    Ok(seed_plugin_exports(registry, exports.iter()))
}

pub fn seed_plugin_exports<'a>(
    registry: &mut BoxCallableRegistry,
    exports: impl IntoIterator<Item = &'a PluginCallableExport>,
) -> usize {
    exports
        .into_iter()
        .map(|export| seed_plugin_export(registry, export))
        .sum()
}

pub fn seed_plugin_export(
    registry: &mut BoxCallableRegistry,
    export: &PluginCallableExport,
) -> usize {
    match export {
        PluginCallableExport::Method {
            box_type,
            method_name,
            arity,
            type_id,
            method_id,
            returns_result,
            ..
        } => {
            let key = BoxCallableKey::new(box_type, BoxCallableRole::Method, method_name, *arity);
            let target = BoxCallableTarget::PluginMethod {
                type_id: *type_id,
                method_id: *method_id,
                returns_result: *returns_result,
            };
            registry.insert(key, target);
            1
        }
        PluginCallableExport::Lifecycle {
            box_type,
            type_id,
            birth_id,
            fini_id,
            ..
        } => {
            let mut seeded = 0;
            if birth_id.is_some() {
                let key = BoxCallableKey::new(box_type, BoxCallableRole::Birth, "birth", 0);
                let target = BoxCallableTarget::PluginLifecycle {
                    type_id: *type_id,
                    birth_id: *birth_id,
                    fini_id: *fini_id,
                };
                registry.insert(key, target);
                seeded += 1;
            }
            if fini_id.is_some() {
                let key = BoxCallableKey::new(box_type, BoxCallableRole::Fini, "fini", 0);
                let target = BoxCallableTarget::PluginLifecycle {
                    type_id: *type_id,
                    birth_id: None,
                    fini_id: *fini_id,
                };
                registry.insert(key, target);
                seeded += 1;
            }
            seeded
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_plugin_method_export_as_plugin_method_target() {
        let export = PluginCallableExport::Method {
            lib_name: "demo".to_string(),
            box_type: "DemoBox".to_string(),
            type_id: 42,
            method_name: "run".to_string(),
            arity: 2,
            method_id: 7,
            returns_result: true,
        };
        let mut registry = BoxCallableRegistry::new();

        let seeded = seed_plugin_export(&mut registry, &export);

        let key = BoxCallableKey::new("DemoBox", BoxCallableRole::Method, "run", 2);
        assert_eq!(seeded, 1);
        assert_eq!(
            registry.get(&key),
            Some(&BoxCallableTarget::PluginMethod {
                type_id: 42,
                method_id: 7,
                returns_result: true,
            })
        );
        assert_eq!(
            registry.get(&key).unwrap().id_space(),
            "plugin_typebox_method_id"
        );
    }

    #[test]
    fn seeds_plugin_lifecycle_export_as_birth_and_fini_targets() {
        let export = PluginCallableExport::Lifecycle {
            lib_name: "demo".to_string(),
            box_type: "DemoBox".to_string(),
            type_id: 42,
            birth_id: Some(1),
            fini_id: Some(999),
        };
        let mut registry = BoxCallableRegistry::new();

        let seeded = seed_plugin_export(&mut registry, &export);

        let birth = BoxCallableKey::new("DemoBox", BoxCallableRole::Birth, "birth", 0);
        let fini = BoxCallableKey::new("DemoBox", BoxCallableRole::Fini, "fini", 0);
        assert_eq!(seeded, 2);
        assert_eq!(
            registry.get(&birth),
            Some(&BoxCallableTarget::PluginLifecycle {
                type_id: 42,
                birth_id: Some(1),
                fini_id: Some(999),
            })
        );
        assert_eq!(
            registry.get(&fini),
            Some(&BoxCallableTarget::PluginLifecycle {
                type_id: 42,
                birth_id: None,
                fini_id: Some(999),
            })
        );
    }
}
