//! Phase 224: DigitPosPromoter Box
//!
//! Handles promotion of A-4 route shape: cascading body-local variables with indexOf()
//!
//! ## Pattern Example
//!
//! ```nyash
//! loop(p < s.length()) {
//!     local ch = s.substring(p, p+1)           // First body-local
//!     local digit_pos = digits.indexOf(ch)     // Second body-local (depends on ch)
//!
//!     if digit_pos < 0 {                       // Comparison condition
//!         break
//!     }
//!
//!     // Continue processing...
//!     p = p + 1
//! }
//! ```
//!
//! ## Design
//!
//! - **Responsibility**: Detect and promote A-4 digit position pattern
//! - **Input**: LoopConditionScope + break/continue condition + loop body
//! - **Output**: CarrierInfo with bool carrier (e.g., "is_digit")
//!
//! ## Key Differences from A-3 Trim
//!
//! | Feature | A-3 Trim | A-4 DigitPos |
//! |---------|----------|--------------|
//! | Method | substring() | substring() → indexOf() |
//! | Dependency | Single | Cascading (2 variables) |
//! | Condition | Equality (==) | Comparison (<, >, !=) |
//! | Structure | OR chain | Single comparison |

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::loop_pattern_detection::loop_condition_scope::LoopConditionScope;
use crate::mir::ValueId;

/// Promotion request for A-4 digit position pattern
pub struct DigitPosPromotionRequest<'a> {
    /// Loop parameter name (e.g., "p")
    #[allow(dead_code)]
    pub loop_param_name: &'a str,

    /// Condition scope analysis result
    pub cond_scope: &'a LoopConditionScope,

    /// Loop structure metadata (for future use)
    #[allow(dead_code)]
    pub(crate) scope_shape: Option<&'a LoopScopeShape>,

    /// Break condition AST (`loop_break`: Some, `loop_continue_only`: None)
    pub break_cond: Option<&'a ASTNode>,

    /// Continue condition AST (`loop_continue_only`: Some, `loop_break`: None)
    pub continue_cond: Option<&'a ASTNode>,

    /// Loop body statements
    pub loop_body: &'a [ASTNode],
}

/// Promotion result
pub enum DigitPosPromotionResult {
    /// Promotion successful
    Promoted {
        /// Carrier metadata
        carrier_info: CarrierInfo,

        /// Variable name that was promoted (e.g., "digit_pos")
        promoted_var: String,

        /// Promoted carrier name (e.g., "is_digit")
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

/// Phase 224: DigitPosPromoter Box
pub struct DigitPosPromoter;

impl DigitPosPromoter {
    /// Try to promote A-4 pattern (cascading indexOf)
    ///
    /// ## Algorithm (Phase 79: Simplified using DigitPosDetector)
    ///
    /// 1. Extract body-local variables from cond_scope
    /// 2. Use DigitPosDetector for pure detection logic
    /// 3. Build CarrierInfo with bool + int carriers
    /// 4. Record BindingId promotion (dev-only)
    pub fn try_promote(req: DigitPosPromotionRequest) -> DigitPosPromotionResult {
        use crate::mir::loop_pattern_detection::digitpos_detector::DigitPosDetector;
        use crate::mir::loop_pattern_detection::loop_condition_scope::CondVarScope;

        // Step 1: Extract body-local variables
        let body_locals: Vec<&String> = req
            .cond_scope
            .vars
            .iter()
            .filter(|v| v.scope == CondVarScope::LoopBodyLocal)
            .map(|v| &v.name)
            .collect();

        if body_locals.is_empty() {
            return DigitPosPromotionResult::CannotPromote {
                reason: "No LoopBodyLocal variables to promote".to_string(),
                vars: vec![],
            };
        }

        use crate::config::env::is_joinir_debug;
        if is_joinir_debug() || std::env::var("JOINIR_TEST_DEBUG").is_ok() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[digitpos_promoter] Phase 224: Found {} body-local variables: {:?}",
                body_locals.len(),
                body_locals
            ));
        }

        // Step 2: Get condition AST
        let condition = req.break_cond.or(req.continue_cond);
        if condition.is_none() {
            return DigitPosPromotionResult::CannotPromote {
                reason: "No break or continue condition provided".to_string(),
                vars: body_locals.iter().map(|s| s.to_string()).collect(),
            };
        }

        // Step 3: Use DigitPosDetector for pure detection
        let detection =
            DigitPosDetector::detect(condition.unwrap(), req.loop_body, req.loop_param_name);

        if detection.is_none() {
            return DigitPosPromotionResult::CannotPromote {
                reason: "No A-4 DigitPos pattern detected (indexOf not found or not cascading)"
                    .to_string(),
                vars: body_locals.iter().map(|s| s.to_string()).collect(),
            };
        }

        let detection = detection.unwrap();
        if is_joinir_debug() || std::env::var("JOINIR_TEST_DEBUG").is_ok() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[digitpos_promoter] Pattern detected: {} → {} (bool) + {} (int)",
                detection.var_name, detection.bool_carrier_name, detection.int_carrier_name
            ));
        }

        // Step 4: Build CarrierInfo
        use crate::mir::join_ir::lowering::carrier_info::{CarrierInit, CarrierRole, CarrierVar};

        // Boolean carrier (condition-only, for break)
        let promoted_carrier_bool = CarrierVar {
            name: detection.bool_carrier_name.clone(),
            host_id: ValueId(0),                 // Placeholder (will be remapped)
            join_id: None,                       // Will be allocated later
            role: CarrierRole::ConditionOnly,    // Phase 227: DigitPos is condition-only
            init: CarrierInit::BoolConst(false), // Phase 228: Initialize with false
        };

        // Integer carrier (loop-state, for NumberAccumulation)
        let promoted_carrier_int = CarrierVar {
            name: detection.int_carrier_name.clone(),
            host_id: ValueId(0), // Placeholder (loop-local; no host slot)
            join_id: None,       // Will be allocated later
            role: CarrierRole::LoopState, // Phase 247-EX: LoopState for accumulation
            init: CarrierInit::LoopLocalZero, // Derived in-loop carrier (no host binding)
        };

        // Create CarrierInfo with a dummy loop_var_name (will be ignored during merge)
        let mut carrier_info = CarrierInfo::with_carriers(
            "__dummy_loop_var__".to_string(), // Placeholder, not used
            ValueId(0),                       // Placeholder
            vec![promoted_carrier_bool, promoted_carrier_int],
        );

        // Phase 229: Record promoted variable (no need for condition_aliases)
        carrier_info
            .promoted_body_locals
            .push(detection.var_name.clone());

        if is_joinir_debug() || std::env::var("JOINIR_TEST_DEBUG").is_ok() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[digitpos_promoter] Phase 247-EX: A-4 DigitPos pattern promoted: {} → {} (bool) + {} (i64)",
                detection.var_name, detection.bool_carrier_name, detection.int_carrier_name
            ));
            ring0.log.debug(&format!(
                "[digitpos_promoter] Phase 229: Recorded promoted variable '{}' (carriers: '{}', '{}')",
                detection.var_name, detection.bool_carrier_name, detection.int_carrier_name
            ));
        }

        DigitPosPromotionResult::Promoted {
            carrier_info,
            promoted_var: detection.var_name,
            carrier_name: detection.bool_carrier_name, // Return bool carrier name for compatibility
        }
    }

        // Phase 79: Helper methods removed - now in DigitPosDetector
    // - find_index_of_definition
    // - is_index_of_method_call
    // - extract_comparison_var
    // - find_first_body_local_dependency
    // - find_definition_in_body
    // - is_substring_method_call
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};
    use crate::mir::loop_pattern_detection::loop_condition_scope::{
        CondVarScope, LoopConditionScope,
    };

    fn cond_scope_with_body_locals(vars: &[&str]) -> LoopConditionScope {
        let mut scope = LoopConditionScope::new();
        for var in vars {
            scope.add_var(var.to_string(), CondVarScope::LoopBodyLocal);
        }
        scope
    }

    fn var_node(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn int_literal(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn method_call(object: &str, method: &str, args: Vec<ASTNode>) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(var_node(object)),
            method: method.to_string(),
            arguments: args,
            span: Span::unknown(),
        }
    }

    fn assignment(target: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(var_node(target)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn comparison(var: &str, op: BinaryOperator, literal: i64) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(var_node(var)),
            right: Box::new(int_literal(literal)),
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_digitpos_no_body_locals() {
        let cond_scope = LoopConditionScope::new();

        let req = DigitPosPromotionRequest {
            loop_param_name: "p",
            cond_scope: &cond_scope,
            scope_shape: None,
            break_cond: None,
            continue_cond: None,
            loop_body: &[],
        };

        match DigitPosPromoter::try_promote(req) {
            DigitPosPromotionResult::CannotPromote { reason, vars } => {
                assert!(vars.is_empty());
                assert!(reason.contains("No LoopBodyLocal"));
            }
            _ => panic!("Expected CannotPromote when no LoopBodyLocal variables"),
        }
    }

    #[test]
    fn test_digitpos_basic_pattern() {
        // Full A-4 pattern:
        // local ch = s.substring(...)
        // local digit_pos = digits.indexOf(ch)
        // if digit_pos < 0 { break }

        let cond_scope = cond_scope_with_body_locals(&["ch", "digit_pos"]);

        let loop_body = vec![
            assignment("ch", method_call("s", "substring", vec![])),
            assignment(
                "digit_pos",
                method_call("digits", "indexOf", vec![var_node("ch")]),
            ),
        ];

        let break_cond = comparison("digit_pos", BinaryOperator::Less, 0);

        let req = DigitPosPromotionRequest {
            loop_param_name: "p",
            cond_scope: &cond_scope,
            scope_shape: None,
            break_cond: Some(&break_cond),
            continue_cond: None,
            loop_body: &loop_body,
        };

        match DigitPosPromoter::try_promote(req) {
            DigitPosPromotionResult::Promoted {
                promoted_var,
                carrier_name,
                ..
            } => {
                assert_eq!(promoted_var, "digit_pos");
                assert_eq!(carrier_name, "is_digit_pos");
            }
            DigitPosPromotionResult::CannotPromote { reason, .. } => {
                panic!("Expected Promoted, got CannotPromote: {}", reason);
            }
        }
    }

    #[test]
    fn test_digitpos_non_index_of_method() {
        // ch = s.substring(...) → pos = s.length() → if pos < 0
        // Should fail: not indexOf()

        let cond_scope = cond_scope_with_body_locals(&["ch", "pos"]);

        let loop_body = vec![
            assignment("ch", method_call("s", "substring", vec![])),
            assignment("pos", method_call("s", "length", vec![])), // NOT indexOf
        ];

        let break_cond = comparison("pos", BinaryOperator::Less, 0);

        let req = DigitPosPromotionRequest {
            loop_param_name: "p",
            cond_scope: &cond_scope,
            scope_shape: None,
            break_cond: Some(&break_cond),
            continue_cond: None,
            loop_body: &loop_body,
        };

        match DigitPosPromoter::try_promote(req) {
            DigitPosPromotionResult::CannotPromote { reason, .. } => {
                assert!(reason.contains("DigitPos pattern"));
            }
            _ => panic!("Expected CannotPromote for non-indexOf pattern"),
        }
    }

    #[test]
    fn test_digitpos_no_body_local_dependency() {
        // digit_pos = fixed_string.indexOf("x")  // No body-local dependency
        // Should fail: indexOf doesn't depend on a body-local variable

        let cond_scope = cond_scope_with_body_locals(&["digit_pos"]);

        let loop_body = vec![assignment(
            "digit_pos",
            method_call(
                "fixed_string",
                "indexOf",
                vec![ASTNode::Literal {
                    value: LiteralValue::String("x".to_string()),
                    span: Span::unknown(),
                }],
            ),
        )];

        let break_cond = comparison("digit_pos", BinaryOperator::Less, 0);

        let req = DigitPosPromotionRequest {
            loop_param_name: "p",
            cond_scope: &cond_scope,
            scope_shape: None,
            break_cond: Some(&break_cond),
            continue_cond: None,
            loop_body: &loop_body,
        };

        match DigitPosPromoter::try_promote(req) {
            DigitPosPromotionResult::CannotPromote { reason, .. } => {
                // Phase 79: Detector returns None when there is no body-local dependency
                assert!(reason.contains("DigitPos pattern"));
            }
            _ => panic!("Expected CannotPromote when no LoopBodyLocal dependency"),
        }
    }

    #[test]
    fn test_digitpos_comparison_operators() {
        // Test different comparison operators: <, >, <=, >=, !=
        let operators = vec![
            BinaryOperator::Less,
            BinaryOperator::Greater,
            BinaryOperator::LessEqual,
            BinaryOperator::GreaterEqual,
            BinaryOperator::NotEqual,
        ];

        for op in operators {
            let cond_scope = cond_scope_with_body_locals(&["ch", "digit_pos"]);

            let loop_body = vec![
                assignment("ch", method_call("s", "substring", vec![])),
                assignment(
                    "digit_pos",
                    method_call("digits", "indexOf", vec![var_node("ch")]),
                ),
            ];

            let break_cond = comparison("digit_pos", op.clone(), 0);

            let req = DigitPosPromotionRequest {
                loop_param_name: "p",
                cond_scope: &cond_scope,
                scope_shape: None,
                break_cond: Some(&break_cond),
                continue_cond: None,
                loop_body: &loop_body,
            };

            match DigitPosPromoter::try_promote(req) {
                DigitPosPromotionResult::Promoted { .. } => {
                    // Success
                }
                DigitPosPromotionResult::CannotPromote { reason, .. } => {
                    panic!("Expected Promoted for operator {:?}, got: {}", op, reason);
                }
            }
        }
    }

    #[test]
    fn test_digitpos_equality_operator() {
        // if digit_pos == -1 { break }
        // Should fail: Equality is A-3 Trim territory, not A-4 DigitPos

        let cond_scope = cond_scope_with_body_locals(&["ch", "digit_pos"]);

        let loop_body = vec![
            assignment("ch", method_call("s", "substring", vec![])),
            assignment(
                "digit_pos",
                method_call("digits", "indexOf", vec![var_node("ch")]),
            ),
        ];

        let break_cond = ASTNode::BinaryOp {
            operator: BinaryOperator::Equal, // Equality, not comparison
            left: Box::new(var_node("digit_pos")),
            right: Box::new(int_literal(-1)),
            span: Span::unknown(),
        };

        let req = DigitPosPromotionRequest {
            loop_param_name: "p",
            cond_scope: &cond_scope,
            scope_shape: None,
            break_cond: Some(&break_cond),
            continue_cond: None,
            loop_body: &loop_body,
        };

        match DigitPosPromoter::try_promote(req) {
            DigitPosPromotionResult::CannotPromote { reason, .. } => {
                // Phase 79: Detector returns None for equality, so we get generic message
                assert!(reason.contains("DigitPos pattern"));
            }
            _ => panic!("Expected CannotPromote for equality operator"),
        }
    }
}
