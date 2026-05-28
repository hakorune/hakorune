//! Storage backends for typed user object exported helpers.
//!
//! `typed_object.rs` owns the C ABI. This module owns the object storage seam so
//! exact-EXE perf lanes can select a narrower backend without changing symbols.

use std::cell::RefCell;
use std::sync::{Mutex, OnceLock};

use super::typed_object::{handle_to_index, TypedSlotObject};

const TYPED_OBJECT_STORE_ENV: &str = "HAKO_TYPED_OBJECT_STORE";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypedObjectStoreBackend {
    SafeMutex,
    SingleThreadExact,
}

static BACKEND: OnceLock<TypedObjectStoreBackend> = OnceLock::new();
static SAFE_MUTEX_OBJECTS: OnceLock<Mutex<Vec<TypedSlotObject>>> = OnceLock::new();

thread_local! {
    static SINGLE_THREAD_OBJECTS: RefCell<Vec<TypedSlotObject>> = const { RefCell::new(Vec::new()) };
}

fn selected_backend() -> TypedObjectStoreBackend {
    *BACKEND.get_or_init(|| match std::env::var(TYPED_OBJECT_STORE_ENV).ok().as_deref() {
        None | Some("") | Some("safe_mutex") => TypedObjectStoreBackend::SafeMutex,
        Some("single_thread_exact") => TypedObjectStoreBackend::SingleThreadExact,
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
    }
}

pub(crate) fn new_typed_object(object: TypedSlotObject) -> Option<i64> {
    with_objects_mut(|objects| {
        objects.push(object);
        -(objects.len() as i64)
    })
}

pub(crate) fn get_legacy_i64(handle: i64, slot: usize) -> Option<i64> {
    let idx = handle_to_index(handle)?;
    with_objects(|objects| {
        let field = objects.get(idx)?.fields.get(slot)?;
        match field.value {
            super::typed_object::TypedSlotValue::I64(value)
            | super::typed_object::TypedSlotValue::Handle(value) => Some(value),
            super::typed_object::TypedSlotValue::Signed(_)
            | super::typed_object::TypedSlotValue::Unsigned(_) => Some(0),
        }
    })?
}

pub(crate) fn set_legacy_i64(handle: i64, slot: usize, value: i64) -> bool {
    let Some(idx) = handle_to_index(handle) else {
        return false;
    };
    with_objects_mut(|objects| {
        objects
            .get_mut(idx)
            .and_then(|object| object.fields.get_mut(slot))
            .is_some_and(|field| field.set_legacy_i64(value))
    })
    .unwrap_or(false)
}

pub(crate) fn field_storage_tag(handle: i64, slot: usize) -> Option<i64> {
    let idx = handle_to_index(handle)?;
    with_objects(|objects| {
        objects
            .get(idx)?
            .fields
            .get(slot)
            .map(|field| field.storage.tag())
    })?
}

pub(crate) fn get_exact_unsigned_u64(handle: i64, slot: usize) -> Option<u64> {
    let idx = handle_to_index(handle)?;
    with_objects(|objects| {
        objects
            .get(idx)?
            .fields
            .get(slot)
            .and_then(|field| field.as_exact_unsigned_u64())
    })?
}

pub(crate) fn set_exact_unsigned_u64(handle: i64, slot: usize, value: u64) -> bool {
    let Some(idx) = handle_to_index(handle) else {
        return false;
    };
    with_objects_mut(|objects| {
        objects
            .get_mut(idx)
            .and_then(|object| object.fields.get_mut(slot))
            .is_some_and(|field| field.set_exact_unsigned_u64(value))
    })
    .unwrap_or(false)
}

pub(crate) fn get_exact_signed_i64(handle: i64, slot: usize) -> Option<i64> {
    let idx = handle_to_index(handle)?;
    with_objects(|objects| {
        objects
            .get(idx)?
            .fields
            .get(slot)
            .and_then(|field| field.as_exact_signed_i64())
    })?
}

pub(crate) fn set_exact_signed_i64(handle: i64, slot: usize, value: i64) -> bool {
    let Some(idx) = handle_to_index(handle) else {
        return false;
    };
    with_objects_mut(|objects| {
        objects
            .get_mut(idx)
            .and_then(|object| object.fields.get_mut(slot))
            .is_some_and(|field| field.set_exact_signed_i64(value))
    })
    .unwrap_or(false)
}
