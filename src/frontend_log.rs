//! Frontend logging facade shared by parser and tokenizer.
//!
//! This keeps parser/tokenizer modules from owning the runtime logger route.

use crate::frontend_host::{FrontendHostBoundary, FrontendLogLevel};

#[inline]
pub(crate) fn debug(message: &str) {
    crate::frontend_host::runtime_host().log(FrontendLogLevel::Debug, message);
}

#[inline]
pub(crate) fn warn(message: &str) {
    crate::frontend_host::runtime_host().log(FrontendLogLevel::Warn, message);
}

#[inline]
pub(crate) fn error(message: &str) {
    crate::frontend_host::runtime_host().log(FrontendLogLevel::Error, message);
}
