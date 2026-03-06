//! TrimDetector - Pure detection logic for trim pattern
//!
//! Extracted from LoopBodyCarrierPromoter to enable:
//! - Single responsibility (detection only)
//! - Independent unit testing
//! - Reusable pattern for future analyzers
//!
//! # Design Philosophy
//!
//! This detector follows the **Detector/Promoter separation** principle:
//! - **Detector**: Pure detection logic (this module)
//! - **Promoter**: Orchestrates carrier building from the detected shape
//!
//! # Pattern: A-3 Trim (Substring + Equality OR Chain)
//!
//! ```nyash
//! loop(start < end) {
//!     local ch = s.substring(start, start+1)
//!     if ch == " " || ch == "\t" || ch == "\n" {
//!         start = start + 1
//!     } else {
//!         break
//!     }
//! }
//! ```

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

/// Detection result for trim pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrimDetectionResult {
    /// Variable name (e.g., "ch")
    pub match_var: String,

    /// Carrier name (e.g., "is_ch_match")
    pub carrier_name: String,

    /// Comparison literals (e.g., [" ", "\t", "\n", "\r"])
    pub comparison_literals: Vec<String>,
}

/// Pure detection logic for A-3 Trim pattern
pub struct TrimDetector;

impl TrimDetector {
    /// Detect trim pattern in condition and body.
    ///
    /// Returns None if pattern not found, Some(result) if detected.
    ///
    /// # Algorithm
    ///
    /// 1. Extract equality literals from condition (e.g., [" ", "\t"])
    /// 2. Find substring() definition in loop body
    /// 3. Generate carrier name (e.g., "is_ch_match")
    ///
    /// # Arguments
    ///
    /// * `condition` - Break or continue condition AST node
    /// * `body` - Loop body statements
    /// * `var_name` - Variable name to check (from LoopBodyLocal analysis)
    ///
    /// # Returns
    ///
    /// * `Some(TrimDetectionResult)` if pattern detected
    /// * `None` if pattern not found
    pub fn detect(
        condition: &ASTNode,
        body: &[ASTNode],
        var_name: &str,
    ) -> Option<TrimDetectionResult> {
        // Step 1: Find substring() definition for the variable
        let definition = Self::find_definition_in_body(body, var_name)?;

        // Step 2: Verify it's a substring() method call
        if !Self::is_substring_method_call(definition) {
            return None;
        }

        // Step 3: Extract equality literals from condition
        let literals = Self::extract_equality_literals(condition, var_name);

        if literals.is_empty() {
            return None;
        }

        // Step 4: Generate carrier name
        let carrier_name = format!("is_{}_match", var_name);

        Some(TrimDetectionResult {
            match_var: var_name.to_string(),
            carrier_name,
            comparison_literals: literals,
        })
    }

    /// Find definition in loop body
    ///
    /// Searches for assignment: `local var = ...` or `var = ...`
    fn find_definition_in_body<'a>(body: &'a [ASTNode], var_name: &str) -> Option<&'a ASTNode> {
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

    /// Check if node is a substring() method call
    fn is_substring_method_call(node: &ASTNode) -> bool {
        matches!(
            node,
            ASTNode::MethodCall { method, .. } if method == "substring"
        )
    }

    /// Extract equality literals from condition
    ///
    /// Handles: `ch == " " || ch == "\t" || ch == "\n"`
    ///
    /// Returns: `[" ", "\t", "\n"]`
    fn extract_equality_literals(cond: &ASTNode, var_name: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut worklist = vec![cond];

        while let Some(node) = worklist.pop() {
            match node {
                // BinaryOp: Or splits, Equal compares
                ASTNode::BinaryOp {
                    operator,
                    left,
                    right,
                    ..
                } => match operator {
                    BinaryOperator::Or => {
                        // OR chain: traverse both sides
                        worklist.push(left.as_ref());
                        worklist.push(right.as_ref());
                    }
                    BinaryOperator::Equal => {
                        // Equality comparison: extract literal
                        if let Some(literal) = Self::extract_literal_from_equality(
                            left.as_ref(),
                            right.as_ref(),
                            var_name,
                        ) {
                            result.push(literal);
                        }
                    }
                    _ => {}
                },

                // UnaryOp: not (...)
                ASTNode::UnaryOp { operand, .. } => {
                    worklist.push(operand.as_ref());
                }

                _ => {}
            }
        }

        result
    }

    /// Extract string literal from equality comparison
    ///
    /// Handles: `ch == " "` or `" " == ch`
    fn extract_literal_from_equality(
        left: &ASTNode,
        right: &ASTNode,
        var_name: &str,
    ) -> Option<String> {
        // Pattern 1: var == literal
        if let ASTNode::Variable { name, .. } = left {
            if name == var_name {
                if let ASTNode::Literal {
                    value: LiteralValue::String(s),
                    ..
                } = right
                {
                    return Some(s.clone());
                }
            }
        }

        // Pattern 2: literal == var
        if let ASTNode::Literal {
            value: LiteralValue::String(s),
            ..
        } = left
        {
            if let ASTNode::Variable { name, .. } = right {
                if name == var_name {
                    return Some(s.clone());
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    fn var_node(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn string_literal(value: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(value.to_string()),
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

    fn equality(var: &str, literal: &str) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(var_node(var)),
            right: Box::new(string_literal(literal)),
            span: Span::unknown(),
        }
    }

    fn or_op(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_detect_basic_trim_pattern() {
        // local ch = s.substring(...)
        // if ch == " " || ch == "\t" { ... }

        let loop_body = vec![assignment("ch", method_call("s", "substring", vec![]))];

        let condition = or_op(equality("ch", " "), equality("ch", "\t"));

        let result = TrimDetector::detect(&condition, &loop_body, "ch");

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.match_var, "ch");
        assert_eq!(result.carrier_name, "is_ch_match");
        assert_eq!(result.comparison_literals.len(), 2);
        assert!(result.comparison_literals.contains(&" ".to_string()));
        assert!(result.comparison_literals.contains(&"\t".to_string()));
    }

    #[test]
    fn test_detect_no_match_not_substring() {
        // ch = s.length()  // Not substring!
        // if ch == " " { ... }

        let loop_body = vec![assignment("ch", method_call("s", "length", vec![]))];

        let condition = equality("ch", " ");

        let result = TrimDetector::detect(&condition, &loop_body, "ch");

        assert!(result.is_none());
    }

    #[test]
    fn test_detect_no_match_no_equality() {
        // local ch = s.substring(...)
        // if ch < "x" { ... }  // Not equality!

        let loop_body = vec![assignment("ch", method_call("s", "substring", vec![]))];

        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(var_node("ch")),
            right: Box::new(string_literal("x")),
            span: Span::unknown(),
        };

        let result = TrimDetector::detect(&condition, &loop_body, "ch");

        assert!(result.is_none());
    }

    #[test]
    fn test_detect_single_equality() {
        // local ch = s.substring(...)
        // if ch == " " { ... }

        let loop_body = vec![assignment("ch", method_call("s", "substring", vec![]))];

        let condition = equality("ch", " ");

        let result = TrimDetector::detect(&condition, &loop_body, "ch");

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.comparison_literals.len(), 1);
        assert_eq!(result.comparison_literals[0], " ");
    }

    #[test]
    fn test_detect_multiple_whitespace() {
        // local ch = s.substring(...)
        // if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" { ... }

        let loop_body = vec![assignment("ch", method_call("s", "substring", vec![]))];

        let condition = or_op(
            or_op(equality("ch", " "), equality("ch", "\t")),
            or_op(equality("ch", "\n"), equality("ch", "\r")),
        );

        let result = TrimDetector::detect(&condition, &loop_body, "ch");

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.comparison_literals.len(), 4);
        assert!(result.comparison_literals.contains(&" ".to_string()));
        assert!(result.comparison_literals.contains(&"\t".to_string()));
        assert!(result.comparison_literals.contains(&"\n".to_string()));
        assert!(result.comparison_literals.contains(&"\r".to_string()));
    }

    #[test]
    fn test_detect_carrier_name_generation() {
        // Test carrier name generation: "ch" → "is_ch_match"

        let loop_body = vec![assignment("ch", method_call("s", "substring", vec![]))];

        let condition = equality("ch", " ");

        let result = TrimDetector::detect(&condition, &loop_body, "ch").unwrap();

        assert_eq!(result.carrier_name, "is_ch_match");
    }

    #[test]
    fn test_detect_different_variable_names() {
        // Test with different variable names

        let test_cases = vec!["ch", "c", "char", "character"];

        for var_name in test_cases {
            let loop_body = vec![assignment(var_name, method_call("s", "substring", vec![]))];

            let condition = equality(var_name, " ");

            let result = TrimDetector::detect(&condition, &loop_body, var_name);

            assert!(
                result.is_some(),
                "Expected detection for var '{}'",
                var_name
            );
            let result = result.unwrap();
            assert_eq!(result.match_var, var_name);
            assert_eq!(result.carrier_name, format!("is_{}_match", var_name));
        }
    }

    #[test]
    fn test_extract_literal_reversed_equality() {
        // Test: " " == ch (literal on left side)

        let loop_body = vec![assignment("ch", method_call("s", "substring", vec![]))];

        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(string_literal(" ")),
            right: Box::new(var_node("ch")),
            span: Span::unknown(),
        };

        let result = TrimDetector::detect(&condition, &loop_body, "ch");

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.comparison_literals.len(), 1);
        assert_eq!(result.comparison_literals[0], " ");
    }
}
