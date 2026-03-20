use super::value_codec::runtime_i64_from_box_ref;
use nyash_rust::{
    box_trait::NyashBox,
    boxes::{array::ArrayBox, map_box::MapBox},
    instance_v2::InstanceBox,
    runtime::host_handles as handles,
};
use std::{cell::RefCell, sync::Arc};

struct HandleCacheEntry {
    handle: i64,
    drop_epoch: u64,
    obj: Arc<dyn NyashBox>,
}

thread_local! {
    static HANDLE_CACHE: RefCell<Option<HandleCacheEntry>> = RefCell::new(None);
}

#[cfg(test)]
#[inline(always)]
fn clear_cache_slot() {
    HANDLE_CACHE.with(|slot| *slot.borrow_mut() = None);
}

#[inline(always)]
fn with_cache_entry<R>(
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
fn cache_load(handle: i64, drop_epoch: u64) -> Option<Arc<dyn NyashBox>> {
    with_cache_entry(handle, drop_epoch, |entry| Some(entry.obj.clone()))
}

#[inline(always)]
fn cache_store(handle: i64, drop_epoch: u64, obj: Arc<dyn NyashBox>) {
    HANDLE_CACHE.with(|slot| {
        *slot.borrow_mut() = Some(HandleCacheEntry {
            handle,
            drop_epoch,
            obj,
        });
    });
}

#[inline(always)]
fn encode_array_item_to_i64(item: &dyn NyashBox, drop_epoch: u64) -> i64 {
    if let Some(iv) = item.as_i64_fast() {
        return iv;
    }
    if let Some(bv) = item.as_bool_fast() {
        return if bv { 1 } else { 0 };
    }
    if let Some((source_handle, source_drop_epoch)) = item.borrowed_handle_source_fast() {
        if source_drop_epoch == drop_epoch {
            return source_handle;
        }
    }
    runtime_i64_from_box_ref(item)
}

#[inline(always)]
pub(crate) fn array_get_index_encoded_i64(handle: i64, idx: i64) -> Option<i64> {
    if handle <= 0 || idx < 0 {
        return None;
    }
    let idx_usize = idx as usize;
    let drop_epoch = handles::drop_epoch();
    if let Some(out) = with_cache_entry(handle, drop_epoch, |entry| {
        let arr = entry.obj.as_any().downcast_ref::<ArrayBox>()?;
        let items = arr.items.read();
        let item = items.get(idx_usize)?;
        Some(encode_array_item_to_i64(item.as_ref(), drop_epoch))
    }) {
        return Some(out);
    }

    let obj = handles::get(handle as u64)?;
    let arr = obj.as_any().downcast_ref::<ArrayBox>()?;
    cache_store(handle, drop_epoch, obj.clone());
    let items = arr.items.read();
    let item = items.get(idx_usize)?;
    Some(encode_array_item_to_i64(item.as_ref(), drop_epoch))
}

#[inline(always)]
fn object_from_handle_cached(handle: i64) -> Option<Arc<dyn NyashBox>> {
    if handle <= 0 {
        return None;
    }
    let drop_epoch = handles::drop_epoch();
    object_from_handle_cached_with_epoch(handle, drop_epoch)
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
fn with_typed_box<T: 'static, R>(handle: i64, f: impl FnOnce(&T) -> R) -> Option<R> {
    if handle <= 0 {
        return None;
    }
    let drop_epoch = handles::drop_epoch();
    let mut f = Some(f);
    with_object_from_handle_cached_with_epoch(handle, drop_epoch, |obj| {
        let typed = obj.as_any().downcast_ref::<T>()?;
        let f = f.take().expect("typed callback");
        Some(f(typed))
    })
}

#[inline(always)]
pub(crate) fn with_array_box<R>(handle: i64, f: impl FnOnce(&ArrayBox) -> R) -> Option<R> {
    if handle <= 0 {
        return None;
    }
    let drop_epoch = handles::drop_epoch();
    let mut f = Some(f);
    if let Some(out) = with_cache_entry(handle, drop_epoch, |entry| {
        let arr = entry.obj.as_any().downcast_ref::<ArrayBox>()?;
        let f = f.take().expect("array callback");
        Some(f(arr))
    }) {
        return Some(out);
    }

    let obj = handles::get(handle as u64)?;
    let arr = obj.as_any().downcast_ref::<ArrayBox>()?;
    cache_store(handle, drop_epoch, obj.clone());
    let f = f.take().expect("array callback");
    Some(f(arr))
}

#[inline(always)]
pub(crate) fn with_map_box<R>(handle: i64, f: impl FnOnce(&MapBox) -> R) -> Option<R> {
    with_typed_box::<MapBox, _>(handle, f)
}

#[inline(always)]
pub(crate) fn with_instance_box<R>(handle: i64, f: impl FnOnce(&InstanceBox) -> R) -> Option<R> {
    with_typed_box::<InstanceBox, _>(handle, f)
}

#[inline(always)]
pub(crate) fn with_array_or_map<R>(
    handle: i64,
    on_array: impl FnOnce(&ArrayBox) -> R,
    on_map: impl FnOnce(&MapBox) -> R,
) -> Option<R> {
    let obj = object_from_handle_cached(handle)?;
    if let Some(arr) = obj.as_any().downcast_ref::<ArrayBox>() {
        return Some(on_array(arr));
    }
    if let Some(map) = obj.as_any().downcast_ref::<MapBox>() {
        return Some(on_map(map));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn cached_handle_lookup_still_resolves_type_routes() {
        clear_cache_slot();

        let arr: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        let map: Arc<dyn NyashBox> = Arc::new(MapBox::new());
        let arr_h = handles::to_handle_arc(arr) as i64;
        let map_h = handles::to_handle_arc(map) as i64;

        let arr_value = with_array_or_map(arr_h, |_| 10, |_| 20).expect("array route");
        assert_eq!(arr_value, 10);

        let map_value = with_array_or_map(map_h, |_| 10, |_| 20).expect("map route");
        assert_eq!(map_value, 20);

        // Touch cache with non-container handle then re-check route dispatch.
        let scalar: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(7));
        let scalar_h = handles::to_handle_arc(scalar) as i64;
        assert!(object_from_handle_cached(scalar_h).is_some());
        let arr_value2 = with_array_or_map(arr_h, |_| 30, |_| 40).expect("array route 2");
        assert_eq!(arr_value2, 30);
    }

    #[test]
    fn invalid_handle_short_circuits_all_routes() {
        clear_cache_slot();

        assert!(object_from_handle_cached(0).is_none());
        assert!(with_array_box(-1, |_| 1).is_none());
        assert!(with_map_box(-1, |_| 1).is_none());
        assert!(with_instance_box(-1, |_| 1).is_none());
        assert!(with_array_or_map(-1, |_| 1, |_| 2).is_none());
    }

    #[test]
    fn array_get_index_fail_safe_contract() {
        clear_cache_slot();

        let arr: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        let handle = handles::to_handle_arc(arr.clone()) as i64;
        assert_eq!(array_get_index_encoded_i64(handle, -1), None);
        assert_eq!(array_get_index_encoded_i64(handle, 0), None);

        let array_box = arr
            .as_any()
            .downcast_ref::<ArrayBox>()
            .expect("array downcast");
        let _ = array_box.push(Box::new(IntegerBox::new(42)));

        assert_eq!(array_get_index_encoded_i64(handle, 0), Some(42));
        assert_eq!(array_get_index_encoded_i64(handle, 1), None);
    }
}
