pub(crate) fn enabled() -> bool {
    super::super::config::enabled()
}

pub(crate) fn bypass_gc_alloc_enabled() -> bool {
    super::super::config::bypass_gc_alloc_enabled()
}

pub(crate) fn mark_latest_fresh_handle(handle: i64) {
    super::super::backend::mark_latest_fresh_handle(handle);
}

pub(crate) fn len_route_matches_latest_fresh_handle(handle: i64) -> bool {
    super::super::backend::matches_latest_fresh_handle(handle)
}

pub(crate) fn flush() {
    if super::super::config::enabled() {
        super::super::sink::emit_summary_to_stderr();
    }
}
