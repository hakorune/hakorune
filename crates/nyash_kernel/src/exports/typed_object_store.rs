//! Storage backends for typed user object exported helpers.
//!
//! `typed_object.rs` owns the C ABI. This module owns the object storage seam so
//! exact-EXE perf lanes can select a narrower backend without changing symbols.

use std::cell::RefCell;
use std::sync::{Mutex, OnceLock};

use super::typed_object::{
    handle_to_index, TypedSlot, TypedSlotObject, TypedSlotStorage, TypedSlotValue,
};
use super::typed_object_pinned_arena::{DirectSlotObjectV0Box, PinnedTypedObjectArena};

const TYPED_OBJECT_STORE_ENV: &str = "HAKO_TYPED_OBJECT_STORE";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypedObjectStoreBackend {
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
    static DIRECT_SLOT_OBJECTS: RefCell<Vec<DirectSlotObjectV0Box>> = const { RefCell::new(Vec::new()) };
    static DIRECT_SLOT_MATERIALIZED_VIEWS: RefCell<Vec<TypedSlotObject>> = const { RefCell::new(Vec::new()) };
}

fn selected_backend() -> TypedObjectStoreBackend {
    *BACKEND.get_or_init(|| match std::env::var(TYPED_OBJECT_STORE_ENV).ok().as_deref() {
        None | Some("") | Some("safe_mutex") => TypedObjectStoreBackend::SafeMutex,
        Some("single_thread_exact") => TypedObjectStoreBackend::SingleThreadExact,
        Some("pinned_arena_exact") => TypedObjectStoreBackend::PinnedArenaExact,
        Some("direct_slot_exact") => TypedObjectStoreBackend::DirectSlotExact,
        Some(value) => panic!(
            "[freeze:contract][typed-object-store/backend] unsupported {TYPED_OBJECT_STORE_ENV}={value}"
        ),
    })
}

fn safe_mutex_objects() -> &'static Mutex<Vec<TypedSlotObject>> {
    SAFE_MUTEX_OBJECTS.get_or_init(|| Mutex::new(Vec::new()))
}

fn with_objects<R>(f: impl FnOnce(&Vec<TypedSlotObject>) -> R) -> Option<R> {
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

fn with_objects_mut<R>(f: impl FnOnce(&mut Vec<TypedSlotObject>) -> R) -> Option<R> {
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

fn with_field<R>(handle: i64, slot: usize, f: impl FnOnce(&TypedSlot) -> R) -> Option<R> {
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

fn with_field_mut<R>(handle: i64, slot: usize, f: impl FnOnce(&mut TypedSlot) -> R) -> Option<R> {
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

fn with_exact_fields<R>(handle: i64, f: impl FnOnce(&[TypedSlot]) -> R) -> Option<R> {
    match selected_backend() {
        TypedObjectStoreBackend::SafeMutex => None,
        TypedObjectStoreBackend::SingleThreadExact => SINGLE_THREAD_OBJECTS.with(|objects| {
            let objects = objects.try_borrow().ok()?;
            let idx = handle_to_index(handle)?;
            let object = objects.get(idx)?;
            Some(f(&object.fields))
        }),
        TypedObjectStoreBackend::PinnedArenaExact => PINNED_ARENA_OBJECTS.with(|objects| {
            let objects = objects.try_borrow().ok()?;
            objects.get_fields(handle).map(f)
        }),
        TypedObjectStoreBackend::DirectSlotExact => {
            with_direct_slot_materialized_view(handle, |object| Some(f(&object.fields)))
        }
    }
}

fn with_exact_fields_mut<R>(handle: i64, f: impl FnOnce(&mut [TypedSlot]) -> R) -> Option<R> {
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

pub(crate) fn new_typed_object(object: TypedSlotObject) -> Option<i64> {
    match selected_backend() {
        TypedObjectStoreBackend::PinnedArenaExact => {
            PINNED_ARENA_OBJECTS.with(|objects| objects.try_borrow_mut().ok()?.insert(object))
        }
        TypedObjectStoreBackend::DirectSlotExact => DIRECT_SLOT_OBJECTS.with(|objects| {
            let object = DirectSlotObjectV0Box::from_typed_object(object)?;
            let handle = object.handle()?;
            objects.try_borrow_mut().ok()?.push(object);
            Some(handle)
        }),
        _ => with_objects_mut(|objects| {
            objects.push(object);
            -(objects.len() as i64)
        }),
    }
}

pub(crate) fn materialize_direct_slot_snapshot(handle: i64) -> Option<TypedSlotObject> {
    match selected_backend() {
        TypedObjectStoreBackend::DirectSlotExact => DIRECT_SLOT_OBJECTS.with(|objects| {
            let objects = objects.try_borrow().ok()?;
            let object = objects
                .iter()
                .find(|object| object.matches_handle(handle))?;
            object.materialize_typed_object_snapshot()
        }),
        _ => None,
    }
}

pub(crate) fn materialize_direct_slot_view_handle(handle: i64) -> Option<i64> {
    match selected_backend() {
        TypedObjectStoreBackend::DirectSlotExact => {
            let snapshot = materialize_direct_slot_snapshot(handle)?;
            DIRECT_SLOT_MATERIALIZED_VIEWS.with(|objects| {
                let mut objects = objects.try_borrow_mut().ok()?;
                objects.push(snapshot);
                Some(-(objects.len() as i64))
            })
        }
        _ => None,
    }
}

fn with_direct_slot_materialized_view<R>(
    handle: i64,
    f: impl FnOnce(&TypedSlotObject) -> Option<R>,
) -> Option<R> {
    let idx = handle_to_index(handle)?;
    DIRECT_SLOT_MATERIALIZED_VIEWS.with(|objects| {
        let objects = objects.try_borrow().ok()?;
        f(objects.get(idx)?)
    })
}

fn with_direct_slot_materialized_view_mut<R>(
    handle: i64,
    f: impl FnOnce(&mut TypedSlotObject) -> Option<R>,
) -> Option<R> {
    let idx = handle_to_index(handle)?;
    DIRECT_SLOT_MATERIALIZED_VIEWS.with(|objects| {
        let mut objects = objects.try_borrow_mut().ok()?;
        f(objects.get_mut(idx)?)
    })
}

fn with_direct_slot_object<R>(
    handle: i64,
    f: impl FnOnce(&DirectSlotObjectV0Box) -> R,
) -> Option<R> {
    if selected_backend() != TypedObjectStoreBackend::DirectSlotExact {
        return None;
    }
    DIRECT_SLOT_OBJECTS.with(|objects| {
        let objects = objects.try_borrow().ok()?;
        let object = objects
            .iter()
            .find(|object| object.matches_handle(handle))?;
        Some(f(object))
    })
}

fn with_direct_slot_object_mut<R>(
    handle: i64,
    f: impl FnOnce(&mut DirectSlotObjectV0Box) -> R,
) -> Option<R> {
    if selected_backend() != TypedObjectStoreBackend::DirectSlotExact {
        return None;
    }
    DIRECT_SLOT_OBJECTS.with(|objects| {
        let mut objects = objects.try_borrow_mut().ok()?;
        let object = objects
            .iter_mut()
            .find(|object| object.matches_handle(handle))?;
        Some(f(object))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_slot_exact_new_object_returns_tagged_pointer_handle() {
        if std::env::var(TYPED_OBJECT_STORE_ENV).ok().as_deref() != Some("direct_slot_exact") {
            eprintln!("skip: set HAKO_TYPED_OBJECT_STORE=direct_slot_exact");
            return;
        }
        let object = TypedSlotObject {
            type_id: 17,
            fields: vec![
                TypedSlot::new(TypedSlotStorage::I64),
                TypedSlot::new(TypedSlotStorage::U64),
                TypedSlot::new(TypedSlotStorage::Handle),
            ],
        };
        let handle = new_typed_object(object).unwrap();

        assert_eq!(handle & 1, 1);
        assert!(DirectSlotObjectV0Box::from_handle(handle).is_some());
        assert_eq!(get_legacy_i64(handle, 0), Some(0));
        assert!(set_legacy_i64(handle, 0, 1));
        assert_eq!(exact_slot_get_i64(handle, 0), Some(1));
        assert!(set_exact_unsigned_u64(handle, 1, 2));
        assert_eq!(get_exact_unsigned_u64(handle, 1), Some(2));
        assert!(exact_slot_set_handle(handle, 2, -5));
        assert_eq!(exact_slot_get_handle(handle, 2), Some(-5));
    }

    #[test]
    fn direct_slot_exact_materializes_typed_slot_snapshot_explicitly() {
        if std::env::var(TYPED_OBJECT_STORE_ENV).ok().as_deref() != Some("direct_slot_exact") {
            eprintln!("skip: set HAKO_TYPED_OBJECT_STORE=direct_slot_exact");
            return;
        }
        let object = TypedSlotObject {
            type_id: 23,
            fields: vec![
                TypedSlot {
                    storage: TypedSlotStorage::I64,
                    value: TypedSlotValue::I64(-7),
                },
                TypedSlot {
                    storage: TypedSlotStorage::U64,
                    value: TypedSlotValue::Unsigned(11),
                },
                TypedSlot {
                    storage: TypedSlotStorage::Handle,
                    value: TypedSlotValue::Handle(-3),
                },
            ],
        };
        let handle = new_typed_object(object).unwrap();

        let snapshot = materialize_direct_slot_snapshot(handle).unwrap();
        assert_eq!(snapshot.type_id, 23);
        assert_eq!(snapshot.fields.len(), 3);
        assert_eq!(snapshot.fields[0].value, TypedSlotValue::I64(-7));
        assert_eq!(snapshot.fields[1].value, TypedSlotValue::Unsigned(11));
        assert_eq!(snapshot.fields[2].value, TypedSlotValue::Handle(-3));
        assert!(materialize_direct_slot_snapshot(-1).is_none());
        assert_eq!(get_legacy_i64(handle, 0), Some(-7));
    }

    #[test]
    fn direct_slot_exact_materialized_view_handle_routes_existing_helpers() {
        if std::env::var(TYPED_OBJECT_STORE_ENV).ok().as_deref() != Some("direct_slot_exact") {
            eprintln!("skip: set HAKO_TYPED_OBJECT_STORE=direct_slot_exact");
            return;
        }
        let object = TypedSlotObject {
            type_id: 31,
            fields: vec![
                TypedSlot {
                    storage: TypedSlotStorage::I64,
                    value: TypedSlotValue::I64(-9),
                },
                TypedSlot {
                    storage: TypedSlotStorage::U64,
                    value: TypedSlotValue::Unsigned(99),
                },
                TypedSlot {
                    storage: TypedSlotStorage::Handle,
                    value: TypedSlotValue::Handle(-5),
                },
            ],
        };
        let direct_handle = new_typed_object(object).unwrap();
        assert!(direct_handle > 0);
        assert_eq!(get_legacy_i64(direct_handle, 0), Some(-9));

        let view_handle = materialize_direct_slot_view_handle(direct_handle).unwrap();
        assert!(view_handle < 0);
        assert_eq!(get_legacy_i64(view_handle, 0), Some(-9));
        assert_eq!(get_exact_unsigned_u64(view_handle, 1), Some(99));
        assert_eq!(get_legacy_i64(view_handle, 2), Some(-5));
        assert!(set_legacy_i64(view_handle, 0, 13));
        assert_eq!(get_legacy_i64(view_handle, 0), Some(13));

        let direct_snapshot = materialize_direct_slot_snapshot(direct_handle).unwrap();
        assert_eq!(direct_snapshot.fields[0].value, TypedSlotValue::I64(-9));
    }

    #[test]
    fn direct_slot_exact_positive_handle_helpers_update_primary_cells() {
        if std::env::var(TYPED_OBJECT_STORE_ENV).ok().as_deref() != Some("direct_slot_exact") {
            eprintln!("skip: set HAKO_TYPED_OBJECT_STORE=direct_slot_exact");
            return;
        }
        let object = TypedSlotObject {
            type_id: 41,
            fields: vec![
                TypedSlot::new(TypedSlotStorage::I64),
                TypedSlot::new(TypedSlotStorage::USize),
                TypedSlot::new(TypedSlotStorage::Handle),
            ],
        };
        let handle = new_typed_object(object).unwrap();

        assert!(set_legacy_i64(handle, 0, -11));
        assert!(set_exact_unsigned_u64(handle, 1, 29));
        assert!(exact_slot_set_handle(handle, 2, -17));
        assert_eq!(get_legacy_i64(handle, 0), Some(-11));
        assert_eq!(get_exact_unsigned_u64(handle, 1), Some(29));
        assert_eq!(exact_slot_get_handle(handle, 2), Some(-17));

        let snapshot = materialize_direct_slot_snapshot(handle).unwrap();
        assert_eq!(snapshot.fields[0].value, TypedSlotValue::I64(-11));
        assert_eq!(snapshot.fields[1].value, TypedSlotValue::Unsigned(29));
        assert_eq!(snapshot.fields[2].value, TypedSlotValue::Handle(-17));
    }
}

pub(crate) fn get_legacy_i64(handle: i64, slot: usize) -> Option<i64> {
    if let Some(value) =
        with_direct_slot_object(handle, |object| object.get_legacy_i64(slot)).flatten()
    {
        return Some(value);
    }
    with_field(handle, slot, |field| match field.value {
        super::typed_object::TypedSlotValue::I64(value)
        | super::typed_object::TypedSlotValue::Handle(value) => Some(value),
        super::typed_object::TypedSlotValue::Signed(_)
        | super::typed_object::TypedSlotValue::Unsigned(_) => Some(0),
    })?
}

pub(crate) fn set_legacy_i64(handle: i64, slot: usize, value: i64) -> bool {
    if let Some(ok) =
        with_direct_slot_object_mut(handle, |object| object.set_legacy_i64(slot, value))
    {
        return ok;
    }
    with_field_mut(handle, slot, |field| field.set_legacy_i64(value)).unwrap_or(false)
}

pub(crate) fn field_storage_tag(handle: i64, slot: usize) -> Option<i64> {
    if let Some(tag) =
        with_direct_slot_object(handle, |object| object.field_storage_tag(slot)).flatten()
    {
        return Some(tag);
    }
    with_field(handle, slot, |field| field.storage.tag())
}

pub(crate) fn get_exact_unsigned_u64(handle: i64, slot: usize) -> Option<u64> {
    if let Some(value) =
        with_direct_slot_object(handle, |object| object.get_exact_unsigned_u64(slot)).flatten()
    {
        return Some(value);
    }
    with_field(handle, slot, |field| field.as_exact_unsigned_u64())?
}

pub(crate) fn set_exact_unsigned_u64(handle: i64, slot: usize, value: u64) -> bool {
    if let Some(ok) =
        with_direct_slot_object_mut(handle, |object| object.set_exact_unsigned_u64(slot, value))
    {
        return ok;
    }
    with_field_mut(handle, slot, |field| field.set_exact_unsigned_u64(value)).unwrap_or(false)
}

pub(crate) fn get_exact_signed_i64(handle: i64, slot: usize) -> Option<i64> {
    if let Some(value) =
        with_direct_slot_object(handle, |object| object.get_exact_signed_i64(slot)).flatten()
    {
        return Some(value);
    }
    with_field(handle, slot, |field| field.as_exact_signed_i64())?
}

pub(crate) fn set_exact_signed_i64(handle: i64, slot: usize, value: i64) -> bool {
    if let Some(ok) =
        with_direct_slot_object_mut(handle, |object| object.set_exact_signed_i64(slot, value))
    {
        return ok;
    }
    with_field_mut(handle, slot, |field| field.set_exact_signed_i64(value)).unwrap_or(false)
}

fn exact_u64_storage_supported(storage: TypedSlotStorage) -> bool {
    matches!(storage, TypedSlotStorage::U64)
        || (cfg!(target_pointer_width = "64") && matches!(storage, TypedSlotStorage::USize))
}

fn exact_set_i64_field(field: &mut TypedSlot, value: i64) -> bool {
    if field.storage != TypedSlotStorage::I64 {
        return false;
    }
    field.value = TypedSlotValue::I64(value);
    true
}

fn exact_increment_u64_field(field: &mut TypedSlot) -> bool {
    if !exact_u64_storage_supported(field.storage) {
        return false;
    }
    let TypedSlotValue::Unsigned(value) = field.value else {
        return false;
    };
    let Some(next) = value.checked_add(1) else {
        return false;
    };
    if u64::try_from(next).is_err() {
        return false;
    }
    field.value = TypedSlotValue::Unsigned(next);
    true
}

pub(crate) fn exact_slot_get_i64(handle: i64, slot: usize) -> Option<i64> {
    if let Some(value) =
        with_direct_slot_object(handle, |object| object.exact_slot_get_i64(slot)).flatten()
    {
        return Some(value);
    }
    with_exact_fields(handle, |fields| {
        let field = fields.get(slot)?;
        if field.storage != TypedSlotStorage::I64 {
            return None;
        }
        match field.value {
            TypedSlotValue::I64(value) => Some(value),
            _ => None,
        }
    })?
}

pub(crate) fn exact_slot_set_i64(handle: i64, slot: usize, value: i64) -> bool {
    if let Some(ok) =
        with_direct_slot_object_mut(handle, |object| object.exact_slot_set_i64(slot, value))
    {
        return ok;
    }
    with_exact_fields_mut(handle, |fields| {
        let Some(field) = fields.get_mut(slot) else {
            return false;
        };
        if field.storage != TypedSlotStorage::I64 {
            return false;
        }
        field.value = TypedSlotValue::I64(value);
        true
    })
    .unwrap_or(false)
}

pub(crate) fn exact_slot_set4_i64(
    handle: i64,
    start_slot: usize,
    value0: i64,
    value1: i64,
    value2: i64,
    value3: i64,
) -> bool {
    if let Some(ok) = with_direct_slot_object_mut(handle, |object| {
        object.exact_slot_set4_i64(start_slot, value0, value1, value2, value3)
    }) {
        return ok;
    }
    let Some(end_slot) = start_slot.checked_add(4) else {
        return false;
    };
    with_exact_fields_mut(handle, |fields| {
        if end_slot > fields.len() {
            return false;
        }
        if fields[start_slot..end_slot]
            .iter()
            .any(|field| field.storage != TypedSlotStorage::I64)
        {
            return false;
        }
        fields[start_slot].value = TypedSlotValue::I64(value0);
        fields[start_slot + 1].value = TypedSlotValue::I64(value1);
        fields[start_slot + 2].value = TypedSlotValue::I64(value2);
        fields[start_slot + 3].value = TypedSlotValue::I64(value3);
        true
    })
    .unwrap_or(false)
}

pub(crate) fn exact_slot_record_alloc_success(handle: i64, selected_kind: i64) -> bool {
    if let Some(ok) = with_direct_slot_object_mut(handle, |object| {
        object.exact_slot_record_alloc_success(selected_kind)
    }) {
        return ok;
    }
    const LAST_REASON: usize = 2;
    const LAST_OK: usize = 3;
    const SUCCESS_COUNT: usize = 5;
    const REUSABLE_SUCCESS_COUNT: usize = 7;
    const ACTIVE_SUCCESS_COUNT: usize = 8;

    with_exact_fields_mut(handle, |fields| {
        if fields.len() <= ACTIVE_SUCCESS_COUNT {
            return false;
        }
        if !exact_set_i64_field(&mut fields[LAST_REASON], 0) {
            return false;
        }
        if !exact_set_i64_field(&mut fields[LAST_OK], 1) {
            return false;
        }
        if !exact_increment_u64_field(&mut fields[SUCCESS_COUNT]) {
            return false;
        }
        if selected_kind == 1 {
            return exact_increment_u64_field(&mut fields[REUSABLE_SUCCESS_COUNT]);
        }
        if selected_kind == 2 {
            return exact_increment_u64_field(&mut fields[ACTIVE_SUCCESS_COUNT]);
        }
        true
    })
    .unwrap_or(false)
}

pub(crate) fn exact_slot_record_release_success(handle: i64, page_id: i64, block_id: i64) -> bool {
    if let Some(ok) = with_direct_slot_object_mut(handle, |object| {
        object.exact_slot_record_release_success(page_id, block_id)
    }) {
        return ok;
    }
    const LAST_PAGE_ID: usize = 0;
    const LAST_BLOCK_ID: usize = 1;
    const LAST_REASON: usize = 2;
    const LAST_OK: usize = 3;
    const SUCCESS_COUNT: usize = 4;

    with_exact_fields_mut(handle, |fields| {
        if fields.len() <= SUCCESS_COUNT {
            return false;
        }
        if !exact_set_i64_field(&mut fields[LAST_PAGE_ID], page_id) {
            return false;
        }
        if !exact_set_i64_field(&mut fields[LAST_BLOCK_ID], block_id) {
            return false;
        }
        if !exact_set_i64_field(&mut fields[LAST_REASON], 0) {
            return false;
        }
        if !exact_set_i64_field(&mut fields[LAST_OK], 1) {
            return false;
        }
        exact_increment_u64_field(&mut fields[SUCCESS_COUNT])
    })
    .unwrap_or(false)
}

pub(crate) fn exact_slot_get_u64(handle: i64, slot: usize) -> Option<u64> {
    if let Some(value) =
        with_direct_slot_object(handle, |object| object.exact_slot_get_u64(slot)).flatten()
    {
        return Some(value);
    }
    with_exact_fields(handle, |fields| {
        let field = fields.get(slot)?;
        if !exact_u64_storage_supported(field.storage) {
            return None;
        }
        let TypedSlotValue::Unsigned(value) = field.value else {
            return None;
        };
        u64::try_from(value).ok()
    })?
}

pub(crate) fn exact_slot_set_u64(handle: i64, slot: usize, value: u64) -> bool {
    if let Some(ok) =
        with_direct_slot_object_mut(handle, |object| object.exact_slot_set_u64(slot, value))
    {
        return ok;
    }
    with_exact_fields_mut(handle, |fields| {
        let Some(field) = fields.get_mut(slot) else {
            return false;
        };
        if !exact_u64_storage_supported(field.storage) {
            return false;
        }
        field.value = TypedSlotValue::Unsigned(value as u128);
        true
    })
    .unwrap_or(false)
}

pub(crate) fn exact_slot_rmw_add_u64(handle: i64, slot: usize, delta: i64) -> Option<i64> {
    let delta = u128::try_from(delta).ok()?;
    if let Some(value) = with_direct_slot_object_mut(handle, |object| {
        let delta = u64::try_from(delta).ok()?;
        object.exact_slot_rmw_add_u64(slot, delta)
    })
    .flatten()
    {
        return Some(value);
    }
    with_exact_fields_mut(handle, |fields| {
        let field = fields.get_mut(slot)?;
        if !exact_u64_storage_supported(field.storage) {
            return None;
        }
        let TypedSlotValue::Unsigned(value) = field.value else {
            return None;
        };
        let next = value.checked_add(delta)?;
        u64::try_from(next).ok()?;
        let next_i64 = i64::try_from(next).ok()?;
        field.value = TypedSlotValue::Unsigned(next);
        Some(next_i64)
    })?
}

pub(crate) fn exact_slot_get_handle(handle: i64, slot: usize) -> Option<i64> {
    if let Some(value) =
        with_direct_slot_object(handle, |object| object.exact_slot_get_handle(slot)).flatten()
    {
        return Some(value);
    }
    with_exact_fields(handle, |fields| {
        let field = fields.get(slot)?;
        if field.storage != TypedSlotStorage::Handle {
            return None;
        }
        match field.value {
            TypedSlotValue::Handle(value) => Some(value),
            _ => None,
        }
    })?
}

pub(crate) fn exact_slot_set_handle(handle: i64, slot: usize, value: i64) -> bool {
    if let Some(ok) =
        with_direct_slot_object_mut(handle, |object| object.exact_slot_set_handle(slot, value))
    {
        return ok;
    }
    with_exact_fields_mut(handle, |fields| {
        let Some(field) = fields.get_mut(slot) else {
            return false;
        };
        if field.storage != TypedSlotStorage::Handle {
            return false;
        }
        field.value = TypedSlotValue::Handle(value);
        true
    })
    .unwrap_or(false)
}
