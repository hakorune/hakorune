//! GC mode selection (user-facing)
use crate::config::env;

pub const GC_MODE_ALLOWED_VALUES: [&str; 3] = ["auto", "rc+cycle", "off"];
pub const GC_MODE_ALLOWED_VALUES_DISPLAY: &str = "auto|rc+cycle|off";
pub const GC_MODE_CLI_VALUE_NAME: &str = "{auto,rc+cycle,off}";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcMode {
    RcCycle,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GcModeParseError {
    raw: String,
}

impl GcModeParseError {
    fn new(raw: impl Into<String>) -> Self {
        Self { raw: raw.into() }
    }

    pub fn freeze_message(&self) -> String {
        format!(
            "[freeze:contract][gc/mode] unsupported NYASH_GC_MODE='{}'. allowed: {}",
            self.raw, GC_MODE_ALLOWED_VALUES_DISPLAY
        )
    }
}

impl GcMode {
    pub fn parse(raw: &str) -> Result<Self, GcModeParseError> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "" | "auto" | "rc+cycle" => Ok(GcMode::RcCycle),
            "off" => Ok(GcMode::Off),
            _ => Err(GcModeParseError::new(raw)),
        }
    }

    pub fn from_env_result() -> Result<Self, GcModeParseError> {
        Self::parse(&env::gc_mode())
    }

    pub fn validate_env_or_exit() {
        if let Err(err) = Self::from_env_result() {
            eprintln!("{}", err.freeze_message());
            std::process::exit(1);
        }
    }

    pub fn from_env() -> Self {
        match Self::from_env_result() {
            Ok(mode) => mode,
            Err(err) => {
                eprintln!("{}", err.freeze_message());
                std::process::exit(1);
            }
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            GcMode::RcCycle => "rc+cycle",
            GcMode::Off => "off",
        }
    }
}
