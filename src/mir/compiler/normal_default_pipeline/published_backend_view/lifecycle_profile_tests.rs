use super::*;

#[test]
fn storage_profile_is_finite_and_rejects_legacy_backends() {
    assert_eq!(
        PublishedObjectStorageProfileV1::from_runtime_name(None).unwrap(),
        PublishedObjectStorageProfileV1::SafeMutex
    );
    assert_eq!(
        PublishedObjectStorageProfileV1::from_runtime_name(Some("single_thread_exact")).unwrap(),
        PublishedObjectStorageProfileV1::SingleThreadExact
    );
    for rejected in ["direct_slot_exact", "pinned_arena_exact", "unknown"] {
        assert!(PublishedObjectStorageProfileV1::from_runtime_name(Some(rejected)).is_err());
    }
}
