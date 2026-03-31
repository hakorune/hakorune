// RuntimeDataBox-compatible dynamic dispatch helpers.
//
// These exports bridge RuntimeDataBox method calls in AOT/LLVM to concrete
// host boxes (ArrayBox/MapBox) without relying on static box-name guesses.
// Manifest reading: all `nyash.runtime_data.*` rows are runtime-facade only.

use super::array_runtime_facade::{
    array_runtime_get_any_key, array_runtime_has_any_key, array_runtime_push_any,
    array_runtime_set_any_key,
};
use super::handle_cache::with_array_or_map;
use super::runtime_data_map_dispatch::{
    runtime_data_map_get_hh, runtime_data_map_has_hh, runtime_data_map_set_hhh,
};

// nyash.runtime_data.get_hh(recv_h, key_any) -> mixed runtime i64/handle value (or 0)
#[export_name = "nyash.runtime_data.get_hh"]
pub extern "C" fn nyash_runtime_data_get_hh(recv_h: i64, key_any: i64) -> i64 {
    with_array_or_map(
        recv_h,
        |_| array_runtime_get_any_key(recv_h, key_any),
        |_| runtime_data_map_get_hh(recv_h, key_any),
    )
    .unwrap_or(0)
}

// nyash.runtime_data.set_hhh(recv_h, key_any, val_any) -> 0/1
#[export_name = "nyash.runtime_data.set_hhh"]
pub extern "C" fn nyash_runtime_data_set_hhh(recv_h: i64, key_any: i64, val_any: i64) -> i64 {
    with_array_or_map(
        recv_h,
        |_| array_runtime_set_any_key(recv_h, key_any, val_any),
        |_| runtime_data_map_set_hhh(recv_h, key_any, val_any),
    )
    .unwrap_or(0)
}

// nyash.runtime_data.has_hh(recv_h, key_any) -> 0/1
#[export_name = "nyash.runtime_data.has_hh"]
pub extern "C" fn nyash_runtime_data_has_hh(recv_h: i64, key_any: i64) -> i64 {
    with_array_or_map(
        recv_h,
        |_| array_runtime_has_any_key(recv_h, key_any),
        |_| runtime_data_map_has_hh(recv_h, key_any),
    )
    .unwrap_or(0)
}

// nyash.runtime_data.push_hh(recv_h, val_any) -> new_len (array) / 0
#[export_name = "nyash.runtime_data.push_hh"]
pub extern "C" fn nyash_runtime_data_push_hh(recv_h: i64, val_any: i64) -> i64 {
    with_array_or_map(
        recv_h,
        |_| array_runtime_push_any(recv_h, val_any),
        |_| 0,
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
    fn runtime_data_map_get_keeps_mixed_runtime_i64_contract() {
        let handle = new_map_handle();
        let key = -70001;
        let value = new_int_handle(42);

        assert_eq!(nyash_runtime_data_set_hhh(handle, key, value), 1);
        assert_eq!(nyash_runtime_data_has_hh(handle, key), 1);
        assert_eq!(nyash_runtime_data_get_hh(handle, key), 42);
    }
}
