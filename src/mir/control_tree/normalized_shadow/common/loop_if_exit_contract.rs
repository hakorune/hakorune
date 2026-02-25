//! Phase 143 R0: Loop-If-Exit Contract (SSOT for pattern shape)
//!
//! ## Purpose
//!
//! Provides contract enums to prevent if-branch explosion when extending Phase 143
//! with P1 (continue) and P2 (else branches).
//!
//! - **LoopIfExitThen**: Discriminates exit action (Break, Continue)
//! - **LoopIfExitShape**: Captures pattern shape (has_else, then, else_, cond_scope)
//! - **OutOfScopeReason**: Explicit out-of-scope cases (graceful Ok(None) fallback)
//!
//! ## Design Principle
//!
//! **Enum discrimination** prevents if-branch explosion:
//! - P0: 1 pattern (break-only)
//! - P1: Add 1 enum variant + 1 match arm (continue)
//! - P2: Add 2 enum variants + 2 match arms (else branches)
//! - **No nested if-statements**: Each pattern = enum variant

use super::expr_lowering_contract::ExprLoweringScope;

/// Exit action for then/else branches (Phase 143 discriminator)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopIfExitThen {
    /// Then branch: break (Phase 143 P0)
    Break,

    /// Then branch: continue (Phase 143 P1+)
    Continue,
}

/// Pattern shape contract (SSOT for what the pattern looks like)
///
/// Separates pattern detection from lowering. Used by:
/// - `LoopTrueIfBreakContinueBuilderBox::extract_pattern_shape()` to build
/// - `LoopTrueIfBreakContinueBuilderBox::lower_with_shape()` to validate & execute
#[derive(Debug, Clone)]
pub struct LoopIfExitShape {
    /// Whether else branch exists
    ///
    /// - P0: false (no else)
    /// - P2+: true (else branch present)
    pub has_else: bool,

    /// Then branch exit action
    ///
    /// - P0: Break only
    /// - P1+: Break or Continue
    pub then: LoopIfExitThen,

    /// Else branch exit action (if exists)
    ///
    /// - P0: None (no else)
    /// - P2+: Some(Break or Continue)
    pub else_: Option<LoopIfExitThen>,

    /// Condition lowering scope
    ///
    /// - P0: PureOnly (variables, literals, arithmetic, comparisons)
    /// - P1+: WithImpure (tentative for future extensions)
    pub cond_scope: ExprLoweringScope,
}

/// Out-of-scope discrimination (graceful Ok(None) fallback reasons)
///
/// Each variant represents a specific out-of-scope case. Lowering code
/// matches on these to determine whether to fall back to Ok(None) (graceful)
/// or return an error (internal mistake).
#[derive(Debug, Clone)]
pub enum OutOfScopeReason {
    /// Loop condition is not `true` literal
    ///
    /// Example: `loop(x > 0) { if(...) break }` → out-of-scope
    NotLoopTrue,

    /// Loop body is not single if statement
    ///
    /// Examples:
    /// - `loop(true) { x = 1; if(...) break }` (assignment before if)
    /// - `loop(true) { if(...) break; x = 1 }` (statement after if)
    /// - `loop(true) { if(...) if(...) break }` (nested if)
    BodyNotSingleIf,

    /// Then branch is not break/continue
    ///
    /// Captures detail about what was found (for diagnostics).
    /// Example: `loop(true) { if(...) { x = 1 } }` (assignment in then)
    ThenNotExit(String),

    /// Else branch not supported yet (P2+ feature)
    ///
    /// Captures what exit action is in else (for error messages).
    /// Example: `loop(true) { if(...) break else continue }` → P0 rejects else
    ElseNotSupported(LoopIfExitThen),

    /// Condition lowering failed
    ///
    /// Captures lowering error detail (impure, unknown variable, etc).
    /// Example: `loop(true) { if(s.length() > 0) break }` → impure (Phase 141+)
    CondOutOfScope(String),
}

impl LoopIfExitShape {
    /// P0 default: no-else, break-only, pure condition
    ///
    /// Used for constructing P0-compatible shapes.
    pub fn p0_break_only() -> Self {
        Self {
            has_else: false,
            then: LoopIfExitThen::Break,
            else_: None,
            cond_scope: ExprLoweringScope::PureOnly,
        }
    }

    /// Validate shape is supported by P0 (break-only, no-else)
    ///
    /// **P0 Scope**:
    /// - No else branch
    /// - Then branch: break only (no continue)
    /// - Condition: pure only (ExprLoweringScope::PureOnly enforced elsewhere)
    ///
    /// Returns:
    /// - `Ok(())` if shape is P0-compatible
    /// - `Err(OutOfScopeReason)` if violates P0 constraints (graceful fallback)
    pub fn validate_for_p0(&self) -> Result<(), OutOfScopeReason> {
        // P0: No else branch allowed
        if self.has_else {
            return Err(OutOfScopeReason::ElseNotSupported(
                self.else_.unwrap_or(LoopIfExitThen::Break)
            ));
        }

        // P0: Then branch must be Break
        if self.then != LoopIfExitThen::Break {
            return Err(OutOfScopeReason::ThenNotExit(format!(
                "{:?} not supported in P0 (expected Break)",
                self.then
            )));
        }

        Ok(())
    }

    /// Validate shape is supported by P1 (break OR continue, no-else)
    ///
    /// **P1 Scope**:
    /// - No else branch
    /// - Then branch: break or continue
    /// - Condition: pure only
    ///
    /// Returns:
    /// - `Ok(())` if shape is P1-compatible
    /// - `Err(OutOfScopeReason)` if violates P1 constraints
    pub fn validate_for_p1(&self) -> Result<(), OutOfScopeReason> {
        // P1: No else branch allowed
        if self.has_else {
            return Err(OutOfScopeReason::ElseNotSupported(
                self.else_.unwrap_or(LoopIfExitThen::Break)
            ));
        }

        // P1: Accept both Break and Continue
        Ok(())
    }

    /// Validate shape is supported by P2 (with-else, symmetric break/continue)
    ///
    /// **P2 Scope**:
    /// - Else branch allowed (must have symmetric break/continue)
    /// - Then branch: break or continue
    /// - Else branch: must be present if has_else=true
    /// - Condition: pure only
    ///
    /// Returns:
    /// - `Ok(())` if shape is P2-compatible
    /// - `Err(OutOfScopeReason)` if violates P2 constraints
    pub fn validate_for_p2(&self) -> Result<(), OutOfScopeReason> {
        // P2: If has_else=true, else_ must be Some
        if self.has_else && self.else_.is_none() {
            return Err(OutOfScopeReason::ThenNotExit(
                "else branch marked but no action specified".to_string()
            ));
        }

        // P2: Accept all else combinations (will be validated at JoinModule construction)
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_p0_break_only() {
        let shape = LoopIfExitShape::p0_break_only();
        assert_eq!(shape.then, LoopIfExitThen::Break);
        assert!(!shape.has_else);
        assert!(shape.else_.is_none());
    }

    #[test]
    fn test_shape_validate_p0_break_ok() {
        let shape = LoopIfExitShape::p0_break_only();
        assert!(shape.validate_for_p0().is_ok());
    }

    #[test]
    fn test_shape_validate_p0_else_not_supported() {
        let shape = LoopIfExitShape {
            has_else: true,
            then: LoopIfExitThen::Break,
            else_: Some(LoopIfExitThen::Continue),
            cond_scope: ExprLoweringScope::PureOnly,
        };
        assert!(matches!(
            shape.validate_for_p0(),
            Err(OutOfScopeReason::ElseNotSupported(_))
        ));
    }

    #[test]
    fn test_shape_validate_p0_continue_not_supported() {
        let shape = LoopIfExitShape {
            has_else: false,
            then: LoopIfExitThen::Continue,
            else_: None,
            cond_scope: ExprLoweringScope::PureOnly,
        };
        assert!(matches!(
            shape.validate_for_p0(),
            Err(OutOfScopeReason::ThenNotExit(_))
        ));
    }

    #[test]
    fn test_shape_validate_p1_continue_ok() {
        let shape = LoopIfExitShape {
            has_else: false,
            then: LoopIfExitThen::Continue,
            else_: None,
            cond_scope: ExprLoweringScope::PureOnly,
        };
        assert!(shape.validate_for_p1().is_ok());
    }

    #[test]
    fn test_shape_validate_p1_break_ok() {
        let shape = LoopIfExitShape::p0_break_only();
        assert!(shape.validate_for_p1().is_ok());
    }

    #[test]
    fn test_shape_validate_p2_break_else_continue_ok() {
        let shape = LoopIfExitShape {
            has_else: true,
            then: LoopIfExitThen::Break,
            else_: Some(LoopIfExitThen::Continue),
            cond_scope: ExprLoweringScope::PureOnly,
        };
        assert!(shape.validate_for_p2().is_ok());
    }

    #[test]
    fn test_shape_validate_p2_continue_else_break_ok() {
        let shape = LoopIfExitShape {
            has_else: true,
            then: LoopIfExitThen::Continue,
            else_: Some(LoopIfExitThen::Break),
            cond_scope: ExprLoweringScope::PureOnly,
        };
        assert!(shape.validate_for_p2().is_ok());
    }

    #[test]
    fn test_loop_if_exit_then_eq() {
        assert_eq!(LoopIfExitThen::Break, LoopIfExitThen::Break);
        assert_eq!(LoopIfExitThen::Continue, LoopIfExitThen::Continue);
        assert_ne!(LoopIfExitThen::Break, LoopIfExitThen::Continue);
    }
}
