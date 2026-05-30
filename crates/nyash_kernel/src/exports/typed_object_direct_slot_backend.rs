//! DirectSlotExact backend storage and explicit compatibility view.
//!
//! DirectSlot storage is the primary representation for the exact-lane path.
//! The typed-object materialized view is kept as an explicit compatibility
//! layer so the export surface can continue to serve legacy helper readers
//! without making the compatibility surface the center of the design.

use std::cell::RefCell;

use super::typed_object::{handle_to_index, TypedSlotObject};
use super::typed_object_pinned_arena::DirectSlotObjectV0Box;

thread_local! {
    static DIRECT_SLOT_OBJECTS: RefCell<Vec<DirectSlotObjectV0Box>> = const { RefCell::new(Vec::new()) };
    static DIRECT_SLOT_MATERIALIZED_VIEWS: RefCell<Vec<TypedSlotObject>> = const { RefCell::new(Vec::new()) };
}

pub(crate) fn new_direct_slot_object(object: TypedSlotObject) -> Option<i64> {
    DIRECT_SLOT_OBJECTS.with(|objects| {
        let object = DirectSlotObjectV0Box::from_typed_object(object)?;
        let handle = object.handle()?;
        objects.try_borrow_mut().ok()?.push(object);
        Some(handle)
    })
}

pub(crate) fn materialize_direct_slot_snapshot(handle: i64) -> Option<TypedSlotObject> {
    DIRECT_SLOT_OBJECTS.with(|objects| {
        let objects = objects.try_borrow().ok()?;
        let object = objects
            .iter()
            .find(|object| object.matches_handle(handle))?;
        object.materialize_typed_object_snapshot()
    })
}

pub(crate) fn materialize_direct_slot_view_handle(handle: i64) -> Option<i64> {
    let snapshot = materialize_direct_slot_snapshot(handle)?;
    DIRECT_SLOT_MATERIALIZED_VIEWS.with(|objects| {
        let mut objects = objects.try_borrow_mut().ok()?;
        objects.push(snapshot);
        Some(-(objects.len() as i64))
    })
}

pub(crate) fn with_direct_slot_materialized_view<R>(
    handle: i64,
    f: impl FnOnce(&TypedSlotObject) -> Option<R>,
) -> Option<R> {
    let idx = handle_to_index(handle)?;
    DIRECT_SLOT_MATERIALIZED_VIEWS.with(|objects| {
        let objects = objects.try_borrow().ok()?;
        f(objects.get(idx)?)
    })
}

pub(crate) fn with_direct_slot_materialized_view_mut<R>(
    handle: i64,
    f: impl FnOnce(&mut TypedSlotObject) -> Option<R>,
) -> Option<R> {
    let idx = handle_to_index(handle)?;
    DIRECT_SLOT_MATERIALIZED_VIEWS.with(|objects| {
        let mut objects = objects.try_borrow_mut().ok()?;
        f(objects.get_mut(idx)?)
    })
}

pub(crate) fn with_direct_slot_object<R>(
    handle: i64,
    f: impl FnOnce(&DirectSlotObjectV0Box) -> R,
) -> Option<R> {
    DIRECT_SLOT_OBJECTS.with(|objects| {
        let objects = objects.try_borrow().ok()?;
        let object = objects
            .iter()
            .find(|object| object.matches_handle(handle))?;
        Some(f(object))
    })
}

pub(crate) fn with_direct_slot_object_mut<R>(
    handle: i64,
    f: impl FnOnce(&mut DirectSlotObjectV0Box) -> R,
) -> Option<R> {
    DIRECT_SLOT_OBJECTS.with(|objects| {
        let mut objects = objects.try_borrow_mut().ok()?;
        let object = objects
            .iter_mut()
            .find(|object| object.matches_handle(handle))?;
        Some(f(object))
    })
}
