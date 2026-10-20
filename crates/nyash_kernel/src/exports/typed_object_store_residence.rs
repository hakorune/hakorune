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


use crate::exports::typed_object::{DynamicFieldStoredV1, DynamicFieldValueV1, TypedSlotValue};

fn validate_dynamic(objects: &IndexedObjects, candidate: &DynamicFieldStoredV1, profile: TypedObjectStoreBackend) -> Option<()> {
    let profile = match profile { TypedObjectStoreBackend::SafeMutex => 1,
        TypedObjectStoreBackend::SingleThreadExact => 2, _ => return None };
    let DynamicFieldStoredV1::Published { value, end } = candidate else { return None; };
    match (value.kind, *end) {
        (0..=3, None) => {}
        (4..=6, Some(end)) if end.profile == profile && end.kind == value.kind
            && end.end.is_some() && if end.kind == 5 { end.type_id > 0 } else { end.type_id == 0 } => {}
        _ => return None,
    }
    if !value.well_formed() { return None; }
    match value.kind {
        0..=3 => end.is_none().then_some(()),
        4 => {
            let token = std::num::NonZeroU64::new(value.token)?;
            nyash_rust::runtime::dynamic_v2_lease::with_end_authorized_text_pair(
                value.value as u64, token, |_| ()).ok()
        }
        5 => {
            let descriptor = (*end)?;
            let child = objects.get(handle_to_index(value.value)?)?.as_ref()?;
            (child.type_id == descriptor.type_id).then_some(())
        }
        6 => crate::plugin::with_array_box_direct(value.value, |_| ()),
        _ => None,
    }
}

/// All fallible validation precedes the swap. Err drops only a prepared wrapper,
/// not the caller's still-owned resource. Old residence leaves the guard by move.
pub(in crate::exports) fn replace_dynamic_field_checked(
    expected: TypedObjectStoreBackend, handle: i64, type_id: i64, slot: usize,
    candidate: DynamicFieldStoredV1, require_empty: bool,
) -> Result<DynamicFieldStoredV1, CheckedStorageError> {
    check_indexed_profile(expected)?;
    with_objects_mut(|objects| {
        validate_dynamic(objects, &candidate, expected)?;
        let DynamicFieldStoredV1::Published { value, .. } = &candidate else { return None; };
        if value.kind == 5 && value.value == handle {
            // A direct owning self-cycle has no admitted source support plan.
            return None;
        }
        let old = dynamic_slot_mut(objects, handle, type_id, slot)?;
        if require_empty && !old.is_empty() { return None; }
        // This profile transfers unique obligations, not independently Shared
        // Homes. A borrowed same-slot read cannot reacquire that obligation.
        if let DynamicFieldStoredV1::Published { value: previous, .. } = old {
            if value.kind >= 4 && previous == value { return None; }
        }
        Some(std::mem::replace(old, candidate))
    }).flatten().ok_or(CheckedStorageError::ObjectOrFieldMismatch)
}

pub(in crate::exports) fn read_dynamic_field_checked(
    expected: TypedObjectStoreBackend, handle: i64, type_id: i64, slot: usize,
    kind: u32,
) -> Result<DynamicFieldValueV1, CheckedStorageError> {
    check_indexed_profile(expected)?;
    with_objects(|objects| {
        let parent = objects.get(handle_to_index(handle)?)?.as_ref()?;
        if parent.type_id != type_id { return None; }
        let field = parent.fields.get(slot)?;
        if field.storage != TypedSlotStorage::Dynamic || field.residence.is_some() { return None; }
        let TypedSlotValue::Dynamic(value) = &field.value else { return None; };
        if value.is_empty() { return None; }
        validate_dynamic(objects, value, expected)?;
        let DynamicFieldStoredV1::Published { value, .. } = value else { return None; };
        if kind != 0 && value.kind != kind { return None; }
        Some(*value) // observation only: no End descriptor or new Home
    }).flatten().ok_or(CheckedStorageError::ObjectOrFieldMismatch)
}

/// Explicit teardown detaches the SAME current residence, leaving known Empty.
/// A plain object reclaim cannot silently discard a nonempty Dynamic slot.
pub(in crate::exports) fn take_dynamic_field_checked(
    expected: TypedObjectStoreBackend, handle: i64, type_id: i64, slot: usize,
) -> Result<DynamicFieldStoredV1, CheckedStorageError> {
    check_indexed_profile(expected)?;
    with_objects_mut(|objects| {
        let old = dynamic_slot_mut(objects, handle, type_id, slot)?;
        Some(std::mem::replace(old, DynamicFieldStoredV1::empty()))
    }).flatten().ok_or(CheckedStorageError::ObjectOrFieldMismatch)
}

fn dynamic_slot_mut(objects: &mut IndexedObjects, handle: i64, type_id: i64, slot: usize)
    -> Option<&mut DynamicFieldStoredV1>
{
    let parent = objects.get_mut(handle_to_index(handle)?)?.as_mut()?;
    if parent.type_id != type_id { return None; }
    let field = parent.fields.get_mut(slot)?;
    if field.storage != TypedSlotStorage::Dynamic || field.residence.is_some() { return None; }
    let TypedSlotValue::Dynamic(value) = &mut field.value else { return None; };
    Some(value)
}

/// All destination validation holds the SAME guard through the source move.
/// On rejection only uninitialized destination bytes are freed; source stays live.
pub(in crate::exports) fn install_map_field_checked(
    expected: TypedObjectStoreBackend, handle: i64, type_id: i64, slot: usize,
    prepared: crate::exports::fault::PreparedMapMoveV1<'_>,
) -> Result<DynamicFieldStoredV1, CheckedStorageError> {
    check_indexed_profile(expected)?;
    if expected != TypedObjectStoreBackend::SafeMutex { return Err(CheckedStorageError::ProfileMismatch); }
    with_objects_mut(|objects| {
        let old = dynamic_slot_mut(objects, handle, type_id, slot)?;
        // Construction first-write is a runtime condition as well: a repeated
        // Birth or borrowed field pointer cannot turn old-End into a store Fault.
        if !old.is_empty() { return None; }
        let candidate = DynamicFieldStoredV1::Map(prepared.commit());
        Some(std::mem::replace(old, candidate))
    }).flatten().ok_or(CheckedStorageError::ObjectOrFieldMismatch)
}

/// Borrow-only view, authorized by a retained source field loan. Generic tagged
/// reads never expose this address or accept it as a candidate Home.
pub(in crate::exports) fn read_map_field_checked(
    expected: TypedObjectStoreBackend, handle: i64, type_id: i64, slot: usize,
) -> Result<*mut std::ffi::c_void, CheckedStorageError> {
    check_indexed_profile(expected)?;
    if expected != TypedObjectStoreBackend::SafeMutex { return Err(CheckedStorageError::ProfileMismatch); }
    with_objects(|objects| {
        let parent = objects.get(handle_to_index(handle)?)?.as_ref()?;
        if parent.type_id != type_id { return None; }
        let field = parent.fields.get(slot)?;
        if field.storage != TypedSlotStorage::Dynamic || field.residence.is_some() { return None; }
        let TypedSlotValue::Dynamic(DynamicFieldStoredV1::Map(map)) = &field.value else { return None; };
        Some(map.as_ptr())
    }).flatten().ok_or(CheckedStorageError::ObjectOrFieldMismatch)
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
