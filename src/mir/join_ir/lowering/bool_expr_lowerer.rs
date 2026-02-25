//! Phase 168: BoolExprLowerer - Boolean Expression Lowering to SSA
//!
//! This module provides lowering of complex boolean expressions (AST → SSA)
//! for use within JoinIR loop patterns. It handles:
//! - Comparisons: `<`, `==`, `!=`, `<=`, `>=`, `>`
//! - Logical operators: `&&`, `||`, `!`
//! - Mixed conditions: `ch == " " || ch == "\t" || ch == "\n"`
//!
//! ## Design Philosophy
//!
//! BoolExprLowerer is a SEPARATE module from loop patterns (Pattern1-4).
//! It focuses purely on expression lowering, while loop patterns handle
//! control flow structure.
//!
//! **Separation of Concerns**:
//! - Loop patterns (Pattern1-4): Loop structure (header, body, exit)
//! - BoolExprLowerer: Expression evaluation (AST → SSA ValueId)
//!
//! ## Target Use Case
//!
//! JsonParserBox methods `_trim` and `_skip_whitespace` have OR chains:
//! ```nyash
//! ch == " " || ch == "\t" || ch == "\n" || ch == "\r"
//! ```
//!
//! This lowerer converts such expressions into SSA form:
//! ```text
//! %cmp1 = Compare Eq %ch " "
//! %cmp2 = Compare Eq %ch "\t"
//! %cmp3 = Compare Eq %ch "\n"
//! %cmp4 = Compare Eq %ch "\r"
//! %or1 = BinOp Or %cmp1 %cmp2
//! %or2 = BinOp Or %or1 %cmp3
//! %result = BinOp Or %or2 %cmp4
//! ```
//!
//! ## Status (Phase 179)
//!
//! **TODO: Consider removal or unification** - This module appears unused (all tests commented).
//! Cannot easily merge with `condition_lowerer.rs` because:
//! - This module: MIR-based (uses MirBuilder, stateful)
//! - condition_lowerer: JoinIR-based (pure functional, no builder)
//!
//! Decision: Keep separate for now, document the distinction clearly.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

/// BoolExprLowerer - Converts boolean expression AST to SSA form
///
/// This box handles lowering of complex boolean expressions within loop conditions.
/// It produces ValueIds that can be used by loop patterns for control flow decisions.
#[allow(dead_code)]
pub struct BoolExprLowerer<'a> {
    builder: &'a mut MirBuilder,
}

impl<'a> BoolExprLowerer<'a> {
    /// Create a new BoolExprLowerer with access to MirBuilder
    #[allow(dead_code)]
    pub fn new(builder: &'a mut MirBuilder) -> Self {
        BoolExprLowerer { builder }
    }

    /// Lower a boolean expression AST to SSA form
    ///
    /// # Arguments
    ///
    /// * `cond_ast` - AST node representing the boolean condition
    ///
    /// # Returns
    ///
    /// * `Result<ValueId, String>` - Register holding the result (bool 0/1), or error
    ///
    /// # Supported Operators
    ///
    /// - Comparisons: `<`, `==`, `!=`, `<=`, `>=`, `>`
    /// - Logical: `&&`, `||`, `!`
    /// - Variables and literals
    #[allow(dead_code)]
    pub fn lower_condition(&mut self, cond_ast: &ASTNode) -> Result<ValueId, String> {
        // CLEAN-D: join_ir 側は builder の生 emit に触れない。
        // 条件式の lowering は MirBuilder facade (`build_expression`) に委譲する。
        self.builder.build_expression(cond_ast.clone())
    }
}

// TODO: These tests need to be updated to use the new MirBuilder API
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
//     use crate::mir::builder::MirBuilder;
//     use crate::mir::FunctionSignature;
//
//     /// Helper to create a test MirBuilder
//     fn create_test_builder() -> MirBuilder {
//         let mut builder = MirBuilder::new();
//         // Initialize a test function
//         let sig = FunctionSignature {
//             name: "test_function".to_string(),
//             params: vec!["i".to_string(), "ch".to_string()],
//             arity: 2,
//             return_type: crate::mir::MirType::Integer,
//         };
//         builder.start_function(sig);
//         builder.start_new_block();
//         builder
//     }
//
//     /// Test: Simple comparison (i < 10)
//     #[test]
//     fn test_simple_comparison() {
//         let mut builder = create_test_builder();
//         let mut lowerer = BoolExprLowerer::new(&mut builder);
//
//         // AST: i < 10
//         let ast = ASTNode::BinaryOp {
//             operator: BinaryOperator::Less,
//             left: Box::new(ASTNode::Variable {
//                 name: "i".to_string(),
//                 span: Span::unknown(),
//             }),
//             right: Box::new(ASTNode::Literal {
//                 value: LiteralValue::Integer(10),
//                 span: Span::unknown(),
//             }),
//             span: Span::unknown(),
//         };
//
//         let result = lowerer.lower_condition(&ast);
//         assert!(result.is_ok(), "Simple comparison should succeed");
//     }
//
//     /// Test: OR chain (ch == " " || ch == "\t")
//     #[test]
//     fn test_or_chain() {
//         let mut builder = create_test_builder();
//         let mut lowerer = BoolExprLowerer::new(&mut builder);
//
//         // AST: ch == " " || ch == "\t"
//         let ast = ASTNode::BinaryOp {
//             operator: BinaryOperator::Or,
//             left: Box::new(ASTNode::BinaryOp {
//                 operator: BinaryOperator::Equal,
//                 left: Box::new(ASTNode::Variable {
//                     name: "ch".to_string(),
//                     span: Span::unknown(),
//                 }),
//                 right: Box::new(ASTNode::Literal {
//                     value: LiteralValue::String(" ".to_string()),
//                     span: Span::unknown(),
//                 }),
//                 span: Span::unknown(),
//             }),
//             right: Box::new(ASTNode::BinaryOp {
//                 operator: BinaryOperator::Equal,
//                 left: Box::new(ASTNode::Variable {
//                     name: "ch".to_string(),
//                     span: Span::unknown(),
//                 }),
//                 right: Box::new(ASTNode::Literal {
//                     value: LiteralValue::String("\t".to_string()),
//                     span: Span::unknown(),
//                 }),
//                 span: Span::unknown(),
//             }),
//             span: Span::unknown(),
//         };
//
//         let result = lowerer.lower_condition(&ast);
//         assert!(result.is_ok(), "OR chain should succeed");
//     }
//
//     /// Test: Complex mixed condition (i < len && (c == " " || c == "\t"))
//     #[test]
//     fn test_complex_mixed_condition() {
//         let mut builder = create_test_builder();
//         let mut lowerer = BoolExprLowerer::new(&mut builder);
//
//         // AST: i < len && (c == " " || c == "\t")
//         let ast = ASTNode::BinaryOp {
//             operator: BinaryOperator::And,
//             left: Box::new(ASTNode::BinaryOp {
//                 operator: BinaryOperator::Less,
//                 left: Box::new(ASTNode::Variable {
//                     name: "i".to_string(),
//                     span: Span::unknown(),
//                 }),
//                 right: Box::new(ASTNode::Variable {
//                     name: "len".to_string(),
//                     span: Span::unknown(),
//                 }),
//                 span: Span::unknown(),
//             }),
//             right: Box::new(ASTNode::BinaryOp {
//                 operator: BinaryOperator::Or,
//                 left: Box::new(ASTNode::BinaryOp {
//                     operator: BinaryOperator::Equal,
//                     left: Box::new(ASTNode::Variable {
//                         name: "c".to_string(),
//                         span: Span::unknown(),
//                     }),
//                     right: Box::new(ASTNode::Literal {
//                         value: LiteralValue::String(" ".to_string()),
//                         span: Span::unknown(),
//                     }),
//                     span: Span::unknown(),
//                 }),
//                 right: Box::new(ASTNode::BinaryOp {
//                     operator: BinaryOperator::Equal,
//                     left: Box::new(ASTNode::Variable {
//                         name: "c".to_string(),
//                         span: Span::unknown(),
//                     }),
//                     right: Box::new(ASTNode::Literal {
//                         value: LiteralValue::String("\t".to_string()),
//                         span: Span::unknown(),
//                     }),
//                     span: Span::unknown(),
//                 }),
//                 span: Span::unknown(),
//             }),
//             span: Span::unknown(),
//         };
//
//         let result = lowerer.lower_condition(&ast);
//         assert!(result.is_ok(), "Complex mixed condition should succeed");
//     }
//
//     /// Test: NOT operator (!condition)
//     #[test]
//     fn test_not_operator() {
//         let mut builder = create_test_builder();
//         let mut lowerer = BoolExprLowerer::new(&mut builder);
//
//         // AST: !(i < 10)
//         let ast = ASTNode::UnaryOp {
//             operator: crate::ast::UnaryOperator::Not,
//             operand: Box::new(ASTNode::BinaryOp {
//                 operator: BinaryOperator::Less,
//                 left: Box::new(ASTNode::Variable {
//                     name: "i".to_string(),
//                     span: Span::unknown(),
//                 }),
//                 right: Box::new(ASTNode::Literal {
//                     value: LiteralValue::Integer(10),
//                     span: Span::unknown(),
//                 }),
//                 span: Span::unknown(),
//             }),
//             span: Span::unknown(),
//         };
//
//         let result = lowerer.lower_condition(&ast);
//         assert!(result.is_ok(), "NOT operator should succeed");
//     }
// }
