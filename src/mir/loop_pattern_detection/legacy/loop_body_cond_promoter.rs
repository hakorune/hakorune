//! Phase 223-3: LoopBodyCondPromoter Box
//!
//! Handles promotion of body-local variables used in loop conditions
//! to bool carriers. Supports `loop_break`, `loop_continue_only`,
//! and future routes.
//!
//! ## Responsibilities
//!
//! - Detect safe promotion shapes (Category A-3 from phase223-loopbodylocal-condition-inventory.md)
//! - Coordinate with LoopBodyCarrierPromoter for actual promotion logic
//! - Provide uniform API for break/continue route integration
//!
//! ## Design Principle
//!
//! This is a **thin coordinator** that reuses existing boxes:
//! - LoopBodyCarrierPromoter: Promotion logic (trim route-shape detection)
//! - TrimLoopHelper: Route-specific metadata
//! - ConditionEnvBuilder: Binding generation
//!
//! ## P0 Requirements (Category A-3)
//!
//! - Single body-local variable (e.g., `ch`)
//! - Definition: `local ch = s.substring(...)` or similar
//! - Condition: Simple equality chain (e.g., `ch == " " || ch == "\t"`)
//! - Route shape: identical to existing trim route

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::loop_pattern_detection::loop_body_carrier_promoter::{
    LoopBodyCarrierPromoter, PromotionRequest, PromotionResult,
};
use crate::mir::loop_pattern_detection::loop_condition_scope::LoopConditionScope;

/// Promotion request for condition variables
///
/// Unified API for `loop_break` (break) and `loop_continue_only` (continue)
pub struct ConditionPromotionRequest<'a> {
    /// Loop parameter name (e.g., "i")
    pub loop_param_name: &'a str,

    /// Condition scope analysis result
    pub cond_scope: &'a LoopConditionScope,

    /// Loop structure metadata (crate-internal)
    pub(crate) scope_shape: Option<&'a LoopScopeShape>,

    /// Break condition AST (`loop_break`: Some, `loop_continue_only`: None)
    pub break_cond: Option<&'a ASTNode>,

    /// Continue condition AST (`loop_continue_only`: Some, `loop_break`: None)
    pub continue_cond: Option<&'a ASTNode>,

    /// Loop body statements
    pub loop_body: &'a [ASTNode],
}

/// Promotion result
pub enum ConditionPromotionResult {
    /// Promotion successful
    Promoted {
        /// Carrier metadata (from TrimLoopHelper)
        carrier_info: CarrierInfo,

        /// Variable name that was promoted (e.g., "ch")
        promoted_var: String,

        /// Promoted carrier name (e.g., "is_whitespace")
        carrier_name: String,
    },

    /// Cannot promote (Fail-Fast)
    CannotPromote {
        /// Human-readable reason
        reason: String,

        /// List of problematic body-local variables
        vars: Vec<String>,
    },
}

/// Phase 223-3: LoopBodyCondPromoter Box
///
/// Coordinates body-local condition promotion for break/continue routes.
pub struct LoopBodyCondPromoter;

impl LoopBodyCondPromoter {
    // ========================================================================
    // Public API
    // ========================================================================

    /// Extract continue condition from loop body
    ///
    /// Finds the first if statement with continue in then-branch and returns its condition.
    /// This is used for `loop_continue_only` skip_whitespace detection.
    ///
    /// # Shape
    ///
    /// ```nyash
    /// loop(i < n) {
    ///     local ch = s.substring(...)
    ///     if ch == " " || ch == "\t" {  // ← This condition is returned
    ///         i = i + 1
    ///         continue
    ///     }
    ///     break
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// The condition AST if found, None otherwise
    pub fn extract_continue_condition(body: &[ASTNode]) -> Option<&ASTNode> {
        for stmt in body {
            if let ASTNode::If {
                condition,
                then_body,
                ..
            } = stmt
            {
                // Check if then_body contains continue
                if Self::contains_continue(then_body) {
                    return Some(condition.as_ref());
                }
            }
        }
        None
    }

    /// Try to promote body-local variables in conditions
    ///
    /// ## P0 Requirements (Category A-3)
    ///
    /// - Single body-local variable (e.g., `ch`)
    /// - Definition: `local ch = s.substring(...)` or similar
    /// - Condition: Simple equality chain (e.g., `ch == " " || ch == "\t"`)
    /// - Route shape: identical to existing trim route
    ///
    /// ## Algorithm (Phase 224: Two-Tier Promotion Strategy)
    ///
    /// 1. Extract body-local names from cond_scope
    /// 2. Try A-3 Trim promotion (LoopBodyCarrierPromoter)
    /// 3. If A-3 fails, try A-4 DigitPos promotion (DigitPosPromoter)
    /// 4. If both fail, return CannotPromote
    ///
    /// ## Differences from TrimLoopLowerer
    ///
    /// - TrimLoopLowerer: Full lowering pipeline (detection + code generation)
    /// - LoopBodyCondPromoter: Detection + metadata only (no code generation)
    pub fn try_promote_for_condition(req: ConditionPromotionRequest) -> ConditionPromotionResult {
        use crate::mir::loop_pattern_detection::loop_body_digitpos_promoter::{
            DigitPosPromoter, DigitPosPromotionRequest, DigitPosPromotionResult,
        };
        use crate::mir::loop_pattern_detection::loop_condition_scope::CondVarScope;

        // P0 constraint: Need LoopScopeShape for LoopBodyCarrierPromoter
        let scope_shape = match req.scope_shape {
            Some(s) => s,
            None => {
                return ConditionPromotionResult::CannotPromote {
                    reason: "No LoopScopeShape provided".to_string(),
                    vars: vec![],
                };
            }
        };

        // Determine which condition to use for break_cond in LoopBodyCarrierPromoter.
        // loop_break: break_cond
        // loop_continue_only: continue_cond (reused as break_cond for trim detection)
        let condition_for_promotion = req.break_cond.or(req.continue_cond);

        // Step 1: Try A-3 Trim promotion
        let promotion_request = PromotionRequest {
            scope: scope_shape,
            cond_scope: req.cond_scope,
            break_cond: condition_for_promotion,
            loop_body: req.loop_body,
        };

        match LoopBodyCarrierPromoter::try_promote(&promotion_request) {
            PromotionResult::Promoted { trim_info } => {
                if crate::config::env::is_joinir_debug() || std::env::var("JOINIR_TEST_DEBUG").is_ok() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[cond_promoter] A-3 Trim pattern promoted: '{}' → carrier '{}'",
                        trim_info.var_name, trim_info.carrier_name
                    ));
                }

                let carrier_info = trim_info.to_carrier_info();

                return ConditionPromotionResult::Promoted {
                    carrier_info,
                    promoted_var: trim_info.var_name,
                    carrier_name: trim_info.carrier_name,
                };
            }
            PromotionResult::CannotPromote { reason, .. } => {
                if crate::config::env::is_joinir_debug() || std::env::var("JOINIR_TEST_DEBUG").is_ok() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!("[cond_promoter] A-3 Trim promotion failed: {}", reason));
                    ring0.log.debug(&format!("[cond_promoter] Trying A-4 DigitPos promotion..."));
                }
            }
        }

        // Step 2: Try A-4 DigitPos promotion
        let digitpos_request = DigitPosPromotionRequest {
            loop_param_name: req.loop_param_name,
            cond_scope: req.cond_scope,
            scope_shape: req.scope_shape,
            break_cond: req.break_cond,
            continue_cond: req.continue_cond,
            loop_body: req.loop_body,
        };

        match DigitPosPromoter::try_promote(digitpos_request) {
            DigitPosPromotionResult::Promoted {
                carrier_info,
                promoted_var,
                carrier_name,
            } => {
                if crate::config::env::is_joinir_debug() || std::env::var("JOINIR_TEST_DEBUG").is_ok() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[cond_promoter] A-4 DigitPos pattern promoted: '{}' → carrier '{}'",
                        promoted_var, carrier_name
                    ));
                }

                return ConditionPromotionResult::Promoted {
                    carrier_info,
                    promoted_var,
                    carrier_name,
                };
            }
            DigitPosPromotionResult::CannotPromote { reason, .. } => {
                if crate::config::env::is_joinir_debug() || std::env::var("JOINIR_TEST_DEBUG").is_ok() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!("[cond_promoter] A-4 DigitPos promotion failed: {}", reason));
                }
            }
        }

        // Step 3: Fail-Fast (no promotable route shape matched)
        let body_local_names: Vec<String> = req
            .cond_scope
            .vars
            .iter()
            .filter(|v| v.scope == CondVarScope::LoopBodyLocal)
            .map(|v| v.name.clone())
            .collect();

        ConditionPromotionResult::CannotPromote {
            reason: "No promotable route shape detected (tried A-3 Trim, A-4 DigitPos)"
                .to_string(),
            vars: body_local_names,
        }
    }

    // ========================================================================
    // Private Helpers
    // ========================================================================

    /// Check if statements contain a continue statement
    fn contains_continue(stmts: &[ASTNode]) -> bool {
        for stmt in stmts {
            match stmt {
                ASTNode::Continue { .. } => return true,
                ASTNode::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    if Self::contains_continue(then_body) {
                        return true;
                    }
                    if let Some(else_stmts) = else_body {
                        if Self::contains_continue(else_stmts) {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};
    use crate::mir::loop_pattern_detection::loop_condition_scope::{
        CondVarScope, LoopConditionScope,
    };
    use crate::mir::BasicBlockId;
    use std::collections::{BTreeMap, BTreeSet};

    fn minimal_scope() -> LoopScopeShape {
        LoopScopeShape {
            header: BasicBlockId(0),
            body: BasicBlockId(1),
            latch: BasicBlockId(2),
            exit: BasicBlockId(3),
            pinned: BTreeSet::new(),
            carriers: BTreeSet::new(),
            body_locals: BTreeSet::new(),
            exit_live: BTreeSet::new(),
            progress_carrier: None,
            variable_definitions: BTreeMap::new(),
        }
    }

    fn cond_scope_with_body_local(var_name: &str) -> LoopConditionScope {
        let mut scope = LoopConditionScope::new();
        scope.add_var(var_name.to_string(), CondVarScope::LoopBodyLocal);
        scope
    }

    // Helper: Create a Variable node
    fn var_node(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    // Helper: Create a String literal node
    fn str_literal(s: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(s.to_string()),
            span: Span::unknown(),
        }
    }

    // Helper: Create an equality comparison (var == literal)
    fn eq_cmp(var_name: &str, literal: &str) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(var_node(var_name)),
            right: Box::new(str_literal(literal)),
            span: Span::unknown(),
        }
    }

    // Helper: Create an Or expression
    fn or_expr(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    // Helper: Create a MethodCall node
    fn method_call(object: &str, method: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(var_node(object)),
            method: method.to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }
    }

    // Helper: Create an Assignment node
    fn assignment(target: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(var_node(target)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_cond_promoter_no_scope_shape() {
        let cond_scope = LoopConditionScope::new();

        let req = ConditionPromotionRequest {
            loop_param_name: "i",
            cond_scope: &cond_scope,
            scope_shape: None, // No scope shape
            break_cond: None,
            continue_cond: None,
            loop_body: &[],
        };

        match LoopBodyCondPromoter::try_promote_for_condition(req) {
            ConditionPromotionResult::CannotPromote { reason, .. } => {
                assert!(reason.contains("No LoopScopeShape"));
            }
            _ => panic!("Expected CannotPromote when no scope shape provided"),
        }
    }

    #[test]
    fn test_cond_promoter_no_body_locals() {
        let scope_shape = minimal_scope();
        let cond_scope = LoopConditionScope::new(); // Empty, no body-local variables

        let req = ConditionPromotionRequest {
            loop_param_name: "i",
            cond_scope: &cond_scope,
            scope_shape: Some(&scope_shape),
            break_cond: None,
            continue_cond: None,
            loop_body: &[],
        };

        match LoopBodyCondPromoter::try_promote_for_condition(req) {
            ConditionPromotionResult::CannotPromote { vars, .. } => {
                assert!(vars.is_empty());
            }
            _ => panic!("Expected CannotPromote when no body-local variables"),
        }
    }

    #[test]
    fn test_cond_promoter_skip_whitespace_pattern() {
        // Full Trim/skip_whitespace route test (Category A-3):
        // - body-local: ch
        // - Definition: ch = s.substring(...)
        // - Continue condition: ch == " " || ch == "\t"

        let scope_shape = minimal_scope();
        let cond_scope = cond_scope_with_body_local("ch");

        let loop_body = vec![assignment("ch", method_call("s", "substring"))];

        let continue_cond = or_expr(eq_cmp("ch", " "), eq_cmp("ch", "\t"));

        let req = ConditionPromotionRequest {
            loop_param_name: "i",
            cond_scope: &cond_scope,
            scope_shape: Some(&scope_shape),
            break_cond: None,
            continue_cond: Some(&continue_cond),
            loop_body: &loop_body,
        };

        match LoopBodyCondPromoter::try_promote_for_condition(req) {
            ConditionPromotionResult::Promoted {
                promoted_var,
                carrier_name,
                carrier_info,
            } => {
                assert_eq!(promoted_var, "ch");
                assert_eq!(carrier_name, "is_ch_match");
                // CarrierInfo should have trim_helper attached
                assert!(carrier_info.trim_helper.is_some());
            }
            ConditionPromotionResult::CannotPromote { reason, .. } => {
                panic!("Expected Promoted, got CannotPromote: {}", reason);
            }
        }
    }

    #[test]
    fn test_cond_promoter_break_condition() {
        // loop_break style: break_cond instead of continue_cond

        let scope_shape = minimal_scope();
        let cond_scope = cond_scope_with_body_local("ch");

        let loop_body = vec![assignment("ch", method_call("s", "substring"))];

        let break_cond = or_expr(eq_cmp("ch", " "), eq_cmp("ch", "\t"));

        let req = ConditionPromotionRequest {
            loop_param_name: "i",
            cond_scope: &cond_scope,
            scope_shape: Some(&scope_shape),
            break_cond: Some(&break_cond),
            continue_cond: None,
            loop_body: &loop_body,
        };

        match LoopBodyCondPromoter::try_promote_for_condition(req) {
            ConditionPromotionResult::Promoted { promoted_var, .. } => {
                assert_eq!(promoted_var, "ch");
            }
            ConditionPromotionResult::CannotPromote { reason, .. } => {
                panic!("Expected Promoted, got CannotPromote: {}", reason);
            }
        }
    }

    #[test]
    fn test_cond_promoter_non_substring_pattern() {
        // Non-substring method call should NOT be promoted
        let scope_shape = minimal_scope();
        let cond_scope = cond_scope_with_body_local("ch");

        // ch = s.length() (not substring)
        let loop_body = vec![assignment("ch", method_call("s", "length"))];

        let continue_cond = eq_cmp("ch", "5"); // Some comparison

        let req = ConditionPromotionRequest {
            loop_param_name: "i",
            cond_scope: &cond_scope,
            scope_shape: Some(&scope_shape),
            break_cond: None,
            continue_cond: Some(&continue_cond),
            loop_body: &loop_body,
        };

        match LoopBodyCondPromoter::try_promote_for_condition(req) {
            ConditionPromotionResult::CannotPromote { vars, reason } => {
                // Should fail because it's not a substring pattern
                assert!(vars.contains(&"ch".to_string()) || reason.contains("Trim pattern"));
            }
            _ => panic!("Expected CannotPromote for non-substring pattern"),
        }
    }
}
