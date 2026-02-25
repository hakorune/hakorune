//! Phase 29ap P10: Pattern6NestedMinimalFacts (Facts SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::facts::pattern9_accum_const_loop_facts::try_extract_pattern9_accum_const_loop_facts;
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
    scan_condition_observation, ConditionShape, StepShape,
};
use crate::mir::builder::control_flow::plan::facts::loop_condition_shape::try_extract_condition_shape;
use crate::mir::builder::control_flow::plan::facts::loop_step_shape::try_extract_step_shape;
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern6NestedMinimalFacts {
    pub outer_loop_var: String,
    pub outer_condition: ASTNode,
    pub outer_increment: ASTNode,
    pub inner_loop_var: String,
    pub inner_condition: ASTNode,
    pub inner_increment: ASTNode,
    pub acc_var: String,
    pub acc_update: ASTNode,
    pub inner_init_lit: i64,
}

pub(in crate::mir::builder) fn try_extract_pattern6_nested_minimal_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<Pattern6NestedMinimalFacts>, Freeze> {
    let Some(outer_loop_var) = extract_loop_var_for_subset(condition) else {
        return Ok(None);
    };

    let (inner_idx, inner_loop) = match find_single_inner_loop(body) {
        Some(loop_pair) => loop_pair,
        None => return Ok(None),
    };

    let ASTNode::Loop {
        condition: inner_condition,
        body: inner_body,
        ..
    } = inner_loop
    else {
        return Ok(None);
    };

    let inner_condition_shape =
        try_extract_condition_shape(inner_condition)?.unwrap_or(ConditionShape::Unknown);
    let inner_step_shape =
        try_extract_step_shape(inner_body)?.unwrap_or(StepShape::Unknown);
    let inner_observation = scan_condition_observation(&inner_condition_shape, &inner_step_shape);
    let Some(inner_facts) =
        try_extract_pattern9_accum_const_loop_facts(inner_condition, inner_body, &inner_observation)?
    else {
        return Ok(None);
    };

    if inner_facts.loop_var == outer_loop_var
        || inner_facts.acc_var == outer_loop_var
        || inner_facts.acc_var == inner_facts.loop_var
    {
        return Ok(None);
    }

    let Some(inner_step) =
        extract_increment_step_one(&inner_facts.loop_increment, &inner_facts.loop_var)
    else {
        return Ok(None);
    };

    let Some(acc_step) =
        extract_accum_add_const(&inner_facts.acc_update, &inner_facts.acc_var)
    else {
        return Ok(None);
    };

    if acc_step != 1 {
        return Ok(None);
    }

    let (inner_init_lit, outer_increment) = match scan_outer_body(
        body,
        inner_idx,
        &outer_loop_var,
        &inner_facts.loop_var,
    ) {
        Some(values) => values,
        None => return Ok(None),
    };

    if inner_init_lit != 0 {
        return Ok(None);
    }

    if extract_increment_step_one(&outer_increment, &outer_loop_var).is_none() {
        return Ok(None);
    }

    Ok(Some(Pattern6NestedMinimalFacts {
        outer_loop_var,
        outer_condition: condition.clone(),
        outer_increment,
        inner_loop_var: inner_facts.loop_var,
        inner_condition: inner_facts.condition,
        inner_increment: inner_step,
        acc_var: inner_facts.acc_var,
        acc_update: inner_facts.acc_update,
        inner_init_lit,
    }))
}

fn find_single_inner_loop(body: &[ASTNode]) -> Option<(usize, &ASTNode)> {
    let mut found = None;
    for (idx, stmt) in body.iter().enumerate() {
        if matches!(stmt, ASTNode::Loop { .. }) {
            if found.is_some() {
                return None;
            }
            found = Some((idx, stmt));
        }
    }
    found
}

fn scan_outer_body(
    body: &[ASTNode],
    inner_idx: usize,
    outer_loop_var: &str,
    inner_loop_var: &str,
) -> Option<(i64, ASTNode)> {
    let mut inner_init_lit = None;
    let mut outer_increment = None;
    let mut outer_increment_idx = None;

    for (idx, stmt) in body.iter().enumerate() {
        if matches!(stmt, ASTNode::Loop { .. }) {
            if idx != inner_idx {
                return None;
            }
            continue;
        }

        match stmt {
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                if variables.len() != 1 || variables[0] != inner_loop_var {
                    return None;
                }
                if idx > inner_idx {
                    return None;
                }
                if let Some(Some(init)) = initial_values.get(0) {
                    let lit = extract_int_literal(init)?;
                    if inner_init_lit.replace(lit).is_some() {
                        return None;
                    }
                }
            }
            ASTNode::Assignment { target, value, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return None;
                };
                if name == inner_loop_var {
                    if idx > inner_idx {
                        return None;
                    }
                    let lit = extract_int_literal(value)?;
                    if inner_init_lit.replace(lit).is_some() {
                        return None;
                    }
                } else if name == outer_loop_var {
                    outer_increment_idx = Some(idx);
                    outer_increment = Some(value.as_ref().clone());
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }

    let inner_init_lit = inner_init_lit?;
    let outer_increment = outer_increment?;
    let outer_increment_idx = outer_increment_idx?;

    if outer_increment_idx <= inner_idx {
        return None;
    }
    if outer_increment_idx + 1 != body.len() {
        return None;
    }

    Some((inner_init_lit, outer_increment))
}

fn extract_loop_var_for_subset(condition: &ASTNode) -> Option<String> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };

    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };

    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(_),
            ..
        }
    ) {
        return None;
    }

    Some(name.clone())
}

fn extract_int_literal(node: &ASTNode) -> Option<i64> {
    match node {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => Some(*value),
        _ => None,
    }
}

fn extract_increment_step_one(value: &ASTNode, loop_var: &str) -> Option<ASTNode> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value
    else {
        return None;
    };

    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };
    if name != loop_var {
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

    Some(value.clone())
}

fn extract_accum_add_const(update: &ASTNode, acc_var: &str) -> Option<i64> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = update
    else {
        return None;
    };

    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };
    if name != acc_var {
        return None;
    }

    extract_int_literal(right)
}

#[cfg(test)]
mod tests {
    use super::try_extract_pattern6_nested_minimal_facts;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

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

    fn condition_lt(loop_var: &str, bound: i64) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v(loop_var)),
            right: Box::new(lit_int(bound)),
            span: Span::unknown(),
        }
    }

    fn increment(loop_var: &str, step: i64) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(loop_var)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(loop_var)),
                right: Box::new(lit_int(step)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn accum_const(acc_var: &str, step: i64) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(acc_var)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(acc_var)),
                right: Box::new(lit_int(step)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    #[test]
    fn facts_extracts_nested_minimal_subset() {
        let inner_loop = ASTNode::Loop {
            condition: Box::new(condition_lt("j", 3)),
            body: vec![accum_const("sum", 1), increment("j", 1)],
            span: Span::unknown(),
        };
        let body = vec![
            ASTNode::Local {
                variables: vec!["j".to_string()],
                initial_values: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(v("j")),
                value: Box::new(lit_int(0)),
                span: Span::unknown(),
            },
            inner_loop,
            increment("i", 1),
        ];
        let condition = condition_lt("i", 3);

        let facts =
            try_extract_pattern6_nested_minimal_facts(&condition, &body).expect("Ok");
        let facts = facts.expect("Some");

        assert_eq!(facts.outer_loop_var, "i");
        assert_eq!(facts.inner_loop_var, "j");
        assert_eq!(facts.acc_var, "sum");
        assert_eq!(facts.inner_init_lit, 0);
    }

    #[test]
    fn facts_rejects_missing_inner_init() {
        let inner_loop = ASTNode::Loop {
            condition: Box::new(condition_lt("j", 3)),
            body: vec![accum_const("sum", 1), increment("j", 1)],
            span: Span::unknown(),
        };
        let body = vec![inner_loop, increment("i", 1)];
        let condition = condition_lt("i", 3);

        let facts =
            try_extract_pattern6_nested_minimal_facts(&condition, &body).expect("Ok");
        assert!(facts.is_none());
    }
}
