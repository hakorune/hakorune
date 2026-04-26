use super::*;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::loop_route_detection::LoopFeatures;

fn span() -> Span {
    Span::unknown()
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: span(),
    }
}

fn lit_i(n: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(n),
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

fn assignment(target: ASTNode, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(target),
        value: Box::new(value),
        span: span(),
    }
}

fn has_continue(node: &ASTNode) -> bool {
    match node {
        ASTNode::Continue { .. } => true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(has_continue)
                || else_body
                    .as_ref()
                    .map_or(false, |b| b.iter().any(has_continue))
        }
        ASTNode::Loop { body, .. } => body.iter().any(has_continue),
        _ => false,
    }
}

fn has_break(node: &ASTNode) -> bool {
    match node {
        ASTNode::Break { .. } => true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(has_break)
                || else_body
                    .as_ref()
                    .map_or(false, |b| b.iter().any(has_break))
        }
        ASTNode::Loop { body, .. } => body.iter().any(has_break),
        _ => false,
    }
}

fn has_if(body: &[ASTNode]) -> bool {
    body.iter().any(|n| matches!(n, ASTNode::If { .. }))
}

fn carrier_count(body: &[ASTNode]) -> usize {
    fn count(nodes: &[ASTNode]) -> usize {
        let mut c = 0;
        for n in nodes {
            match n {
                ASTNode::Assignment { .. } => c += 1,
                ASTNode::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    c += count(then_body);
                    if let Some(else_body) = else_body {
                        c += count(else_body);
                    }
                }
                _ => {}
            }
        }
        c
    }
    if count(body) > 0 {
        1
    } else {
        0
    }
}

fn classify_body(body: &[ASTNode]) -> LoopRouteKind {
    let has_continue_flag = body.iter().any(has_continue);
    let has_break_flag = body.iter().any(has_break);
    let features = LoopFeatures {
        has_break: has_break_flag,
        has_continue: has_continue_flag,
        has_if: has_if(body),
        carrier_count: carrier_count(body),
        break_count: if has_break_flag { 1 } else { 0 },
        continue_count: if has_continue_flag { 1 } else { 0 },
        is_infinite_loop: false, // テストでは通常ループを想定
        ..Default::default()     // Phase 188.1: Use default for new fields
    };
    classify(&features)
}

#[test]
fn loop_simple_while_is_detected() {
    // loop(i < len) { i = i + 1 }
    let body = vec![assignment(
        var("i"),
        bin(BinaryOperator::Add, var("i"), lit_i(1)),
    )];
    let kind = classify_body(&body);
    assert_eq!(kind, LoopRouteKind::LoopSimpleWhile);
}

#[test]
fn loop_break_is_detected() {
    // loop(i < len) { if i > 0 { break } i = i + 1 }
    let cond = bin(BinaryOperator::Greater, var("i"), lit_i(0));
    let body = vec![
        ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![ASTNode::Break { span: span() }],
            else_body: None,
            span: span(),
        },
        assignment(var("i"), bin(BinaryOperator::Add, var("i"), lit_i(1))),
    ];
    let kind = classify_body(&body);
    assert_eq!(kind, LoopRouteKind::LoopBreak);
}

#[test]
fn parse_number_like_loop_is_classified_as_loop_break() {
    // loop(p < len) {
    //   if digit_pos < 0 { break }
    //   p = p + 1
    // }
    let break_cond = bin(BinaryOperator::Less, var("digit_pos"), lit_i(0));
    let body = vec![
        ASTNode::If {
            condition: Box::new(break_cond),
            then_body: vec![ASTNode::Break { span: span() }],
            else_body: None,
            span: span(),
        },
        assignment(var("p"), bin(BinaryOperator::Add, var("p"), lit_i(1))),
    ];

    let kind = classify_body(&body);
    assert_eq!(kind, LoopRouteKind::LoopBreak);
}

#[test]
fn if_phi_join_shape_is_detected() {
    // loop(i < len) { if i % 2 == 1 { sum = sum + 1 } i = i + 1 }
    let cond = bin(
        BinaryOperator::Equal,
        bin(BinaryOperator::Modulo, var("i"), lit_i(2)),
        lit_i(1),
    );
    let body = vec![
        ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![assignment(
                var("sum"),
                bin(BinaryOperator::Add, var("sum"), lit_i(1)),
            )],
            else_body: None,
            span: span(),
        },
        assignment(var("i"), bin(BinaryOperator::Add, var("i"), lit_i(1))),
    ];
    let kind = classify_body(&body);
    assert_eq!(kind, LoopRouteKind::IfPhiJoin);
}

#[test]
fn loop_continue_only_shape_is_detected() {
    // loop(i < len) { if (i % 2 == 0) { continue } sum = sum + i; i = i + 1 }
    let cond = bin(
        BinaryOperator::Equal,
        bin(BinaryOperator::Modulo, var("i"), lit_i(2)),
        lit_i(0),
    );
    let body = vec![
        ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![ASTNode::Continue { span: span() }],
            else_body: Some(vec![assignment(
                var("sum"),
                bin(BinaryOperator::Add, var("sum"), var("i")),
            )]),
            span: span(),
        },
        assignment(var("i"), bin(BinaryOperator::Add, var("i"), lit_i(1))),
    ];
    let kind = classify_body(&body);
    assert_eq!(kind, LoopRouteKind::LoopContinueOnly);
}

#[test]
fn atoi_loop_is_classified_as_loop_break() {
    // Phase 246-EX Step 1: _atoi loop pattern classification
    // loop(i < len) {
    //     local ch = s.substring(i, i+1)
    //     local digit_pos = digits.indexOf(ch)
    //     if digit_pos < 0 { break }
    //     result = result * 10 + digit_pos
    //     i = i + 1
    // }

    // Simplified: loop with break + two carrier updates
    let break_cond = bin(BinaryOperator::Less, var("digit_pos"), lit_i(0));

    // result = result * 10 + digit_pos (NumberAccumulation pattern)
    let mul_expr = bin(BinaryOperator::Multiply, var("result"), lit_i(10));
    let result_update = assignment(
        var("result"),
        bin(BinaryOperator::Add, mul_expr, var("digit_pos")),
    );

    // i = i + 1
    let i_update = assignment(var("i"), bin(BinaryOperator::Add, var("i"), lit_i(1)));

    let body = vec![
        ASTNode::If {
            condition: Box::new(break_cond),
            then_body: vec![ASTNode::Break { span: span() }],
            else_body: None,
            span: span(),
        },
        result_update,
        i_update,
    ];

    let kind = classify_body(&body);
    assert_eq!(
        kind,
        LoopRouteKind::LoopBreak,
        "_atoi loop should be classified as LoopBreak due to if-break structure"
    );
}
