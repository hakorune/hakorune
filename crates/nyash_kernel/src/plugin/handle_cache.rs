// typed handle cache / typed dispatch helper for native metal keep
//
// Responsibilities:
// - short-lived TLS cache keyed by `(handle, drop_epoch)`
// - typed downcast helpers for ArrayBox / MapBox / InstanceBox
// - shared typed cache entry reuse for array/map/instance routes
//
// Non-goals:
// - ABI manifest truth
// - value representation policy ownership
// - array/map algorithm policy
use nyash_rust::{
    box_trait::NyashBox,
    boxes::{array::ArrayBox, map_box::MapBox},
    instance_v2::InstanceBox,
    runtime::host_handles as handles,
};
use std::{cell::RefCell, ptr::NonNull, sync::Arc};

pub(super) struct HandleCacheEntry {
    handle: i64,
    drop_epoch: u64,
    obj: Arc<dyn NyashBox>,
    array_ptr: Option<NonNull<ArrayBox>>,
    map_ptr: Option<NonNull<MapBox>>,
    instance_ptr: Option<NonNull<InstanceBox>>,
}

impl HandleCacheEntry {
    #[inline(always)]
    pub(super) fn array_ref(&self) -> Option<&ArrayBox> {
        let ptr = self.array_ptr?;
        // SAFETY: pointers are created from `self.obj` and remain valid
        // while this cache entry keeps the Arc alive.
        Some(unsafe { ptr.as_ref() })
    }

    #[inline(always)]
    fn map_ref(&self) -> Option<&MapBox> {
        let ptr = self.map_ptr?;
        // SAFETY: pointers are created from `self.obj` and remain valid
        // while this cache entry keeps the Arc alive.
        Some(unsafe { ptr.as_ref() })
    }

    #[inline(always)]
    fn instance_ref(&self) -> Option<&InstanceBox> {
        let ptr = self.instance_ptr?;
        // SAFETY: pointers are created from `self.obj` and remain valid
        // while this cache entry keeps the Arc alive.
        Some(unsafe { ptr.as_ref() })
    }
}

#[inline(always)]
fn build_cache_entry(handle: i64, drop_epoch: u64, obj: Arc<dyn NyashBox>) -> HandleCacheEntry {
    let array_ptr = obj.as_any().downcast_ref::<ArrayBox>().map(NonNull::from);
    let map_ptr = obj.as_any().downcast_ref::<MapBox>().map(NonNull::from);
    let instance_ptr = obj
        .as_any()
        .downcast_ref::<InstanceBox>()
        .map(NonNull::from);
    HandleCacheEntry {
        handle,
        drop_epoch,
        obj,
        array_ptr,
        map_ptr,
        instance_ptr,
    }
}

thread_local! {
    static HANDLE_CACHE: RefCell<Option<HandleCacheEntry>> = RefCell::new(None);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CacheProbeKind {
    Hit,
    MissHandle,
    MissDropEpoch,
}

#[cfg(test)]
#[inline(always)]
pub(crate) fn clear_cache_slot() {
    HANDLE_CACHE.with(|slot| *slot.borrow_mut() = None);
}

#[inline(always)]
pub(super) fn with_cache_entry<R>(
    handle: i64,
    drop_epoch: u64,
    f: impl FnOnce(&HandleCacheEntry) -> Option<R>,
) -> Option<R> {
    HANDLE_CACHE.with(|slot| {
        let cached = slot.borrow();
        let entry = cached.as_ref()?;
        if entry.handle != handle || entry.drop_epoch != drop_epoch {
            return None;
        }
        f(entry)
    })
}

#[inline(always)]
pub(super) fn cache_probe_kind(handle: i64, drop_epoch: u64) -> CacheProbeKind {
    HANDLE_CACHE.with(|slot| {
        let cached = slot.borrow();
        let Some(entry) = cached.as_ref() else {
            return CacheProbeKind::MissHandle;
        };
        if entry.handle != handle {
            return CacheProbeKind::MissHandle;
        }
        if entry.drop_epoch != drop_epoch {
            return CacheProbeKind::MissDropEpoch;
        }
        CacheProbeKind::Hit
    })
}

#[inline(always)]
pub(super) fn cache_store(handle: i64, drop_epoch: u64, obj: Arc<dyn NyashBox>) {
    HANDLE_CACHE.with(|slot| {
        *slot.borrow_mut() = Some(build_cache_entry(handle, drop_epoch, obj));
    });
}

#[inline(always)]
fn object_from_handle_cached_impl(handle: i64) -> Option<Arc<dyn NyashBox>> {
    if handle <= 0 {
        return None;
    }
    let drop_epoch = handles::drop_epoch();
    object_from_handle_cached_with_epoch(handle, drop_epoch)
}

#[inline(always)]
pub(crate) fn object_from_handle_cached(handle: i64) -> Option<Arc<dyn NyashBox>> {
    object_from_handle_cached_impl(handle)
}

#[inline(always)]
fn with_object_from_handle_cached_with_epoch<R>(
    handle: i64,
    drop_epoch: u64,
    f: impl FnMut(&Arc<dyn NyashBox>) -> Option<R>,
) -> Option<R> {
    let mut f = Some(f);
    if let Some(out) = with_cache_entry(handle, drop_epoch, |entry| {
        let mut f = f.take().expect("cache callback");
        f(&entry.obj)
    }) {
        return Some(out);
    }

    let obj = handles::get(handle as u64)?;
    cache_store(handle, drop_epoch, obj.clone());
    let mut f = f.take().expect("cache callback");
    f(&obj)
}

#[inline(always)]
fn object_from_handle_cached_with_epoch(handle: i64, drop_epoch: u64) -> Option<Arc<dyn NyashBox>> {
    with_object_from_handle_cached_with_epoch(handle, drop_epoch, |obj| Some(obj.clone()))
}

#[inline(always)]
pub(crate) fn with_map_box<R>(handle: i64, f: impl FnOnce(&MapBox) -> R) -> Option<R> {
    if handle <= 0 {
        return None;
    }
    let drop_epoch = handles::drop_epoch();
    let mut f = Some(f);
    if let Some(out) = with_cache_entry(handle, drop_epoch, |entry| {
        let map = entry.map_ref()?;
        let f = f.take().expect("map callback");
        Some(f(map))
    }) {
        return Some(out);
    }

    with_object_from_handle_cached_with_epoch(handle, drop_epoch, |obj| {
        let map = obj.as_any().downcast_ref::<MapBox>()?;
        let f = f.take().expect("map callback");
        Some(f(map))
    })
}

#[inline(always)]
pub(crate) fn with_instance_box<R>(handle: i64, f: impl FnOnce(&InstanceBox) -> R) -> Option<R> {
    if handle <= 0 {
        return None;
    }
    let drop_epoch = handles::drop_epoch();
    let mut f = Some(f);
    if let Some(out) = with_cache_entry(handle, drop_epoch, |entry| {
        let instance = entry.instance_ref()?;
        let f = f.take().expect("instance callback");
        Some(f(instance))
    }) {
        return Some(out);
    }

    with_object_from_handle_cached_with_epoch(handle, drop_epoch, |obj| {
        let instance = obj.as_any().downcast_ref::<InstanceBox>()?;
        let f = f.take().expect("instance callback");
        Some(f(instance))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::array_handle_cache::with_array_box;
    use nyash_rust::box_trait::IntegerBox;

    #[test]
    fn cache_invalidates_on_drop_epoch_when_handle_is_reused() {
        clear_cache_slot();

        let arr_a: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        let h = handles::to_handle_arc(arr_a.clone()) as i64;
        let first = object_from_handle_cached(h).expect("first object");
        assert!(Arc::ptr_eq(&first, &arr_a));

        handles::drop_handle(h as u64);
        let after_drop = object_from_handle_cached(h);
        if let Some(obj) = after_drop {
            assert!(!Arc::ptr_eq(&obj, &arr_a));
        }

        // Keep arr_a alive intentionally. If cache invalidation fails,
        // stale Weak upgrade would incorrectly return arr_a.
        let arr_b: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        let h2 = handles::to_handle_arc(arr_b.clone()) as i64;

        let second = object_from_handle_cached(h2).expect("second object");
        assert!(Arc::ptr_eq(&second, &arr_b));
        assert!(!Arc::ptr_eq(&second, &arr_a));
    }

    #[test]
    fn cached_handle_lookup_still_rehydrates_container_types() {
        clear_cache_slot();

        let arr: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        let map: Arc<dyn NyashBox> = Arc::new(MapBox::new());
        let arr_h = handles::to_handle_arc(arr) as i64;
        let map_h = handles::to_handle_arc(map) as i64;

        let arr_obj = object_from_handle_cached(arr_h).expect("array object");
        assert!(arr_obj.as_any().downcast_ref::<ArrayBox>().is_some());

        let map_obj = object_from_handle_cached(map_h).expect("map object");
        assert!(map_obj.as_any().downcast_ref::<MapBox>().is_some());

        // Touch cache with non-container handle then re-check route dispatch.
        let scalar: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(7));
        let scalar_h = handles::to_handle_arc(scalar) as i64;
        assert!(object_from_handle_cached(scalar_h).is_some());
        let arr_obj2 = object_from_handle_cached(arr_h).expect("array object 2");
        assert!(arr_obj2.as_any().downcast_ref::<ArrayBox>().is_some());
    }

    #[test]
    fn invalid_handle_short_circuits_all_routes() {
        clear_cache_slot();

        assert!(object_from_handle_cached(0).is_none());
        assert!(with_array_box(-1, |_| 1).is_none());
        assert!(with_map_box(-1, |_| 1).is_none());
        assert!(with_instance_box(-1, |_| 1).is_none());
    }
}
