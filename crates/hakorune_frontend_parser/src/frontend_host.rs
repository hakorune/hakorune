//! Frontend host boundary vocabulary without runtime dependency.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontendLogLevel {
    Debug,
    Warn,
    Error,
}

pub trait FrontendHostBoundary {
    fn log(&self, level: FrontendLogLevel, message: &str);

    fn warn_alias_once(&self, alias: &'static str, primary: &'static str);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoopFrontendHost;

impl FrontendHostBoundary for NoopFrontendHost {
    fn log(&self, _level: FrontendLogLevel, _message: &str) {}

    fn warn_alias_once(&self, _alias: &'static str, _primary: &'static str) {}
}
