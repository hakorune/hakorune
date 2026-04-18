use super::handle_cache::with_map_box;

pub(super) fn map_entry_count_raw(handle: i64) -> i64 {
    if super::map_debug::map_debug_enabled() {
        eprintln!("[MAP] entry_count_i64(handle={})", handle);
    }
    let size = with_map_box(handle, |map| map.entry_count_i64()).unwrap_or(0);
    if super::map_debug::map_debug_enabled() {
        eprintln!("[MAP] entry_count_i64 => {}", size);
    }
    size
}

pub(super) fn map_capacity_raw(handle: i64) -> i64 {
    with_map_box(handle, |map| map.capacity_i64()).unwrap_or(0)
}
