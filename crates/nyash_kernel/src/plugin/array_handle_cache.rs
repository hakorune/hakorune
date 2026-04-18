use super::handle_cache::{cache_store, with_cache_entry};
use super::value_codec::{runtime_i64_from_box_ref_caller, BorrowedAliasEncodeCaller};
use nyash_rust::{box_trait::NyashBox, boxes::array::ArrayBox, runtime::host_handles as handles};

#[inline(always)]
fn encode_array_item_to_i64(item: &dyn NyashBox) -> i64 {
    // Keep scalar/bool before borrowed-handle reuse so immediate classes stay canonical.
    if let Some(iv) = item.as_i64_fast() {
        return iv;
    }
    if let Some(bv) = item.as_bool_fast() {
        return if bv { 1 } else { 0 };
    }
    // Borrowed alias reuse policy lives in value_codec so array/string/map reads
    // share the same live-source vs cached-handle boundary.
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
            Some(encode_array_item_to_i64(item.as_ref()))
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
    use crate::plugin::value_codec::{maybe_borrow_string_keep_with_epoch, SourceLifetimeKeep};
    use nyash_rust::box_trait::StringBox;
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

    #[test]
    fn array_get_index_reuses_cached_runtime_handle_for_unpublished_alias() {
        clear_cache_slot();

        let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("array-cached-alias".to_string()));
        let alias = maybe_borrow_string_keep_with_epoch(
            SourceLifetimeKeep::string_box(value),
            0,
            handles::drop_epoch(),
        );
        let arr: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        let handle = handles::to_handle_arc(arr.clone()) as i64;
        let array_box = arr
            .as_any()
            .downcast_ref::<ArrayBox>()
            .expect("array downcast");
        let _ = array_box.push(alias);

        let first = array_get_index_encoded_i64(handle, 0).expect("first encoded handle");
        let second = array_get_index_encoded_i64(handle, 0).expect("second encoded handle");

        assert!(first > 0);
        assert_eq!(first, second);

        let out_obj = handles::get(first as u64).expect("cached runtime handle");
        let out_sb = out_obj
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("runtime value should remain StringBox");
        assert_eq!(out_sb.value, "array-cached-alias");
    }

    #[cfg(feature = "perf-observe")]
    #[test]
    fn array_get_index_records_cached_handle_hit_for_array_lane() {
        clear_cache_slot();
        crate::test_support::with_env_var("NYASH_PERF_COUNTERS", "1", || {
            let value: Arc<dyn NyashBox> =
                Arc::new(StringBox::new("array-cached-observe".to_string()));
            let alias = maybe_borrow_string_keep_with_epoch(
                SourceLifetimeKeep::string_box(value),
                0,
                handles::drop_epoch(),
            );
            let arr: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
            let handle = handles::to_handle_arc(arr.clone()) as i64;
            let array_box = arr
                .as_any()
                .downcast_ref::<ArrayBox>()
                .expect("array downcast");
            let _ = array_box.push(alias);

            let warmup = array_get_index_encoded_i64(handle, 0).expect("warm cached handle");
            let before = crate::observe::borrowed_alias_encode_snapshot_for_tests();
            let cached = array_get_index_encoded_i64(handle, 0).expect("cached encoded handle");
            let after = crate::observe::borrowed_alias_encode_snapshot_for_tests();

            assert_eq!(warmup, cached);
            assert_eq!(after.cached_handle_hit - before.cached_handle_hit, 1);
            assert_eq!(
                after.cached_handle_hit_array_get_index
                    - before.cached_handle_hit_array_get_index,
                1
            );
            assert_eq!(
                after.cached_handle_hit_map_runtime_data_get_any
                    - before.cached_handle_hit_map_runtime_data_get_any,
                0
            );
        });
    }

    #[cfg(feature = "perf-observe")]
    #[test]
    fn array_get_index_records_live_source_hit_for_array_lane() {
        clear_cache_slot();
        crate::test_support::with_env_var("NYASH_PERF_COUNTERS", "1", || {
            let value: Arc<dyn NyashBox> =
                Arc::new(StringBox::new("array-live-observe".to_string()));
            let value_h = handles::to_handle_arc(value) as i64;
            let alias = crate::plugin::value_codec::any_arg_to_box_with_profile(
                value_h,
                crate::plugin::value_codec::CodecProfile::ArrayFastBorrowString,
            );
            let arr: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
            let handle = handles::to_handle_arc(arr.clone()) as i64;
            let array_box = arr
                .as_any()
                .downcast_ref::<ArrayBox>()
                .expect("array downcast");
            let _ = array_box.push(alias);

            let before = crate::observe::borrowed_alias_encode_snapshot_for_tests();
            let live = array_get_index_encoded_i64(handle, 0).expect("live encoded handle");
            let after = crate::observe::borrowed_alias_encode_snapshot_for_tests();

            assert_eq!(live, value_h);
            assert_eq!(after.live_source_hit - before.live_source_hit, 1);
            assert_eq!(
                after.live_source_hit_array_get_index
                    - before.live_source_hit_array_get_index,
                1
            );
            assert_eq!(
                after.live_source_hit_map_runtime_data_get_any
                    - before.live_source_hit_map_runtime_data_get_any,
                0
            );
            assert_eq!(after.cached_handle_hit - before.cached_handle_hit, 0);
        });
    }

    #[cfg(feature = "perf-observe")]
    #[test]
    fn array_get_index_records_cold_fallback_for_array_lane() {
        clear_cache_slot();
        crate::test_support::with_env_var("NYASH_PERF_COUNTERS", "1", || {
            let value: Arc<dyn NyashBox> =
                Arc::new(StringBox::new("array-fallback-observe".to_string()));
            let alias = maybe_borrow_string_keep_with_epoch(
                SourceLifetimeKeep::string_box(value),
                0,
                handles::drop_epoch(),
            );
            let arr: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
            let handle = handles::to_handle_arc(arr.clone()) as i64;
            let array_box = arr
                .as_any()
                .downcast_ref::<ArrayBox>()
                .expect("array downcast");
            let _ = array_box.push(alias);

            let before = crate::observe::borrowed_alias_encode_snapshot_for_tests();
            let cold = array_get_index_encoded_i64(handle, 0).expect("cold encoded handle");
            let after = crate::observe::borrowed_alias_encode_snapshot_for_tests();

            assert!(cold > 0);
            assert_eq!(
                after.fallback_to_handle_arc - before.fallback_to_handle_arc,
                1
            );
            assert_eq!(
                after.fallback_to_handle_arc_array_get_index
                    - before.fallback_to_handle_arc_array_get_index,
                1
            );
            assert_eq!(
                after.fallback_to_handle_arc_map_runtime_data_get_any
                    - before.fallback_to_handle_arc_map_runtime_data_get_any,
                0
            );
            assert_eq!(after.live_source_hit - before.live_source_hit, 0);
            assert_eq!(after.cached_handle_hit - before.cached_handle_hit, 0);
        });
    }
}
