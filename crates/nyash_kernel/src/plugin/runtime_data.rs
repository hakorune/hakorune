// RuntimeDataBox-compatible dynamic dispatch helpers.
//
// These exports bridge RuntimeDataBox method calls in AOT/LLVM to concrete
// host boxes (ArrayBox/MapBox) without relying on static box-name guesses.

use super::handle_helpers::with_array_or_map;
use super::runtime_data_array_route::{
    runtime_data_array_get_hh, runtime_data_array_has_hh, runtime_data_array_push_hh,
    runtime_data_array_set_hhh,
};
use super::runtime_data_map_route::{
    runtime_data_map_get_hh, runtime_data_map_has_hh, runtime_data_map_set_hhh,
};

// nyash.runtime_data.get_hh(recv_h, key_any) -> value_handle (or 0)
#[export_name = "nyash.runtime_data.get_hh"]
pub extern "C" fn nyash_runtime_data_get_hh(recv_h: i64, key_any: i64) -> i64 {
    with_array_or_map(
        recv_h,
        |arr| runtime_data_array_get_hh(arr, key_any),
        |map| runtime_data_map_get_hh(map, key_any),
    )
    .unwrap_or(0)
}

// nyash.runtime_data.set_hhh(recv_h, key_any, val_any) -> 0/1
#[export_name = "nyash.runtime_data.set_hhh"]
pub extern "C" fn nyash_runtime_data_set_hhh(recv_h: i64, key_any: i64, val_any: i64) -> i64 {
    with_array_or_map(
        recv_h,
        |arr| runtime_data_array_set_hhh(arr, key_any, val_any),
        |map| runtime_data_map_set_hhh(map, key_any, val_any),
    )
    .unwrap_or(0)
}

// nyash.runtime_data.has_hh(recv_h, key_any) -> 0/1
#[export_name = "nyash.runtime_data.has_hh"]
pub extern "C" fn nyash_runtime_data_has_hh(recv_h: i64, key_any: i64) -> i64 {
    with_array_or_map(
        recv_h,
        |arr| runtime_data_array_has_hh(arr, key_any),
        |map| runtime_data_map_has_hh(map, key_any),
    )
    .unwrap_or(0)
}

// nyash.runtime_data.push_hh(recv_h, val_any) -> new_len (array) / 0
#[export_name = "nyash.runtime_data.push_hh"]
pub extern "C" fn nyash_runtime_data_push_hh(recv_h: i64, val_any: i64) -> i64 {
    with_array_or_map(
        recv_h,
        |arr| runtime_data_array_push_hh(arr, val_any),
        |_map| 0,
    )
    .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_data_invalid_handle_returns_zero() {
        assert_eq!(nyash_runtime_data_get_hh(0, 1), 0);
        assert_eq!(nyash_runtime_data_set_hhh(0, 1, 2), 0);
        assert_eq!(nyash_runtime_data_has_hh(0, 1), 0);
        assert_eq!(nyash_runtime_data_push_hh(0, 1), 0);
    }
}
