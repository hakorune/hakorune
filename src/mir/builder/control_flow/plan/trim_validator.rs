//! Trim Validation
//!
//! Validates and prepares Trim route information for JoinIR lowering.
//! Responsible for:
//! - Safety checks on Trim pattern structure
//! - Whitespace check generation
//! - Substring argument extraction

#[cfg(test)]
use crate::ast::ASTNode;
#[cfg(test)]
use crate::mir::builder::MirBuilder;
#[cfg(test)]
use crate::mir::ValueId;

#[cfg(test)]
pub(in crate::mir::builder) struct TrimValidator;

#[cfg(test)]
impl TrimValidator {
    /// Generate MIR for OR chain of whitespace character comparisons
    ///
    /// Creates: ch == " " || ch == "\t" || ch == "\n" || ch == "\r" ...
    ///
    /// # Arguments
    ///
    /// * `builder` - MirBuilder for emitting instructions
    /// * `ch_value` - ValueId of character to check
    /// * `whitespace_chars` - List of whitespace characters to compare against
    ///
    /// # Returns
    ///
    /// ValueId of bool result (true if ch matches any whitespace char)
    pub(in crate::mir::builder) fn emit_whitespace_check(
        builder: &mut MirBuilder,
        ch_value: ValueId,
        whitespace_chars: &[String],
    ) -> Result<ValueId, String> {
        use crate::mir::builder::emission::compare::emit_to as emit_compare_to;
        use crate::mir::builder::emission::constant::emit_string;
        use crate::mir::instruction::MirInstruction;
        use crate::mir::{CompareOp, types::BinaryOp};

        if whitespace_chars.is_empty() {
            return Err("[emit_whitespace_check] Empty whitespace_chars".to_string());
        }

        let mut result_opt: Option<ValueId> = None;

        for ws_char in whitespace_chars {
            // ws_const = const " " (or "\t", etc.)
            let ws_const = emit_string(builder, ws_char.clone())?;

            // eq_check = ch == ws_const
            // Phase 135 P0: Use function-level ValueId (SSOT)
            let eq_dst = builder.next_value_id();
            emit_compare_to(builder, eq_dst, CompareOp::Eq, ch_value, ws_const)?;

            result_opt = Some(if let Some(prev_result) = result_opt {
                // result = prev_result || eq_check
                // Phase 135 P0: Use function-level ValueId (SSOT)
                let dst = builder.next_value_id();
                builder.emit_instruction(MirInstruction::BinOp {
                    dst,
                    op: BinaryOp::Or,
                    lhs: prev_result,
                    rhs: eq_dst,
                })?;
                dst
            } else {
                // First comparison, no OR needed yet
                eq_dst
            });
        }

        result_opt.ok_or_else(|| {
            "[emit_whitespace_check] Internal error: result should be Some".to_string()
        })
    }

    /// Extract the substring method call arguments from loop body
    ///
    /// Looks for pattern: local ch = s.substring(start, start+1)
    ///
    /// # Arguments
    ///
    /// * `loop_body` - Loop body AST nodes
    /// * `var_name` - Variable name to search for (e.g., "ch")
    ///
    /// # Returns
    ///
    /// (object_name, start_expr) tuple if found
    pub(in crate::mir::builder) fn extract_substring_args(
        loop_body: &[ASTNode],
        var_name: &str,
    ) -> Option<(String, Box<ASTNode>)> {
        for stmt in loop_body {
            // Look for: local ch = ...
            if let ASTNode::Local {
                variables,
                initial_values,
                ..
            } = stmt
            {
                for (i, var) in variables.iter().enumerate() {
                    if var == var_name {
                        if let Some(Some(init_expr_box)) = initial_values.get(i) {
                            // Check if it's a substring method call
                            if let ASTNode::MethodCall {
                                object,
                                method,
                                arguments,
                                ..
                            } = init_expr_box.as_ref()
                            {
                                if method == "substring" && arguments.len() == 2 {
                                    // Extract object name
                                    if let ASTNode::Variable { name, .. } = object.as_ref() {
                                        // Return object name and start expression
                                        // (We assume second arg is start+1, first arg is start)
                                        return Some((
                                            name.clone(),
                                            Box::new(arguments[0].clone()),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};

    #[test]
    fn test_extract_substring_args_valid() {
        // Create: local ch = s.substring(start, start+1)
        let body = vec![ASTNode::Local {
            variables: vec!["ch".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "s".to_string(),
                    span: Span::unknown(),
                }),
                method: "substring".to_string(),
                arguments: vec![
                    ASTNode::Variable {
                        name: "start".to_string(),
                        span: Span::unknown(),
                    },
                    ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(ASTNode::Variable {
                            name: "start".to_string(),
                            span: Span::unknown(),
                        }),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(1),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        }];

        let result = TrimValidator::extract_substring_args(&body, "ch");
        assert!(result.is_some());
        let (s_name, _) = result.unwrap();
        assert_eq!(s_name, "s");
    }

    #[test]
    fn test_extract_substring_args_not_found() {
        // Empty body
        let body = vec![];
        let result = TrimValidator::extract_substring_args(&body, "ch");
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_substring_args_wrong_var() {
        // local other_var = s.substring(0, 1)
        let body = vec![ASTNode::Local {
            variables: vec!["other_var".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "s".to_string(),
                    span: Span::unknown(),
                }),
                method: "substring".to_string(),
                arguments: vec![
                    ASTNode::Literal {
                        value: LiteralValue::Integer(0),
                        span: Span::unknown(),
                    },
                    ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        }];

        let result = TrimValidator::extract_substring_args(&body, "ch");
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_substring_args_wrong_method() {
        // local ch = s.charAt(0)
        let body = vec![ASTNode::Local {
            variables: vec!["ch".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "s".to_string(),
                    span: Span::unknown(),
                }),
                method: "charAt".to_string(),
                arguments: vec![ASTNode::Literal {
                    value: LiteralValue::Integer(0),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        }];

        let result = TrimValidator::extract_substring_args(&body, "ch");
        assert!(result.is_none());
    }
}
