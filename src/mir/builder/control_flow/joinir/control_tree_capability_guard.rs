//! Phase 112: StepTree Capability Guard (strict-only)
//!
//! ## Purpose
//! Detects unsupported capabilities in StepTree required_caps during strict mode.
//! Provides actionable 1-line hints for developers to fix their code.
//!
//! ## Design
//! - **Allowlist**: If, NestedIf, Loop, Return, Break, Continue
//! - **Deny (strict)**: NestedLoop, TryCatch, Throw, Lambda, While, ForRange, Match, Arrow
//! - **Default behavior unchanged**: strict=false always returns Ok(())
//!
//! ## Integration
//! - Called from `lower_function_body()` in `calls/lowering.rs`
//! - Uses existing `HAKO_JOINIR_STRICT` / `NYASH_JOINIR_STRICT` env vars
//! - Error format: `[joinir/control_tree/cap_missing/<Cap>] <msg>  Hint: <hint>`

use crate::mir::control_tree::{StepCapability, StepTree};
use crate::mir::join_ir::lowering::error_tags;
use std::collections::BTreeSet;

/// Check StepTree capabilities against allowlist (Phase 112)
///
/// # Arguments
/// - `tree`: StepTree to check
/// - `func_name`: Function name for error messages
/// - `strict`: If false, always returns Ok (default behavior unchanged)
/// - `dev`: If true, trace missing caps (debug info)
/// - `planner_required`: If true, relaxes depth cap (strict/dev-only gate)
///
/// # Returns
/// - Ok(()) if all required_caps are allowed (or strict=false)
/// - Err with freeze_with_hint format if unsupported cap found in strict mode
pub fn check(
    tree: &StepTree,
    func_name: &str,
    strict: bool,
    dev: bool,
    planner_required: bool,
) -> Result<(), String> {
    if !strict {
        return Ok(()); // Default behavior: always pass
    }

    // Phase 188.2: Check nesting depth BEFORE capability check
    // Reject max_loop_depth > 2 (only 1-level nesting supported)
    let max_depth = if planner_required { 4 } else { 2 };
    if tree.features.max_loop_depth > max_depth {
        let tag = "control_tree/nested_loop/depth_exceeded";
        let msg = format!(
            "Nesting depth {} exceeds limit (max={}) in '{}' (step_tree_sig={})",
            tree.features.max_loop_depth,
            max_depth,
            func_name,
            tree.signature.to_hex()
        );
        let hint = "Refactor to reduce loop nesting, or run without HAKO_JOINIR_STRICT=1";

        if dev {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[joinir/control_tree] depth exceeded: max_loop_depth={} in {}",
                tree.features.max_loop_depth, func_name
            ));
        }

        return Err(error_tags::freeze_with_hint(tag, &msg, hint));
    }

    // Allowlist (supported capabilities)
    let allowed: BTreeSet<StepCapability> = [
        StepCapability::If,
        StepCapability::NestedIf,
        StepCapability::Loop,
        StepCapability::NestedLoop,  // Phase 188.1: nested_loop_minimal route support
        StepCapability::Return,
        StepCapability::Break,
        StepCapability::Continue,
        StepCapability::While,  // Phase 29bq: while is semantically equivalent to loop(cond)
    ]
    .into_iter()
    .collect();

    // Check for unsupported caps
    for cap in &tree.contract.required_caps {
        if !allowed.contains(cap) {
            let cap_name = format!("{:?}", cap);
            let tag = format!("control_tree/cap_missing/{}", cap_name);
            let msg = format!(
                "{} detected in '{}' (step_tree_sig={})",
                cap_name,
                func_name,
                tree.signature.to_hex()
            );
            let hint = get_hint_for_cap(cap);

            if dev {
                // Trace for debugging
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[joinir/control_tree] missing cap: {} in {}",
                    cap_name, func_name
                ));
            }

            return Err(error_tags::freeze_with_hint(&tag, &msg, &hint));
        }
    }

    Ok(())
}

fn get_hint_for_cap(cap: &StepCapability) -> String {
    match cap {
        StepCapability::NestedLoop => {
            // Phase 188.2: NestedLoop (1-level) is now supported
            "1-level nested loops supported; for 2+ levels use depth check error hint".to_string()
        }
        StepCapability::TryCatch => {
            "try/catch not supported in JoinIR yet, use HAKO_JOINIR_STRICT=0".to_string()
        }
        StepCapability::Throw => {
            "throw not supported in JoinIR yet, use HAKO_JOINIR_STRICT=0".to_string()
        }
        StepCapability::Lambda => {
            "lambda not supported in JoinIR yet, extract to named function".to_string()
        }
        StepCapability::While => {
            "use 'loop(cond)' instead of 'while(cond)' syntax".to_string()
        }
        StepCapability::ForRange => {
            "use 'loop(i < n)' instead of 'for i in range' syntax".to_string()
        }
        StepCapability::Match => {
            "match expressions not supported in JoinIR yet, use if-else chain".to_string()
        }
        StepCapability::Arrow => {
            "arrow functions not supported in JoinIR yet, use regular functions".to_string()
        }
        _ => format!("{:?} not supported in JoinIR yet", cap),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::control_tree::StepTreeBuilderBox;

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn int_lit(v: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(v),
            span: Span::unknown(),
        }
    }

    fn bin_lt(lhs: ASTNode, rhs: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_nested_loop_1level_strict_passes() {
        // Phase 188.2: 1-level nested loop (depth=2) should PASS
        // AST: loop(i < 3) { loop(j < 2) { ... } }
        let nested_loop_ast = vec![ASTNode::Loop {
            condition: Box::new(bin_lt(var("i"), int_lit(3))),
            body: vec![ASTNode::Loop {
                condition: Box::new(bin_lt(var("j"), int_lit(2))),
                body: vec![],
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }];

        let tree = StepTreeBuilderBox::build_from_block(&nested_loop_ast);

        // strict=true should PASS (NestedLoop is in allowlist, depth=2 is OK)
        let result = check(&tree, "test_func", true, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nested_loop_2level_strict_rejects() {
        // Phase 188.2: 2+ level nested loop (depth=3) should FAIL
        // AST: loop { loop { loop { ... } } }
        let deeply_nested_ast = vec![ASTNode::Loop {
            condition: Box::new(bin_lt(var("i"), int_lit(3))),
            body: vec![ASTNode::Loop {
                condition: Box::new(bin_lt(var("j"), int_lit(2))),
                body: vec![ASTNode::Loop {
                    condition: Box::new(bin_lt(var("k"), int_lit(1))),
                    body: vec![],
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }];

        let tree = StepTreeBuilderBox::build_from_block(&deeply_nested_ast);

        // strict=true should reject depth > 2
        let result = check(&tree, "test_func", true, false, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("[joinir/control_tree/nested_loop/depth_exceeded]"));
        assert!(err.contains("max=2"));
    }

    #[test]
    fn test_nested_loop_2level_planner_required_passes() {
        // planner_required allows depth=3
        let deeply_nested_ast = vec![ASTNode::Loop {
            condition: Box::new(bin_lt(var("i"), int_lit(3))),
            body: vec![ASTNode::Loop {
                condition: Box::new(bin_lt(var("j"), int_lit(2))),
                body: vec![ASTNode::Loop {
                    condition: Box::new(bin_lt(var("k"), int_lit(1))),
                    body: vec![],
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }];

        let tree = StepTreeBuilderBox::build_from_block(&deeply_nested_ast);

        let result = check(&tree, "test_func", true, false, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_only_strict_passes() {
        // AST: if x == 1 { ... } else { ... }
        let if_only_ast = vec![ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(var("x")),
                right: Box::new(int_lit(1)),
                span: Span::unknown(),
            }),
            then_body: vec![],
            else_body: Some(vec![]),
            span: Span::unknown(),
        }];

        let tree = StepTreeBuilderBox::build_from_block(&if_only_ast);

        // strict=true should pass (If is allowed)
        let result = check(&tree, "test_func", true, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_strict_false_always_passes() {
        // Even with NestedLoop, strict=false should pass
        let nested_loop_ast = vec![ASTNode::Loop {
            condition: Box::new(bin_lt(var("i"), int_lit(3))),
            body: vec![ASTNode::Loop {
                condition: Box::new(bin_lt(var("j"), int_lit(2))),
                body: vec![],
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }];

        let tree = StepTreeBuilderBox::build_from_block(&nested_loop_ast);

        // strict=false should always pass
        let result = check(&tree, "test_func", false, false, false);
        assert!(result.is_ok());
    }
}
