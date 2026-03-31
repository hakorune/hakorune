use super::*;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::analysis::expr_view;
use crate::parser::NyashParser;

fn span() -> Span {
    Span::unknown()
}

fn var_node(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: span(),
    }
}

fn int_lit(n: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(n),
        span: span(),
    }
}

fn str_lit(s: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(s.to_string()),
        span: span(),
    }
}

fn bin(op: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: op,
        left: Box::new(left),
        right: Box::new(right),
        span: span(),
    }
}

fn if_then(cond: ASTNode, then_body: Vec<ASTNode>) -> ASTNode {
    ASTNode::If {
        condition: Box::new(cond),
        then_body,
        else_body: None,
        span: span(),
    }
}

fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(target),
        value: Box::new(value),
        span: span(),
    }
}

fn blockexpr(prelude_stmts: Vec<ASTNode>, tail_expr: ASTNode) -> ASTNode {
    ASTNode::BlockExpr {
        prelude_stmts,
        tail_expr: Box::new(tail_expr),
        span: span(),
    }
}

fn find_first_loop<'a>(node: &'a ASTNode) -> Option<(&'a ASTNode, &'a [ASTNode])> {
    match node {
        ASTNode::Loop {
            condition, body, ..
        } => Some((condition.as_ref(), body.as_slice())),
        ASTNode::Program { statements, .. } => statements.iter().find_map(find_first_loop),
        ASTNode::BoxDeclaration {
            methods,
            constructors,
            static_init,
            ..
        } => {
            for v in methods.values() {
                if let Some(found) = find_first_loop(v) {
                    return Some(found);
                }
            }
            for v in constructors.values() {
                if let Some(found) = find_first_loop(v) {
                    return Some(found);
                }
            }
            if let Some(init) = static_init {
                if let Some(found) = init.iter().find_map(find_first_loop) {
                    return Some(found);
                }
            }
            None
        }
        ASTNode::FunctionDeclaration { body, .. } => body.iter().find_map(find_first_loop),
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => then_body
            .iter()
            .find_map(find_first_loop)
            .or_else(|| else_body.as_ref()?.iter().find_map(find_first_loop)),
        _ => None,
    }
}

#[test]
fn detects_balanced_array_end_min_shape() {
    // loop(i < n) {
    //   local ch = s.substring(i, i+1)
    //   if ch == "[" { depth = depth + 1 }
    //   if ch == "]" { depth = depth - 1; if depth == 0 { return i } }
    //   i = i + 1
    // }
    let condition = bin(BinaryOperator::Less, var_node("i"), var_node("n"));
    let local_ch = ASTNode::Local {
        variables: vec!["ch".to_string()],
        initial_values: vec![Some(Box::new(ASTNode::MethodCall {
            object: Box::new(var_node("s")),
            method: "substring".to_string(),
            arguments: vec![
                var_node("i"),
                bin(BinaryOperator::Add, var_node("i"), int_lit(1)),
            ],
            span: span(),
        }))],
        span: span(),
    };
    let open_branch = if_then(
        bin(BinaryOperator::Equal, var_node("ch"), str_lit("[")),
        vec![ASTNode::Assignment {
            target: Box::new(var_node("depth")),
            value: Box::new(bin(BinaryOperator::Add, var_node("depth"), int_lit(1))),
            span: span(),
        }],
    );
    let close_branch = if_then(
        bin(BinaryOperator::Equal, var_node("ch"), str_lit("]")),
        vec![
            ASTNode::Assignment {
                target: Box::new(var_node("depth")),
                value: Box::new(bin(BinaryOperator::Subtract, var_node("depth"), int_lit(1))),
                span: span(),
            },
            if_then(
                bin(BinaryOperator::Equal, var_node("depth"), int_lit(0)),
                vec![ASTNode::Return {
                    value: Some(Box::new(var_node("i"))),
                    span: span(),
                }],
            ),
        ],
    );
    let tail_inc = ASTNode::Assignment {
        target: Box::new(var_node("i")),
        value: Box::new(bin(BinaryOperator::Add, var_node("i"), int_lit(1))),
        span: span(),
    };

    let body = vec![local_ch, open_branch, close_branch, tail_inc];
    let decision = classify_balanced_depth_scan_array_end(&condition, &body);
    let result = match decision {
        crate::mir::policies::PolicyDecision::Use(v) => v,
        other => panic!("expected Use, got {:?}", other),
    };

    assert!(
        matches!(
            &result.post_loop_early_return.cond,
            ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                left,
                right,
                ..
            } if matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == "i")
                && matches!(right.as_ref(), ASTNode::Variable { name, .. } if name == "n")
        ),
        "post-loop cond must be `i < n`, got {:?}",
        result.post_loop_early_return.cond
    );
    assert!(
        matches!(
            &result.post_loop_early_return.ret_expr,
            ASTNode::Variable { name, .. } if name == "i"
        ),
        "post-loop ret_expr must be `i`, got {:?}",
        result.post_loop_early_return.ret_expr
    );
    assert!(result
        .allowed_body_locals_for_conditions
        .contains(&"ch".to_string()));
    assert!(result
        .allowed_body_locals_for_conditions
        .contains(&"depth_next".to_string()));
    assert!(result.carrier_updates_override.contains_key("i"));
    assert!(result.carrier_updates_override.contains_key("depth"));
}

#[test]
fn detects_balanced_array_end_min_shape_from_parser_ast() {
    let src = r#"
static box Main {
  f(s, idx) {
    local n = s.length()
    if s.substring(idx, idx+1) != "[" { return -1 }
    local depth = 0
    local i = idx
    loop (i < n) {
      local ch = s.substring(i, i+1)
      if ch == "[" { depth = depth + 1 }
      if ch == "]" { depth = depth - 1  if depth == 0 { return i } }
      i = i + 1
    }
    return -1
  }
}
"#;
    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let (condition, body) = find_first_loop(&ast).expect("find loop");
    let decision = classify_balanced_depth_scan_array_end(condition, body);
    assert!(
        matches!(decision, crate::mir::policies::PolicyDecision::Use(_)),
        "got {:?}",
        decision
    );
}

#[test]
fn analysis_view_accepts_add_commutative_but_sub_is_non_commutative() {
    let depth_add = vec![assign(
        var_node("depth"),
        bin(BinaryOperator::Add, var_node("depth"), int_lit(1)),
    )];
    let v = expr_view::find_single_self_update_assign_by_const_any_target(&depth_add, 1, true)
        .expect("depth = depth + 1");
    assert_eq!(v.target_var, "depth");
    assert_eq!(v.rhs.op, expr_view::SelfUpdateOp::Add);
    assert_eq!(v.rhs.step, 1);
    assert_eq!(v.rhs.commute, expr_view::Commute::AsWritten);

    let depth_add_swapped = vec![assign(
        var_node("depth"),
        bin(BinaryOperator::Add, int_lit(1), var_node("depth")),
    )];
    let v =
        expr_view::find_single_self_update_assign_by_const_any_target(&depth_add_swapped, 1, true)
            .expect("depth = 1 + depth");
    assert_eq!(v.target_var, "depth");
    assert_eq!(v.rhs.op, expr_view::SelfUpdateOp::Add);
    assert_eq!(v.rhs.step, 1);
    assert_eq!(v.rhs.commute, expr_view::Commute::Swapped);

    let depth_sub = vec![assign(
        var_node("depth"),
        bin(BinaryOperator::Subtract, var_node("depth"), int_lit(1)),
    )];
    let v = expr_view::find_single_self_update_assign_by_const_any_target(&depth_sub, -1, true)
        .expect("depth = depth - 1");
    assert_eq!(v.target_var, "depth");
    assert_eq!(v.rhs.op, expr_view::SelfUpdateOp::Sub);
    assert_eq!(v.rhs.step, 1);
    assert_eq!(v.rhs.commute, expr_view::Commute::AsWritten);

    let depth_sub_wrong = vec![assign(
        var_node("depth"),
        bin(BinaryOperator::Subtract, int_lit(1), var_node("depth")),
    )];
    assert!(
        expr_view::find_single_self_update_assign_by_const_any_target(&depth_sub_wrong, -1, true)
            .is_none(),
        "1 - depth must be rejected"
    );
}

#[test]
fn analysis_view_blockexpr_accepts_single_update_and_rejects_ambiguous_or_control_flow_prelude() {
    let ok_tail_update = vec![blockexpr(
        vec![ASTNode::Local {
            variables: vec!["t".to_string()],
            initial_values: vec![Some(Box::new(int_lit(0)))],
            span: span(),
        }],
        assign(
            var_node("depth"),
            bin(BinaryOperator::Add, var_node("depth"), int_lit(1)),
        ),
    )];
    assert!(
        expr_view::find_single_self_update_assign_by_const_any_target(&ok_tail_update, 1, true)
            .is_some(),
        "BlockExpr with single update (tail) must be accepted"
    );

    let two_updates = vec![blockexpr(
        vec![
            assign(
                var_node("depth"),
                bin(BinaryOperator::Add, var_node("depth"), int_lit(1)),
            ),
            assign(
                var_node("depth"),
                bin(BinaryOperator::Add, var_node("depth"), int_lit(1)),
            ),
        ],
        int_lit(0),
    )];
    assert!(
        expr_view::find_single_self_update_assign_by_const_any_target(&two_updates, 1, true)
            .is_none(),
        "BlockExpr with 2 updates must be rejected"
    );

    let prelude_has_if = vec![blockexpr(
        vec![if_then(
            int_lit(1),
            vec![assign(
                var_node("depth"),
                bin(BinaryOperator::Add, var_node("depth"), int_lit(1)),
            )],
        )],
        int_lit(0),
    )];
    assert!(
        expr_view::find_single_self_update_assign_by_const_any_target(&prelude_has_if, 1, true)
            .is_none(),
        "BlockExpr prelude with If must be rejected"
    );
}

