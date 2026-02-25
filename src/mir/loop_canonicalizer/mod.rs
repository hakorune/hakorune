//! Loop Canonicalizer - AST Level Loop Preprocessing
//!
//! ## Purpose
//!
//! Decomposes AST-level loops into a normalized "skeleton" representation
//! to prevent combinatorial explosion in pattern detection and lowering.
//!
//! ## Design Principle
//!
//! - **Input**: AST (LoopExpr)
//! - **Output**: LoopSkeleton only (no JoinIR generation)
//! - **Boundary**: No JoinIR-specific information (BlockId, ValueId, etc.)
//!
//! ## Architecture
//!
//! ```
//! AST → LoopSkeleton → Capability Guard → RoutingDecision → Pattern Lowerer
//! ```
//!
//! ## Module Structure (Phase 138 Refactoring)
//!
//! - `skeleton_types` - Core data structures (LoopSkeleton, SkeletonStep, etc.)
//! - `capability_guard` - Routing decisions and capability tags
//! - `pattern_recognizer` - Pattern detection logic (skip_whitespace, etc.)
//! - `canonicalizer` - Main canonicalization entry point
//!
//! ## References
//!
//! - Design SSOT: `docs/development/current/main/design/loop-canonicalizer.md`
//! - JoinIR Architecture: `docs/development/current/main/joinir-architecture-overview.md`
//! - Pattern Space: `docs/development/current/main/loop_pattern_space.md`

// ============================================================================
// Module Declarations
// ============================================================================

mod canonicalizer;
mod capability_guard;
mod pattern_recognizer;
mod skeleton_types;

#[cfg(test)]
mod canonicalizer_tests;

// ============================================================================
// Public Re-exports
// ============================================================================

// Skeleton Types
pub use skeleton_types::{
    CapturedSlot, CarrierRole, CarrierSlot, ExitContract, LoopSkeleton, SkeletonStep, UpdateKind,
};

// Capability Guard
pub use capability_guard::{CapabilityTag, RoutingDecision};

// Canonicalization Entry Point
pub use canonicalizer::canonicalize_loop_expr;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    #[test]
    fn test_skeleton_creation() {
        let skeleton = LoopSkeleton::new(Span::unknown());
        assert_eq!(skeleton.steps.len(), 0);
        assert_eq!(skeleton.carriers.len(), 0);
        assert!(!skeleton.exits.has_any_exit());
    }

    #[test]
    fn test_exit_contract() {
        let mut contract = ExitContract::none();
        assert!(!contract.has_any_exit());

        contract.has_break = true;
        assert!(contract.has_any_exit());
    }

    #[test]
    fn test_routing_decision() {
        use crate::mir::loop_pattern_detection::LoopPatternKind;

        let success = RoutingDecision::success(LoopPatternKind::Pattern1SimpleWhile);
        assert!(success.is_success());
        assert!(!success.is_fail_fast());

        let fail =
            RoutingDecision::fail_fast(vec![CapabilityTag::ConstStep], "Test failure".to_string());
        assert!(!fail.is_success());
        assert!(fail.is_fail_fast());
        assert_eq!(fail.missing_caps.len(), 1);
        assert_eq!(fail.missing_caps[0], CapabilityTag::ConstStep);
    }

    #[test]
    fn test_carrier_role_display() {
        assert_eq!(CarrierRole::Counter.to_string(), "Counter");
        assert_eq!(CarrierRole::Accumulator.to_string(), "Accumulator");
        assert_eq!(CarrierRole::ConditionVar.to_string(), "ConditionVar");
        assert_eq!(CarrierRole::Derived.to_string(), "Derived");
    }

    #[test]
    fn test_skeleton_count_helpers() {
        use crate::ast::{ASTNode, LiteralValue};

        let mut skeleton = LoopSkeleton::new(Span::unknown());

        skeleton.steps.push(SkeletonStep::BreakCheck {
            cond: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            has_value: false,
        });

        skeleton.steps.push(SkeletonStep::ContinueCheck {
            cond: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
        });

        assert_eq!(skeleton.count_break_checks(), 1);
        assert_eq!(skeleton.count_continue_checks(), 1);
    }

    #[test]
    fn test_skeleton_carrier_names() {
        let mut skeleton = LoopSkeleton::new(Span::unknown());

        skeleton.carriers.push(CarrierSlot {
            name: "i".to_string(),
            role: CarrierRole::Counter,
            update_kind: UpdateKind::ConstStep { delta: 1 },
        });

        skeleton.carriers.push(CarrierSlot {
            name: "sum".to_string(),
            role: CarrierRole::Accumulator,
            update_kind: UpdateKind::Arbitrary,
        });

        let names = skeleton.carrier_names();
        assert_eq!(names, vec!["i", "sum"]);
    }
}
