//! Phase 145 P0: ANF Contract (SSOT for diagnostic tags, out-of-scope reasons, plan structure)
//!
//! ## Purpose
//!
//! Defines the contract for ANF transformation in Normalized JoinIR:
//! - **AnfDiagnosticTag**: Diagnostic categories for ANF violations
//! - **AnfOutOfScopeReason**: Explicit out-of-scope cases (graceful Ok(None) fallback)
//! - **AnfPlan**: What ANF transformation is needed (requires_anf?, impure_count?)
//!
//! ## Design Principle (Box-First)
//!
//! **Enum discrimination** prevents branching explosion:
//! - P0: Skeleton only (no actual transformation)
//! - P1: Add whitelist check + BinaryOp pattern detection
//! - P2: Add recursive processing for compound expressions
//! - **No nested if-statements**: Each out-of-scope case = enum variant
//!
//! ## Phase Scope
//!
//! - **P0**: Contract definition only (execute_box is stub)
//! - **P1+**: Add hoist_targets, parent_kind to AnfPlan

/// Diagnostic tag for ANF-related errors (SSOT for error categorization)
///
/// Used to generate structured error messages in Phase 145+.
/// Tags follow the format: `[joinir/anf/{tag}]`
///
/// ## Phase Scope
///
/// - **P0**: Enum definition only (not yet used in execute_box)
/// - **P1+**: Used in error_tags.rs helper functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnfDiagnosticTag {
    /// Order violation: Impure expression in immediate position
    ///
    /// Example: `x = f() + g()` (both f() and g() not hoisted)
    /// Tag: `[joinir/anf/order_violation]`
    OrderViolation,

    /// Pure required: Impure expression in pure-only scope
    ///
    /// Example: `loop(iter.hasNext()) { ... }` (impure in loop condition)
    /// Tag: `[joinir/anf/pure_required]`
    PureRequired,

    /// Hoist failed: Loop/If condition hoist failed
    ///
    /// Example: `loop(f(g(), h())) { ... }` (complex nested call)
    /// Tag: `[joinir/anf/hoist_failed]`
    HoistFailed,
}

/// Out-of-scope reason for ANF transformation (graceful Ok(None) fallback)
///
/// Each variant represents a specific case where ANF transformation is not applicable.
/// Lowering code matches on these to determine whether to fall back to Ok(None) (graceful)
/// or return an error (internal mistake).
///
/// ## Phase Scope
///
/// - **P0**: ContainsCall, ContainsMethodCall only (basic detection)
/// - **P1+**: Add more granular reasons (e.g., IntrinsicNotWhitelisted)
#[derive(Debug, Clone)]
pub enum AnfOutOfScopeReason {
    /// Expression contains Call (function call)
    ///
    /// Example: `f()` (P0 does not transform Call)
    ContainsCall,

    /// Expression contains MethodCall
    ///
    /// Example: `obj.method()` (P0 does not transform MethodCall)
    ContainsMethodCall,

    /// Expression contains nested impure (P2+ feature)
    ///
    /// Example: `f(g())` (nested call requires recursive ANF, out-of-scope for P0/P1)
    NestedImpure,

    /// Condition lowering failed (impure in pure-only scope)
    ///
    /// Captures lowering error detail.
    /// Example: `loop(s.length() > 0) { ... }` (impure in loop condition)
    CondLoweringFailed(String),

    /// P0 catch-all: Unknown expression type
    ///
    /// Used when AST node is not recognized by plan_box (safe fallback).
    /// Example: `new SomeBox()`, `field.access`, etc.
    UnknownExpressionType,
}

/// Phase 145 P1: Hoist position in parent expression
///
/// Indicates where a MethodCall appears in its parent BinaryOp/UnaryOp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoistPosition {
    /// MethodCall is the left operand of BinaryOp
    Left,
    /// MethodCall is the right operand of BinaryOp
    Right,
    /// MethodCall is the operand of UnaryOp (P2+)
    Operand,
}

/// Phase 145 P1: Parent expression kind
///
/// Indicates the context where ANF transformation occurs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnfParentKind {
    /// Parent is BinaryOp (e.g., `x + s.length()`)
    BinaryOp,
    /// Phase 146 P1: Parent is Compare (e.g., `s.length() == 3`)
    Compare,
    /// Parent is UnaryOp (e.g., `not s.isEmpty()`) (P2+)
    UnaryOp,
    /// Parent is MethodCall (chained, e.g., `s.trim().length()`) (P2+)
    MethodCall,
    /// Parent is Call (nested, e.g., `f(g())`) (P2+)
    Call,
}

/// Phase 145 P1: Hoist target metadata
///
/// Describes a MethodCall that needs to be hoisted to a temporary variable.
#[derive(Debug, Clone)]
pub struct AnfHoistTarget {
    /// The known intrinsic type (e.g., KnownIntrinsic::Length0)
    pub intrinsic: crate::mir::control_tree::normalized_shadow::common::expr_lowering_contract::KnownIntrinsic,

    /// The AST node of the MethodCall to hoist
    pub ast_node: crate::ast::ASTNode,

    /// Position in parent expression (Left/Right for BinaryOp)
    pub position: HoistPosition,
}

/// ANF Plan: What ANF transformation is needed for an expression
///
/// Built by `AnfPlanBox::plan_expr()` to communicate what transformation is required.
///
/// ## Phase Scope
///
/// - **P0**: Minimal plan (requires_anf + impure_count only)
/// - **P1**: Add hoist_targets (which MethodCalls to hoist), parent_kind (BinaryOp context)
/// - **P2**: Add recursive processing for compound expressions
#[derive(Debug, Clone)]
pub struct AnfPlan {
    /// Whether ANF transformation is required
    ///
    /// - `true`: Expression contains impure subexpressions (Call/MethodCall)
    /// - `false`: Expression is pure (variables, literals, arithmetic, comparisons)
    pub requires_anf: bool,

    /// Number of impure subexpressions detected
    ///
    /// Used for diagnostic logging (P0) and future optimization (P1+).
    /// Example: `f() + g()` → impure_count = 2
    pub impure_count: usize,

    /// Phase 145 P1: Which MethodCalls to hoist
    ///
    /// Contains metadata about each MethodCall that needs to be hoisted
    /// (intrinsic type, AST node, position in parent expression).
    pub hoist_targets: Vec<AnfHoistTarget>,

    /// Phase 145 P1: Parent expression kind
    ///
    /// Indicates the context where hoisting occurs (BinaryOp, UnaryOp, etc).
    pub parent_kind: AnfParentKind,
}

impl AnfPlan {
    /// P0/P1 default: No ANF transformation needed (pure expression)
    ///
    /// Used for constructing plans for pure expressions (variables, literals, etc).
    pub fn pure() -> Self {
        Self {
            requires_anf: false,
            impure_count: 0,
            hoist_targets: vec![],
            parent_kind: AnfParentKind::BinaryOp,  // Default, will be overridden if needed
        }
    }

    /// P0 constructor: ANF transformation needed (impure expression)
    ///
    /// Used when plan_box detects impure subexpressions.
    /// P1: Use `with_hoists()` instead to specify hoist targets.
    pub fn impure(impure_count: usize) -> Self {
        Self {
            requires_anf: true,
            impure_count,
            hoist_targets: vec![],
            parent_kind: AnfParentKind::BinaryOp,
        }
    }

    /// P1 constructor: ANF transformation with specific hoist targets
    ///
    /// Used for BinaryOp patterns like `x + s.length()`.
    pub fn with_hoists(hoist_targets: Vec<AnfHoistTarget>, parent_kind: AnfParentKind) -> Self {
        let impure_count = hoist_targets.len();
        Self {
            requires_anf: !hoist_targets.is_empty(),
            impure_count,
            hoist_targets,
            parent_kind,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anf_plan_pure() {
        let plan = AnfPlan::pure();
        assert!(!plan.requires_anf);
        assert_eq!(plan.impure_count, 0);
    }

    #[test]
    fn test_anf_plan_impure() {
        let plan = AnfPlan::impure(2);
        assert!(plan.requires_anf);
        assert_eq!(plan.impure_count, 2);
    }

    #[test]
    fn test_diagnostic_tag_eq() {
        assert_eq!(AnfDiagnosticTag::OrderViolation, AnfDiagnosticTag::OrderViolation);
        assert_ne!(AnfDiagnosticTag::OrderViolation, AnfDiagnosticTag::PureRequired);
    }

    // P0: 2 contract tests (plan_pure + plan_impure)
}
