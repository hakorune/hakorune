//! Phase 29ai P3: Typed Freeze (Fail-Fast) — SSOT implementation
//!
//! Tag taxonomy SSOT:
//! - docs/development/current/main/design/planfrag-freeze-taxonomy.md

use std::fmt;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Freeze {
    pub tag: &'static str,
    pub message: String,
    pub hint: Option<String>,
}

impl Freeze {
    pub(in crate::mir::builder) fn contract(message: impl Into<String>) -> Self {
        Self {
            tag: "contract",
            message: message.into(),
            hint: None,
        }
    }

    pub(in crate::mir::builder) fn ambiguous(message: impl Into<String>) -> Self {
        Self {
            tag: "ambiguous",
            message: message.into(),
            hint: None,
        }
    }

    pub(in crate::mir::builder) fn unsupported(message: impl Into<String>) -> Self {
        Self {
            tag: "unsupported",
            message: message.into(),
            hint: None,
        }
    }

    pub(in crate::mir::builder) fn unstructured(message: impl Into<String>) -> Self {
        Self {
            tag: "unstructured",
            message: message.into(),
            hint: None,
        }
    }

    pub(in crate::mir::builder) fn bug(message: impl Into<String>) -> Self {
        Self {
            tag: "bug",
            message: message.into(),
            hint: None,
        }
    }

    pub(in crate::mir::builder) fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

impl fmt::Display for Freeze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[plan/freeze:{}] {}", self.tag, self.message)?;
        if let Some(hint) = &self.hint {
            write!(f, "\n  hint: {}", hint)?;
        }
        Ok(())
    }
}

impl std::error::Error for Freeze {}

#[cfg(test)]
mod tests {
    use super::Freeze;

    #[test]
    fn unstructured_tag_formats_as_expected() {
        let text = Freeze::unstructured("x").to_string();
        assert!(text.contains("[plan/freeze:unstructured] x"));
    }
}
