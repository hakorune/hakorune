use super::handle_cache::{cache_store, with_cache_entry};
use super::value_codec::{runtime_i64_from_box_ref_caller, BorrowedAliasEncodeCaller};
use nyash_rust::{box_trait::NyashBox, boxes::array::ArrayBox, runtime::host_handles as handles};

#[inline(always)]
fn encode_array_item_to_i64(item: &dyn NyashBox, drop_epoch: u64) -> i64 {
    // Keep scalar/bool before borrowed-handle reuse so immediate classes stay canonical.
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
    runtime_i64_from_box_ref_caller(item, BorrowedAliasEncodeCaller::ArrayGetIndexEncoded)
}

#[inline(always)]
pub(crate) fn array_get_index_encoded_i64(handle: i64, idx: i64) -> Option<i64> {
    if handle <= 0 || idx < 0 {
        return None;
    }
    let idx_usize = idx as usize;
    let drop_epoch = handles::drop_epoch();
    with_array_box_at_epoch(handle, drop_epoch, |arr| {
        if let Some(value) = arr.slot_load_i64_raw(idx) {
            return Some(value);
        }
        arr.with_items_read(|items| {
            let item = items.get(idx_usize)?;
            Some(encode_array_item_to_i64(item.as_ref(), drop_epoch))
        })
    })
    .flatten()
}

#[inline(always)]
pub(crate) fn with_array_box<R>(handle: i64, f: impl FnOnce(&ArrayBox) -> R) -> Option<R> {
    with_array_box_at_epoch(handle, handles::drop_epoch(), f)
}

#[inline(always)]
pub(crate) fn with_array_box_at_epoch<R>(
    handle: i64,
    drop_epoch: u64,
    f: impl FnOnce(&ArrayBox) -> R,
) -> Option<R> {
    // Array-specialized fast path keeps the same contract as with_typed_box:
    // invalid handle or type mismatch returns None.
    if handle <= 0 {
        return None;
    }
    let mut f = Some(f);
    if let Some(out) = with_cache_entry(handle, drop_epoch, |entry| {
        let arr = entry.array_ref()?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::handle_cache::clear_cache_slot;
    use std::sync::Arc;

    #[test]
    fn invalid_handle_short_circuits_array_route() {
        clear_cache_slot();
        assert!(with_array_box(-1, |_| 1).is_none());
    }

    #[test]
    fn array_get_index_fail_safe_contract() {
        use nyash_rust::box_trait::IntegerBox;

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
