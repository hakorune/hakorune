//! Diagnostics contract helpers (SSOT-friendly)
//!
//! Keep freeze-contract messages one-line and key/value formatted.

use std::fmt::Display;

pub(crate) struct FreezeContract {
    message: String,
}

impl FreezeContract {
    pub(crate) fn new(tag: &str) -> Self {
        Self {
            message: format!("[freeze:contract][{}]", tag),
        }
    }

    pub(crate) fn field<V: Display>(mut self, key: &str, value: V) -> Self {
        self.message.push(' ');
        self.message.push_str(key);
        self.message.push('=');
        self.message.push_str(&value.to_string());
        self
    }

    pub(crate) fn build(self) -> String {
        self.message
    }
}

pub(crate) fn caller_string(caller: &std::panic::Location<'_>) -> String {
    format!("{}:{}:{}", caller.file(), caller.line(), caller.column())
}

pub(crate) fn mir_dump_value(path: Option<String>) -> String {
    path.unwrap_or_else(|| "disabled".to_string())
}
