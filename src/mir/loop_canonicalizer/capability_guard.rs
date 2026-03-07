//! Capability Guard - Fail-Fast Reasons and Routing Decisions
//!
//! This module defines the vocabulary for route selection and failure reasons.
//! It provides standardized capability tags and routing decision structures.

use crate::mir::loop_pattern_detection::LoopRouteKind;

// ============================================================================
// Routing Decision
// ============================================================================

/// Routing decision - The result of route selection
///
/// This contains both the chosen route (if any) and detailed
/// diagnostic information about why other routes were rejected.
#[derive(Debug, Clone)]
pub struct RoutingDecision {
    /// Selected route (None = Fail-Fast)
    pub chosen: Option<LoopRouteKind>,

    /// Missing capabilities that prevented other routes (type-safe!)
    pub missing_caps: Vec<CapabilityTag>,

    /// Selection reasoning (for debugging)
    pub notes: Vec<String>,

    /// Error tags for contract_checks integration
    pub error_tags: Vec<String>,
}

impl RoutingDecision {
    /// Create a successful routing decision
    pub fn success(route_kind: LoopRouteKind) -> Self {
        Self {
            chosen: Some(route_kind),
            missing_caps: Vec::new(),
            notes: Vec::new(),
            error_tags: Vec::new(),
        }
    }

    /// Create a failed routing decision (Fail-Fast)
    pub fn fail_fast(missing_caps: Vec<CapabilityTag>, reason: String) -> Self {
        let error_tags = missing_caps
            .iter()
            .map(|cap| cap.to_tag().to_string())
            .collect();

        Self {
            chosen: None,
            missing_caps,
            notes: vec![reason.clone()],
            error_tags,
        }
    }

    /// Add a diagnostic note
    pub fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }

    /// Check if routing succeeded
    pub fn is_success(&self) -> bool {
        self.chosen.is_some()
    }

    /// Check if routing failed
    pub fn is_fail_fast(&self) -> bool {
        self.chosen.is_none()
    }
}

// ============================================================================
// Capability Tags (Type-Safe Enum)
// ============================================================================

/// Capability tag - Type-safe vocabulary for route requirements
///
/// Each tag represents a specific capability that a loop route requires.
/// Using an enum (instead of string constants) provides:
/// - Compile-time error detection for typos
/// - IDE auto-completion
/// - Exhaustiveness checking in match expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapabilityTag {
    /// Requires: Carrier update is constant step (i = i + const)
    ConstStep,
    /// Requires: Single break point only
    SingleBreak,
    /// Requires: Single continue point only
    SingleContinue,
    /// Requires: Pure header condition (no side effects)
    PureHeader,
    /// Requires: Outer local condition variable
    OuterLocalCond,
    /// Requires: Complete exit bindings
    ExitBindings,
    /// Requires: Carrier promotion support
    CarrierPromotion,
    /// Requires: Consistent break value types
    BreakValueType,
}

impl CapabilityTag {
    /// Error message tag for contract_checks integration
    pub fn to_tag(&self) -> &'static str {
        match self {
            Self::ConstStep => "CAP_MISSING_CONST_STEP",
            Self::SingleBreak => "CAP_MISSING_SINGLE_BREAK",
            Self::SingleContinue => "CAP_MISSING_SINGLE_CONTINUE",
            Self::PureHeader => "CAP_MISSING_PURE_HEADER",
            Self::OuterLocalCond => "CAP_MISSING_OUTER_LOCAL_COND",
            Self::ExitBindings => "CAP_MISSING_EXIT_BINDINGS",
            Self::CarrierPromotion => "CAP_MISSING_CARRIER_PROMOTION",
            Self::BreakValueType => "CAP_MISSING_BREAK_VALUE_TYPE",
        }
    }

    /// Human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            Self::ConstStep => "Carrier update is constant step (i = i + const)",
            Self::SingleBreak => "break statement appears in single location only",
            Self::SingleContinue => "continue statement appears in single location only",
            Self::PureHeader => "Loop header condition has no side effects",
            Self::OuterLocalCond => "Condition variable is defined in outer scope",
            Self::ExitBindings => "Exit bindings are complete (no missing values)",
            Self::CarrierPromotion => "LoopBodyLocal can be promoted to carrier",
            Self::BreakValueType => "break value types are consistent across all branches",
        }
    }
}
