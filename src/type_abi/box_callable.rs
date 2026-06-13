//! Type ABI / BoxDescriptor projection for BoxCallableRegistry entries.
//!
//! This adapter is read-only. It projects registry entries for tooling and
//! snapshots; it is not used for execution or planning truth.

use crate::box_callable::{
    BoxCallableKey, BoxCallableRegistry, BoxCallableRole, BoxCallableTarget,
};

use super::catalog::TypeAbiCatalog;
use super::pack::{build_type_abi_pack, TypeAbiPack};
use super::{TypeAbiError, TypeAbiPayloadSink, TypeAbiTag, TypeAbiView};

pub const TYPE_ABI_BOX_CALLABLE_SCHEMA_V0: u16 = 1;

pub struct BoxCallableEntryView<'a> {
    key: &'a BoxCallableKey,
    target: &'a BoxCallableTarget,
}

impl<'a> BoxCallableEntryView<'a> {
    pub fn new(key: &'a BoxCallableKey, target: &'a BoxCallableTarget) -> Self {
        Self { key, target }
    }
}

impl TypeAbiView for BoxCallableEntryView<'_> {
    fn abi_tag(&self) -> TypeAbiTag {
        TypeAbiTag::BoxCallable
    }

    fn abi_id(&self) -> u32 {
        stable_callable_id(self.key)
    }

    fn abi_name(&self) -> Option<&str> {
        Some(self.key.name.as_str())
    }

    fn payload_schema(&self) -> u16 {
        TYPE_ABI_BOX_CALLABLE_SCHEMA_V0
    }

    fn encode_payload(&self, out: &mut TypeAbiPayloadSink) -> Result<(), TypeAbiError> {
        write_key(out, self.key)?;
        write_target(out, self.target);
        Ok(())
    }
}

pub fn publish_box_callable_registry(
    catalog: &mut TypeAbiCatalog,
    registry: &BoxCallableRegistry,
) -> usize {
    let mut count = 0;
    for (key, target) in registry.iter() {
        let view = BoxCallableEntryView::new(key, target);
        catalog.publish(&view);
        count += 1;
    }
    count
}

pub fn build_catalog_from_box_callable_registry_snapshot(
    registry: &BoxCallableRegistry,
) -> TypeAbiCatalog {
    let mut catalog = TypeAbiCatalog::builder_from_refreshed_world();
    for (key, target) in registry.iter() {
        let view = BoxCallableEntryView::new(key, target);
        catalog.publish(&view);
    }
    catalog.finish()
}

pub fn build_box_callable_registry_pack(
    registry: &BoxCallableRegistry,
) -> Result<TypeAbiPack, TypeAbiError> {
    let views: Vec<_> = registry
        .iter()
        .map(|(key, target)| BoxCallableEntryView::new(key, target))
        .collect();
    let refs: Vec<&dyn TypeAbiView> = views.iter().map(|view| view as &dyn TypeAbiView).collect();
    build_type_abi_pack(&refs)
}

fn stable_callable_id(key: &BoxCallableKey) -> u32 {
    let mut hash = 0x811c_9dc5_u32;
    hash = fnv1a(hash, key.box_key.as_str().as_bytes());
    hash = fnv1a(hash, &[role_code(key.role)]);
    hash = fnv1a(hash, key.name.as_str().as_bytes());
    fnv1a(hash, &[key.arity])
}

fn fnv1a(mut hash: u32, bytes: &[u8]) -> u32 {
    for byte in bytes {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }
    hash
}

fn write_key(out: &mut TypeAbiPayloadSink, key: &BoxCallableKey) -> Result<(), TypeAbiError> {
    out.write_u8(role_code(key.role));
    out.write_u8(key.arity);
    write_string(out, key.box_key.as_str())?;
    write_string(out, key.name.as_str())?;
    Ok(())
}

fn write_target(out: &mut TypeAbiPayloadSink, target: &BoxCallableTarget) {
    match target {
        BoxCallableTarget::InternalSlot { slot } => {
            out.write_u8(1);
            out.write_u16_le(*slot);
        }
        BoxCallableTarget::PluginMethod {
            type_id,
            method_id,
            returns_result,
        } => {
            out.write_u8(2);
            out.write_u32_le(*type_id);
            out.write_u32_le(*method_id);
            out.write_u8(u8::from(*returns_result));
        }
        BoxCallableTarget::PluginLifecycle {
            type_id,
            birth_id,
            fini_id,
        } => {
            out.write_u8(3);
            out.write_u32_le(*type_id);
            write_optional_u32(out, *birth_id);
            write_optional_u32(out, *fini_id);
        }
        BoxCallableTarget::UserFunction { function_id } => {
            out.write_u8(4);
            out.write_u32_le(function_id.0);
        }
        BoxCallableTarget::Intrinsic { intrinsic_id } => {
            out.write_u8(5);
            out.write_u32_le(intrinsic_id.0);
        }
    }
}

fn write_optional_u32(out: &mut TypeAbiPayloadSink, value: Option<u32>) {
    match value {
        Some(value) => {
            out.write_u8(1);
            out.write_u32_le(value);
        }
        None => {
            out.write_u8(0);
            out.write_u32_le(0);
        }
    }
}

fn write_string(out: &mut TypeAbiPayloadSink, value: &str) -> Result<(), TypeAbiError> {
    let bytes = value.as_bytes();
    let len = u16::try_from(bytes.len())
        .map_err(|_| TypeAbiError::EncodeFailed("box callable string too long"))?;
    out.write_u16_le(len);
    out.write_bytes(bytes);
    Ok(())
}

fn role_code(role: BoxCallableRole) -> u8 {
    match role {
        BoxCallableRole::Birth => 1,
        BoxCallableRole::Fini => 2,
        BoxCallableRole::Method => 3,
        BoxCallableRole::StaticMethod => 4,
        BoxCallableRole::PropertyGet => 5,
        BoxCallableRole::PropertySet => 6,
        BoxCallableRole::Operator => 7,
    }
}

#[cfg(test)]
mod tests {
    use crate::box_callable::providers::plugin_loader::seed_plugin_exports;
    use crate::box_callable::{BoxCallableKey, BoxCallableRegistry, BoxCallableRole};
    use crate::runtime::plugin_loader_v2::PluginCallableExport;

    use super::*;

    #[test]
    fn box_callable_entry_view_projects_registry_entry() {
        let key = BoxCallableKey::new("StringBox", BoxCallableRole::Method, "contains", 1);
        let target = BoxCallableTarget::InternalSlot { slot: 309 };
        let view = BoxCallableEntryView::new(&key, &target);

        assert_eq!(view.abi_tag(), TypeAbiTag::BoxCallable);
        assert_eq!(view.abi_name(), Some("contains"));
        assert_eq!(view.payload_schema(), TYPE_ABI_BOX_CALLABLE_SCHEMA_V0);
        assert_ne!(view.abi_id(), 309);
    }

    #[test]
    fn box_callable_entry_payload_keeps_key_and_target_separate() {
        let key = BoxCallableKey::new("DemoBox", BoxCallableRole::Method, "run", 2);
        let target = BoxCallableTarget::PluginMethod {
            type_id: 42,
            method_id: 7,
            returns_result: true,
        };
        let view = BoxCallableEntryView::new(&key, &target);
        let mut sink = TypeAbiPayloadSink::new();

        view.encode_payload(&mut sink).unwrap();

        let mut expected = Vec::new();
        expected.extend_from_slice(&[
            3, 2, 7, 0, b'D', b'e', b'm', b'o', b'B', b'o', b'x', 3, 0, b'r', b'u', b'n', 2,
        ]);
        expected.extend_from_slice(&42_u32.to_le_bytes());
        expected.extend_from_slice(&7_u32.to_le_bytes());
        expected.push(1);
        assert_eq!(sink.into_vec(), expected);
    }

    #[test]
    fn registry_projection_publishes_catalog_headers() {
        let mut registry = BoxCallableRegistry::new();
        registry.insert(
            BoxCallableKey::new("StringBox", BoxCallableRole::Method, "contains", 1),
            BoxCallableTarget::InternalSlot { slot: 309 },
        );
        registry.insert(
            BoxCallableKey::new("DemoBox", BoxCallableRole::Birth, "birth", 0),
            BoxCallableTarget::PluginLifecycle {
                type_id: 42,
                birth_id: Some(1),
                fini_id: Some(999),
            },
        );
        let mut catalog = TypeAbiCatalog::new();

        let count = publish_box_callable_registry(&mut catalog, &registry);

        assert_eq!(count, 2);
        assert_eq!(catalog.query_by_tag(TypeAbiTag::BoxCallable).len(), 2);
    }

    #[test]
    fn registry_projection_can_build_cold_pack() {
        let mut registry = BoxCallableRegistry::new();
        registry.insert(
            BoxCallableKey::new("StringBox", BoxCallableRole::Method, "contains", 1),
            BoxCallableTarget::InternalSlot { slot: 309 },
        );

        let pack = build_box_callable_registry_pack(&registry).unwrap();

        assert_eq!(pack.entry_count(), 1);
    }

    #[test]
    fn plugin_snapshot_registry_projects_to_empty_catalog() {
        let registry = BoxCallableRegistry::new();

        let catalog = build_catalog_from_box_callable_registry_snapshot(&registry);

        assert!(catalog.is_empty());
    }

    #[test]
    fn plugin_callable_exports_project_to_catalog_through_registry() {
        let exports = [
            PluginCallableExport::Method {
                lib_name: "demo".to_string(),
                box_type: "DemoBox".to_string(),
                type_id: 42,
                method_name: "run".to_string(),
                arity: 2,
                method_id: 7,
                returns_result: true,
            },
            PluginCallableExport::Lifecycle {
                lib_name: "demo".to_string(),
                box_type: "DemoBox".to_string(),
                type_id: 42,
                birth_id: Some(1),
                fini_id: Some(999),
            },
        ];

        let mut registry = BoxCallableRegistry::new();
        seed_plugin_exports(&mut registry, exports.iter());

        let catalog = build_catalog_from_box_callable_registry_snapshot(&registry);
        let entries = catalog.query_by_tag(TypeAbiTag::BoxCallable);

        assert_eq!(entries.len(), 3);
        assert!(entries
            .iter()
            .any(|entry| entry.name.as_deref() == Some("run")));
        assert!(entries
            .iter()
            .any(|entry| entry.name.as_deref() == Some("birth")));
        assert!(entries
            .iter()
            .any(|entry| entry.name.as_deref() == Some("fini")));
    }
}
