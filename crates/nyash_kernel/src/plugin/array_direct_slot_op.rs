//! Fused ArrayBox direct-slot helper for selected exact-EXE hot paths.
//!
//! This module keeps the exported helper ABI narrow. It delegates storage to the
//! existing Array slot backend seam, so default `safe_rwlock` semantics and the
//! diagnostic `single_thread_exact` backend stay in one place.

use super::array_slot_backend;

#[export_name = "nyash.array.slot_load_store_i64_hihi"]
pub extern "C" fn nyash_array_slot_load_store_i64_hihi(
    src_handle: i64,
    src_idx: i64,
    dst_handle: i64,
    value_i64: i64,
) -> i64 {
    let loaded_idx = array_slot_backend::load_encoded_i64(src_handle, src_idx);
    let _stored = array_slot_backend::store_i64(dst_handle, loaded_idx, value_i64);
    loaded_idx
}
