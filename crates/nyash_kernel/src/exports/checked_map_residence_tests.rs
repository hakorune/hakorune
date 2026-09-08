use super::*;
use crate::exports::typed_object::TypedSlotStorage;
use crate::exports::typed_object_store_backend::{get_checked_indexed, new_checked_indexed};
use nyash_rust::boxes::{
    map_box::checked::{CheckedMap, CheckedMapError},
    map_key_domain::MapKeyDomain,
};

#[test]
fn checked_map_uses_real_indexed_residence_without_native_publication() {
    let profile = TypedObjectStoreBackend::SafeMutex;
    let allocate = || new_checked_indexed(profile, 904, &[TypedSlotStorage::I64]).unwrap();
    let live = |handle| get_checked_indexed(profile, handle, 904, 0).is_ok();
    let a = allocate();
    let b = allocate();
    let c = allocate();
    assert!(matches!(
        prepare(TypedObjectStoreBackend::SingleThreadExact, a, 904),
        Err(CheckedStorageError::ProfileMismatch)
    ));
    assert!(matches!(
        prepare(profile, a, 905),
        Err(CheckedStorageError::ObjectOrFieldMismatch)
    ));
    assert!(live(a));
    let map = CheckedMap::unissued();
    let rejected = map
        .install(
            MapKeyDomain::from_text("a"),
            prepare(profile, a, 904).unwrap(),
        )
        .err()
        .expect("unissued install rejects");
    assert_eq!(rejected.error, CheckedMapError::InvalidState);
    drop(rejected.candidate); // wrapper refusal does not reclaim the source's object
    assert!(live(a));
    map.acquire().unwrap();
    map.install(
        MapKeyDomain::from_text("a"),
        prepare(profile, a, 904).unwrap(),
    )
    .ok()
    .unwrap()
    .end()
    .unwrap();
    map.install(
        MapKeyDomain::from_text("b"),
        prepare(profile, b, 904).unwrap(),
    )
    .ok()
    .unwrap()
    .end()
    .unwrap();
    let old = map
        .install(
            MapKeyDomain::from_text("a"),
            prepare(profile, c, 904).unwrap(),
        )
        .ok()
        .unwrap();
    assert!(live(a) && live(b) && live(c));
    old.end().unwrap();
    assert!(!live(a) && live(b) && live(c));
    assert!(matches!(
        map.observe_native(&MapKeyDomain::from_text("a")),
        Err(CheckedMapError::ProjectionUnavailable)
    ));
    assert_eq!(map.end().unwrap().first, None);
    assert!(!live(a) && !live(b) && !live(c));
    assert_eq!(map.end(), Err(CheckedMapError::InvalidState));
    map.require_disposable().unwrap();
}

#[test]
fn failed_old_end_does_not_undo_new_install_or_skip_remaining_real_payloads() {
    let profile = TypedObjectStoreBackend::SafeMutex;
    let a = new_checked_indexed(profile, 905, &[]).unwrap();
    let b = new_checked_indexed(profile, 905, &[]).unwrap();
    let map = CheckedMap::unissued();
    map.acquire().unwrap();
    map.install(MapKeyDomain::from_i64(1), prepare(profile, a, 905).unwrap())
        .ok()
        .unwrap()
        .end()
        .unwrap();
    let old = map
        .install(
            MapKeyDomain::from_text("1"),
            prepare(profile, b, 905).unwrap(),
        )
        .ok()
        .unwrap();
    // Physical stale-identity negative test; this is not a second source owner.
    reclaim_checked_indexed(profile, a, 905).unwrap();
    assert_eq!(old.end(), Err(MapEndError::InvalidIdentity));
    assert_eq!(map.end().unwrap().first, None);
    assert!(matches!(
        prepare(profile, b, 905),
        Err(CheckedStorageError::ObjectOrFieldMismatch)
    ));
}
