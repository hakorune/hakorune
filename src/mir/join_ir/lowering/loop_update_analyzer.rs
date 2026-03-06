//! Loop Update Expression Analyzer
//!
//! Phase 197: Extracts update expressions from loop body to generate
//! correct carrier update semantics.
//!
//! # Purpose
//!
//! The `loop_continue_only` lowerer needs to know how each carrier variable is updated
//! in the loop body. Instead of hardcoding "count uses +1, sum uses +i",
//! we extract the actual update expressions from the AST.
//!
//! # Example
//!
//! ```nyash
//! loop(i < 10) {
//!   i = i + 1         // UpdateExpr::BinOp { lhs: "i", op: Add, rhs: Const(1) }
//!   sum = sum + i     // UpdateExpr::BinOp { lhs: "sum", op: Add, rhs: "i" }
//!   count = count + 1 // UpdateExpr::BinOp { lhs: "count", op: Add, rhs: Const(1) }
//! }
//! ```

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::join_ir::lowering::carrier_info::CarrierVar;
use crate::mir::join_ir::BinOpKind;
use std::collections::BTreeMap; // Phase 222.5-D: HashMap → BTreeMap for determinism

/// Update expression for a carrier variable
#[derive(Debug, Clone)]
pub enum UpdateExpr {
    /// Constant increment: carrier = carrier + N
    Const(i64),
    /// Binary operation: carrier = carrier op rhs
    BinOp {
        lhs: String,
        op: BinOpKind,
        rhs: UpdateRhs,
    },
}

/// Right-hand side of update expression
///
/// Phase 178: Extended to detect string updates for multi-carrier loops.
/// Phase 190: Extended to detect number accumulation pattern (result = result * base + digit).
/// The goal is "carrier detection", not full semantic understanding.
#[derive(Debug, Clone)]
pub enum UpdateRhs {
    /// Numeric constant: count + 1
    Const(i64),
    /// Variable reference: sum + i
    Variable(String),
    /// Phase 178: String literal: result + "x"
    StringLiteral(String),
    /// Phase 190: Number accumulation pattern: result = result * base + digit
    /// Represents expressions like: result * 10 + digit
    NumberAccumulation {
        /// Multiplication base (e.g., 10 for decimal, 2 for binary)
        base: i64,
        /// Variable name containing the digit to add
        digit_var: String,
    },
    /// Phase 178: Other expression (method call, complex expr)
    /// Used to detect "carrier has an update" without understanding semantics
    Other,
}

pub struct LoopUpdateAnalyzer;

impl LoopUpdateAnalyzer {
    /// Analyze carrier update expressions from loop body
    ///
    /// Extracts update patterns like:
    /// - `sum = sum + i` → BinOp { lhs: "sum", op: Add, rhs: Variable("i") }
    /// - `count = count + 1` → BinOp { lhs: "count", op: Add, rhs: Const(1) }
    ///
    /// # Parameters
    /// - `body`: Loop body AST nodes
    /// - `carriers`: Carrier variables to analyze
    ///
    /// # Returns
    /// Map from carrier name to UpdateExpr
    pub fn analyze_carrier_updates(
        body: &[ASTNode],
        carriers: &[CarrierVar],
    ) -> BTreeMap<String, UpdateExpr> {
        // Phase 222.5-D: HashMap → BTreeMap for determinism
        let mut updates = BTreeMap::new();

        // Extract carrier names for quick lookup
        let carrier_names: Vec<&str> = carriers.iter().map(|c| c.name.as_str()).collect();

        // Recursively scan all statements in the loop body
        Self::scan_nodes(body, &carrier_names, &mut updates);

        updates
    }

    /// Recursively scan AST nodes for carrier updates
    ///
    /// Phase 33-19: Extended to scan into if-else branches to handle
    /// else-continue route shape after normalization.
    fn scan_nodes(
        nodes: &[ASTNode],
        carrier_names: &[&str],
        updates: &mut BTreeMap<String, UpdateExpr>, // Phase 222.5-D: HashMap → BTreeMap for determinism
    ) {
        for node in nodes {
            match node {
                ASTNode::Assignment { target, value, .. } => {
                    // Check if this is a carrier update (e.g., sum = sum + i)
                    if let Some(target_name) = Self::extract_variable_name(target) {
                        if carrier_names.contains(&target_name.as_str()) {
                            // This is a carrier update, analyze the RHS
                            if let Some(update_expr) =
                                Self::analyze_update_value(&target_name, value)
                            {
                                updates.insert(target_name, update_expr);
                            }
                        }
                    }
                }
                // Phase 33-19: Recursively scan if-else branches
                ASTNode::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    Self::scan_nodes(then_body, carrier_names, updates);
                    if let Some(else_stmts) = else_body {
                        Self::scan_nodes(else_stmts, carrier_names, updates);
                    }
                }
                // Add more recursive cases as needed (loops, etc.)
                _ => {}
            }
        }
    }

    /// Extract variable name from AST node (for assignment target)
    fn extract_variable_name(node: &ASTNode) -> Option<String> {
        match node {
            ASTNode::Variable { name, .. } => Some(name.clone()),
            _ => None,
        }
    }

    /// Analyze update value expression
    ///
    /// Recognizes patterns like:
    /// - `sum + i` → BinOp { lhs: "sum", op: Add, rhs: Variable("i") }
    /// - `count + 1` → BinOp { lhs: "count", op: Add, rhs: Const(1) }
    /// - Phase 190: `result * 10 + digit` → BinOp { lhs: "result", op: Add, rhs: NumberAccumulation {...} }
    fn analyze_update_value(carrier_name: &str, value: &ASTNode) -> Option<UpdateExpr> {
        match value {
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                // Phase 190: Check for number accumulation pattern first
                // Pattern: (carrier * base) + digit
                if matches!(operator, BinaryOperator::Add | BinaryOperator::Subtract) {
                    if let ASTNode::BinaryOp {
                        operator: BinaryOperator::Multiply,
                        left: mul_left,
                        right: mul_right,
                        ..
                    } = left.as_ref()
                    {
                        // Check if multiplication is: carrier * base
                        if let Some(mul_lhs_name) = Self::extract_variable_name(mul_left) {
                            if mul_lhs_name == carrier_name {
                                if let ASTNode::Literal {
                                    value: LiteralValue::Integer(base),
                                    ..
                                } = mul_right.as_ref()
                                {
                                    // Check if RHS is a variable (digit)
                                    if let Some(digit_var) = Self::extract_variable_name(right) {
                                        // This is number accumulation pattern!
                                        let op = Self::convert_operator(operator)?;
                                        return Some(UpdateExpr::BinOp {
                                            lhs: carrier_name.to_string(),
                                            op,
                                            rhs: UpdateRhs::NumberAccumulation {
                                                base: *base,
                                                digit_var,
                                            },
                                        });
                                    }
                                }
                            }
                        }
                    }
                }

                // Check if LHS is the carrier itself (e.g., sum in "sum + i")
                if let Some(lhs_name) = Self::extract_variable_name(left) {
                    if lhs_name == carrier_name {
                        // Convert operator
                        let op = Self::convert_operator(operator)?;

                        // Analyze RHS
                        let rhs = Self::analyze_rhs(right)?;

                        return Some(UpdateExpr::BinOp {
                            lhs: lhs_name,
                            op,
                            rhs,
                        });
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Analyze right-hand side of update expression
    ///
    /// Phase 178: Extended to detect string updates.
    /// Phase 190: Extended to detect number accumulation pattern.
    /// Goal: "carrier has update" detection, not semantic understanding.
    fn analyze_rhs(node: &ASTNode) -> Option<UpdateRhs> {
        match node {
            // Constant: count + 1
            ASTNode::Literal {
                value: LiteralValue::Integer(n),
                ..
            } => Some(UpdateRhs::Const(*n)),

            // Phase 178: String literal: result + "x"
            ASTNode::Literal {
                value: LiteralValue::String(s),
                ..
            } => Some(UpdateRhs::StringLiteral(s.clone())),

            // Variable: sum + i (also handles: result + ch)
            ASTNode::Variable { name, .. } => Some(UpdateRhs::Variable(name.clone())),

            // Phase 190: Number accumulation pattern detection
            // This is called from analyze_update_value, so we're analyzing the full RHS of an assignment
            // Pattern not detected at this level - handled in analyze_update_value
            // BinaryOp nodes that aren't simple Add/Sub are treated as complex
            ASTNode::BinaryOp { .. } => Some(UpdateRhs::Other),

            // Phase 178: Method call or other complex expression
            // e.g., result + s.substring(pos, end)
            ASTNode::Call { .. } | ASTNode::MethodCall { .. } | ASTNode::UnaryOp { .. } => {
                Some(UpdateRhs::Other)
            }

            _ => None,
        }
    }

    /// Convert AST operator to MIR BinOpKind
    fn convert_operator(op: &BinaryOperator) -> Option<BinOpKind> {
        match op {
            BinaryOperator::Add => Some(BinOpKind::Add),
            BinaryOperator::Subtract => Some(BinOpKind::Sub),
            BinaryOperator::Multiply => Some(BinOpKind::Mul),
            BinaryOperator::Divide => Some(BinOpKind::Div),
            _ => None, // Only support arithmetic operators for now
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_simple_increment() {
        // Test case: count = count + 1
        use crate::ast::Span;

        let body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "count".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::Variable {
                    name: "count".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let carriers = vec![CarrierVar {
            name: "count".to_string(),
            host_id: crate::mir::ValueId(0),
            join_id: None, // Phase 177-STRUCT-1
            role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
            init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
        }];

        let updates = LoopUpdateAnalyzer::analyze_carrier_updates(&body, &carriers);

        assert_eq!(updates.len(), 1);
        assert!(updates.contains_key("count"));

        if let Some(UpdateExpr::BinOp { lhs, op, rhs }) = updates.get("count") {
            assert_eq!(lhs, "count");
            assert_eq!(*op, BinOpKind::Add);
            if let UpdateRhs::Const(n) = rhs {
                assert_eq!(*n, 1);
            } else {
                panic!("Expected Const(1), got {:?}", rhs);
            }
        } else {
            panic!("Expected BinOp, got {:?}", updates.get("count"));
        }
    }

    #[test]
    fn test_analyze_number_accumulation_base10() {
        // Test case: result = result * 10 + digit
        use crate::ast::Span;

        let body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "result".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Multiply,
                    left: Box::new(ASTNode::Variable {
                        name: "result".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(10),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "digit".to_string(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let carriers = vec![CarrierVar {
            name: "result".to_string(),
            host_id: crate::mir::ValueId(0),
            join_id: None,
            role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
            init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
        }];

        let updates = LoopUpdateAnalyzer::analyze_carrier_updates(&body, &carriers);

        assert_eq!(updates.len(), 1);
        assert!(updates.contains_key("result"));

        if let Some(UpdateExpr::BinOp { lhs, op, rhs }) = updates.get("result") {
            assert_eq!(lhs, "result");
            assert_eq!(*op, BinOpKind::Add);
            if let UpdateRhs::NumberAccumulation { base, digit_var } = rhs {
                assert_eq!(*base, 10);
                assert_eq!(digit_var, "digit");
            } else {
                panic!("Expected NumberAccumulation, got {:?}", rhs);
            }
        } else {
            panic!("Expected BinOp, got {:?}", updates.get("result"));
        }
    }

    #[test]
    fn test_analyze_number_accumulation_base2() {
        // Test case: result = result * 2 + bit
        use crate::ast::Span;

        let body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "result".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Multiply,
                    left: Box::new(ASTNode::Variable {
                        name: "result".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(2),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "bit".to_string(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let carriers = vec![CarrierVar {
            name: "result".to_string(),
            host_id: crate::mir::ValueId(0),
            join_id: None,
            role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
            init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
        }];

        let updates = LoopUpdateAnalyzer::analyze_carrier_updates(&body, &carriers);

        assert_eq!(updates.len(), 1);
        assert!(updates.contains_key("result"));

        if let Some(UpdateExpr::BinOp { lhs, op, rhs }) = updates.get("result") {
            assert_eq!(lhs, "result");
            assert_eq!(*op, BinOpKind::Add);
            if let UpdateRhs::NumberAccumulation { base, digit_var } = rhs {
                assert_eq!(*base, 2);
                assert_eq!(digit_var, "bit");
            } else {
                panic!("Expected NumberAccumulation, got {:?}", rhs);
            }
        } else {
            panic!("Expected BinOp, got {:?}", updates.get("result"));
        }
    }

    #[test]
    fn test_analyze_number_accumulation_wrong_lhs() {
        // Test case: result = other * 10 + digit (should be Complex/Other)
        use crate::ast::Span;

        let body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "result".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Multiply,
                    left: Box::new(ASTNode::Variable {
                        name: "other".to_string(), // Wrong variable!
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(10),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "digit".to_string(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let carriers = vec![CarrierVar {
            name: "result".to_string(),
            host_id: crate::mir::ValueId(0),
            join_id: None,
            role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
            init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
        }];

        let updates = LoopUpdateAnalyzer::analyze_carrier_updates(&body, &carriers);

        // Should detect assignment but with Other (complex) RHS
        assert_eq!(updates.len(), 0); // Won't match because lhs != carrier
    }

    #[test]
    fn test_analyze_num_str_string_append() {
        // Phase 245B: Test case: num_str = num_str + ch (string append pattern)
        use crate::ast::Span;

        let body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "num_str".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::Variable {
                    name: "num_str".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "ch".to_string(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let carriers = vec![CarrierVar {
            name: "num_str".to_string(),
            host_id: crate::mir::ValueId(0),
            join_id: None,
            role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
            init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost,
        }];

        let updates = LoopUpdateAnalyzer::analyze_carrier_updates(&body, &carriers);

        assert_eq!(updates.len(), 1);
        assert!(updates.contains_key("num_str"));

        if let Some(UpdateExpr::BinOp { lhs, op, rhs }) = updates.get("num_str") {
            assert_eq!(lhs, "num_str");
            assert_eq!(*op, BinOpKind::Add);
            if let UpdateRhs::Variable(var_name) = rhs {
                assert_eq!(var_name, "ch");
            } else {
                panic!("Expected Variable('ch'), got {:?}", rhs);
            }
        } else {
            panic!("Expected BinOp, got {:?}", updates.get("num_str"));
        }
    }

    #[test]
    fn test_atoi_update_expr_detection() {
        // Phase 246-EX Step 3: _atoi loop multi-carrier update detection
        // Tests two carriers with different update patterns:
        // - i = i + 1 (Const increment)
        // - result = result * 10 + digit_pos (NumberAccumulation)
        use crate::ast::Span;

        let body = vec![
            // result = result * 10 + digit_pos
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "result".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Multiply,
                        left: Box::new(ASTNode::Variable {
                            name: "result".to_string(),
                            span: Span::unknown(),
                        }),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(10),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Variable {
                        name: "digit_pos".to_string(),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            // i = i + 1
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "i".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
        ];

        let carriers = vec![
            CarrierVar {
                name: "result".to_string(),
                host_id: crate::mir::ValueId(0),
                join_id: None,
                role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost,
            },
            CarrierVar {
                name: "i".to_string(),
                host_id: crate::mir::ValueId(1),
                join_id: None,
                role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost,
            },
        ];

        let updates = LoopUpdateAnalyzer::analyze_carrier_updates(&body, &carriers);

        // Verify both carriers have updates
        assert_eq!(
            updates.len(),
            2,
            "Should detect updates for both i and result"
        );

        // Verify i = i + 1 (Const increment)
        if let Some(UpdateExpr::BinOp { lhs, op, rhs }) = updates.get("i") {
            assert_eq!(lhs, "i");
            assert_eq!(*op, BinOpKind::Add);
            if let UpdateRhs::Const(n) = rhs {
                assert_eq!(*n, 1, "i should increment by 1");
            } else {
                panic!("Expected Const(1) for i update, got {:?}", rhs);
            }
        } else {
            panic!("Expected BinOp for i update, got {:?}", updates.get("i"));
        }

        // Verify result = result * 10 + digit_pos (NumberAccumulation)
        if let Some(UpdateExpr::BinOp { lhs, op, rhs }) = updates.get("result") {
            assert_eq!(lhs, "result");
            assert_eq!(*op, BinOpKind::Add);
            if let UpdateRhs::NumberAccumulation { base, digit_var } = rhs {
                assert_eq!(*base, 10, "NumberAccumulation should use base 10");
                assert_eq!(digit_var, "digit_pos", "Should use digit_pos variable");
            } else {
                panic!(
                    "Expected NumberAccumulation for result update, got {:?}",
                    rhs
                );
            }
        } else {
            panic!(
                "Expected BinOp for result update, got {:?}",
                updates.get("result")
            );
        }
    }
}
