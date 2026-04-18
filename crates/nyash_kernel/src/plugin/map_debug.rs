#[inline]
pub(super) fn map_debug_enabled() -> bool {
    std::env::var("NYASH_LLVM_MAP_DEBUG").ok().as_deref() == Some("1")
}
