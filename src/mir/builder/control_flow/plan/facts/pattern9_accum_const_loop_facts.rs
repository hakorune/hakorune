//! Phase 29aj P8: Pattern9AccumConstLoopFacts (Facts SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    extract_loop_increment_plan, has_break_statement, has_continue_statement, has_if_else_statement,
};
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
    loop_var_from_profile, step_delta_from_profile, ConditionShape, ScanConditionObservation,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::policies::CondProfile;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern9AccumConstLoopFacts {
    pub loop_var: String,
    pub acc_var: String,
    pub condition: ASTNode,
    pub acc_update: ASTNode,
    pub loop_increment: ASTNode,
    pub cond_profile: CondProfile,
}

pub(in crate::mir::builder) fn try_extract_pattern9_accum_const_loop_facts(
    condition: &ASTNode,
    body: &[ASTNode],
    observation: &ScanConditionObservation,
) -> Result<Option<Pattern9AccumConstLoopFacts>, Freeze> {
    let _ = step_delta_from_profile(&observation.cond_profile);
    let ConditionShape::VarLessLiteral { .. } = observation.condition_shape else {
        return Ok(None);
    };
    let Some(loop_var) = loop_var_from_profile(&observation.cond_profile) else {
        return Ok(None);
    };

    if has_break_statement(body) || has_continue_statement(body) || has_if_else_statement(body) {
        return Ok(None);
    }

    if body.len() != 2 {
        return Ok(None);
    }

    let (acc_var, acc_update) = match extract_accum_const_update(&body[0], &loop_var) {
        Some(values) => values,
        None => return Ok(None),
    };

    let loop_increment = match extract_loop_increment_plan(&body[1..], &loop_var) {
        Ok(Some(inc)) => inc,
        _ => return Ok(None),
    };

    Ok(Some(Pattern9AccumConstLoopFacts {
        loop_var,
        acc_var,
        condition: condition.clone(),
        acc_update,
        loop_increment,
        cond_profile: observation.cond_profile.clone(),
    }))
}

fn extract_accum_const_update(
    stmt: &ASTNode,
    loop_var: &str,
) -> Option<(String, ASTNode)> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    let ASTNode::Variable { name: acc_var, .. } = target.as_ref() else {
        return None;
    };
    if acc_var == loop_var {
        return None;
    }

    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };

    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == acc_var) {
        return None;
    }

    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(_),
            ..
        }
    ) {
        return None;
    }

    Some((acc_var.clone(), value.as_ref().clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
        scan_condition_observation, ConditionShape, StepShape,
    };

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

    fn accum_const(acc_var: &str, value: i64) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(acc_var)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(acc_var)),
                right: Box::new(lit_int(value)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn accum_var(acc_var: &str, rhs: &str) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(acc_var)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(acc_var)),
                right: Box::new(v(rhs)),
                span: Span::unknown(),
            }),
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

    fn if_else_stub() -> ASTNode {
        ASTNode::If {
            condition: Box::new(v("cond")),
            then_body: vec![ASTNode::Return {
                value: None,
                span: Span::unknown(),
            }],
            else_body: Some(vec![ASTNode::Return {
                value: None,
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        }
    }

    #[test]
    fn facts_extracts_pattern9_const_accum_success() {
        let condition = condition_lt("i", 3);
        let body = vec![accum_const("sum", 1), increment("i", 1)];
        let observation =
            scan_condition_observation(&ConditionShape::Unknown, &StepShape::Unknown);

        let facts =
            try_extract_pattern9_accum_const_loop_facts(&condition, &body, &observation)
                .expect("Ok");
        let facts = facts.expect("Some");

        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.acc_var, "sum");
    }

    #[test]
    fn facts_rejects_break_or_continue() {
        let condition = condition_lt("i", 3);
        let body = vec![
            ASTNode::Break { span: Span::unknown() },
            increment("i", 1),
        ];
        let observation =
            scan_condition_observation(&ConditionShape::Unknown, &StepShape::Unknown);

        let facts =
            try_extract_pattern9_accum_const_loop_facts(&condition, &body, &observation)
                .expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn facts_rejects_if_else() {
        let condition = condition_lt("i", 3);
        let body = vec![if_else_stub(), increment("i", 1)];
        let observation =
            scan_condition_observation(&ConditionShape::Unknown, &StepShape::Unknown);

        let facts =
            try_extract_pattern9_accum_const_loop_facts(&condition, &body, &observation)
                .expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn facts_rejects_var_accumulation() {
        let condition = condition_lt("i", 3);
        let body = vec![accum_var("sum", "i"), increment("i", 1)];
        let observation =
            scan_condition_observation(&ConditionShape::Unknown, &StepShape::Unknown);

        let facts =
            try_extract_pattern9_accum_const_loop_facts(&condition, &body, &observation)
                .expect("Ok");
        assert!(facts.is_none());
    }
}
