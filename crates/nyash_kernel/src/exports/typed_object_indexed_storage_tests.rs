use super::*;

#[test]
fn unknown_checked_profile_rejects_without_initializing_storage() {
    const UNKNOWN: &str = "fault-test-unknown-profile";
    if std::env::var(TYPED_OBJECT_STORE_ENV).as_deref() == Ok(UNKNOWN) {
        assert_eq!(
            new_checked_indexed(TypedObjectStoreBackend::SafeMutex, 901, &[]),
            Err(CheckedStorageError::ProfileMismatch)
        );
        assert_eq!(
            set_checked_indexed(TypedObjectStoreBackend::SafeMutex, -1, 901, 0, 1),
            Err(CheckedStorageError::ProfileMismatch)
        );
        assert_eq!(
            reclaim_checked_indexed(TypedObjectStoreBackend::SafeMutex, -1, 901),
            Err(CheckedStorageError::ProfileMismatch)
        );
        assert!(BACKEND.get().is_none());
        assert!(SAFE_MUTEX_OBJECTS.get().is_none());
        return;
    }
    let result = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["exports::typed_object_store_backend::indexed_storage_tests::unknown_checked_profile_rejects_without_initializing_storage", "--exact", "--test-threads=1"])
        .env(TYPED_OBJECT_STORE_ENV, UNKNOWN).output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stdout).contains("1 passed"));
}

#[test]
fn checked_operations_preserve_identity_and_reject_before_mutation() {
    let profile = selected_backend();
    residence::check_atomic_residence_operations(profile);
    if !matches!(
        profile,
        TypedObjectStoreBackend::SafeMutex | TypedObjectStoreBackend::SingleThreadExact
    ) {
        assert_eq!(
            new_checked_indexed(profile, 901, &[]),
            Err(CheckedStorageError::ProfileMismatch)
        );
        assert_eq!(
            set_checked_indexed(profile, -1, 901, 0, 1),
            Err(CheckedStorageError::ProfileMismatch)
        );
        assert_eq!(
            reclaim_checked_indexed(profile, -1, 901),
            Err(CheckedStorageError::ProfileMismatch)
        );
        return;
    }
    let other = if profile == TypedObjectStoreBackend::SafeMutex {
        TypedObjectStoreBackend::SingleThreadExact
    } else {
        TypedObjectStoreBackend::SafeMutex
    };
    assert_eq!(
        new_checked_indexed(other, 901, &[]),
        Err(CheckedStorageError::ProfileMismatch)
    );
    assert_eq!(
        new_checked_indexed(profile, 901, &[TypedSlotStorage::Handle]),
        Err(CheckedStorageError::InvalidLayout)
    );
    let handle = new_checked_indexed(profile, 901, &[TypedSlotStorage::I64]).unwrap();
    set_checked_indexed(profile, handle, 901, 0, 42).unwrap();
    assert_eq!(
        set_checked_indexed(profile, handle, 902, 0, 99),
        Err(CheckedStorageError::ObjectOrFieldMismatch)
    );
    assert_eq!(
        set_checked_indexed(profile, handle, 901, 1, 99),
        Err(CheckedStorageError::ObjectOrFieldMismatch)
    );
    assert_eq!(
        with_field(handle, 0, |field| field.as_exact_signed_i64()),
        Some(Some(42))
    );
    assert_eq!(
        reclaim_checked_indexed(profile, handle, 902),
        Err(CheckedStorageError::ObjectOrFieldMismatch)
    );
    reclaim_checked_indexed(profile, handle, 901).unwrap();
    assert_eq!(
        reclaim_checked_indexed(profile, handle, 901),
        Err(CheckedStorageError::ObjectOrFieldMismatch)
    );
}

fn object(type_id: i64) -> TypedSlotObject {
    TypedSlotObject {
        dispatch: None,
        type_id,
        fields: vec![TypedSlot::new(TypedSlotStorage::I64)],
    }
}

#[test]
fn reclaim_keeps_neighbors_and_never_reuses_a_handle() {
    let owner = Mutex::new(Vec::new());
    for id in [11, 22, 33] {
        insert_indexed(&mut owner.lock().unwrap(), object(id)).unwrap();
    }
    let detached = detach_mutex(&owner, -2, 22).unwrap();
    // Detachment ends the storage lock before the caller drops the payload.
    let mut slots = owner.try_lock().unwrap();
    assert_eq!(slots[0].as_ref().unwrap().type_id, 11);
    assert!(slots[1].is_none());
    assert_eq!(slots[2].as_ref().unwrap().type_id, 33);
    assert_eq!(insert_indexed(&mut slots, object(44)), Some(-4));
    assert!(take_indexed(&mut slots, -2, 22).is_none());
    assert!(slots.get_mut(1).and_then(Option::as_mut).is_none());
    drop(slots);
    drop(detached);
}

#[test]
fn invalid_or_mismatched_reclaim_does_not_take_storage() {
    let mut slots = vec![Some(object(11)), None];
    for (handle, ty) in [
        (0, 11),
        (1, 11),
        (i64::MIN, 11),
        (-3, 11),
        (-2, 11),
        (-1, 12),
    ] {
        assert!(take_indexed(&mut slots, handle, ty).is_none());
        assert_eq!(slots.len(), 2);
        assert_eq!(slots[0].as_ref().unwrap().type_id, 11);
        assert!(slots[1].is_none());
    }
    assert_eq!(next_indexed_handle(0), Some(-1));
    if let Ok(limit) = usize::try_from(i64::MAX) {
        assert_eq!(next_indexed_handle(limit), None);
    }
}

#[test]
fn reclaim_recovers_poison_without_reenabling_ordinary_access() {
    let owner = Mutex::new(vec![Some(object(11))]);
    let panic = std::panic::catch_unwind(|| {
        let _guard = owner.lock().unwrap();
        panic!("poison local test owner");
    });
    assert!(panic.is_err());
    assert!(owner.lock().is_err());
    let detached = detach_mutex(&owner, -1, 11).unwrap();
    assert!(owner.is_poisoned());
    assert!(owner.try_lock().is_err());
    assert!(detach_mutex(&owner, -1, 11).is_none());
    drop(detached);
}

#[test]
fn local_reclaim_respects_borrow_and_releases_it_before_payload_drop() {
    let owner = RefCell::new(vec![Some(object(11)), Some(object(22))]);
    let borrowed = owner.borrow();
    assert!(detach_local(&owner, -1, 11).is_none());
    assert!(borrowed[0].is_some());
    drop(borrowed);
    let detached = detach_local(&owner, -1, 11).unwrap();
    let mut slots = owner.try_borrow_mut().unwrap();
    assert!(slots[0].is_none());
    assert_eq!(slots[1].as_ref().unwrap().type_id, 22);
    assert_eq!(insert_indexed(&mut slots, object(33)), Some(-3));
    drop(slots);
    drop(detached);
    assert!(detach_local(&owner, -1, 11).is_none());
}

#[test]
fn rejected_scalar_store_preserves_the_committed_prefix() {
    let mut first = TypedSlot::new(TypedSlotStorage::I64);
    let mut second = TypedSlot::new(TypedSlotStorage::I8);
    assert!(first.set_exact_signed_i64(10));
    assert!(second.set_exact_signed_i64(20));
    assert!(!second.set_exact_signed_i64(128));
    assert_eq!(first.as_exact_signed_i64(), Some(10));
    assert_eq!(second.as_exact_signed_i64(), Some(20));
    assert!(second.set_exact_signed_i64(21));
    assert_eq!(second.as_exact_signed_i64(), Some(21));
    let mut handle = TypedSlot::new(TypedSlotStorage::Handle);
    assert!(!handle.set_exact_signed_i64(1));
}

#[test]
fn selected_storage_reclaim_invalidates_every_indexed_accessor() {
    if matches!(
        selected_backend(),
        TypedObjectStoreBackend::PinnedArenaExact | TypedObjectStoreBackend::DirectSlotExact
    ) {
        assert!(!reclaim_typed_object_storage(-1, 901));
        return;
    }
    let first = new_typed_object(object(901)).unwrap();
    let middle = new_typed_object(object(902)).unwrap();
    let last = new_typed_object(object(903)).unwrap();
    assert!(with_field_mut(middle, 0, |field| field.set_exact_signed_i64(42)).unwrap());
    assert!(!reclaim_typed_object_storage(middle, 901));
    assert_eq!(
        with_field(middle, 0, |field| field.as_exact_signed_i64()),
        Some(Some(42))
    );
    assert!(reclaim_typed_object_storage(middle, 902));
    assert_eq!(typed_object_type_id(middle), None);
    assert!(with_field(middle, 0, |_| ()).is_none());
    assert!(with_field_mut(middle, 0, |_| ()).is_none());
    assert!(with_exact_fields_mut(middle, |_| ()).is_none());
    assert!(!reclaim_typed_object_storage(middle, 902));
    assert_eq!(typed_object_type_id(first), Some(901));
    assert_eq!(typed_object_type_id(last), Some(903));
    let fresh = new_typed_object(object(904)).unwrap();
    assert!(fresh < last, "new index, never the reclaimed handle");
    for (handle, ty) in [(first, 901), (last, 903), (fresh, 904)] {
        assert!(reclaim_typed_object_storage(handle, ty));
    }
}
