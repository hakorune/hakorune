use super::*;
#[test]
fn poisoned_store_is_unavailable_not_missing_identity() {
    const CHILD: &str = "HAKO_TEST_MAP_IDENTITY_POISON";
    if std::env::var_os(CHILD).is_some() {
        let profile = TypedObjectStoreBackend::SafeMutex;
        let handle = new_checked_indexed(profile, 920, &[]).unwrap();
        let _ = std::panic::catch_unwind(|| {
            let _store = safe_mutex_objects().lock().unwrap();
            panic!("isolated real storage poison");
        });
        assert_eq!(
            validate_checked_indexed_identity(profile, handle, 920),
            Err(CheckedStorageError::AllocationOrStorageUnavailable)
        );
        reclaim_checked_indexed(profile, handle, 920).unwrap();
        return;
    }
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["exports::typed_object_store_backend::map_identity_tests::poisoned_store_is_unavailable_not_missing_identity", "--exact", "--test-threads=1"])
        .env(CHILD, "1").env(TYPED_OBJECT_STORE_ENV, "safe_mutex").output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("1 passed"));
}
