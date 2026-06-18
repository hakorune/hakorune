//! Parser logging facade.
//!
//! Parser code should not call the runtime logger directly. Keeping logging
//! behind this facade makes parser extraction possible without carrying the
//! runtime ring0 API into a future frontend crate.

pub(crate) fn debug(message: &str) {
    crate::frontend_log::debug(message);
}

pub(crate) fn warn(message: &str) {
    crate::frontend_log::warn(message);
}

pub(crate) fn error(message: &str) {
    crate::frontend_log::error(message);
}
