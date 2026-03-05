//! Phase 29ai P12: loop_break loopbodylocal facts (legacy label: Pattern2, SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir::builder) enum LoopBodyLocalShape {
    TrimSeg { s_var: String, i_var: String },
    DigitPos { digits_var: String, ch_var: String },
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern2LoopBodyLocalFacts {
    pub loop_var: String,
    pub loopbodylocal_var: String,
    pub break_uses_loopbodylocal: bool,
    pub shape: LoopBodyLocalShape,
}

pub(in crate::mir::builder) fn try_extract_pattern2_loopbodylocal_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<Pattern2LoopBodyLocalFacts>, Freeze> {
    let Some(loop_var) = extract_loop_var(condition) else {
        return Ok(None);
    };
    let Some((break_condition, break_idx)) = find_break_guard_if(body) else {
        return Ok(None);
    };

    if let Some((loopbodylocal_var, shape)) =
        try_match_trim_seg(break_condition, body, break_idx, &loop_var)
    {
        return Ok(Some(Pattern2LoopBodyLocalFacts {
            loop_var,
            loopbodylocal_var,
            break_uses_loopbodylocal: true,
            shape,
        }));
    }

    if let Some((loopbodylocal_var, shape)) =
        try_match_digit_pos(break_condition, body, break_idx, &loop_var)
    {
        return Ok(Some(Pattern2LoopBodyLocalFacts {
            loop_var,
            loopbodylocal_var,
            break_uses_loopbodylocal: true,
            shape,
        }));
    }

    Ok(None)
}

fn extract_loop_var(condition: &ASTNode) -> Option<String> {
    let ASTNode::BinaryOp {
        operator,
        left,
        ..
    } = condition
    else {
        return None;
    };
    if !matches!(operator, BinaryOperator::Less | BinaryOperator::LessEqual) {
        return None;
    }
    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };
    Some(name.clone())
}

fn find_break_guard_if(body: &[ASTNode]) -> Option<(&ASTNode, usize)> {
    for (idx, stmt) in body.iter().enumerate() {
        let ASTNode::If { condition, then_body, else_body, .. } = stmt else {
            continue;
        };
        if else_body.is_some() {
            continue;
        }
        let has_break_at_end = then_body
            .last()
            .map(|n| matches!(n, ASTNode::Break { .. }))
            .unwrap_or(false);
        if !has_break_at_end {
            continue;
        }
        return Some((condition.as_ref(), idx));
    }
    None
}

fn try_match_trim_seg(
    break_condition: &ASTNode,
    body: &[ASTNode],
    break_idx: usize,
    loop_var: &str,
) -> Option<(String, LoopBodyLocalShape)> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Or,
        left,
        right,
        ..
    } = break_condition
    else {
        return None;
    };
    let left_var = extract_eq_whitespace(left.as_ref())?;
    let right_var = extract_eq_whitespace(right.as_ref())?;
    if left_var != right_var {
        return None;
    }
    let (seg_idx, seg_expr) = find_local_init_expr(body, &left_var)?;
    if seg_idx >= break_idx {
        return None;
    }
    let s_var = extract_substring_loop_slice(&seg_expr, loop_var)?;
    Some((
        left_var.clone(),
        LoopBodyLocalShape::TrimSeg {
            s_var,
            i_var: loop_var.to_string(),
        },
    ))
}

fn try_match_digit_pos(
    break_condition: &ASTNode,
    body: &[ASTNode],
    break_idx: usize,
    loop_var: &str,
) -> Option<(String, LoopBodyLocalShape)> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = break_condition
    else {
        return None;
    };
    let ASTNode::Variable { name: loopbodylocal_var, .. } = left.as_ref() else {
        return None;
    };
    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(0),
            ..
        }
    ) {
        return None;
    }

    let (digit_idx, digit_expr) = find_local_init_expr(body, loopbodylocal_var)?;
    if digit_idx >= break_idx {
        return None;
    }
    let (digits_var, ch_var) = extract_indexof_expr(&digit_expr)?;

    let (ch_idx, ch_expr) = find_local_init_expr(body, &ch_var)?;
    if ch_idx >= break_idx || ch_idx >= digit_idx {
        return None;
    }
    let _s_var = extract_substring_loop_slice(&ch_expr, loop_var)?;

    Some((
        loopbodylocal_var.clone(),
        LoopBodyLocalShape::DigitPos {
            digits_var,
            ch_var,
        },
    ))
}

fn extract_eq_whitespace(node: &ASTNode) -> Option<String> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left,
        right,
        ..
    } = node
    else {
        return None;
    };
    extract_var_eq_whitespace(left.as_ref(), right.as_ref())
        .or_else(|| extract_var_eq_whitespace(right.as_ref(), left.as_ref()))
}

fn extract_var_eq_whitespace(var_node: &ASTNode, lit_node: &ASTNode) -> Option<String> {
    let ASTNode::Variable { name, .. } = var_node else {
        return None;
    };
    let ASTNode::Literal {
        value: LiteralValue::String(value),
        ..
    } = lit_node
    else {
        return None;
    };
    if value == " " || value == "\t" {
        Some(name.clone())
    } else {
        None
    }
}

fn find_local_init_expr(body: &[ASTNode], name: &str) -> Option<(usize, ASTNode)> {
    for (idx, stmt) in body.iter().enumerate() {
        let ASTNode::Local {
            variables,
            initial_values,
            ..
        } = stmt
        else {
            continue;
        };
        if variables.len() != 1 || initial_values.len() != 1 {
            continue;
        }
        if variables[0] != name {
            continue;
        }
        let Some(expr) = initial_values[0].as_ref() else {
            return None;
        };
        return Some((idx, (*expr.clone()).clone()));
    }
    None
}

fn extract_substring_loop_slice(expr: &ASTNode, loop_var: &str) -> Option<String> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr
    else {
        return None;
    };
    if method != "substring" || arguments.len() != 2 {
        return None;
    }
    let ASTNode::Variable { name: s_var, .. } = object.as_ref() else {
        return None;
    };
    let ASTNode::Variable { name: i_var, .. } = &arguments[0] else {
        return None;
    };
    if i_var != loop_var {
        return None;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = &arguments[1]
    else {
        return None;
    };
    let ASTNode::Variable { name: left_var, .. } = left.as_ref() else {
        return None;
    };
    if left_var != loop_var {
        return None;
    }
    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            ..
        }
    ) {
        return None;
    }
    Some(s_var.clone())
}

fn extract_indexof_expr(expr: &ASTNode) -> Option<(String, String)> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr
    else {
        return None;
    };
    if method != "indexOf" || arguments.len() != 1 {
        return None;
    }
    let ASTNode::Variable { name: digits_var, .. } = object.as_ref() else {
        return None;
    };
    let ASTNode::Variable { name: ch_var, .. } = &arguments[0] else {
        return None;
    };
    Some((digits_var.clone(), ch_var.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn lit_str(value: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(value.to_string()),
            span: Span::unknown(),
        }
    }

    fn local(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(value))],
            span: Span::unknown(),
        }
    }

    fn assign(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(name)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn add(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn less(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn eq(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn or(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn substring(obj: &str, i_var: &str, step: i64) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(v(obj)),
            method: "substring".to_string(),
            arguments: vec![v(i_var), add(v(i_var), lit_int(step))],
            span: Span::unknown(),
        }
    }

    fn index_of(obj: &str, arg: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(v(obj)),
            method: "indexOf".to_string(),
            arguments: vec![v(arg)],
            span: Span::unknown(),
        }
    }

    fn length_call(obj: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(v(obj)),
            method: "length".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }
    }

    #[test]
    fn loopbodylocal_facts_detect_trim_seg() {
        let condition = less(v("i"), length_call("s"));
        let body = vec![
            local("seg", substring("s", "i", 1)),
            ASTNode::If {
                condition: Box::new(or(eq(v("seg"), lit_str(" ")), eq(v("seg"), lit_str("\t")))),
                then_body: vec![ASTNode::Break { span: Span::unknown() }],
                else_body: None,
                span: Span::unknown(),
            },
            assign("i", add(v("i"), lit_int(1))),
        ];

        let facts = try_extract_pattern2_loopbodylocal_facts(&condition, &body).expect("Ok");
        let facts = facts.expect("Some facts");
        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.loopbodylocal_var, "seg");
        assert!(facts.break_uses_loopbodylocal);
        match facts.shape {
            LoopBodyLocalShape::TrimSeg { s_var, i_var } => {
                assert_eq!(s_var, "s");
                assert_eq!(i_var, "i");
            }
            other => panic!("expected TrimSeg, got {:?}", other),
        }
    }

    #[test]
    fn loopbodylocal_facts_detect_digit_pos() {
        let condition = less(v("p"), length_call("s"));
        let body = vec![
            local("ch", substring("s", "p", 1)),
            local("digit_pos", index_of("digits", "ch")),
            ASTNode::If {
                condition: Box::new(less(v("digit_pos"), lit_int(0))),
                then_body: vec![ASTNode::Break { span: Span::unknown() }],
                else_body: None,
                span: Span::unknown(),
            },
            assign("p", add(v("p"), lit_int(1))),
        ];

        let facts = try_extract_pattern2_loopbodylocal_facts(&condition, &body).expect("Ok");
        let facts = facts.expect("Some facts");
        assert_eq!(facts.loop_var, "p");
        assert_eq!(facts.loopbodylocal_var, "digit_pos");
        assert!(facts.break_uses_loopbodylocal);
        match facts.shape {
            LoopBodyLocalShape::DigitPos { digits_var, ch_var } => {
                assert_eq!(digits_var, "digits");
                assert_eq!(ch_var, "ch");
            }
            other => panic!("expected DigitPos, got {:?}", other),
        }
    }

    #[test]
    fn loopbodylocal_facts_none_when_substring_step_not_one() {
        let condition = less(v("i"), length_call("s"));
        let body = vec![
            local("seg", substring("s", "i", 2)),
            ASTNode::If {
                condition: Box::new(or(eq(v("seg"), lit_str(" ")), eq(v("seg"), lit_str("\t")))),
                then_body: vec![ASTNode::Break { span: Span::unknown() }],
                else_body: None,
                span: Span::unknown(),
            },
        ];

        let facts = try_extract_pattern2_loopbodylocal_facts(&condition, &body).expect("Ok");
        assert!(facts.is_none());
    }
}
