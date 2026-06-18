//! Parser logging facade.
//!
//! Parser code should not call the runtime logger directly. Keeping logging
//! behind this facade makes parser extraction possible without carrying the
//! runtime ring0 API into a future frontend crate.

pub(crate) fn debug(message: &str) {
    crate::runtime::get_global_ring0().log.debug(message);
}

pub(crate) fn warn(message: &str) {
    crate::runtime::get_global_ring0().log.warn(message);
}

pub(crate) fn error(message: &str) {
    crate::runtime::get_global_ring0().log.error(message);
}
