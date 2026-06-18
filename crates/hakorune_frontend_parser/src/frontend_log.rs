//! Frontend logging facade for moved frontend modules.

use crate::frontend_host::{frontend_host, FrontendLogLevel};

#[inline]
pub fn debug(message: &str) {
    frontend_host().log(FrontendLogLevel::Debug, message);
}

#[inline]
pub fn warn(message: &str) {
    frontend_host().log(FrontendLogLevel::Warn, message);
}

#[inline]
pub fn error(message: &str) {
    frontend_host().log(FrontendLogLevel::Error, message);
}
