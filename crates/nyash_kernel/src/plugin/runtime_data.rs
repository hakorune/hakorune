// RuntimeDataBox-compatible dynamic dispatch helpers.
//
// These exports bridge RuntimeDataBox method calls in AOT/LLVM to concrete
// host boxes (ArrayBox/MapBox) without relying on static box-name guesses.
// Manifest reading: all `nyash.runtime_data.*` rows are runtime-facade only.

use super::handle_cache::object_from_handle_cached;
use super::map_runtime_facade::{
    map_runtime_data_get_any_key, map_runtime_data_has_any_key, map_runtime_data_set_any_key,
};
use super::runtime_data_array_dispatch::{
    runtime_data_array_get_any, runtime_data_array_has_any, runtime_data_array_push_any,
    runtime_data_array_set_any,
};
use nyash_rust::boxes::{array::ArrayBox, map_box::MapBox};

#[inline(always)]
fn with_runtime_data_route<R>(
    recv_h: i64,
    on_array: impl FnOnce() -> R,
    on_map: impl FnOnce() -> R,
) -> Option<R> {
    // RuntimeData stays facade-only: it owns the Array/Map branch decision here,
    // then delegates behavior to the array/map runtime facades.
    let obj = object_from_handle_cached(recv_h)?;
    if obj.as_any().downcast_ref::<ArrayBox>().is_some() {
        return Some(on_array());
    }
    if obj.as_any().downcast_ref::<MapBox>().is_some() {
        return Some(on_map());
    }
    None
}

// nyash.runtime_data.get_hh(recv_h, key_any) -> mixed runtime i64/handle value (or 0)
#[export_name = "nyash.runtime_data.get_hh"]
pub extern "C" fn nyash_runtime_data_get_hh(recv_h: i64, key_any: i64) -> i64 {
    with_runtime_data_route(
        recv_h,
        || runtime_data_array_get_any(recv_h, key_any),
        || map_runtime_data_get_any_key(recv_h, key_any),
    )
    .unwrap_or(0)
}

// nyash.runtime_data.set_hhh(recv_h, key_any, val_any) -> 0/1
#[export_name = "nyash.runtime_data.set_hhh"]
pub extern "C" fn nyash_runtime_data_set_hhh(recv_h: i64, key_any: i64, val_any: i64) -> i64 {
    with_runtime_data_route(
        recv_h,
        || runtime_data_array_set_any(recv_h, key_any, val_any),
        || map_runtime_data_set_any_key(recv_h, key_any, val_any),
    )
    .unwrap_or(0)
}

// nyash.runtime_data.has_hh(recv_h, key_any) -> 0/1
// K2-core keeps array `has` on the runtime facade until a narrower raw seam is
// explicitly accepted. Array bounds/missing-key remain fail-safe here.
#[export_name = "nyash.runtime_data.has_hh"]
pub extern "C" fn nyash_runtime_data_has_hh(recv_h: i64, key_any: i64) -> i64 {
    with_runtime_data_route(
        recv_h,
        || runtime_data_array_has_any(recv_h, key_any),
        || map_runtime_data_has_any_key(recv_h, key_any),
    )
    .unwrap_or(0)
}

// nyash.runtime_data.push_hh(recv_h, val_any) -> new_len (array) / 0
#[export_name = "nyash.runtime_data.push_hh"]
pub extern "C" fn nyash_runtime_data_push_hh(recv_h: i64, val_any: i64) -> i64 {
    with_runtime_data_route(
        recv_h,
        || runtime_data_array_push_any(recv_h, val_any),
        || 0,
    )
    .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nyash_rust::box_trait::NyashBox;
    use nyash_rust::boxes::array::ArrayBox;
    use nyash_rust::boxes::basic::IntegerBox;
    use nyash_rust::boxes::map_box::MapBox;
    use nyash_rust::runtime::host_handles as handles;
    use std::sync::Arc;

    fn new_map_handle() -> i64 {
        let map: Arc<dyn NyashBox> = Arc::new(MapBox::new());
        handles::to_handle_arc(map) as i64
    }

    fn new_array_handle() -> i64 {
        let arr: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        handles::to_handle_arc(arr) as i64
    }

    fn new_int_handle(value: i64) -> i64 {
        let integer: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(value));
        handles::to_handle_arc(integer) as i64
    }

    fn new_string_handle(value: &str) -> i64 {
        let string: Arc<dyn NyashBox> =
            Arc::new(nyash_rust::box_trait::StringBox::new(value.to_string()));
        handles::to_handle_arc(string) as i64
    }

    #[test]
    fn runtime_data_invalid_handle_returns_zero() {
        assert_eq!(nyash_runtime_data_get_hh(0, 1), 0);
        assert_eq!(nyash_runtime_data_set_hhh(0, 1, 2), 0);
        assert_eq!(nyash_runtime_data_has_hh(0, 1), 0);
        assert_eq!(nyash_runtime_data_push_hh(0, 1), 0);
    }

    #[test]
    fn runtime_data_array_round_trip_keeps_rawarray_contract() {
        let handle = new_array_handle();
        let value_h = new_int_handle(11);
        let updated_h = new_int_handle(22);

        assert_eq!(nyash_runtime_data_push_hh(handle, value_h), 1);
        assert_eq!(nyash_runtime_data_has_hh(handle, 0), 1);
        assert_eq!(nyash_runtime_data_get_hh(handle, 0), 11);
        assert_eq!(nyash_runtime_data_set_hhh(handle, 0, updated_h), 1);
        assert_eq!(nyash_runtime_data_get_hh(handle, 0), 22);
        assert_eq!(nyash_runtime_data_get_hh(handle, -1), 0);
    }

    #[test]
    fn runtime_data_array_has_keeps_runtime_facade_fail_safe_contract() {
        let handle = new_array_handle();
        let string_key = new_string_handle("not-an-index");

        assert_eq!(nyash_runtime_data_push_hh(handle, new_int_handle(11)), 1);

        assert_eq!(nyash_runtime_data_has_hh(handle, 0), 1);
        assert_eq!(nyash_runtime_data_has_hh(handle, 1), 0);
        assert_eq!(nyash_runtime_data_has_hh(handle, -1), 0);
        assert_eq!(nyash_runtime_data_has_hh(handle, string_key), 0);
    }

    #[test]
    fn runtime_data_array_non_i64_keys_keep_fail_safe_fallback_contract() {
        let handle = new_array_handle();
        let string_key = new_string_handle("not-an-index");
        let original_h = new_int_handle(11);
        let updated_h = new_int_handle(22);

        assert_eq!(nyash_runtime_data_push_hh(handle, original_h), 1);

        // K2-core keeps non-i64 array keys on the runtime-data facade.
        // The current fallback contract remains fail-safe and must not mutate
        // the array when the key cannot be treated as an index.
        assert_eq!(nyash_runtime_data_get_hh(handle, string_key), 0);
        assert_eq!(nyash_runtime_data_set_hhh(handle, string_key, updated_h), 0);
        assert_eq!(nyash_runtime_data_get_hh(handle, 0), 11);
    }

    #[test]
    fn runtime_data_scalar_handle_keeps_facade_only_contract() {
        let scalar_h = new_int_handle(7);

        assert_eq!(nyash_runtime_data_get_hh(scalar_h, 0), 0);
        assert_eq!(
            nyash_runtime_data_set_hhh(scalar_h, 0, new_int_handle(11)),
            0
        );
        assert_eq!(nyash_runtime_data_has_hh(scalar_h, 0), 0);
        assert_eq!(nyash_runtime_data_push_hh(scalar_h, new_int_handle(11)), 0);
    }

    #[test]
    fn runtime_data_map_get_keeps_mixed_runtime_i64_contract() {
        let handle = new_map_handle();
        let key = -70001;
        let value = new_int_handle(42);

        assert_eq!(nyash_runtime_data_set_hhh(handle, key, value), 1);
        assert_eq!(nyash_runtime_data_has_hh(handle, key), 1);
        assert_eq!(nyash_runtime_data_get_hh(handle, key), 42);
    }

    #[test]
    fn runtime_data_map_any_key_keeps_shared_facade_contract() {
        let handle = new_map_handle();
        let key = new_string_handle("map-any-key");
        let value = new_int_handle(77);

        assert_eq!(nyash_runtime_data_set_hhh(handle, key, value), 1);
        assert_eq!(nyash_runtime_data_has_hh(handle, key), 1);
        assert_eq!(nyash_runtime_data_get_hh(handle, key), 77);
        assert_eq!(
            nyash_runtime_data_get_hh(handle, new_string_handle("missing")),
            0
        );
    }

    #[test]
    fn runtime_data_map_string_value_reuses_live_source_handle() {
        let handle = new_map_handle();
        let key = new_string_handle("map-string-live-key");
        let value = new_string_handle("map-string-live");

        assert_eq!(nyash_runtime_data_set_hhh(handle, key, value), 1);
        assert_eq!(nyash_runtime_data_has_hh(handle, key), 1);
        assert_eq!(nyash_runtime_data_get_hh(handle, key), value);
    }

    #[test]
    fn runtime_data_map_string_value_reuses_cached_handle_after_source_drop() {
        let handle = new_map_handle();
        let key = new_string_handle("map-string-cached-key");
        let value = new_string_handle("map-string-cached");

        assert_eq!(nyash_runtime_data_set_hhh(handle, key, value), 1);
        handles::drop_handle(value as u64);
        assert!(handles::get(value as u64).is_none());

        let first = nyash_runtime_data_get_hh(handle, key);
        let second = nyash_runtime_data_get_hh(handle, key);

        assert!(first > 0);
        assert_eq!(first, second);

        let out_obj = handles::get(first as u64).expect("cached map string handle");
        let out_sb = out_obj
            .as_any()
            .downcast_ref::<nyash_rust::box_trait::StringBox>()
            .expect("runtime value should remain StringBox");
        assert_eq!(out_sb.value, "map-string-cached");
    }

    #[cfg(feature = "perf-observe")]
    #[test]
    fn runtime_data_map_read_records_cached_handle_hit_for_map_lane() {
        crate::test_support::with_env_var("NYASH_PERF_COUNTERS", "1", || {
            let handle = new_map_handle();
            let key = new_string_handle("map-observe-cached-key");
            let value = new_string_handle("map-observe-cached");

            assert_eq!(nyash_runtime_data_set_hhh(handle, key, value), 1);
            handles::drop_handle(value as u64);
            assert!(handles::get(value as u64).is_none());

            let warmup = nyash_runtime_data_get_hh(handle, key);
            let before = crate::observe::borrowed_alias_encode_snapshot_for_tests();
            let cached = nyash_runtime_data_get_hh(handle, key);
            let after = crate::observe::borrowed_alias_encode_snapshot_for_tests();

            assert!(warmup > 0);
            assert_eq!(warmup, cached);
            assert_eq!(after.cached_handle_hit - before.cached_handle_hit, 1);
            assert_eq!(
                after.cached_handle_hit_map_runtime_data_get_any
                    - before.cached_handle_hit_map_runtime_data_get_any,
                1
            );
            assert_eq!(
                after.cached_handle_hit_array_get_index
                    - before.cached_handle_hit_array_get_index,
                0
            );
        });
    }

    #[cfg(feature = "perf-observe")]
    #[test]
    fn runtime_data_map_read_records_live_source_hit_for_map_lane() {
        crate::test_support::with_env_var("NYASH_PERF_COUNTERS", "1", || {
            let handle = new_map_handle();
            let key = new_string_handle("map-observe-live-key");
            let value = new_string_handle("map-observe-live");

            assert_eq!(nyash_runtime_data_set_hhh(handle, key, value), 1);

            let before = crate::observe::borrowed_alias_encode_snapshot_for_tests();
            let live = nyash_runtime_data_get_hh(handle, key);
            let after = crate::observe::borrowed_alias_encode_snapshot_for_tests();

            assert_eq!(live, value);
            assert_eq!(after.live_source_hit - before.live_source_hit, 1);
            assert_eq!(
                after.live_source_hit_map_runtime_data_get_any
                    - before.live_source_hit_map_runtime_data_get_any,
                1
            );
            assert_eq!(
                after.live_source_hit_array_get_index
                    - before.live_source_hit_array_get_index,
                0
            );
            assert_eq!(after.cached_handle_hit - before.cached_handle_hit, 0);
        });
    }

    #[cfg(feature = "perf-observe")]
    #[test]
    fn runtime_data_map_read_records_cold_fallback_for_map_lane() {
        crate::test_support::with_env_var("NYASH_PERF_COUNTERS", "1", || {
            let handle = new_map_handle();
            let key = new_string_handle("map-observe-fallback-key");
            let value = new_string_handle("map-observe-fallback");

            assert_eq!(nyash_runtime_data_set_hhh(handle, key, value), 1);
            handles::drop_handle(value as u64);
            assert!(handles::get(value as u64).is_none());

            let before = crate::observe::borrowed_alias_encode_snapshot_for_tests();
            let cold = nyash_runtime_data_get_hh(handle, key);
            let after = crate::observe::borrowed_alias_encode_snapshot_for_tests();

            assert!(cold > 0);
            assert_eq!(
                after.fallback_to_handle_arc - before.fallback_to_handle_arc,
                1
            );
            assert_eq!(
                after.fallback_to_handle_arc_map_runtime_data_get_any
                    - before.fallback_to_handle_arc_map_runtime_data_get_any,
                1
            );
            assert_eq!(
                after.fallback_to_handle_arc_array_get_index
                    - before.fallback_to_handle_arc_array_get_index,
                0
            );
            assert_eq!(after.live_source_hit - before.live_source_hit, 0);
            assert_eq!(after.cached_handle_hit - before.cached_handle_hit, 0);
        });
    }

    #[test]
    fn runtime_data_map_immediate_zero_key_keeps_shared_facade_contract() {
        let handle = new_map_handle();
        let value = new_int_handle(88);

        assert_eq!(nyash_runtime_data_set_hhh(handle, 0, value), 1);
        assert_eq!(nyash_runtime_data_has_hh(handle, 0), 1);
        assert_eq!(nyash_runtime_data_get_hh(handle, 0), 88);
    }

    #[test]
    fn runtime_data_array_result_can_feed_map_zero_key_contract() {
        let array_handle = new_array_handle();
        assert_eq!(nyash_runtime_data_push_hh(array_handle, 0), 1);
        let array_value = nyash_runtime_data_get_hh(array_handle, 0);

        let map_handle = new_map_handle();
        assert_eq!(nyash_runtime_data_set_hhh(map_handle, 0, array_value), 1);
        assert_eq!(nyash_runtime_data_has_hh(map_handle, 0), 1);
        assert_eq!(nyash_runtime_data_get_hh(map_handle, 0), array_value);
    }
}
