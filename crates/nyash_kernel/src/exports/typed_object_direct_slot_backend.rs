//! DirectSlotExact backend storage and materialized export view.
//!
//! DirectSlot storage is the primary representation for the exact-lane path.
//! The typed-object materialized view exists as a separate export view, not as
//! the storage source of truth.

use std::cell::RefCell;

use super::typed_object::{handle_to_index, TypedSlotObject};
use super::typed_object_pinned_arena::DirectSlotObjectV0Box;

thread_local! {
    static DIRECT_SLOT_OBJECTS: RefCell<Vec<DirectSlotObjectV0Box>> = const { RefCell::new(Vec::new()) };
    static DIRECT_SLOT_MATERIALIZED_VIEWS: RefCell<Vec<TypedSlotObject>> = const { RefCell::new(Vec::new()) };
}

#[allow(dead_code)]
fn with_direct_slot_objects<R>(f: impl FnOnce(&[DirectSlotObjectV0Box]) -> Option<R>) -> Option<R> {
    DIRECT_SLOT_OBJECTS.with(|objects| {
        let objects = objects.try_borrow().ok()?;
        f(&objects)
    })
}

fn with_direct_slot_objects_mut<R>(
    f: impl FnOnce(&mut Vec<DirectSlotObjectV0Box>) -> Option<R>,
) -> Option<R> {
    DIRECT_SLOT_OBJECTS.with(|objects| {
        let mut objects = objects.try_borrow_mut().ok()?;
        f(&mut objects)
    })
}

fn with_direct_slot_views<R>(f: impl FnOnce(&[TypedSlotObject]) -> Option<R>) -> Option<R> {
    DIRECT_SLOT_MATERIALIZED_VIEWS.with(|objects| {
        let objects = objects.try_borrow().ok()?;
        f(&objects)
    })
}

fn with_direct_slot_views_mut<R>(
    f: impl FnOnce(&mut Vec<TypedSlotObject>) -> Option<R>,
) -> Option<R> {
    DIRECT_SLOT_MATERIALIZED_VIEWS.with(|objects| {
        let mut objects = objects.try_borrow_mut().ok()?;
        f(&mut objects)
    })
}

pub(crate) fn new_direct_slot_object(object: TypedSlotObject) -> Option<i64> {
    with_direct_slot_objects_mut(|objects| {
        let object = DirectSlotObjectV0Box::from_typed_object(object)?;
        let handle = object.handle()?;
        objects.push(object);
        Some(handle)
    })
}

#[cfg(test)]
pub(crate) fn materialize_direct_slot_snapshot(handle: i64) -> Option<TypedSlotObject> {
    with_direct_slot_objects(|objects| {
        let object = objects
            .iter()
            .find(|object| object.matches_handle(handle))?;
        object.materialize_typed_object_snapshot()
    })
}

#[cfg(test)]
pub(crate) fn materialize_direct_slot_view_handle(handle: i64) -> Option<i64> {
    let snapshot = materialize_direct_slot_snapshot(handle)?;
    with_direct_slot_views_mut(|objects| {
        objects.push(snapshot);
        Some(-(objects.len() as i64))
    })
}

pub(crate) fn with_direct_slot_materialized_view<R>(
    handle: i64,
    f: impl FnOnce(&TypedSlotObject) -> Option<R>,
) -> Option<R> {
    let idx = handle_to_index(handle)?;
    with_direct_slot_views(|objects| f(objects.get(idx)?))
}

pub(crate) fn with_direct_slot_materialized_view_mut<R>(
    handle: i64,
    f: impl FnOnce(&mut TypedSlotObject) -> Option<R>,
) -> Option<R> {
    let idx = handle_to_index(handle)?;
    with_direct_slot_views_mut(|objects| f(objects.get_mut(idx)?))
}

pub(crate) fn with_direct_slot_object_mut<R>(
    handle: i64,
    f: impl FnOnce(&mut DirectSlotObjectV0Box) -> R,
) -> Option<R> {
    with_direct_slot_objects_mut(|objects| {
        let object = objects
            .iter_mut()
            .find(|object| object.matches_handle(handle))?;
        Some(f(object))
    })
}
