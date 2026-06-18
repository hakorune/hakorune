//! Tokenizer-local logging facade.
//!
//! This keeps runtime logging access out of tokenizer implementation modules
//! while preserving the current global logger behavior.

#[inline]
pub(crate) fn debug(message: &str) {
    crate::runtime::get_global_ring0().log.debug(message);
}

#[inline]
pub(crate) fn warn(message: &str) {
    crate::runtime::get_global_ring0().log.warn(message);
}
