use super::*;
use crate::box_trait::IntegerBox;
use crate::runtime::object_identity::ObjectGeneration;
use std::sync::Mutex;

static HOST_HANDLE_POLICY_ENV_LOCK: Mutex<()> = Mutex::new(());

fn with_host_handle_policy_env<F: FnOnce()>(value: &str, f: F) {
    let _guard = HOST_HANDLE_POLICY_ENV_LOCK.lock().expect("env lock");
    let prev = std::env::var("NYASH_HOST_HANDLE_ALLOC_POLICY").ok();
    std::env::set_var("NYASH_HOST_HANDLE_ALLOC_POLICY", value);
    f();
    if let Some(v) = prev {
        std::env::set_var("NYASH_HOST_HANDLE_ALLOC_POLICY", v);
    } else {
        std::env::remove_var("NYASH_HOST_HANDLE_ALLOC_POLICY");
    }
}

fn with_global_host_handles<F: FnOnce()>(f: F) {
    let _guard = HOST_HANDLE_POLICY_ENV_LOCK
        .lock()
        .expect("host handle lock");
    f();
}

fn int_box(value: i64) -> Arc<dyn NyashBox> {
    Arc::new(IntegerBox::new(value))
}

#[test]
fn host_handles_registry_lifo_reuses_dropped_handle() {
    with_host_handle_policy_env("lifo", || {
        let registry = Registry::new();
        let first = registry.alloc(int_box(1));
        registry.drop_handle(first);
        let second = registry.alloc(int_box(2));
        assert_eq!(second, first);
    });
}

#[test]
fn host_handles_registry_none_issues_fresh_handle_after_drop() {
    with_host_handle_policy_env("none", || {
        let registry = Registry::new();
        let first = registry.alloc(int_box(1));
        registry.drop_handle(first);
        let second = registry.alloc(int_box(2));
        assert!(second > first);
        assert_ne!(second, first);
    });
}

#[test]
fn latest_fresh_stable_box_returns_current_object() {
    with_global_host_handles(|| {
        let handle = to_handle_arc(int_box(41));
        let got = with_latest_fresh_stable_box(handle, |obj| {
            obj.as_any()
                .downcast_ref::<IntegerBox>()
                .expect("integer latest fresh object")
                .value
        });
        assert_eq!(got, Some(41));
        drop_handle(handle);
    });
}

#[test]
fn latest_fresh_stable_box_invalidates_after_drop_epoch_changes() {
    with_global_host_handles(|| {
        let handle = to_handle_arc(int_box(52));
        assert!(with_latest_fresh_stable_box(handle, |_| ()).is_some());
        drop_handle(handle);
        assert!(with_latest_fresh_stable_box(handle, |_| ()).is_none());
    });
}

#[test]
fn object_handle_projection_preserves_raw_host_abi() {
    with_global_host_handles(|| {
        let raw = to_handle_arc(int_box(63));
        let handle = to_object_handle(raw).expect("non-zero object handle");

        assert_eq!(to_raw_handle(handle), raw);
        assert_eq!(ObjectHandle::new(0), None);

        drop_handle(raw);
    });
}

#[test]
fn live_host_handle_identity_is_legacy_generation() {
    with_global_host_handles(|| {
        let raw = to_handle_arc(int_box(74));
        let box_identity = identity(raw).expect("live host handle identity");

        assert_eq!(box_identity.handle().raw(), raw);
        assert_eq!(
            box_identity.generation(),
            ObjectGeneration::LEGACY_UNVERSIONED
        );

        drop_handle(raw);
        assert_eq!(identity(raw), None);
    });
}

#[test]
fn with_object_handle_borrows_without_arc_clone_api() {
    with_global_host_handles(|| {
        let raw = to_handle_arc(int_box(85));
        let handle = to_object_handle(raw).expect("object handle");

        let got = with_object_handle(handle, |obj| {
            obj.and_then(|obj| obj.as_i64_fast())
                .expect("integer value")
        });
        assert_eq!(got, 85);

        drop_handle(raw);
    });
}

#[test]
fn identity_descriptor_reports_current_strong_root() {
    with_global_host_handles(|| {
        let raw = to_handle_arc(int_box(96));
        let descriptor = descriptor(raw).expect("identity descriptor");

        assert_eq!(descriptor.identity.handle().raw(), raw);
        assert_eq!(descriptor.root_visibility, RootVisibility::StrongRoot);
        assert_eq!(descriptor.fini_owner, FiniOwner::ObjectDrop);

        drop_handle(raw);
    });
}

#[test]
fn identity_snapshot_includes_live_handle() {
    with_global_host_handles(|| {
        let raw = to_handle_arc(int_box(107));
        let handle = to_object_handle(raw).expect("object handle");
        let snapshot = identity_snapshot();

        assert!(snapshot
            .iter()
            .any(|entry| entry.identity.handle() == handle));

        drop_handle(raw);
    });
}

#[test]
fn text_payload_handle_reads_without_arc_payload() {
    with_global_host_handles(|| {
        let raw = to_handle_text("arc-free-text");
        let got = with_str_handle(raw, |text| text.to_string());

        assert_eq!(got.as_deref(), Some("arc-free-text"));
        assert!(with_handle(raw, |obj| obj.is_none()));

        drop_handle(raw);
    });
}

#[test]
fn text_payload_get_materializes_compat_string_box() {
    with_global_host_handles(|| {
        let raw = to_handle_text("compat-text");
        let got = get(raw).expect("compat materialized string box");

        assert_eq!(got.as_str_fast(), Some("compat-text"));
        assert_eq!(got.type_name(), "StringBox");

        drop_handle(raw);
    });
}

#[test]
fn text_payload_identity_has_no_fini_owner() {
    with_global_host_handles(|| {
        let raw = to_handle_text("identity-text");
        let descriptor = descriptor(raw).expect("text identity descriptor");

        assert_eq!(descriptor.identity.handle().raw(), raw);
        assert_eq!(descriptor.root_visibility, RootVisibility::StrongRoot);
        assert_eq!(descriptor.fini_owner, FiniOwner::None);

        drop_handle(raw);
    });
}

#[test]
fn host_handle_identity_report_fields_are_explicit() {
    let fields = host_handle_identity_report_fields();

    assert!(fields.contains(&("external_host_abi_changed", "0")));
    assert!(fields.contains(&("object_handle_contract_used_by_host_handles", "1")));
    assert!(fields.contains(&("borrowed_access_preserved", "1")));
    assert!(fields.contains(&("host_handle_backing_arc_replaced", "0")));
    assert!(fields.contains(&("host_handle_text_payload_arc_replaced", "1")));
}
