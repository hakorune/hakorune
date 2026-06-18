//! Frontend host boundary vocabulary without runtime dependency.

use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontendLogLevel {
    Debug,
    Warn,
    Error,
}

pub trait FrontendHostBoundary: Sync {
    fn log(&self, level: FrontendLogLevel, message: &str);

    fn warn_alias_once(&self, alias: &'static str, primary: &'static str);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoopFrontendHost;

impl FrontendHostBoundary for NoopFrontendHost {
    fn log(&self, _level: FrontendLogLevel, _message: &str) {}

    fn warn_alias_once(&self, _alias: &'static str, _primary: &'static str) {}
}

static NOOP_FRONTEND_HOST: NoopFrontendHost = NoopFrontendHost;
static FRONTEND_HOST: OnceLock<&'static dyn FrontendHostBoundary> = OnceLock::new();

pub fn install_frontend_host(host: &'static dyn FrontendHostBoundary) {
    let _ = FRONTEND_HOST.set(host);
}

pub fn frontend_host() -> &'static dyn FrontendHostBoundary {
    *FRONTEND_HOST.get_or_init(|| &NOOP_FRONTEND_HOST)
}
