//! Backend state and routing helpers for typed object storage.
//!
//! `typed_object_store` stays focused on the exported API surface. This module
//! owns the backend selection, object storage, and field routing details.

use std::cell::RefCell;
use std::sync::{Mutex, OnceLock};

use super::typed_object::{handle_to_index, TypedSlot, TypedSlotObject, TypedSlotStorage};
use super::typed_object_direct_slot_backend::{
    direct_slot_object_type_id, new_direct_slot_object, with_direct_slot_materialized_view,
    with_direct_slot_materialized_view_mut, with_direct_slot_object_mut,
};
use super::typed_object_pinned_arena::PinnedTypedObjectArena;
use crate::backend_env::{cached_env_choice, panic_unsupported_env_value, try_cached_env_choice};

pub(super) const TYPED_OBJECT_STORE_ENV: &str = "HAKO_TYPED_OBJECT_STORE";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TypedObjectStoreBackend {
    SafeMutex,
    SingleThreadExact,
    PinnedArenaExact,
    DirectSlotExact,
}

static BACKEND: OnceLock<TypedObjectStoreBackend> = OnceLock::new();
// Never shift or reuse a slot: negative handles encode its permanent index.
// Vacant metadata remains; taking a payload releases its field allocation only.
type IndexedObjects = Vec<Option<TypedSlotObject>>;
static SAFE_MUTEX_OBJECTS: OnceLock<Mutex<IndexedObjects>> = OnceLock::new();

thread_local! {
    static SINGLE_THREAD_OBJECTS: RefCell<IndexedObjects> = const { RefCell::new(Vec::new()) };
    static PINNED_ARENA_OBJECTS: RefCell<PinnedTypedObjectArena> = RefCell::new(PinnedTypedObjectArena::new());
}

pub(super) fn selected_backend() -> TypedObjectStoreBackend {
    cached_env_choice(&BACKEND, TYPED_OBJECT_STORE_ENV, |value| {
        parse_backend(value).unwrap_or_else(|()| panic_unsupported_env_value(
            "typed-object-store/backend", TYPED_OBJECT_STORE_ENV, value.unwrap_or("")))
    })
}

fn parse_backend(value: Option<&str>) -> Result<TypedObjectStoreBackend, ()> {
    match value {
        None | Some("") | Some("safe_mutex") => Ok(TypedObjectStoreBackend::SafeMutex),
        Some("single_thread_exact") => Ok(TypedObjectStoreBackend::SingleThreadExact),
        Some("pinned_arena_exact") => Ok(TypedObjectStoreBackend::PinnedArenaExact),
        Some("direct_slot_exact") => Ok(TypedObjectStoreBackend::DirectSlotExact),
        Some(_) => Err(()),
    }
}

fn safe_mutex_objects() -> &'static Mutex<IndexedObjects> {
    SAFE_MUTEX_OBJECTS.get_or_init(|| Mutex::new(Vec::new()))
}

fn with_objects<R>(f: impl FnOnce(&IndexedObjects) -> R) -> Option<R> {
    match selected_backend() {
        TypedObjectStoreBackend::SafeMutex => {
            let objects = safe_mutex_objects().lock().ok()?;
            Some(f(&objects))
        }
        TypedObjectStoreBackend::SingleThreadExact => SINGLE_THREAD_OBJECTS
            .with(|objects| objects.try_borrow().ok().map(|objects| f(&objects))),
        TypedObjectStoreBackend::PinnedArenaExact | TypedObjectStoreBackend::DirectSlotExact => {
            None
        }
    }
}

fn with_objects_mut<R>(f: impl FnOnce(&mut IndexedObjects) -> R) -> Option<R> {
    match selected_backend() {
        TypedObjectStoreBackend::SafeMutex => {
            let mut objects = safe_mutex_objects().lock().ok()?;
            Some(f(&mut objects))
        }
        TypedObjectStoreBackend::SingleThreadExact => SINGLE_THREAD_OBJECTS.with(|objects| {
            objects
                .try_borrow_mut()
                .ok()
                .map(|mut objects| f(&mut objects))
        }),
        TypedObjectStoreBackend::PinnedArenaExact | TypedObjectStoreBackend::DirectSlotExact => {
            None
        }
    }
}

pub(super) fn with_field<R>(
    handle: i64,
    slot: usize,
    f: impl FnOnce(&TypedSlot) -> R,
) -> Option<R> {
    match selected_backend() {
        TypedObjectStoreBackend::PinnedArenaExact => PINNED_ARENA_OBJECTS.with(|objects| {
            let objects = objects.try_borrow().ok()?;
            objects.get_field(handle, slot).map(f)
        }),
        TypedObjectStoreBackend::DirectSlotExact => {
            with_direct_slot_materialized_view(handle, |object| {
                let field = object.fields.get(slot)?;
                Some(f(field))
            })
        }
        _ => {
            let idx = handle_to_index(handle)?;
            with_objects(|objects| {
                let field = objects.get(idx)?.as_ref()?.fields.get(slot)?;
                Some(f(field))
            })?
        }
    }
}

pub(super) fn with_field_mut<R>(
    handle: i64,
    slot: usize,
    f: impl FnOnce(&mut TypedSlot) -> R,
) -> Option<R> {
    match selected_backend() {
        TypedObjectStoreBackend::PinnedArenaExact => PINNED_ARENA_OBJECTS.with(|objects| {
            let mut objects = objects.try_borrow_mut().ok()?;
            objects.get_field_mut(handle, slot).map(f)
        }),
        TypedObjectStoreBackend::DirectSlotExact => {
            with_direct_slot_materialized_view_mut(handle, |object| {
                let field = object.fields.get_mut(slot)?;
                Some(f(field))
            })
        }
        _ => {
            let idx = handle_to_index(handle)?;
            with_objects_mut(|objects| {
                let field = objects.get_mut(idx)?.as_mut()?.fields.get_mut(slot)?;
                Some(f(field))
            })?
        }
    }
}

pub(super) fn typed_object_type_id(handle: i64) -> Option<i64> {
    match selected_backend() {
        TypedObjectStoreBackend::DirectSlotExact => direct_slot_object_type_id(handle),
        _ => {
            let idx = handle_to_index(handle)?;
            with_objects(|objects| Some(objects.get(idx)?.as_ref()?.type_id))?
        }
    }
}

pub(super) fn with_exact_fields_mut<R>(
    handle: i64,
    f: impl FnOnce(&mut [TypedSlot]) -> R,
) -> Option<R> {
    match selected_backend() {
        TypedObjectStoreBackend::SafeMutex => None,
        TypedObjectStoreBackend::SingleThreadExact => SINGLE_THREAD_OBJECTS.with(|objects| {
            let mut objects = objects.try_borrow_mut().ok()?;
            let idx = handle_to_index(handle)?;
            let object = objects.get_mut(idx)?.as_mut()?;
            Some(f(&mut object.fields))
        }),
        TypedObjectStoreBackend::PinnedArenaExact => PINNED_ARENA_OBJECTS.with(|objects| {
            let mut objects = objects.try_borrow_mut().ok()?;
            objects.get_fields_mut(handle).map(f)
        }),
        TypedObjectStoreBackend::DirectSlotExact => {
            with_direct_slot_materialized_view_mut(handle, |object| Some(f(&mut object.fields)))
        }
    }
}

fn with_exact_fields_mut_len_at_least<R>(
    handle: i64,
    required_len: usize,
    f: impl FnOnce(&mut [TypedSlot]) -> R,
) -> Option<R> {
    with_exact_fields_mut(handle, |fields| {
        if fields.len() < required_len {
            return None;
        }
        Some(f(fields))
    })?
}

fn apply_exact_success_header(
    fields: &mut [TypedSlot],
    reason_idx: usize,
    ok_idx: usize,
    success_count_idx: usize,
) -> bool {
    if !fields[reason_idx].set_i64_exact(0) {
        return false;
    }
    if !fields[ok_idx].set_i64_exact(1) {
        return false;
    }
    if fields[success_count_idx]
        .rmw_add_exact_unsigned_u64(1)
        .is_none()
    {
        return false;
    }
    true
}

fn apply_exact_alloc_success_record(
    fields: &mut [TypedSlot],
    reason_idx: usize,
    ok_idx: usize,
    success_count_idx: usize,
    reusable_success_count_idx: usize,
    active_success_count_idx: usize,
    selected_kind: i64,
) -> bool {
    if !apply_exact_success_header(fields, reason_idx, ok_idx, success_count_idx) {
        return false;
    }
    match selected_kind {
        1 => {
            return fields[reusable_success_count_idx]
                .rmw_add_exact_unsigned_u64(1)
                .is_some();
        }
        2 => {
            return fields[active_success_count_idx]
                .rmw_add_exact_unsigned_u64(1)
                .is_some();
        }
        _ => {}
    }
    true
}

fn apply_exact_release_success_record(
    fields: &mut [TypedSlot],
    reason_idx: usize,
    ok_idx: usize,
    success_count_idx: usize,
) -> bool {
    apply_exact_success_header(fields, reason_idx, ok_idx, success_count_idx)
}

pub(super) fn new_typed_object(object: TypedSlotObject) -> Option<i64> {
    match selected_backend() {
        TypedObjectStoreBackend::PinnedArenaExact => {
            PINNED_ARENA_OBJECTS.with(|objects| objects.try_borrow_mut().ok()?.insert(object))
        }
        TypedObjectStoreBackend::DirectSlotExact => new_direct_slot_object(object),
        _ => with_objects_mut(|objects| insert_indexed(objects, object))?,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CheckedStorageError {
    ProfileMismatch,
    InvalidLayout,
    AllocationOrStorageUnavailable,
    ObjectOrFieldMismatch,
}

pub(super) fn check_indexed_profile(expected: TypedObjectStoreBackend) -> Result<(), CheckedStorageError> {
    if !matches!(expected, TypedObjectStoreBackend::SafeMutex | TypedObjectStoreBackend::SingleThreadExact)
        || try_cached_env_choice(&BACKEND, TYPED_OBJECT_STORE_ENV, parse_backend) != Ok(expected)
    {
        return Err(CheckedStorageError::ProfileMismatch);
    }
    Ok(())
}

/// The caller projects the admitted definition; never consult the legacy layout
/// registry or fill missing fields. Uninstalled payload drops on every failure.
pub(super) fn new_checked_indexed(
    expected: TypedObjectStoreBackend, type_id: i64, layout: &[TypedSlotStorage],
) -> Result<i64, CheckedStorageError> {
    check_indexed_profile(expected)?;
    let count = i64::try_from(layout.len()).map_err(|_| CheckedStorageError::InvalidLayout)?;
    if super::typed_object::normalize_field_count(count).is_none()
        || layout.iter().any(|slot| *slot != TypedSlotStorage::I64) {
        return Err(CheckedStorageError::InvalidLayout);
    }
    allocate_i64_indexed(type_id, layout.len())
}

pub(super) fn new_checked_wire_indexed(
    expected: TypedObjectStoreBackend, type_id: i64, layout: &[u32],
) -> Result<i64, CheckedStorageError> {
    check_indexed_profile(expected)?;
    let count = i64::try_from(layout.len()).map_err(|_| CheckedStorageError::InvalidLayout)?;
    if super::typed_object::normalize_field_count(count).is_none()
        || layout.iter().any(|tag| *tag != TypedSlotStorage::I64.tag() as u32) {
        return Err(CheckedStorageError::InvalidLayout);
    }
    allocate_i64_indexed(type_id, layout.len())
}

fn allocate_i64_indexed(type_id: i64, count: usize) -> Result<i64, CheckedStorageError> {
    let mut fields = Vec::new();
    fields.try_reserve_exact(count)
        .map_err(|_| CheckedStorageError::AllocationOrStorageUnavailable)?;
    fields.extend((0..count).map(|_| TypedSlot::new(TypedSlotStorage::I64)));
    new_typed_object(TypedSlotObject { type_id, fields })
        .ok_or(CheckedStorageError::AllocationOrStorageUnavailable)
}

/// Expected definition type and slot are checked under the same storage guard.
/// A failed check leaves the object untouched; there is no numeric-handle repair.
pub(super) fn get_checked_indexed(
    expected: TypedObjectStoreBackend, handle: i64, type_id: i64, slot: usize,
) -> Result<i64, CheckedStorageError> {
    check_indexed_profile(expected)?;
    with_objects(|objects| {
        let object = objects.get(handle_to_index(handle)?)?.as_ref()?;
        if object.type_id != type_id { return None; }
        let field = object.fields.get(slot)?;
        if field.storage != TypedSlotStorage::I64 { return None; }
        field.as_exact_signed_i64()
    }).flatten().ok_or(CheckedStorageError::ObjectOrFieldMismatch)
}

/// Store under the same identity/profile checks as the exact read.
pub(super) fn set_checked_indexed(
    expected: TypedObjectStoreBackend, handle: i64, type_id: i64, slot: usize, value: i64,
) -> Result<(), CheckedStorageError> {
    check_indexed_profile(expected)?;
    let changed = with_objects_mut(|objects| {
        let object = objects.get_mut(handle_to_index(handle)?)?.as_mut()?;
        if object.type_id != type_id { return None; }
        let field = object.fields.get_mut(slot)?;
        if field.storage != TypedSlotStorage::I64 { return None; }
        field.set_exact_signed_i64(value).then_some(())
    }).flatten();
    changed.ok_or(CheckedStorageError::ObjectOrFieldMismatch)
}

/// Source-level Reclaim/HomeRelease permission belongs to the published caller;
/// this checks only the admitted physical profile and exact storage identity.
pub(super) fn reclaim_checked_indexed(
    expected: TypedObjectStoreBackend, handle: i64, type_id: i64,
) -> Result<(), CheckedStorageError> {
    check_indexed_profile(expected)?;
    if reclaim_typed_object_storage(handle, type_id) { Ok(()) }
    else { Err(CheckedStorageError::ObjectOrFieldMismatch) }
}

fn insert_indexed(objects: &mut IndexedObjects, object: TypedSlotObject) -> Option<i64> {
    let handle = next_indexed_handle(objects.len())?;
    objects.try_reserve(1).ok()?;
    objects.push(Some(object));
    Some(handle)
}

fn next_indexed_handle(len: usize) -> Option<i64> {
    i64::try_from(len).ok()?.checked_add(1)?.checked_neg()
}

fn take_indexed(
    objects: &mut IndexedObjects,
    handle: i64,
    expected_type: i64,
) -> Option<TypedSlotObject> {
    let slot = objects.get_mut(handle_to_index(handle)?)?;
    if slot.as_ref()?.type_id != expected_type {
        return None;
    }
    slot.take()
}

fn detach_mutex(
    owner: &Mutex<IndexedObjects>,
    handle: i64,
    expected_type: i64,
) -> Option<TypedSlotObject> {
    // Poison does not invalidate Vec/Option memory. Only detach may recover;
    // ordinary access stays failed and poison is deliberately not cleared.
    let mut objects = owner
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    take_indexed(&mut objects, handle, expected_type)
}

fn detach_local(
    owner: &RefCell<IndexedObjects>,
    handle: i64,
    expected_type: i64,
) -> Option<TypedSlotObject> {
    let mut objects = owner.try_borrow_mut().ok()?;
    take_indexed(&mut objects, handle, expected_type)
}

/// Storage-only primitive. The future exact source/CFG consumer must authorize
/// unpublished reclamation or terminal structural release. This does not infer
/// Home ownership, recurse into numeric handles, or run fini. Local handles
/// require thread confinement; pinned/direct storage is not admitted here.
pub(crate) fn reclaim_typed_object_storage(handle: i64, expected_type: i64) -> bool {
    let detached = match selected_backend() {
        TypedObjectStoreBackend::SafeMutex => {
            detach_mutex(safe_mutex_objects(), handle, expected_type)
        }
        TypedObjectStoreBackend::SingleThreadExact => {
            SINGLE_THREAD_OBJECTS.with(|owner| detach_local(owner, handle, expected_type))
        }
        TypedObjectStoreBackend::PinnedArenaExact | TypedObjectStoreBackend::DirectSlotExact => {
            None
        }
    };
    let reclaimed = detached.is_some();
    // Both owner guards have ended. TypedSlotValue is inert scalar/handle data;
    // dropping this payload cannot discharge child Homes or invoke user code.
    drop(detached);
    reclaimed
}

pub(super) fn exact_slot_set4_i64(
    handle: i64,
    start_slot: usize,
    value0: i64,
    value1: i64,
    value2: i64,
    value3: i64,
) -> bool {
    if let Some(result) = with_direct_slot_object_mut(handle, |object| {
        object.exact_slot_set4_i64(start_slot, value0, value1, value2, value3)
    }) {
        return result;
    }
    let Some(end_slot) = start_slot.checked_add(4) else {
        return false;
    };
    with_exact_fields_mut_len_at_least(handle, end_slot, |fields| {
        if fields[start_slot..end_slot]
            .iter()
            .any(|field| field.storage != TypedSlotStorage::I64)
        {
            return false;
        }
        fields[start_slot].set_i64_exact(value0)
            && fields[start_slot + 1].set_i64_exact(value1)
            && fields[start_slot + 2].set_i64_exact(value2)
            && fields[start_slot + 3].set_i64_exact(value3)
    })
    .unwrap_or(false)
}

pub(super) fn exact_slot_record_alloc_success(handle: i64, selected_kind: i64) -> bool {
    if let Some(result) = with_direct_slot_object_mut(handle, |object| {
        object.exact_slot_record_alloc_success(selected_kind)
    }) {
        return result;
    }
    const LAST_REASON: usize = 2;
    const LAST_OK: usize = 3;
    const SUCCESS_COUNT: usize = 5;
    const REUSABLE_SUCCESS_COUNT: usize = 7;
    const ACTIVE_SUCCESS_COUNT: usize = 8;

    with_exact_fields_mut_len_at_least(handle, ACTIVE_SUCCESS_COUNT + 1, |fields| {
        apply_exact_alloc_success_record(
            fields,
            LAST_REASON,
            LAST_OK,
            SUCCESS_COUNT,
            REUSABLE_SUCCESS_COUNT,
            ACTIVE_SUCCESS_COUNT,
            selected_kind,
        )
    })
    .unwrap_or(false)
}

pub(super) fn exact_slot_record_release_success(handle: i64, page_id: i64, block_id: i64) -> bool {
    if let Some(result) = with_direct_slot_object_mut(handle, |object| {
        object.exact_slot_record_release_success(page_id, block_id)
    }) {
        return result;
    }
    const LAST_PAGE_ID: usize = 0;
    const LAST_BLOCK_ID: usize = 1;
    const LAST_REASON: usize = 2;
    const LAST_OK: usize = 3;
    const SUCCESS_COUNT: usize = 4;

    with_exact_fields_mut_len_at_least(handle, SUCCESS_COUNT + 1, |fields| {
        if !fields[LAST_PAGE_ID].set_i64_exact(page_id) {
            return false;
        }
        if !fields[LAST_BLOCK_ID].set_i64_exact(block_id) {
            return false;
        }
        apply_exact_release_success_record(fields, LAST_REASON, LAST_OK, SUCCESS_COUNT)
    })
    .unwrap_or(false)
}

pub(super) fn exact_slot_rmw_add_u64(handle: i64, slot: usize, delta: i64) -> Option<i64> {
    let delta = u128::try_from(delta).ok()?;
    if let Some(result) = with_direct_slot_object_mut(handle, |object| {
        object.rmw_add_exact_unsigned_u64(slot, delta)
    }) {
        return result;
    }
    with_field_mut(handle, slot, |field| {
        field.rmw_add_exact_unsigned_u64(delta)
    })?
}

#[cfg(test)]
mod indexed_storage_tests {
    use super::*;

    #[test]
    fn unknown_checked_profile_rejects_without_initializing_storage() {
        const UNKNOWN: &str = "fault-test-unknown-profile";
        if std::env::var(TYPED_OBJECT_STORE_ENV).as_deref() == Ok(UNKNOWN) {
            assert_eq!(new_checked_indexed(TypedObjectStoreBackend::SafeMutex, 901, &[]),
                Err(CheckedStorageError::ProfileMismatch));
            assert_eq!(set_checked_indexed(TypedObjectStoreBackend::SafeMutex, -1, 901, 0, 1),
                Err(CheckedStorageError::ProfileMismatch));
            assert_eq!(reclaim_checked_indexed(TypedObjectStoreBackend::SafeMutex, -1, 901),
                Err(CheckedStorageError::ProfileMismatch));
            assert!(BACKEND.get().is_none());
            assert!(SAFE_MUTEX_OBJECTS.get().is_none());
            return;
        }
        let result = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["exports::typed_object_store_backend::indexed_storage_tests::unknown_checked_profile_rejects_without_initializing_storage", "--exact", "--test-threads=1"])
            .env(TYPED_OBJECT_STORE_ENV, UNKNOWN).output().unwrap();
        assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
        assert!(String::from_utf8_lossy(&result.stdout).contains("1 passed"));
    }

    #[test]
    fn checked_operations_preserve_identity_and_reject_before_mutation() {
        let profile = selected_backend();
        if !matches!(profile, TypedObjectStoreBackend::SafeMutex | TypedObjectStoreBackend::SingleThreadExact) {
            assert_eq!(new_checked_indexed(profile, 901, &[]), Err(CheckedStorageError::ProfileMismatch));
            assert_eq!(set_checked_indexed(profile, -1, 901, 0, 1), Err(CheckedStorageError::ProfileMismatch));
            assert_eq!(reclaim_checked_indexed(profile, -1, 901), Err(CheckedStorageError::ProfileMismatch));
            return;
        }
        let other = if profile == TypedObjectStoreBackend::SafeMutex {
            TypedObjectStoreBackend::SingleThreadExact
        } else { TypedObjectStoreBackend::SafeMutex };
        assert_eq!(new_checked_indexed(other, 901, &[]), Err(CheckedStorageError::ProfileMismatch));
        assert_eq!(new_checked_indexed(profile, 901, &[TypedSlotStorage::Handle]), Err(CheckedStorageError::InvalidLayout));
        let handle = new_checked_indexed(profile, 901, &[TypedSlotStorage::I64]).unwrap();
        set_checked_indexed(profile, handle, 901, 0, 42).unwrap();
        assert_eq!(set_checked_indexed(profile, handle, 902, 0, 99), Err(CheckedStorageError::ObjectOrFieldMismatch));
        assert_eq!(set_checked_indexed(profile, handle, 901, 1, 99), Err(CheckedStorageError::ObjectOrFieldMismatch));
        assert_eq!(with_field(handle, 0, |field| field.as_exact_signed_i64()), Some(Some(42)));
        assert_eq!(reclaim_checked_indexed(profile, handle, 902), Err(CheckedStorageError::ObjectOrFieldMismatch));
        reclaim_checked_indexed(profile, handle, 901).unwrap();
        assert_eq!(reclaim_checked_indexed(profile, handle, 901), Err(CheckedStorageError::ObjectOrFieldMismatch));
    }

    fn object(type_id: i64) -> TypedSlotObject {
        TypedSlotObject {
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
}
