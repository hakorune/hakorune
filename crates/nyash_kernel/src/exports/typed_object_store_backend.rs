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
use crate::backend_env::{cached_env_choice, panic_unsupported_env_value};

pub(super) const TYPED_OBJECT_STORE_ENV: &str = "HAKO_TYPED_OBJECT_STORE";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TypedObjectStoreBackend {
    SafeMutex,
    SingleThreadExact,
    PinnedArenaExact,
    DirectSlotExact,
}

static BACKEND: OnceLock<TypedObjectStoreBackend> = OnceLock::new();
static SAFE_MUTEX_OBJECTS: OnceLock<Mutex<Vec<TypedSlotObject>>> = OnceLock::new();

thread_local! {
    static SINGLE_THREAD_OBJECTS: RefCell<Vec<TypedSlotObject>> = const { RefCell::new(Vec::new()) };
    static PINNED_ARENA_OBJECTS: RefCell<PinnedTypedObjectArena> = RefCell::new(PinnedTypedObjectArena::new());
}

pub(super) fn selected_backend() -> TypedObjectStoreBackend {
    cached_env_choice(&BACKEND, TYPED_OBJECT_STORE_ENV, |value| match value {
        None | Some("") | Some("safe_mutex") => TypedObjectStoreBackend::SafeMutex,
        Some("single_thread_exact") => TypedObjectStoreBackend::SingleThreadExact,
        Some("pinned_arena_exact") => TypedObjectStoreBackend::PinnedArenaExact,
        Some("direct_slot_exact") => TypedObjectStoreBackend::DirectSlotExact,
        Some(value) => {
            panic_unsupported_env_value("typed-object-store/backend", TYPED_OBJECT_STORE_ENV, value)
        }
    })
}

fn safe_mutex_objects() -> &'static Mutex<Vec<TypedSlotObject>> {
    SAFE_MUTEX_OBJECTS.get_or_init(|| Mutex::new(Vec::new()))
}

pub(super) fn with_objects<R>(f: impl FnOnce(&Vec<TypedSlotObject>) -> R) -> Option<R> {
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

pub(super) fn with_objects_mut<R>(f: impl FnOnce(&mut Vec<TypedSlotObject>) -> R) -> Option<R> {
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
                let field = objects.get(idx)?.fields.get(slot)?;
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
                let field = objects.get_mut(idx)?.fields.get_mut(slot)?;
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
            with_objects(|objects| Some(objects.get(idx)?.type_id))?
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
            let object = objects.get_mut(idx)?;
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
        _ => with_objects_mut(|objects| {
            objects.push(object);
            -(objects.len() as i64)
        }),
    }
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
