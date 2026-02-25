//! Phase 170-D-impl-2: Condition Variable Analyzer Box
//!
//! Pure functions for analyzing condition AST nodes and determining
//! variable scopes based on LoopScopeShape information.
//!
//! This Box extracts all variable references from condition expressions
//! and determines whether they are from the loop parameter, outer scope,
//! or loop body scope.
//!
//! # Design Philosophy
//!
//! - **Pure functions**: No side effects, only analysis
//! - **Composable**: Can be used independently or as part of LoopConditionScopeBox
//! - **Fail-Fast**: Defaults to conservative classification (LoopBodyLocal)

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use std::collections::HashSet;

/// Extract all variable names from an AST expression
///
/// Iteratively traverses the AST node and collects all Variable references.
/// Handles: Variables, UnaryOp, BinaryOp, MethodCall, FieldAccess, Index, If
///
/// # Arguments
///
/// * `node` - AST node to analyze
///
/// # Returns
///
/// HashSet of all variable names found in the expression
///
/// # Example
///
/// For expression `(i < 10) && (ch != ' ')`, returns `{"i", "ch"}`
///
/// # Implementation Note
///
/// Uses a worklist-based iterative algorithm instead of recursion
/// to prevent stack overflow with deeply nested OR chains or complex expressions.
pub fn extract_all_variables(node: &ASTNode) -> HashSet<String> {
    let mut vars = HashSet::new();
    let mut worklist = vec![node];

    while let Some(current) = worklist.pop() {
        match current {
            ASTNode::Variable { name, .. } => {
                vars.insert(name.clone());
            }
            ASTNode::UnaryOp { operand, .. } => {
                worklist.push(operand);
            }
            ASTNode::BinaryOp { left, right, .. } => {
                worklist.push(left);
                worklist.push(right);
            }
            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                worklist.push(object);
                for arg in arguments {
                    worklist.push(arg);
                }
            }
            ASTNode::FieldAccess { object, .. } => {
                worklist.push(object);
            }
            ASTNode::Index { target, index, .. } => {
                worklist.push(target);
                worklist.push(index);
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                worklist.push(condition);
                for stmt in then_body {
                    worklist.push(stmt);
                }
                if let Some(else_body) = else_body {
                    for stmt in else_body {
                        worklist.push(stmt);
                    }
                }
            }
            _ => {} // Skip literals, constants, etc.
        }
    }

    vars
}

/// Determine if a variable is from the outer scope
///
/// # Phase 170-D-impl-2: Simple heuristic (Phase 170-ultrathink: Extended)
///
/// A variable is "outer local" if:
/// 1. It appears in LoopScopeShape.pinned (loop parameters or outer variables)
/// 2. It appears in LoopScopeShape.variable_definitions as being defined
///    in the header block ONLY (NOT in body/exit)
/// 3. **Phase 170-ultrathink**: It appears ONLY in header and latch blocks
///    (carrier variables that are updated in latch but not defined in body)
///
/// # Arguments
///
/// * `var_name` - Name of the variable to check
/// * `scope` - Optional LoopScopeShape with variable definition information
///
/// # Returns
///
/// - `true` if variable is definitively from outer scope
/// - `false` if unknown, from body scope, or no scope info available
///
/// # Notes
///
/// This is a simplified implementation for Phase 170-D.
/// Phase 170-ultrathink extended it to support carrier variables:
/// - Carrier variables are defined in header, updated in latch
/// - They should be classified as OuterLocal for condition analysis
/// - Example: `i = i + 1` in latch - `i` is a carrier, not body-local
///
/// Future versions may include:
/// - Dominance tree analysis
/// - More sophisticated scope inference
#[allow(dead_code)]
pub(crate) fn is_outer_scope_variable(var_name: &str, scope: Option<&LoopScopeShape>) -> bool {
    match scope {
        // No scope information: be conservative but *not* over‑strict.
        // We treat unknown as body-local only when we have a LoopScopeShape
        // that explicitly marks it so (via body_locals / definitions).
        // Here we simply say "unknown" and let the caller decide.
        None => false,
        Some(scope) => {
            // If the variable is explicitly marked as body-local, it is NOT outer.
            if scope.body_locals.contains(var_name) {
                return false;
            }

            // Check 1: Is it a pinned variable (loop parameter or passed-in)?
            if scope.pinned.contains(var_name) {
                return true;
            }

            // Check 2: Does it appear in variable_definitions?
            if let Some(def_blocks) = scope.variable_definitions.get(var_name) {
                // Case 2a: Defined ONLY in header block → outer scope
                // Phase 170-D: This is conservative - we only accept variables
                // that are EXCLUSIVELY defined in the header (before loop enters)
                if def_blocks.len() == 1 && def_blocks.contains(&scope.header) {
                    return true;
                }

                // Case 2b (Phase 170-ultrathink): Defined in header AND latch ONLY
                // → carrier variable (updated each iteration, but not body-local)
                // This supports loop patterns like:
                //   local i = 0  (header)
                //   loop(i < 10) {
                //       ...
                //       i = i + 1  (latch)
                //   }
                if def_blocks
                    .iter()
                    .all(|b| *b == scope.header || *b == scope.latch)
                {
                    return true;
                }

                // Any other definition pattern (e.g. body-only or body+header)
                // is treated as body-local / internal.
                return false;
            }

            // At this point:
            // - The variable is NOT in body_locals
            // - There is no explicit definition info for it
            //
            // This typically means "function parameter" or "outer local"
            // (e.g. JsonParserBox.s, .pos, etc.). Those should be treated
            // as OuterLocal for condition analysis, otherwise we wrongly
            // block valid loops as using loop-body-local variables.
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::BasicBlockId;

    // Helper: Create a Variable node
    fn var_node(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: crate::ast::Span::unknown(),
        }
    }

    // Helper: Create a BinaryOp node
    fn binop_node(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Add, // Placeholder operator
            left: Box::new(left),
            right: Box::new(right),
            span: crate::ast::Span::unknown(),
        }
    }

    // Helper: Create a UnaryOp node
    fn unary_node(operand: ASTNode) -> ASTNode {
        ASTNode::UnaryOp {
            operator: crate::ast::UnaryOperator::Not,
            operand: Box::new(operand),
            span: crate::ast::Span::unknown(),
        }
    }

    #[test]
    fn test_extract_single_variable() {
        let node = var_node("x");
        let vars = extract_all_variables(&node);
        assert_eq!(vars.len(), 1);
        assert!(vars.contains("x"));
    }

    #[test]
    fn test_extract_multiple_variables() {
        let node = binop_node(var_node("x"), var_node("y"));
        let vars = extract_all_variables(&node);
        assert_eq!(vars.len(), 2);
        assert!(vars.contains("x"));
        assert!(vars.contains("y"));
    }

    #[test]
    fn test_extract_deduplicated_variables() {
        let node = binop_node(var_node("x"), var_node("x"));
        let vars = extract_all_variables(&node);
        assert_eq!(vars.len(), 1); // HashSet deduplicates
        assert!(vars.contains("x"));
    }

    #[test]
    fn test_extract_nested_variables() {
        // Create (x + y) + z structure
        let inner = binop_node(var_node("x"), var_node("y"));
        let outer = binop_node(inner, var_node("z"));
        let vars = extract_all_variables(&outer);

        assert_eq!(vars.len(), 3);
        assert!(vars.contains("x"));
        assert!(vars.contains("y"));
        assert!(vars.contains("z"));
    }

    #[test]
    fn test_extract_with_unary_op() {
        // Create !(x) structure
        let node = unary_node(var_node("x"));
        let vars = extract_all_variables(&node);

        assert_eq!(vars.len(), 1);
        assert!(vars.contains("x"));
    }

    #[test]
    fn test_extract_no_variables_from_literal() {
        let node = ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(42),
            span: crate::ast::Span::unknown(),
        };
        let vars = extract_all_variables(&node);

        assert!(vars.is_empty());
    }

    #[test]
    fn test_is_outer_scope_variable_with_no_scope() {
        let result = is_outer_scope_variable("x", None);
        assert!(!result); // No scope → assume body-local
    }

    #[test]
    fn test_is_outer_scope_variable_pinned() {
        use std::collections::{BTreeMap, BTreeSet};

        let mut pinned = BTreeSet::new();
        pinned.insert("len".to_string());

        let scope = LoopScopeShape {
            header: BasicBlockId(0),
            body: BasicBlockId(1),
            latch: BasicBlockId(2),
            exit: BasicBlockId(3),
            pinned,
            carriers: BTreeSet::new(),
            body_locals: BTreeSet::new(),
            exit_live: BTreeSet::new(),
            progress_carrier: None,
            variable_definitions: BTreeMap::new(),
        };

        assert!(is_outer_scope_variable("len", Some(&scope)));
        // Note: "unknown" variables (not in pinned, variable_definitions, or body_locals)
        // are treated as OuterLocal by default (function parameters/outer locals).
        // See test_is_outer_scope_variable_function_param_like for rationale.
    }

    #[test]
    fn test_is_outer_scope_variable_from_header_only() {
        use std::collections::{BTreeMap, BTreeSet};

        let mut variable_definitions = BTreeMap::new();
        let mut header_only = BTreeSet::new();
        header_only.insert(BasicBlockId(0)); // Only in header

        variable_definitions.insert("start".to_string(), header_only);

        let scope = LoopScopeShape {
            header: BasicBlockId(0),
            body: BasicBlockId(1),
            latch: BasicBlockId(2),
            exit: BasicBlockId(3),
            pinned: BTreeSet::new(),
            carriers: BTreeSet::new(),
            body_locals: BTreeSet::new(),
            exit_live: BTreeSet::new(),
            progress_carrier: None,
            variable_definitions,
        };

        assert!(is_outer_scope_variable("start", Some(&scope)));
    }

    #[test]
    fn test_is_outer_scope_variable_from_body() {
        use std::collections::{BTreeMap, BTreeSet};

        let mut variable_definitions = BTreeMap::new();
        let mut header_and_body = BTreeSet::new();
        header_and_body.insert(BasicBlockId(0)); // header
        header_and_body.insert(BasicBlockId(1)); // body

        variable_definitions.insert("ch".to_string(), header_and_body);

        let scope = LoopScopeShape {
            header: BasicBlockId(0),
            body: BasicBlockId(1),
            latch: BasicBlockId(2),
            exit: BasicBlockId(3),
            pinned: BTreeSet::new(),
            carriers: BTreeSet::new(),
            body_locals: BTreeSet::new(),
            exit_live: BTreeSet::new(),
            progress_carrier: None,
            variable_definitions,
        };

        // Variable defined in body (in addition to header) → NOT outer-only
        assert!(!is_outer_scope_variable("ch", Some(&scope)));
    }

    #[test]
    fn test_is_outer_scope_variable_function_param_like() {
        // Variables that are *not* marked as body_locals and have no explicit
        // variable_definitions entry represent things like function parameters
        // or outer locals. These must be treated as OuterLocal so that valid
        // conditions such as `p < s.length()` (with `s` a parameter) are
        // accepted by Pattern 2/4.
        use std::collections::{BTreeMap, BTreeSet};

        let scope = LoopScopeShape {
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
        };

        assert!(
            is_outer_scope_variable("s", Some(&scope)),
            "Function parameter–like variable should be classified as OuterLocal"
        );
    }

    // ========================================================================
    // Phase 170-ultrathink: Additional Edge Case Tests (Issue #3)
    // ========================================================================

    #[test]
    fn test_extract_with_array_index() {
        // Test extraction of both array and index variable
        // Example: arr[i] should extract both "arr" and "i"
        let arr_var = var_node("arr");
        let index_var = var_node("i");

        let index_expr = ASTNode::Index {
            target: Box::new(arr_var),
            index: Box::new(index_var),
            span: crate::ast::Span::unknown(),
        };

        let vars = extract_all_variables(&index_expr);

        assert_eq!(vars.len(), 2);
        assert!(vars.contains("arr"));
        assert!(vars.contains("i"));
    }

    #[test]
    fn test_extract_literal_only_condition() {
        // Test edge case: loop(true) with no variables
        let literal_node = ASTNode::Literal {
            value: crate::ast::LiteralValue::Bool(true),
            span: crate::ast::Span::unknown(),
        };

        let vars = extract_all_variables(&literal_node);

        assert!(
            vars.is_empty(),
            "Literal-only condition should extract no variables"
        );
    }

    #[test]
    fn test_scope_header_and_latch_variable() {
        // Test Phase 170-ultrathink carrier variable support
        // Variable defined in header AND latch ONLY → should be OuterLocal
        use std::collections::{BTreeMap, BTreeSet};

        let mut variable_definitions = BTreeMap::new();
        let mut header_and_latch = BTreeSet::new();
        header_and_latch.insert(BasicBlockId(0)); // header
        header_and_latch.insert(BasicBlockId(2)); // latch

        variable_definitions.insert("i".to_string(), header_and_latch);

        let scope = LoopScopeShape {
            header: BasicBlockId(0),
            body: BasicBlockId(1),
            latch: BasicBlockId(2),
            exit: BasicBlockId(3),
            pinned: BTreeSet::new(),
            carriers: BTreeSet::new(),
            body_locals: BTreeSet::new(),
            exit_live: BTreeSet::new(),
            progress_carrier: None,
            variable_definitions,
        };

        // Phase 170-ultrathink: header+latch ONLY → OuterLocal (carrier variable)
        assert!(
            is_outer_scope_variable("i", Some(&scope)),
            "Carrier variable (header+latch only) should be classified as OuterLocal"
        );
    }

    #[test]
    fn test_scope_priority_in_add_var() {
        // Test Phase 170-ultrathink scope priority system
        use super::super::loop_condition_scope::{CondVarScope, LoopConditionScope};

        let mut scope = LoopConditionScope::new();

        // Add variable as LoopBodyLocal first
        scope.add_var("x".to_string(), CondVarScope::LoopBodyLocal);
        assert_eq!(scope.vars.len(), 1);
        assert_eq!(scope.vars[0].scope, CondVarScope::LoopBodyLocal);

        // Add same variable as OuterLocal (more restrictive)
        scope.add_var("x".to_string(), CondVarScope::OuterLocal);
        assert_eq!(scope.vars.len(), 1, "Should not duplicate variable");
        assert_eq!(
            scope.vars[0].scope,
            CondVarScope::OuterLocal,
            "Should upgrade to more restrictive OuterLocal"
        );

        // Try to downgrade to LoopBodyLocal (should be rejected)
        scope.add_var("x".to_string(), CondVarScope::LoopBodyLocal);
        assert_eq!(scope.vars.len(), 1);
        assert_eq!(
            scope.vars[0].scope,
            CondVarScope::OuterLocal,
            "Should NOT downgrade from OuterLocal to LoopBodyLocal"
        );

        // Add same variable as LoopParam (most restrictive)
        scope.add_var("x".to_string(), CondVarScope::LoopParam);
        assert_eq!(scope.vars.len(), 1);
        assert_eq!(
            scope.vars[0].scope,
            CondVarScope::LoopParam,
            "Should upgrade to most restrictive LoopParam"
        );
    }
}
