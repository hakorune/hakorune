//! Frontend logging facade shared by parser and tokenizer.
//!
//! This keeps parser/tokenizer modules from owning the runtime logger route.

#[inline]
pub(crate) fn debug(message: &str) {
    crate::runtime::get_global_ring0().log.debug(message);
}

#[inline]
pub(crate) fn warn(message: &str) {
    crate::runtime::get_global_ring0().log.warn(message);
}

#[inline]
pub(crate) fn error(message: &str) {
    crate::runtime::get_global_ring0().log.error(message);
}
