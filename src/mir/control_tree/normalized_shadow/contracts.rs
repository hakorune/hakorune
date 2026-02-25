//! Phase 121: Capability checking and if-only validation
//!
//! ## Responsibility
//!
//! - Check if StepTree is "if-only" (no loops, breaks, continues)
//! - Provide SSOT for capability rejection reasons
//! - Explicit enumeration of unsupported capabilities

use crate::mir::control_tree::step_tree::StepTree;

/// Unsupported capability classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedCapability {
    /// Contains loop constructs
    Loop,
    /// Contains break statements
    Break,
    /// Contains continue statements
    Continue,
    /// Other unsupported feature
    Other,
}

impl UnsupportedCapability {
    /// Get human-readable reason string
    pub fn reason(&self) -> &'static str {
        match self {
            UnsupportedCapability::Loop => "contains loop (if-only scope)",
            UnsupportedCapability::Break => "contains break (if-only scope)",
            UnsupportedCapability::Continue => "contains continue (if-only scope)",
            UnsupportedCapability::Other => "unsupported feature for if-only",
        }
    }
}

/// Result of capability check
#[derive(Debug, Clone)]
pub enum CapabilityCheckResult {
    /// Supported (if-only)
    Supported,
    /// Unsupported with specific reason
    Unsupported(UnsupportedCapability),
}

/// Check if StepTree is if-only (Phase 121 scope)
///
/// ## Contract
///
/// - Input: `&StepTree` with already-computed `features` and `contract`
/// - No AST re-analysis (uses contract fields only)
/// - Returns `Supported` only if no loops/breaks/continues
pub fn check_if_only(step_tree: &StepTree) -> CapabilityCheckResult {
    // Check features (already computed during StepTree construction)
    if step_tree.features.has_loop {
        return CapabilityCheckResult::Unsupported(UnsupportedCapability::Loop);
    }
    if step_tree.features.has_break {
        return CapabilityCheckResult::Unsupported(UnsupportedCapability::Break);
    }
    if step_tree.features.has_continue {
        return CapabilityCheckResult::Unsupported(UnsupportedCapability::Continue);
    }

    // If-only scope is supported
    CapabilityCheckResult::Supported
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsupported_capability_reasons() {
        assert_eq!(UnsupportedCapability::Loop.reason(), "contains loop (if-only scope)");
        assert_eq!(UnsupportedCapability::Break.reason(), "contains break (if-only scope)");
        assert_eq!(
            UnsupportedCapability::Continue.reason(),
            "contains continue (if-only scope)"
        );
    }
}
