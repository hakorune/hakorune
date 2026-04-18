pub(super) fn map_entry_count_raw(handle: i64) -> i64 {
    if super::map_debug::map_debug_enabled() {
        eprintln!("[MAP] entry_count_i64(handle={})", handle);
    }
    let size = super::map_runtime_facade::map_runtime_entry_count(handle);
    if super::map_debug::map_debug_enabled() {
        eprintln!("[MAP] entry_count_i64 => {}", size);
    }
    size
}

pub(super) fn map_capacity_raw(handle: i64) -> i64 {
    super::map_runtime_facade::map_runtime_cap(handle)
}
