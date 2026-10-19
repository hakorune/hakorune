//! Atomic field payload/outcome transport under the existing object guard.
//! These checks establish storage identity, never source ownership permission.
use super::*;
use crate::exports::typed_object::ObjectResidenceOutcomeV1;

fn validate_child(
    objects: &IndexedObjects,
    value: i64,
    child_type: i64,
    outcome: ObjectResidenceOutcomeV1,
) -> Option<()> {
    if !outcome.accepts_payload(value) {
        return None;
    }
    if outcome == ObjectResidenceOutcomeV1::Null {
        return Some(());
    }
    let child = objects.get(handle_to_index(value)?)?.as_ref()?;
    (child.type_id == child_type).then_some(())
}

pub(in crate::exports) fn install_object_residence_checked(
    expected: TypedObjectStoreBackend,
    handle: i64,
    type_id: i64,
    slot: usize,
    child_type: i64,
    value: i64,
    outcome: ObjectResidenceOutcomeV1,
) -> Result<(), CheckedStorageError> {
    check_indexed_profile(expected)?;
    with_objects_mut(|objects| {
        // Do not re-enter the backend while its mutex/RefCell is borrowed.
        validate_child(objects, value, child_type, outcome)?;
        let parent = objects.get_mut(handle_to_index(handle)?)?.as_mut()?;
        if parent.type_id != type_id {
            return None;
        }
        parent
            .fields
            .get_mut(slot)?
            .install_object_residence(value, outcome)
            .then_some(())
    })
    .flatten()
    .ok_or(CheckedStorageError::ObjectOrFieldMismatch)
}

pub(in crate::exports) fn get_object_residence_checked(
    expected: TypedObjectStoreBackend,
    handle: i64,
    type_id: i64,
    slot: usize,
    child_type: i64,
) -> Result<(i64, ObjectResidenceOutcomeV1), CheckedStorageError> {
    check_indexed_profile(expected)?;
    with_objects(|objects| {
        let parent = objects.get(handle_to_index(handle)?)?.as_ref()?;
        if parent.type_id != type_id {
            return None;
        }
        let (value, outcome) = parent.fields.get(slot)?.object_residence()?;
        validate_child(objects, value, child_type, outcome)?;
        Some((value, outcome))
    })
    .flatten()
    .ok_or(CheckedStorageError::ObjectOrFieldMismatch)
}

#[cfg(test)]
pub(super) fn check_atomic_residence_operations(profile: TypedObjectStoreBackend) {
    use ObjectResidenceOutcomeV1::{Forward, Fresh, Null};
    if !matches!(
        profile,
        TypedObjectStoreBackend::SafeMutex | TypedObjectStoreBackend::SingleThreadExact
    ) {
        assert_eq!(
            install_object_residence_checked(profile, -1, 901, 0, 902, 0, Null),
            Err(CheckedStorageError::ProfileMismatch)
        );
        assert_eq!(
            get_object_residence_checked(profile, -1, 901, 0, 902),
            Err(CheckedStorageError::ProfileMismatch)
        );
        return;
    }
    let parent = new_checked_indexed(profile, 901, &[TypedSlotStorage::I64; 3]).unwrap();
    let child = new_checked_indexed(profile, 902, &[]).unwrap();
    // Observe the existing I64 payload/outcome without cloning its slot.
    let snapshot = |field: &TypedSlot| (field.storage, field.as_compat_i64(), field.residence);
    for (slot, value, outcome) in [(0, 0, Null), (1, child, Fresh), (2, child, Forward)] {
        let before = with_field(parent, slot, snapshot).unwrap();
        for (wrong_parent, wrong_type, wrong_slot, wrong_child, bad_value, bad_outcome) in [
            (parent, 900, slot, 902, value, outcome),
            (parent, 901, 9, 902, value, outcome),
            (i64::MIN, 901, slot, 902, value, outcome),
            (parent, 901, slot, 902, child, Null),
            (parent, 901, slot, 902, 0, Fresh),
            (parent, 901, slot, 902, 0, Forward),
            (parent, 901, slot, 903, child, Fresh),
            (parent, 901, slot, 902, i64::MIN, Forward),
        ] {
            assert!(install_object_residence_checked(
                profile,
                wrong_parent,
                wrong_type,
                wrong_slot,
                wrong_child,
                bad_value,
                bad_outcome
            )
            .is_err());
            assert_eq!(with_field(parent, slot, snapshot).unwrap(), before);
        }
        install_object_residence_checked(profile, parent, 901, slot, 902, value, outcome).unwrap();
        assert_eq!(
            get_object_residence_checked(profile, parent, 901, slot, 902).unwrap(),
            (value, outcome)
        );
        assert!(
            install_object_residence_checked(profile, parent, 901, slot, 902, value, outcome)
                .is_err()
        );
        assert!(set_checked_indexed(profile, parent, 901, slot, 44).is_err());
        assert_eq!(
            with_field_mut(parent, slot, |field| field.set_compat_i64(44)),
            Some(false)
        );
        assert_eq!(
            with_field_mut(parent, slot, |field| field.set_i64_exact(44)),
            Some(false)
        );
        assert_eq!(
            with_field_mut(parent, slot, |field| field.set_exact_unsigned_u64(44)),
            Some(false)
        );
        assert_eq!(
            get_object_residence_checked(profile, parent, 901, slot, 902).unwrap(),
            (value, outcome)
        );
        assert!(get_object_residence_checked(profile, parent, 900, slot, 902).is_err());
        if outcome != Null {
            assert!(get_object_residence_checked(profile, parent, 901, slot, 903).is_err());
        }
    }
    reclaim_checked_indexed(profile, parent, 901).unwrap();
    reclaim_checked_indexed(profile, child, 902).unwrap();
}
