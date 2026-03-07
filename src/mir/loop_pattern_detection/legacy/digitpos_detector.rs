//! DigitPosDetector - Pure detection logic for digit-position route shape
//!
//! Extracted from DigitPosPromoter to enable:
//! - Single responsibility (detection only)
//! - Independent unit testing
//! - Reusable detector shape for future analyzers
//!
//! # Design Philosophy
//!
//! This detector follows the **Detector/Promoter separation** principle:
//! - **Detector**: Pure detection logic (this module)
//! - **Promoter**: Orchestrates carrier building from the detected shape
//!
//! # Route Shape: A-4 DigitPos (Cascading indexOf)
//!
//! ```nyash
//! loop(p < s.length()) {
//!     local ch = s.substring(p, p+1)           // First LoopBodyLocal
//!     local digit_pos = digits.indexOf(ch)     // Second LoopBodyLocal (cascading)
//!
//!     if digit_pos < 0 {                       // Comparison condition
//!         break
//!     }
//!
//!     // Continue processing...
//!     p = p + 1
//! }
//! ```

use crate::ast::{ASTNode, BinaryOperator};

/// Detection result for digit-position route shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitPosDetectionResult {
    /// Variable name that was promoted (e.g., "digit_pos")
    pub var_name: String,

    /// Bool carrier name (e.g., "is_digit_pos")
    pub bool_carrier_name: String,

    /// Integer carrier name (e.g., "digit_value")
    pub int_carrier_name: String,
}

/// Pure detection logic for A-4 DigitPos route shape
pub struct DigitPosDetector;

impl DigitPosDetector {
    /// Detect digit-position route shape in condition and body.
    ///
    /// Returns None if the route shape is not found, Some(result) if detected.
    ///
    /// # Algorithm
    ///
    /// 1. Extract comparison variable from condition (e.g., "digit_pos")
    /// 2. Find indexOf() definition in loop body
    /// 3. Verify cascading dependency (indexOf depends on another LoopBodyLocal via substring)
    /// 4. Generate carrier names (bool + int)
    ///
    /// # Arguments
    ///
    /// * `condition` - Break or continue condition AST node
    /// * `body` - Loop body statements
    /// * `loop_var` - Loop parameter name (currently unused, for future use)
    ///
    /// # Returns
    ///
    /// * `Some(DigitPosDetectionResult)` if the route shape is detected
    /// * `None` if the route shape is not found
    pub fn detect(
        condition: &ASTNode,
        body: &[ASTNode],
        _loop_var: &str,
    ) -> Option<DigitPosDetectionResult> {
        // Step 1: Extract comparison variable from condition
        let var_in_cond = Self::extract_comparison_var(condition)?;

        // Step 2: Find indexOf() definition for the comparison variable
        let definition = Self::find_index_of_definition(body, &var_in_cond)?;

        // Step 3: Verify it's an indexOf() method call
        if !Self::is_index_of_method_call(definition) {
            return None;
        }

        // Step 4: Verify cascading dependency (indexOf depends on LoopBodyLocal)
        let _dependency = Self::find_first_body_local_dependency(body, definition)?;

        // Step 5: Generate carrier names
        // Phase 247-EX: DigitPos generates TWO carriers (dual-value model)
        // - is_<var> (boolean): for break condition
        // - <prefix>_value (integer): for NumberAccumulation
        // Naming: "digit_pos" → "is_digit_pos" + "digit_value" (not "digit_pos_value")
        let bool_carrier_name = format!("is_{}", var_in_cond);

        // Extract the base name for integer carrier (e.g., "digit_pos" → "digit")
        let base_name = if var_in_cond.ends_with("_pos") {
            &var_in_cond[..var_in_cond.len() - 4] // Remove "_pos" suffix
        } else {
            var_in_cond.as_str()
        };
        let int_carrier_name = format!("{}_value", base_name);

        Some(DigitPosDetectionResult {
            var_name: var_in_cond,
            bool_carrier_name,
            int_carrier_name,
        })
    }

    /// Find indexOf() definition in loop body
    ///
    /// Searches for assignment: `local var = ...indexOf(...)` or `var = ...indexOf(...)`
    fn find_index_of_definition<'a>(body: &'a [ASTNode], var_name: &str) -> Option<&'a ASTNode> {
        let mut worklist: Vec<&'a ASTNode> = body.iter().collect();

        while let Some(node) = worklist.pop() {
            match node {
                // Assignment: target = value
                ASTNode::Assignment { target, value, .. } => {
                    if let ASTNode::Variable { name, .. } = target.as_ref() {
                        if name == var_name {
                            return Some(value.as_ref());
                        }
                    }
                }

                // Local: local var = value
                ASTNode::Local {
                    variables,
                    initial_values,
                    ..
                } if initial_values.len() == variables.len() => {
                    for (i, var) in variables.iter().enumerate() {
                        if var == var_name {
                            if let Some(Some(init_expr)) = initial_values.get(i) {
                                return Some(init_expr.as_ref());
                            }
                        }
                    }
                }

                // Nested structures
                ASTNode::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    for stmt in then_body {
                        worklist.push(stmt);
                    }
                    if let Some(else_stmts) = else_body {
                        for stmt in else_stmts {
                            worklist.push(stmt);
                        }
                    }
                }

                ASTNode::Loop {
                    body: loop_body, ..
                } => {
                    for stmt in loop_body {
                        worklist.push(stmt);
                    }
                }

                _ => {}
            }
        }

        None
    }

    /// Check if node is an indexOf() method call
    fn is_index_of_method_call(node: &ASTNode) -> bool {
        matches!(
            node,
            ASTNode::MethodCall { method, .. } if method == "indexOf"
        )
    }

    /// Extract variable used in comparison condition
    ///
    /// Handles: `if digit_pos < 0`, `if digit_pos >= 0`, etc.
    fn extract_comparison_var(cond: &ASTNode) -> Option<String> {
        match cond {
            ASTNode::BinaryOp { operator, left, .. } => {
                // Check if it's a comparison operator (not equality)
                match operator {
                    BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::NotEqual => {
                        // Extract variable from left side
                        if let ASTNode::Variable { name, .. } = left.as_ref() {
                            return Some(name.clone());
                        }
                    }
                    _ => {}
                }
            }

            // UnaryOp: not (...)
            ASTNode::UnaryOp { operand, .. } => {
                return Self::extract_comparison_var(operand.as_ref());
            }

            _ => {}
        }

        None
    }

    /// Find first LoopBodyLocal dependency in indexOf() call
    ///
    /// Example: `digits.indexOf(ch)` → returns "ch" if it's a LoopBodyLocal
    fn find_first_body_local_dependency<'a>(
        body: &'a [ASTNode],
        index_of_call: &'a ASTNode,
    ) -> Option<&'a str> {
        if let ASTNode::MethodCall { arguments, .. } = index_of_call {
            // Check first argument (e.g., "ch" in indexOf(ch))
            if let Some(arg) = arguments.first() {
                if let ASTNode::Variable { name, .. } = arg {
                    // Verify it's defined by substring() in body
                    let def = Self::find_definition_in_body(body, name);
                    if let Some(def_node) = def {
                        if Self::is_substring_method_call(def_node) {
                            return Some(name.as_str());
                        }
                    }
                }
            }
        }

        None
    }

    /// Find definition in loop body (helper)
    fn find_definition_in_body<'a>(body: &'a [ASTNode], var_name: &str) -> Option<&'a ASTNode> {
        let mut worklist: Vec<&'a ASTNode> = body.iter().collect();

        while let Some(node) = worklist.pop() {
            match node {
                ASTNode::Assignment { target, value, .. } => {
                    if let ASTNode::Variable { name, .. } = target.as_ref() {
                        if name == var_name {
                            return Some(value.as_ref());
                        }
                    }
                }

                ASTNode::Local {
                    variables,
                    initial_values,
                    ..
                } if initial_values.len() == variables.len() => {
                    for (i, var) in variables.iter().enumerate() {
                        if var == var_name {
                            if let Some(Some(init_expr)) = initial_values.get(i) {
                                return Some(init_expr.as_ref());
                            }
                        }
                    }
                }

                ASTNode::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    for stmt in then_body {
                        worklist.push(stmt);
                    }
                    if let Some(else_stmts) = else_body {
                        for stmt in else_stmts {
                            worklist.push(stmt);
                        }
                    }
                }

                ASTNode::Loop {
                    body: loop_body, ..
                } => {
                    for stmt in loop_body {
                        worklist.push(stmt);
                    }
                }

                _ => {}
            }
        }

        None
    }

    /// Check if node is a substring() method call
    fn is_substring_method_call(node: &ASTNode) -> bool {
        matches!(
            node,
            ASTNode::MethodCall { method, .. } if method == "substring"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};

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
    fn test_detect_basic_pattern() {
        // Full A-4 pattern:
        // local ch = s.substring(...)
        // local digit_pos = digits.indexOf(ch)
        // if digit_pos < 0 { break }

        let loop_body = vec![
            assignment("ch", method_call("s", "substring", vec![])),
            assignment(
                "digit_pos",
                method_call("digits", "indexOf", vec![var_node("ch")]),
            ),
        ];

        let condition = comparison("digit_pos", BinaryOperator::Less, 0);

        let result = DigitPosDetector::detect(&condition, &loop_body, "p");

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.var_name, "digit_pos");
        assert_eq!(result.bool_carrier_name, "is_digit_pos");
        assert_eq!(result.int_carrier_name, "digit_value");
    }

    #[test]
    fn test_detect_no_match_non_index_of() {
        // ch = s.substring(...) → pos = s.length() → if pos < 0
        // Should fail: not indexOf()

        let loop_body = vec![
            assignment("ch", method_call("s", "substring", vec![])),
            assignment("pos", method_call("s", "length", vec![])), // NOT indexOf
        ];

        let condition = comparison("pos", BinaryOperator::Less, 0);

        let result = DigitPosDetector::detect(&condition, &loop_body, "p");

        assert!(result.is_none());
    }

    #[test]
    fn test_detect_no_match_no_body_local_dependency() {
        // digit_pos = fixed_string.indexOf("x")  // No LoopBodyLocal dependency
        // Should fail: indexOf doesn't depend on substring LoopBodyLocal

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

        let condition = comparison("digit_pos", BinaryOperator::Less, 0);

        let result = DigitPosDetector::detect(&condition, &loop_body, "p");

        assert!(result.is_none());
    }

    #[test]
    fn test_detect_comparison_operators() {
        // Test different comparison operators: <, >, <=, >=, !=
        let operators = vec![
            BinaryOperator::Less,
            BinaryOperator::Greater,
            BinaryOperator::LessEqual,
            BinaryOperator::GreaterEqual,
            BinaryOperator::NotEqual,
        ];

        for op in operators {
            let loop_body = vec![
                assignment("ch", method_call("s", "substring", vec![])),
                assignment(
                    "digit_pos",
                    method_call("digits", "indexOf", vec![var_node("ch")]),
                ),
            ];

            let condition = comparison("digit_pos", op.clone(), 0);

            let result = DigitPosDetector::detect(&condition, &loop_body, "p");

            assert!(result.is_some(), "Expected detection for operator {:?}", op);
        }
    }

    #[test]
    fn test_detect_equality_operator_fails() {
        // if digit_pos == -1 { break }
        // Should fail: Equality is A-3 Trim territory, not A-4 DigitPos

        let loop_body = vec![
            assignment("ch", method_call("s", "substring", vec![])),
            assignment(
                "digit_pos",
                method_call("digits", "indexOf", vec![var_node("ch")]),
            ),
        ];

        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Equal, // Equality, not comparison
            left: Box::new(var_node("digit_pos")),
            right: Box::new(int_literal(-1)),
            span: Span::unknown(),
        };

        let result = DigitPosDetector::detect(&condition, &loop_body, "p");

        assert!(result.is_none());
    }

    #[test]
    fn test_carrier_name_generation() {
        // Test carrier name generation: "digit_pos" → "is_digit_pos" + "digit_value"

        let loop_body = vec![
            assignment("ch", method_call("s", "substring", vec![])),
            assignment(
                "digit_pos",
                method_call("digits", "indexOf", vec![var_node("ch")]),
            ),
        ];

        let condition = comparison("digit_pos", BinaryOperator::Less, 0);

        let result = DigitPosDetector::detect(&condition, &loop_body, "p").unwrap();

        assert_eq!(result.bool_carrier_name, "is_digit_pos");
        assert_eq!(result.int_carrier_name, "digit_value"); // Not "digit_pos_value"
    }

    #[test]
    fn test_carrier_name_without_pos_suffix() {
        // Test carrier name for variable without "_pos" suffix

        let loop_body = vec![
            assignment("ch", method_call("s", "substring", vec![])),
            assignment(
                "index",
                method_call("digits", "indexOf", vec![var_node("ch")]),
            ),
        ];

        let condition = comparison("index", BinaryOperator::Less, 0);

        let result = DigitPosDetector::detect(&condition, &loop_body, "p").unwrap();

        assert_eq!(result.bool_carrier_name, "is_index");
        assert_eq!(result.int_carrier_name, "index_value");
    }
}
