//! Passive frontend host boundary vocabulary.
//!
//! Parser/tokenizer extraction will eventually need a host for logging and
//! alias-warning sinks. This module defines that boundary without wiring it
//! into the current parser/tokenizer execution path.

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RuntimeFrontendHost;

impl FrontendHostBoundary for RuntimeFrontendHost {
    fn log(&self, level: FrontendLogLevel, message: &str) {
        let logger = &crate::runtime::get_global_ring0().log;
        match level {
            FrontendLogLevel::Debug => logger.debug(message),
            FrontendLogLevel::Warn => logger.warn(message),
            FrontendLogLevel::Error => logger.error(message),
        }
    }

    fn warn_alias_once(&self, alias: &'static str, primary: &'static str) {
        let ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
        ring0.log.warn(&format!(
            "[deprecate/env] '{}' is deprecated; use '{}'",
            alias, primary
        ));
    }
}

pub(crate) fn runtime_host() -> RuntimeFrontendHost {
    install_frontend_parser_host();
    RuntimeFrontendHost
}

impl hakorune_frontend_parser::frontend_host::FrontendHostBoundary for RuntimeFrontendHost {
    fn log(&self, level: hakorune_frontend_parser::frontend_host::FrontendLogLevel, message: &str) {
        let level = match level {
            hakorune_frontend_parser::frontend_host::FrontendLogLevel::Debug => {
                FrontendLogLevel::Debug
            }
            hakorune_frontend_parser::frontend_host::FrontendLogLevel::Warn => {
                FrontendLogLevel::Warn
            }
            hakorune_frontend_parser::frontend_host::FrontendLogLevel::Error => {
                FrontendLogLevel::Error
            }
        };
        FrontendHostBoundary::log(self, level, message);
    }

    fn warn_alias_once(&self, alias: &'static str, primary: &'static str) {
        FrontendHostBoundary::warn_alias_once(self, alias, primary);
    }
}

static RUNTIME_FRONTEND_HOST: RuntimeFrontendHost = RuntimeFrontendHost;

pub(crate) fn install_frontend_parser_host() {
    hakorune_frontend_parser::frontend_host::install_frontend_host(&RUNTIME_FRONTEND_HOST);
}
