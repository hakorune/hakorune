//! Phase 94: P5b escape + body-local derived policy (Box)
//!
//! Purpose: detect the minimal "escape handling" shape that requires:
//! - loop counter conditional step (skip escape char)
//! - body-local reassignment (e.g. `ch = substring(...)`) represented as a derived Select
//!
//! This is a *route* decision (not a fallback). In strict mode, if we detect
//! a body-local reassignment that matches the P5b entry shape but cannot be
//! converted to a derived recipe, we fail-fast with a reason tag.

use super::PolicyDecision;
use crate::ast::ASTNode;
use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::plan::escape_shape_recognizer::EscapeSkipShapeInfo;
use crate::mir::join_ir::lowering::common::body_local_derived_emitter::BodyLocalDerivedRecipe;
use crate::mir::join_ir::lowering::error_tags;

pub type P5bEscapeDerivedDecision = PolicyDecision<BodyLocalDerivedRecipe>;

/// Detect a P5b derived body-local (`ch`) recipe from a loop_break route body.
///
/// Minimal supported shape (SSOT):
/// - `local ch = <expr>` exists at top level
/// - escape if exists (detected by EscapeSkipShapeInfo)
/// - inside the escape if's then-body, after pre-increment:
///   - optional bounds `if i < n { ch = <override_expr> }`
///   - or direct `ch = <override_expr>`
pub fn classify_p5b_escape_derived(
    body: &[ASTNode],
    loop_var_name: &str,
) -> P5bEscapeDerivedDecision {
    let strict = joinir_dev::strict_enabled();
    let has_ch_init = find_local_init_expr(body, "ch").is_some();
    let has_ch_reassign = has_assignment_to_var(body, "ch");

    let Some(info) =
        crate::mir::builder::control_flow::plan::ast_feature_extractor::detect_escape_skip_shape(
            body,
        )
    else {
        if strict && has_ch_init && has_ch_reassign {
            return P5bEscapeDerivedDecision::Reject(error_tags::freeze(
                "[phase94/body_local_derived/contract/unhandled_reassign] Body-local reassignment to 'ch' detected but escape shape is not recognized",
            ));
        }
        return P5bEscapeDerivedDecision::None;
    };

    if info.counter_name != loop_var_name {
        // Not the loop counter we lower as the JoinIR loop var; ignore to avoid misrouting.
        if strict && has_ch_init && has_ch_reassign {
            return P5bEscapeDerivedDecision::Reject(error_tags::freeze(
                "[phase94/body_local_derived/contract/loop_counter_mismatch] Body-local reassignment to 'ch' detected but loop counter does not match loop_break route loop var",
            ));
        }
        return P5bEscapeDerivedDecision::None;
    }

    match build_recipe_from_info(body, &info) {
        Ok(Some(recipe)) => P5bEscapeDerivedDecision::Use(recipe),
        Ok(None) => {
            // Escape pattern exists but there is no body-local reassignment to cover.
            P5bEscapeDerivedDecision::None
        }
        Err(e) => {
            if joinir_dev::strict_enabled() {
                P5bEscapeDerivedDecision::Reject(error_tags::freeze(&e))
            } else {
                // Non-strict mode: keep legacy behavior (no derived slot); still loggable via dev.
                P5bEscapeDerivedDecision::None
            }
        }
    }
}

fn has_assignment_to_var(body: &[ASTNode], name: &str) -> bool {
    fn node_has_assignment(node: &ASTNode, name: &str) -> bool {
        match node {
            ASTNode::Assignment { target, .. } => is_var_named(target.as_ref(), name),
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                then_body.iter().any(|n| node_has_assignment(n, name))
                    || else_body
                        .as_ref()
                        .map_or(false, |e| e.iter().any(|n| node_has_assignment(n, name)))
            }
            ASTNode::Loop { body, .. } => body.iter().any(|n| node_has_assignment(n, name)),
            ASTNode::ScopeBox { body, .. } => body.iter().any(|n| node_has_assignment(n, name)),
            _ => false,
        }
    }

    body.iter().any(|n| node_has_assignment(n, name))
}

fn build_recipe_from_info(
    body: &[ASTNode],
    info: &EscapeSkipShapeInfo,
) -> Result<Option<BodyLocalDerivedRecipe>, String> {
    // 1) Find base init: `local ch = <expr>`
    let Some(base_init_expr) = find_local_init_expr(body, "ch") else {
        return Err(
            "[phase94/body_local_derived/contract/missing_local_init] Missing `local ch = <expr>`"
                .to_string(),
        );
    };

    // 2) Locate escape if and find override assignment to ch
    let escape_if = body.get(info.escape_idx).ok_or_else(|| {
        format!(
            "[phase94/body_local_derived/contract/escape_idx_oob] escape_idx={} out of bounds (body.len={})",
            info.escape_idx,
            body.len()
        )
    })?;
    let (escape_cond, then_body) = match escape_if {
        ASTNode::If {
            condition,
            then_body,
            else_body: _,
            ..
        } => (condition.as_ref().clone(), then_body.as_slice()),
        other => {
            return Err(format!(
                "[phase94/body_local_derived/contract/escape_node_kind] escape_idx points to non-If: {:?}",
                other
            ));
        }
    };

    let override_assignment = find_ch_override_in_escape_then(then_body)?;
    let Some((bounds_check, override_expr)) = override_assignment else {
        return Ok(None);
    };

    // EscapeSkipShapeInfo uses "escape_delta" for the then-body increment, and "normal_delta" for the unconditional tail.
    // For the common P5b shape:
    // - escape iteration total delta = escape_delta + normal_delta
    // - normal iteration total delta = normal_delta
    let recipe = BodyLocalDerivedRecipe {
        name: "ch".to_string(),
        base_init_expr,
        escape_cond,
        loop_counter_name: info.counter_name.clone(),
        pre_delta: info.escape_delta,
        post_delta: info.normal_delta,
        bounds_check,
        override_expr,
    };
    Ok(Some(recipe))
}

fn find_local_init_expr(body: &[ASTNode], name: &str) -> Option<ASTNode> {
    for node in body {
        if let ASTNode::Local {
            variables,
            initial_values,
            ..
        } = node
        {
            for (var_name, maybe_expr) in variables.iter().zip(initial_values.iter()) {
                if var_name == name {
                    if let Some(expr) = maybe_expr.as_ref() {
                        return Some((**expr).clone());
                    }
                }
            }
        }
    }
    None
}

/// Find `ch = <expr>` either directly or under an inner bounds `if`.
///
/// Returns:
/// - Ok(Some((bounds_opt, override_expr))) when an override assignment exists
/// - Ok(None) when no override assignment exists (no derived slot needed)
/// - Err when an override exists but violates minimal contract
fn find_ch_override_in_escape_then(
    then_body: &[ASTNode],
) -> Result<Option<(Option<ASTNode>, ASTNode)>, String> {
    // Direct assignment form: `ch = <expr>`
    for stmt in then_body {
        if let ASTNode::Assignment { target, value, .. } = stmt {
            if is_var_named(target.as_ref(), "ch") {
                return Ok(Some((None, value.as_ref().clone())));
            }
        }
    }

    // Nested bounds form: `if <cond> { ch = <expr> }`
    for stmt in then_body {
        if let ASTNode::If {
            condition,
            then_body,
            else_body: None,
            ..
        } = stmt
        {
            if then_body.len() != 1 {
                continue;
            }
            if let ASTNode::Assignment { target, value, .. } = &then_body[0] {
                if is_var_named(target.as_ref(), "ch") {
                    return Ok(Some((
                        Some(condition.as_ref().clone()),
                        value.as_ref().clone(),
                    )));
                }
            }
        }
    }

    Ok(None)
}

fn is_var_named(node: &ASTNode, name: &str) -> bool {
    matches!(node, ASTNode::Variable { name: n, .. } if n == name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, Span};
    use crate::tests::helpers::joinir_env::with_joinir_env_lock;

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn str_lit(s: &str) -> ASTNode {
        ASTNode::Literal {
            value: crate::ast::LiteralValue::String(s.to_string()),
            span: Span::unknown(),
        }
    }

    fn int_lit(v: i64) -> ASTNode {
        ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(v),
            span: Span::unknown(),
        }
    }

    fn binop(op: BinaryOperator, lhs: ASTNode, rhs: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: Span::unknown(),
        }
    }

    fn assignment(target: ASTNode, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(target),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn method_call(obj: &str, method: &str, args: Vec<ASTNode>) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(var(obj)),
            method: method.to_string(),
            arguments: args,
            span: Span::unknown(),
        }
    }

    #[test]
    fn detects_p5b_shape_and_builds_recipe() {
        // Body layout:
        // 0: local ch = s.substring(i, i+1)
        // 1: if ch == "\"" { break }
        // 2: if ch == "\\" { i = i + 1; ch = s.substring(i, i+1) }
        // 3: i = i + 1
        let body = vec![
            ASTNode::Local {
                variables: vec!["ch".to_string()],
                initial_values: vec![Some(Box::new(method_call(
                    "s",
                    "substring",
                    vec![var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))],
                )))],
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Equal, var("ch"), str_lit("\""))),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Equal, var("ch"), str_lit("\\"))),
                then_body: vec![
                    assignment(var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))),
                    assignment(
                        var("ch"),
                        method_call(
                            "s",
                            "substring",
                            vec![var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))],
                        ),
                    ),
                ],
                else_body: None,
                span: Span::unknown(),
            },
            assignment(var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))),
        ];

        match classify_p5b_escape_derived(&body, "i") {
            P5bEscapeDerivedDecision::Use(recipe) => {
                assert_eq!(recipe.name, "ch");
                assert_eq!(recipe.loop_counter_name, "i");
                assert_eq!(recipe.pre_delta, 1);
                assert_eq!(recipe.post_delta, 1);
                match recipe.override_expr {
                    ASTNode::MethodCall { ref method, .. } => assert_eq!(method, "substring"),
                    other => panic!("expected override MethodCall, got {:?}", other),
                }
            }
            other => panic!("expected UseDerived recipe, got {:?}", other),
        }
    }

    #[test]
    fn strict_rejects_when_local_init_missing() {
        with_joinir_env_lock(|| {
            // escape pattern exists, but `local ch = ...` is absent -> strict should reject
            let body = vec![
                ASTNode::If {
                    condition: Box::new(binop(BinaryOperator::Equal, var("ch"), str_lit("\""))),
                    then_body: vec![ASTNode::Break {
                        span: Span::unknown(),
                    }],
                    else_body: None,
                    span: Span::unknown(),
                },
                ASTNode::If {
                    condition: Box::new(binop(BinaryOperator::Equal, var("ch"), str_lit("\\"))),
                    then_body: vec![
                        assignment(var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))),
                        assignment(
                            var("ch"),
                            method_call(
                                "s",
                                "substring",
                                vec![var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))],
                            ),
                        ),
                    ],
                    else_body: None,
                    span: Span::unknown(),
                },
                assignment(var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))),
            ];

            let prev = crate::config::env::joinir_dev::strict_enabled();
            std::env::set_var("HAKO_JOINIR_STRICT", "1");
            let decision = classify_p5b_escape_derived(&body, "i");
            if prev {
                std::env::set_var("HAKO_JOINIR_STRICT", "1");
            } else {
                std::env::remove_var("HAKO_JOINIR_STRICT");
            }

            match decision {
                P5bEscapeDerivedDecision::Reject(reason) => {
                    assert!(
                        reason.contains("missing_local_init"),
                        "unexpected reason: {}",
                        reason
                    );
                }
                other => panic!("expected Reject, got {:?}", other),
            }
        });
    }
}
